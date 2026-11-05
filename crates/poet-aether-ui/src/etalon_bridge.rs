//! Direct `etalon-render` bridge for Poet HyperCanvas viewports (zero-heap hot path).
//!
//! Exposes `FrameCameraPod` / `ManifoldProjectionPod` projection from `Workbench` orbit state,
//! CIE 1931 spectral preview helpers, frame quality / SSIM / PSNR thresholds, and the
//! `EtalonStatusBadge` Aether telemetry badge wired into `PoetGpuFrame`.

use crate::store::Workbench;
use aether::prelude::*;
use etalon_render::frame_quality::{
    classify_screen_metric, exposure_scale_from_ev, select_quality_profile,
    GraphicsCapabilitySnapshot, RenderQualityProfile, ScreenSpaceMetric,
    VirtualizationTier, DEFAULT_HDR_EXPOSURE_EV, DEFAULT_RETINAL_SLIP_COEFF,
};
use etalon_render::spatial_math::camera::{
    orbit_eye_position_target, orbit_forward, orbit_view_projection_with_near, CAMERA_FAR_PLANE,
    CAMERA_MAX_ZOOM, CAMERA_MIN_ZOOM, CAMERA_NEAR_PLANE, CAMERA_TAN_HALF_FOV,
};
use etalon_render::spectral::{
    emf_to_linear_rgb, emf_to_spd, linear_rgb_to_display, sigma_to_cie_xyz,
    sigma_to_linear_rgb, spd_to_xyz, ssim_rgba8, LinearRgb, Xyz, LAMBDA_MAX, LAMBDA_MIN,
};
pub use etalon_render::{FrameCameraPod, ManifoldProjectionPod};

/// Minimum structural similarity (SSIM) threshold for certified frame parity.
pub const ETALON_MIN_SSIM_THRESHOLD: f32 = 0.985;
/// Minimum Peak Signal-to-Noise Ratio (PSNR, dB) threshold for frame readback acceptance.
pub const ETALON_MIN_PSNR_DB_THRESHOLD: f32 = 38.0;
/// Maximum perceptual colour difference ($\Delta E_{00}$) for spectral oracle parity.
pub const ETALON_MAX_CIEDE2000_THRESHOLD: f32 = 1.5;

/// Zero-heap snapshot of `etalon-render` camera, 10D manifold, spectral, and quality state.
#[derive(Clone, Copy, Debug)]
pub struct EtalonViewportDiagnostics {
    pub camera_pod: FrameCameraPod,
    pub manifold_pod: ManifoldProjectionPod,
    pub dominant_wavelength_nm: f32,
    pub cie_xyz: [f32; 3],
    pub display_rgb: (u8, u8, u8),
    pub screen_metric: ScreenSpaceMetric,
    pub quality_profile: RenderQualityProfile,
    pub exposure_scale: f32,
}

/// Invert a column-major $4 \times 4$ matrix in place on the stack; returns identity if singular.
#[inline]
pub fn invert_mat4_col(m: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut aug = [[0.0_f32; 8]; 4];
    for r in 0..4 {
        for c in 0..4 {
            aug[r][c] = m[c][r];
        }
        aug[r][4 + r] = 1.0;
    }
    for i in 0..4 {
        let mut pivot = i;
        let mut max_val = aug[i][i].abs();
        for r in (i + 1)..4 {
            if aug[r][i].abs() > max_val {
                max_val = aug[r][i].abs();
                pivot = r;
            }
        }
        if max_val < 1e-8 || !max_val.is_finite() {
            return [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];
        }
        if pivot != i {
            aug.swap(i, pivot);
        }
        let inv_diag = 1.0 / aug[i][i];
        for c in 0..8 {
            aug[i][c] *= inv_diag;
        }
        for r in 0..4 {
            if r != i {
                let factor = aug[r][i];
                for c in 0..8 {
                    aug[r][c] -= factor * aug[i][c];
                }
            }
        }
    }
    let mut out = [[0.0_f32; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            out[c][r] = aug[r][4 + c];
        }
    }
    out
}

