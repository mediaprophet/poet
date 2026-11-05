//! Webizen Poet Mobile WASM — Touch-First Viewport Profile & Packaged VibeScript Runner.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! Principal / inventor: Timothy Charles Holborn <timothy.holborn@gmail.com>
//! Assignment: COPYRIGHT.md  Licence: LICENSE (CC BY-NC-ND 4.0)

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub use poet_aether_ui::{
    ContainerKind, IconBadge, ManifoldId, PoetHarness, PoetWorkbench, RadialActionRing,
    RadialState, ToolboxId,
};

/// Mobile manifold dock layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MobileDockMode {
    /// Single-column stacked container flow with collapsible bottom tray (phones & narrow portrait).
    CompactSingleColumn,
    /// Bottom sheet drawer with split two-column stage (tablets & foldable landscape).
    BottomDrawer,
    /// Minimal floating pill bar for full-bleed radial canvas authoring.
    FloatingPill,
}

impl MobileDockMode {
    /// Canonical DOM attribute token for CSS / layout adaptation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompactSingleColumn => "compact-single-column",
            Self::BottomDrawer => "bottom-drawer",
            Self::FloatingPill => "floating-pill",
        }
    }
}

/// Touch, pinch-zoom, pan, and radial wheel gesture configuration for mobile surfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchGestureConfig {
    /// Enable two-finger pinch-to-zoom on the manifold stage.
    pub pinch_zoom_enabled: bool,
    /// Minimum allowed zoom scale on mobile viewports.
    pub min_zoom: f64,
    /// Maximum allowed zoom scale on mobile viewports.
    pub max_zoom: f64,
    /// Enable touch pan across the manifold canvas.
    pub pan_gesture_enabled: bool,
    /// Require two-finger pan when a single-column card scroll is active.
    pub single_column_two_finger_pan: bool,
    /// Long-press duration in milliseconds before summoning the 8-sector radial wheel.
    pub radial_long_press_ms: u32,
    /// Diameter in CSS pixels of the mobile radial action ring.
    pub radial_size_px: u32,
}

impl Default for TouchGestureConfig {
    fn default() -> Self {
        Self {
            pinch_zoom_enabled: true,
            min_zoom: 0.35,
            max_zoom: 3.0,
            pan_gesture_enabled: true,
            single_column_two_finger_pan: true,
            radial_long_press_ms: poet::browser::radial_gesture::LONG_PRESS_MS,
            radial_size_px: poet::browser::radial_gesture::RADIAL_SIZE_PX,
        }
    }
}

impl TouchGestureConfig {
    /// Clamp a raw pinch-zoom factor to the configured mobile bounds.
    pub fn clamp_zoom(&self, zoom: f64) -> f64 {
        if !self.pinch_zoom_enabled {
            return 1.0;
        }
        zoom.clamp(self.min_zoom, self.max_zoom)
    }

    /// Apply a touch pan delta `(dx, dy)` in screen pixels at the given zoom level.
    pub fn apply_pan_delta(&self, current: (f64, f64), delta: (f64, f64), zoom: f64) -> (f64, f64) {
        if !self.pan_gesture_enabled {
            return current;
        }
        let effective_zoom = self.clamp_zoom(zoom).max(0.1);
        (
            current.0 + delta.0 / effective_zoom,
            current.1 + delta.1 / effective_zoom,
        )
    }
}

/// Adaptive mobile viewport profile governing dock layout, touch gestures, and initial manifold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MobileViewportProfile {
    /// Physical or CSS viewport width in pixels.
    pub width_px: u32,
    /// Physical or CSS viewport height in pixels.
    pub height_px: u32,
    /// Device pixel ratio / HiDPI scale factor.
    pub scale_factor: f32,
    /// Classified device form factor (`Phone` or `Tablet`).
    pub device_type: poet::tool_chest::core::DeviceType,
    /// Active dock arrangement for this viewport.
    pub dock_mode: MobileDockMode,
    /// Touch & radial gesture configuration.
    pub touch_gestures: TouchGestureConfig,
    /// Default manifold ID opened on launch.
    pub default_manifold_id: String,
}

impl Default for MobileViewportProfile {
    fn default() -> Self {
        Self::for_phone()
    }
}

