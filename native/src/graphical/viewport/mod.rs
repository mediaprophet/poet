//! Retained native Portal viewport. Derived state only; SourceSession owns edits.
mod camera;
mod highlight;
mod picking;
mod renderer;
pub use camera::{apply_gesture, camera_state, frame_bounds, ViewportGesture};
pub use picking::PartSelection;
pub use renderer::{RetainedViewport, ViewportBudget, ViewportCounters};

/// Coordinates refer to the square native RGBA attachment, after UI letterbox mapping.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportCommand {
    Redraw,
    FrameAll,
    FramePart {
        key: String,
    },
    Orbit {
        dx: f32,
        dy: f32,
    },
    Pan {
        dx: f32,
        dy: f32,
    },
    Zoom {
        steps: f32,
    },
    Pick {
        frame_id: u64,
        x: f32,
        y: f32,
    },
    ClearSelection,
    /// View-only skeletal step against an exact selected HMC resource.
    RigPreview { resource: crate::graphical::checkpoints::ResourceInspectionRequest, delta_seconds: f64 },
    /// Reload source geometry and discard disposable rig playback.
    RestoreSourceGeometry,
    /// View-only illumination/exposure; never writes material, source or world physics.
    Lighting {
        exposure_ev: f32,
        sun_direction: [f32; 3],
        sun_intensity: f32,
        ambient_intensity: f32,
        shadows: bool,
        screen_space_ao: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewLighting {
    pub exposure_ev: f32,
    pub sun_direction: [f32; 3],
    pub sun_intensity: f32,
    pub ambient_intensity: f32,
    pub shadows: bool,
    pub screen_space_ao: bool,
}
impl ViewLighting {
    pub fn existing_scene() -> Self {
        let settings = crate::graphical::assembly::AssemblyRenderSettings::for_camera(
            crate::graphical::preview::PreviewCamera {
                yaw: 0.0,
                pitch: 0.0,
                distance: 1.0,
                target: [0.0; 3],
            },
        );
        Self::from_settings(&settings)
    }
    /// Candidate studio viewing preset. Visual acceptance must be reviewed per content/profile.
    pub fn neutral_studio() -> Self {
        Self {
            exposure_ev: 1.0,
            sun_direction: [0.45, 0.8, -0.6],
            sun_intensity: 1.8,
            ambient_intensity: 0.5,
            shadows: true,
            screen_space_ao: true,
        }
    }
    pub fn command(self) -> ViewportCommand {
        preview_lighting(self)
    }
    /// Construct a pure `etalon_render::FrameCameraPod` for this view lighting at `aspect_ratio`.
    pub fn etalon_frame_camera(&self, aspect_ratio: f32) -> etalon_render::FrameCameraPod {
        let aspect = if aspect_ratio.is_finite() {
            aspect_ratio.max(0.05)
        } else {
            1.0
        };
        let cam = crate::graphical::preview::PreviewCamera {
            yaw: 0.0,
            pitch: 0.25,
            distance: 3.5,
            target: [0.0; 3],
        };
        let mut pod = etalon_camera_pod(cam, self.exposure_ev);
        pod.view_proj = etalon_render::spatial_math::camera::orbit_view_projection_with_near(
            cam.yaw,
            cam.pitch,
            cam.distance,
            cam.target,
            aspect,
            etalon_render::spatial_math::camera::CAMERA_NEAR_PLANE,
        );
        pod.inv_view_proj = poet_aether_ui::etalon_bridge::invert_mat4_col(pod.view_proj);
        pod.viewport_wh_near_far[0] = 512.0 * aspect;
        pod.viewport_wh_near_far[1] = 512.0;
        pod
    }
    pub fn validate(self) -> Result<(), String> {
        use qualia_core_db::render::output::{MAX_HDR_EXPOSURE_EV, MIN_HDR_EXPOSURE_EV};
        if !self.exposure_ev.is_finite()
            || !(MIN_HDR_EXPOSURE_EV..=MAX_HDR_EXPOSURE_EV).contains(&self.exposure_ev)
        {
            return Err("view lighting exposure exceeds canonical EV bounds".into());
        }
        let mut settings = crate::graphical::assembly::AssemblyRenderSettings::for_camera(
            crate::graphical::preview::PreviewCamera {
                yaw: 0.0,
                pitch: 0.0,
                distance: 1.0,
                target: [0.0; 3],
            },
        );
        self.apply_to(&mut settings);
        crate::graphical::assembly::validate_render_settings(&settings)
    }
    pub(crate) fn from_settings(s: &crate::graphical::assembly::AssemblyRenderSettings) -> Self {
        Self {
            exposure_ev: s.exposure_ev,
            sun_direction: s.sun_direction,
            sun_intensity: s.sun_intensity,
            ambient_intensity: s.ambient_intensity,
            shadows: s.shadows,
            screen_space_ao: s.screen_space_ao,
        }
    }
    pub(crate) fn apply_to(self, s: &mut crate::graphical::assembly::AssemblyRenderSettings) {
        s.exposure_ev = self.exposure_ev;
        s.sun_direction = self.sun_direction;
        s.sun_intensity = self.sun_intensity;
        s.ambient_intensity = self.ambient_intensity;
        s.shadows = self.shadows;
        s.screen_space_ao = self.screen_space_ao;
    }
    pub(crate) fn from_command(command: &ViewportCommand) -> Option<Self> {
        if let ViewportCommand::Lighting {
            exposure_ev,
            sun_direction,
            sun_intensity,
            ambient_intensity,
            shadows,
            screen_space_ao,
        } = command
        {
            Some(Self {
                exposure_ev: *exposure_ev,
                sun_direction: *sun_direction,
                sun_intensity: *sun_intensity,
                ambient_intensity: *ambient_intensity,
                shadows: *shadows,
                screen_space_ao: *screen_space_ao,
            })
        } else {
            None
        }
    }
}
pub fn preview_lighting(s: ViewLighting) -> ViewportCommand {
    ViewportCommand::Lighting {
        exposure_ev: s.exposure_ev,
        sun_direction: s.sun_direction,
        sun_intensity: s.sun_intensity,
        ambient_intensity: s.ambient_intensity,
        shadows: s.shadows,
        screen_space_ao: s.screen_space_ao,
    }
}

/// Build an `etalon_render::FrameCameraPod` from a `PreviewCamera` and `exposure_ev`.
pub fn etalon_camera_pod(
    camera: crate::graphical::preview::PreviewCamera,
    exposure_ev: f32,
) -> etalon_render::FrameCameraPod {
    use etalon_render::spatial_math::camera::{
        orbit_eye_position_target, orbit_view_projection_with_near, CAMERA_FAR_PLANE,
        CAMERA_NEAR_PLANE,
    };
    let view_proj = orbit_view_projection_with_near(
        camera.yaw,
        camera.pitch,
        camera.distance,
        camera.target,
        1.0,
        CAMERA_NEAR_PLANE,
    );
    let inv_view_proj = poet_aether_ui::etalon_bridge::invert_mat4_col(view_proj);
    let eye = orbit_eye_position_target(camera.yaw, camera.pitch, camera.distance, camera.target);
    let ev_scale = etalon_render::frame_quality::exposure_scale_from_ev(exposure_ev).unwrap_or(1.0);
    etalon_render::FrameCameraPod {
        view_proj,
        inv_view_proj,
        eye_pos: [eye[0], eye[1], eye[2], ev_scale],
        viewport_wh_near_far: [512.0, 512.0, CAMERA_NEAR_PLANE, CAMERA_FAR_PLANE],
    }
}

#[cfg(test)]
pub(crate) mod tests;