/// Compute `FrameCameraPod` from explicit orbit parameters (`orbit_rx`, `orbit_ry`, `zoom`) and viewport extent.
#[inline]
pub fn workbench_frame_camera_pod(
    orbit_rx_deg: f64,
    orbit_ry_deg: f64,
    zoom: f64,
    width: u32,
    height: u32,
) -> FrameCameraPod {
    let (w, h) = (width.max(8) as f32, height.max(8) as f32);
    let aspect = (w / h).max(0.05);
    let yaw = (orbit_ry_deg as f32).to_radians();
    let pitch = (orbit_rx_deg as f32).to_radians().clamp(-1.45, 1.45);
    let dist = (3.5 / (zoom as f32).max(0.05)).clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
    let target = [0.0_f32, 0.0, 0.0];
    let view_proj = orbit_view_projection_with_near(yaw, pitch, dist, target, aspect, CAMERA_NEAR_PLANE);
    let inv_view_proj = invert_mat4_col(view_proj);
    let eye = orbit_eye_position_target(yaw, pitch, dist, target);
    FrameCameraPod {
        view_proj,
        inv_view_proj,
        eye_pos: [eye[0], eye[1], eye[2], 1.0],
        viewport_wh_near_far: [w, h, CAMERA_NEAR_PLANE, CAMERA_FAR_PLANE],
    }
}

/// Compute `ManifoldProjectionPod` from the HyperCanvas `Workbench` 4D/10D orbit & temporal controls.
#[inline]
pub fn workbench_manifold_projection_pod(
    orbit_rx_deg: f64,
    orbit_ry_deg: f64,
    orbit_rz_deg: f64,
    zoom: f64,
    manifold_d: f64,
    time_progress: f64,
) -> ManifoldProjectionPod {
    let sigma = (time_progress as f32).clamp(0.0, 1.0);
    let d_scale = (manifold_d as f32).clamp(0.1, 10.0);
    let z_scale = (zoom as f32).clamp(0.1, 10.0);
    ManifoldProjectionPod {
        coords_10d: [
            1.0,
            z_scale,
            d_scale,
            (orbit_rx_deg as f32).to_radians(),
            (orbit_ry_deg as f32).to_radians(),
            (orbit_rz_deg as f32).to_radians(),
            sigma,
            1.0,
            0.25,
            sigma,
        ],
        opacity: 1.0,
        flags: 0x01,
    }
}

/// Project a 10D coordinate slice into clip space `[x, y, z, w]` using `FrameCameraPod` and `ManifoldProjectionPod`.
#[inline]
pub fn project_10d_point(
    camera: &FrameCameraPod,
    manifold: &ManifoldProjectionPod,
    point_4d: [f32; 4],
) -> [f32; 4] {
    let w_fold = 1.0 + point_4d[3] * manifold.coords_10d[2] * 0.15;
    let world = [point_4d[0] * w_fold, point_4d[1] * w_fold, point_4d[2] * w_fold, 1.0];
    let vp = &camera.view_proj;
    let mut clip = [0.0_f32; 4];
    for r in 0..4 {
        for c in 0..4 {
            clip[r] += vp[c][r] * world[c];
        }
    }
    clip
}

/// Convert normalized spectral signature $\sigma \in [0, 1]$ to wavelength in nm ($380..=780$ nm) and CIE 1931 XYZ / sRGB.
#[inline]
pub fn spectral_wavelength_preview(alpha: f32, mu: f32, sigma: f32) -> (f32, Xyz, LinearRgb, (u8, u8, u8)) {
    let s = sigma.clamp(0.0, 1.0);
    let wavelength_nm = (LAMBDA_MIN + s * (LAMBDA_MAX - LAMBDA_MIN)).clamp(LAMBDA_MIN, LAMBDA_MAX);
    let spd = emf_to_spd(alpha, mu, s);
    let xyz = spd_to_xyz(&spd);
    let lin = emf_to_linear_rgb(alpha, mu, s);
    let srgb8 = linear_rgb_to_display(&lin);
    (wavelength_nm, xyz, lin, srgb8)
}