impl MobileViewportProfile {
    /// Compact portrait phone preset (390×844 @ 3x, compact single-column dock).
    pub fn for_phone() -> Self {
        Self {
            width_px: 390,
            height_px: 844,
            scale_factor: 3.0,
            device_type: poet::tool_chest::core::DeviceType::Phone,
            dock_mode: MobileDockMode::CompactSingleColumn,
            touch_gestures: TouchGestureConfig::default(),
            default_manifold_id: ManifoldId::Research.id().to_string(),
        }
    }

    /// Tablet preset (834×1194 @ 2x, bottom drawer dock).
    pub fn for_tablet() -> Self {
        Self {
            width_px: 834,
            height_px: 1194,
            scale_factor: 2.0,
            device_type: poet::tool_chest::core::DeviceType::Tablet,
            dock_mode: MobileDockMode::BottomDrawer,
            touch_gestures: TouchGestureConfig {
                single_column_two_finger_pan: false,
                ..TouchGestureConfig::default()
            },
            default_manifold_id: ManifoldId::Research.id().to_string(),
        }
    }

    /// Classify a viewport from CSS pixel dimensions and device pixel ratio.
    pub fn from_dimensions(width_px: u32, height_px: u32, scale_factor: f32) -> Self {
        let min_edge = width_px.min(height_px);
        if min_edge < 600 {
            let mut profile = Self::for_phone();
            profile.width_px = width_px;
            profile.height_px = height_px;
            profile.scale_factor = scale_factor.max(1.0);
            profile
        } else {
            let mut profile = Self::for_tablet();
            profile.width_px = width_px;
            profile.height_px = height_px;
            profile.scale_factor = scale_factor.max(1.0);
            profile
        }
    }

    /// Returns `true` when the viewport uses the compact single-column manifold dock mode.
    pub fn is_compact_single_column(&self) -> bool {
        self.dock_mode == MobileDockMode::CompactSingleColumn
    }

    /// Stamp mobile viewport attributes onto the document `<body>` element.
    pub fn apply_to_document(&self, document: &web_sys::Document) -> Result<(), String> {
        let body: web_sys::HtmlElement = document
            .body()
            .ok_or_else(|| "document.body() returned None".to_string())?;
        let el: &web_sys::Element = body.as_ref();
        el.set_attribute("data-poet-surface", "mobile-wasm")
            .map_err(|e| format!("set_attribute(data-poet-surface): {e:?}"))?;
        el.set_attribute("data-dock-mode", self.dock_mode.as_str())
            .map_err(|e| format!("set_attribute(data-dock-mode): {e:?}"))?;
        el.set_attribute("data-touch-first", "true")
            .map_err(|e| format!("set_attribute(data-touch-first): {e:?}"))?;
        el.set_attribute(
            "data-radial-long-press-ms",
            &self.touch_gestures.radial_long_press_ms.to_string(),
        )
        .map_err(|e| format!("set_attribute(data-radial-long-press-ms): {e:?}"))?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Packaged VibeScript Catalog & Runner
// ---------------------------------------------------------------------------

/// A packaged `.vibe` script bundled into the Poet Mobile WASM artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackagedVibeScript {
    /// Stable script identifier (e.g. `"mobile.aura_tray"`).
    pub id: &'static str,
    /// Virtual `.vibe` file path inside the mobile package.
    pub path: &'static str,
    /// Human-readable title.
    pub title: &'static str,
    /// VibeScript 0.1 source text.
    pub source: &'static str,
}

/// Mobile compact dock layout script in VibeScript 0.1.
pub const MOBILE_COMPACT_DOCK_VIBE: &str = r#"// Mobile Compact Single-Column Dock in VibeScript 0.1
let ui = {
  type: "dock_panel",
  id: "mobile-compact-dock",
  title: "Mobile Manifold Dock",
  badge: "Single-Column",
  collapsed: false,
  children: [
    {
      type: "subtray",
      title: "Viewport & Touch",
      badge: "Active",
      collapsed: false,
      children: [
        { type: "metric", label: "Dock Mode:", value: "compact-single-column", color: "var(--accent-cyan)" },
        { type: "metric", label: "Radial Trigger:", value: "550ms Long-Press", color: "var(--accent-emerald)" },
        { type: "metric", label: "Pinch Zoom:", value: "0.35x .. 3.00x", color: "var(--accent-cyan)" }
      ]
    },
    {
      type: "subtray",
      title: "Quick Actions",
      badge: "Touch",
      collapsed: false,
      children: [
        { type: "button", id: "btn-radial", label: "Open Radial Wheel", action: "open_radial_ring" },
        { type: "button", id: "btn-tidy", label: "Auto-Stack Cards", action: "auto_arrange_single_column" }
      ]
    }
  ]
};
return ui;
"#;

/// Mobile radial gesture diagnostic script in VibeScript 0.1.
pub const MOBILE_RADIAL_PROBE_VIBE: &str = r#"// Mobile Radial Wheel & Sentinel Probe in VibeScript 0.1
let probe = {
  type: "dock_panel",
  id: "mobile-radial-probe",
  title: "Radial & Sentinel Probe",
  badge: "42MB Cap",
  collapsed: false,
  children: [
    { type: "metric", label: "Host ABI:", value: "vibe-host-0.1", color: "var(--accent-emerald)" },
    { type: "metric", label: "Quin Layout:", value: "48-byte Super-Quin", color: "var(--accent-cyan)" }
  ]
};
return probe;
"#;

/// Return all packaged `.vibe` scripts bundled with `poet-mobile-wasm`.
pub fn packaged_vibe_scripts() -> Vec<PackagedVibeScript> {
    vec![
        PackagedVibeScript {
            id: "mobile.compact_dock",
            path: "scripts/mobile_compact_dock.vibe",
            title: "Mobile Compact Single-Column Dock",
            source: MOBILE_COMPACT_DOCK_VIBE,
        },
        PackagedVibeScript {
            id: "mobile.aura_tray",
            path: "scripts/aura_tray.vibe",
            title: "Aura Tray Schema & SHACL Monitor",
            source: poet::browser::vibe_ui::default_aura_tray_vibe_script(),
        },
        PackagedVibeScript {
            id: "mobile.radial_probe",
            path: "scripts/mobile_radial_probe.vibe",
            title: "Mobile Radial & Sentinel Probe",
            source: MOBILE_RADIAL_PROBE_VIBE,
        },
    ]
}

/// Look up a packaged `.vibe` script by its identifier or virtual path.
pub fn get_packaged_vibe_script(id_or_path: &str) -> Option<PackagedVibeScript> {
    packaged_vibe_scripts()
        .into_iter()
        .find(|script| script.id == id_or_path || script.path == id_or_path)
}

/// Result of executing a packaged `.vibe` script on the mobile runner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MobileVibeRunReceipt {
    /// Script identifier.
    pub script_id: String,
    /// Whether parsing, checking, and evaluation succeeded.
    pub ok: bool,
    /// Host ABI version stamp (`vibe-host-0.1`).
    pub host_version: String,
    /// Language version stamp.
    pub language_version: String,
    /// Evaluated top-level UI panel title when the script yields a `VibeUiNode`.
    pub panel_title: Option<String>,
    /// Number of child UI nodes produced by the script.
    pub child_count: usize,
    /// Diagnostic message if parsing or evaluation failed.
    pub diagnostic: Option<String>,
}

/// Validate and execute a packaged `.vibe` script by ID, returning a [`MobileVibeRunReceipt`].
pub fn run_packaged_vibe_script(id_or_path: &str) -> Result<MobileVibeRunReceipt, String> {
    let script = get_packaged_vibe_script(id_or_path)
        .ok_or_else(|| format!("unknown packaged .vibe script `{id_or_path}`"))?;
    run_vibe_source(script.id, script.source)
}

/// Validate and execute arbitrary `.vibe` source through the mobile runner.
pub fn run_vibe_source(script_id: &str, source: &str) -> Result<MobileVibeRunReceipt, String> {
    let report = poet::diagnose(source);
    if !report.valid {
        return Ok(MobileVibeRunReceipt {
            script_id: script_id.to_string(),
            ok: false,
            host_version: poet::host_version().to_string(),
            language_version: vibe::LANGUAGE_VERSION.to_string(),
            panel_title: None,
            child_count: 0,
            diagnostic: Some(report.to_json()),
        });
    }

    let node = poet::browser::vibe_ui::eval_vibe_ui_script(source)?;
    let (panel_title, child_count) = match &node {
        poet::browser::vibe_ui::VibeUiNode::DockPanel {
            title, children, ..
        }
        | poet::browser::vibe_ui::VibeUiNode::Subtray {
            title, children, ..
        } => (Some(title.clone()), children.len()),
        poet::browser::vibe_ui::VibeUiNode::Container { children, .. } => (None, children.len()),
        _ => (None, 0),
    };

    Ok(MobileVibeRunReceipt {
        script_id: script_id.to_string(),
        ok: true,
        host_version: poet::host_version().to_string(),
        language_version: vibe::LANGUAGE_VERSION.to_string(),
        panel_title,
        child_count,
        diagnostic: None,
    })
}