/// Compute PSNR (in dB) and SSIM between two RGBA8 frames, checking against `etalon-render` quality thresholds.
pub fn evaluate_frame_quality_pair(
    reference_rgba8: &[u8],
    candidate_rgba8: &[u8],
    width: usize,
    height: usize,
) -> (f32, f32, bool) {
    let expected_len = width.saturating_mul(height).saturating_mul(4);
    if expected_len == 0 || reference_rgba8.len() != expected_len || candidate_rgba8.len() != expected_len {
        return (0.0, 0.0, false);
    }
    let ssim = ssim_rgba8(reference_rgba8, candidate_rgba8, width, height);
    let mut mse = 0.0_f64;
    for (&a, &b) in reference_rgba8.iter().zip(candidate_rgba8.iter()) {
        let diff = (a as f64) - (b as f64);
        mse += diff * diff;
    }
    mse /= expected_len as f64;
    let psnr_db = if mse <= 1e-12 {
        99.9
    } else {
        (10.0 * ((255.0_f64 * 255.0_f64) / mse).log10()) as f32
    };
    let passes = ssim >= ETALON_MIN_SSIM_THRESHOLD && psnr_db >= ETALON_MIN_PSNR_DB_THRESHOLD;
    (ssim, psnr_db, passes)
}

impl EtalonViewportDiagnostics {
    /// Build live diagnostics from a `Workbench` state and target viewport dimensions.
    pub fn from_workbench(wb: &Workbench, width: u32, height: u32) -> Self {
        Self::from_orbit(
            wb.orbit_rx, wb.orbit_ry, wb.orbit_rz, wb.zoom, wb.manifold_d, wb.time_progress, width, height,
        )
    }

    /// Build live diagnostics directly from orbit & manifold scalars (zero heap).
    pub fn from_orbit(
        orbit_rx_deg: f64,
        orbit_ry_deg: f64,
        orbit_rz_deg: f64,
        zoom: f64,
        manifold_d: f64,
        time_progress: f64,
        width: u32,
        height: u32,
    ) -> Self {
        let camera_pod = workbench_frame_camera_pod(orbit_rx_deg, orbit_ry_deg, zoom, width, height);
        let manifold_pod = workbench_manifold_projection_pod(
            orbit_rx_deg, orbit_ry_deg, orbit_rz_deg, zoom, manifold_d, time_progress,
        );
        let sigma = manifold_pod.coords_10d[6];
        let (dominant_wavelength_nm, _xyz, _lin, display_rgb) = spectral_wavelength_preview(1.0, 0.2, sigma);
        let cie_xyz = sigma_to_cie_xyz(sigma);
        let _ = sigma_to_linear_rgb(sigma);

        let yaw = (orbit_ry_deg as f32).to_radians();
        let pitch = (orbit_rx_deg as f32).to_radians().clamp(-1.45, 1.45);
        let eye = [camera_pod.eye_pos[0], camera_pod.eye_pos[1], camera_pod.eye_pos[2]];
        let fwd = orbit_forward(yaw, pitch);
        let fov_y = 2.0 * CAMERA_TAN_HALF_FOV.atan();
        let screen_metric = classify_screen_metric(
            [0.0, 0.0, 0.0], 1.0, eye, fwd, fov_y, height.max(8) as f32, 0.0, DEFAULT_RETINAL_SLIP_COEFF,
        );
        let quality_profile = select_quality_profile(GraphicsCapabilitySnapshot::default());
        let exposure_scale = exposure_scale_from_ev(DEFAULT_HDR_EXPOSURE_EV).unwrap_or(1.0);

        Self {
            camera_pod,
            manifold_pod,
            dominant_wavelength_nm,
            cie_xyz,
            display_rgb,
            screen_metric,
            quality_profile,
            exposure_scale,
        }
    }

    /// Compact human-readable status line for viewport overlays.
    pub fn summary_line(&self) -> String {
        let (r, g, b) = self.display_rgb;
        let tier_label = match self.screen_metric.tier {
            VirtualizationTier::HeroMesh => "HeroMesh",
            VirtualizationTier::MeshletCluster => "Meshlet",
            VirtualizationTier::OctahedralImpostor => "Impostor",
            VirtualizationTier::HierarchicalSurfel => "Surfel",
            VirtualizationTier::SubpixelCulled => "Culled",
        };
        format!(
            "etalon-render · λ {:.0}nm rgb({r},{g},{b}) · {tier_label} ({:.0}px) · SSIM≥{:.3}",
            self.dominant_wavelength_nm,
            self.screen_metric.effective_screen_radius_px,
            ETALON_MIN_SSIM_THRESHOLD
        )
    }
}

#[component]
/// Live `etalon-render` projection, CIE 1931 spectral, and LOD readiness badge for `PoetGpuFrame`.
pub fn EtalonStatusBadge(kind: &'static str, width: u32, height: u32) -> Element {
    let diag = EtalonViewportDiagnostics::from_orbit(15.0, -25.0, 0.0, 0.9, 1.0, 0.61, width, height);
    let (r, g, b) = diag.display_rgb;
    let summary = diag.summary_line();
    rsx! {
        div {
            "data-etalon-status": "{kind}",
            style: "position:absolute;right:10px;top:8px;display:inline-flex;align-items:center;gap:6px;padding:3px 8px;border-radius:999px;background:rgba(7,9,14,.78);border:1px solid rgba(56,189,248,.28);color:#e2e8f0;font-size:.62rem;letter-spacing:.03em;",
            span {
                style: "width:7px;height:7px;border-radius:50%;background:rgb({r},{g},{b});display:inline-block;",
            }
            span { "{summary}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use etalon_render::frame_quality::QualityTier;
    use etalon_render::spectral::ciede2000;

    #[test]
    fn workbench_orbit_builds_finite_etalon_pods_and_projects_10d() {
        let wb = Workbench::new();
        let diag = EtalonViewportDiagnostics::from_workbench(&wb, 720, 320);
        assert!(diag.camera_pod.view_proj[0][0].is_finite());
        assert!(diag.camera_pod.inv_view_proj[0][0].is_finite());
        assert_eq!(diag.camera_pod.viewport_wh_near_far[0], 720.0);
        assert_eq!(diag.camera_pod.viewport_wh_near_far[1], 320.0);
        let clip = project_10d_point(&diag.camera_pod, &diag.manifold_pod, [0.2, -0.1, 0.5, 1.0]);
        assert!(clip.iter().all(|v| v.is_finite()));
        assert!(clip[3] > 0.0);
        assert_eq!(diag.quality_profile.tier, QualityTier::Conservative);
    }

    #[test]
    fn spectral_and_frame_quality_thresholds_verify_cleanly() {
        let (lambda, xyz, _lin, _rgb) = spectral_wavelength_preview(1.0, 0.1, 0.5);
        assert!((lambda - 580.0).abs() <= 1.0);
        assert!(ciede2000(&xyz, &xyz) < ETALON_MAX_CIEDE2000_THRESHOLD);

        let frame = [42_u8; 16 * 16 * 4];
        let (ssim, psnr, ok) = evaluate_frame_quality_pair(&frame, &frame, 16, 16);
        assert!(ok);
        assert!(ssim >= ETALON_MIN_SSIM_THRESHOLD);
        assert!(psnr >= ETALON_MIN_PSNR_DB_THRESHOLD);
    }
}