// ---------------------------------------------------------------------------
// WASM Entrypoint
// ---------------------------------------------------------------------------

/// Mobile-adaptive WASM entrypoint for Poet Mobile (`poet-mobile-wasm`).
#[wasm_bindgen]
pub fn start_mobile_poet() {
    if let Some(window) = web_sys::window() {
        let width = window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(390.0) as u32;
        let height = window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(844.0) as u32;
        let dpr = window.device_pixel_ratio() as f32;
        let profile = MobileViewportProfile::from_dimensions(width, height, dpr);
        if let Some(document) = window.document() {
            let _ = profile.apply_to_document(&document);
        }
    }
    poet::start();
}

/// Exported JSON helper returning the default phone viewport profile for host inspection.
#[wasm_bindgen]
pub fn mobile_viewport_profile_json(width_px: u32, height_px: u32, scale_factor: f32) -> String {
    let profile = MobileViewportProfile::from_dimensions(width_px, height_px, scale_factor);
    serde_json::to_string(&profile).unwrap_or_else(|_| "{}".to_string())
}

// ---------------------------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_viewport_profile_uses_compact_single_column_dock() {
        let profile = MobileViewportProfile::from_dimensions(390, 844, 3.0);
        assert_eq!(
            profile.device_type,
            poet::tool_chest::core::DeviceType::Phone
        );
        assert_eq!(profile.dock_mode, MobileDockMode::CompactSingleColumn);
        assert!(profile.is_compact_single_column());
        assert_eq!(profile.dock_mode.as_str(), "compact-single-column");
        assert_eq!(profile.default_manifold_id, ManifoldId::Research.id());
    }

    #[test]
    fn tablet_viewport_profile_uses_bottom_drawer_dock() {
        let profile = MobileViewportProfile::from_dimensions(834, 1194, 2.0);
        assert_eq!(
            profile.device_type,
            poet::tool_chest::core::DeviceType::Tablet
        );
        assert_eq!(profile.dock_mode, MobileDockMode::BottomDrawer);
        assert!(!profile.is_compact_single_column());
        assert!(!profile.touch_gestures.single_column_two_finger_pan);
    }

    #[test]
    fn pinch_zoom_and_pan_gesture_math_respects_bounds() {
        let cfg = TouchGestureConfig::default();
        assert_eq!(cfg.clamp_zoom(0.1), cfg.min_zoom);
        assert_eq!(cfg.clamp_zoom(1.25), 1.25);
        assert_eq!(cfg.clamp_zoom(9.0), cfg.max_zoom);

        let panned = cfg.apply_pan_delta((10.0, 20.0), (50.0, -30.0), 2.0);
        assert!((panned.0 - 35.0).abs() < 1e-6);
        assert!((panned.1 - 5.0).abs() < 1e-6);
    }

    #[test]
    fn packaged_vibe_scripts_are_accessible_and_evaluate_cleanly() {
        let scripts = packaged_vibe_scripts();
        assert_eq!(scripts.len(), 3);

        for script in &scripts {
            let fetched = get_packaged_vibe_script(script.id)
                .expect("packaged script should be resolvable by id");
            assert_eq!(fetched.path, script.path);

            let receipt = run_packaged_vibe_script(script.id)
                .expect("packaged .vibe script should execute without error");
            assert!(
                receipt.ok,
                "script `{}` failed diagnostics: {:?}",
                script.id, receipt.diagnostic
            );
            assert_eq!(receipt.host_version, poet::HOST_VERSION);
            assert!(receipt.child_count > 0);
        }

        let compact = run_packaged_vibe_script("scripts/mobile_compact_dock.vibe")
            .expect("lookup by virtual .vibe path should succeed");
        assert_eq!(
            compact.panel_title.as_deref(),
            Some("Mobile Manifold Dock")
        );
        assert_eq!(compact.child_count, 2);
    }
}
