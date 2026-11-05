//! HyperCanvas 3D CAD-Style Trackball & 4D XYZD Spatial Manifold Gizmo.
//!
//! Ported from `Canvas_Workbench/js/xyzd-gizmo.js` into a native Aether component.
//! Dual-mode gizmo:
//! - **ORBIT mode**: rotates the camera/canvas view (`orbit_rx`, `orbit_ry`, `orbit_rz`)
//!   and adjusts the 4th-dimensional manifold aperture (`manifold_d`).
//! - **POSITION mode**: translates the selected container's `x`, `y`, `z`, `d` in canvas space.

use super::kinds::DimMode;
use super::store::Workbench;
use dioxus::prelude::*;

const RAD: f64 = std::f64::consts::PI / 180.0;
const AXIS_RADIUS: f64 = 36.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GizmoMode {
    Orbit,
    Position,
}

#[derive(Clone, Copy, Debug)]
struct GizmoDrag {
    active: bool,
    start_client_x: f64,
    start_client_y: f64,
    start_rx: f64,
    start_ry: f64,
    start_pos_x: f64,
    start_pos_y: f64,
}

impl Default for GizmoDrag {
    fn default() -> Self {
        Self {
            active: false,
            start_client_x: 0.0,
            start_client_y: 0.0,
            start_rx: 15.0,
            start_ry: -25.0,
            start_pos_x: 0.0,
            start_pos_y: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectedAxis {
    pub key: &'static str,
    pub axis: &'static str,
    pub label: &'static str,
    pub color: &'static str,
    pub text_fill: &'static str,
    pub is_negative: bool,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
}

/// Rotate a 3D point `(px, py, pz)` by `(rx_deg, ry_deg)` in degrees.
pub fn rotate_point_3d(px: f64, py: f64, pz: f64, rx_deg: f64, ry_deg: f64) -> (f64, f64, f64) {
    let (sin_y, cos_y) = (ry_deg * RAD).sin_cos();
    let (sin_x, cos_x) = (rx_deg * RAD).sin_cos();
    let x = px * cos_y + pz * sin_y;
    let z = -px * sin_y + pz * cos_y;
    let y2 = py * cos_x - z * sin_x;
    let z2 = py * sin_x + z * cos_x;
    (x, y2, z2)
}

/// Project and depth-sort the 6 cardinal axes (`+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z`) back-to-front.
pub fn project_axes(rx_deg: f64, ry_deg: f64) -> [ProjectedAxis; 6] {
    let raw = [
        ([1.0, 0.0, 0.0], "X", "x", "X", "#ef4444", "#fff", false),
        ([-1.0, 0.0, 0.0], "NX", "x", "-X", "#ef4444", "#fff", true),
        ([0.0, 1.0, 0.0], "Y", "y", "Y", "#22c55e", "#fff", false),
        ([0.0, -1.0, 0.0], "NY", "y", "-Y", "#22c55e", "#fff", true),
        ([0.0, 0.0, 1.0], "Z", "z", "Z", "#06b6d4", "#000", false),
        ([0.0, 0.0, -1.0], "NZ", "z", "-Z", "#06b6d4", "#000", true),
    ];

    let mut out = raw.map(
        |(dir, key, axis, label, color, text_fill, is_negative)| {
            let (x, y, z) = rotate_point_3d(
                dir[0] * AXIS_RADIUS,
                dir[1] * AXIS_RADIUS,
                dir[2] * AXIS_RADIUS,
                rx_deg,
                ry_deg,
            );
            ProjectedAxis {
                key,
                axis,
                label,
                color,
                text_fill,
                is_negative,
                px: x,
                py: -y,
                pz: z,
            }
        },
    );

    out.sort_by(|a, b| a.pz.partial_cmp(&b.pz).unwrap_or(std::cmp::Ordering::Equal));
    out
}

#[component]
/// 4D `XYZD` Orbit/Position Trackball Gizmo HUD component mounted on `CanvasStage`.
pub fn XyzdGizmo(wb: Signal<Workbench>) -> Element {
    let mut mode = use_signal(|| GizmoMode::Orbit);
    let mut drag = use_signal(GizmoDrag::default);

    let w = wb();
    let is_orbit = mode() == GizmoMode::Orbit;
    let is_dragging = drag().active;
    let axes = project_axes(w.orbit_rx, w.orbit_ry);
    let d_ring_r = (48.0 + w.manifold_d * 6.0).round();
    let d_slider_val = (w.manifold_d * 100.0).round() as i32;
    let selected_node = w.selected.as_ref().and_then(|id| w.node(id)).cloned();
    let pos_x_val = selected_node.as_ref().map(|n| n.x.round() as i32).unwrap_or(0);
    let pos_y_val = selected_node.as_ref().map(|n| n.y.round() as i32).unwrap_or(0);
    let pos_z_val = selected_node.as_ref().map(|n| n.z.round() as i32).unwrap_or(0);
    let pos_d_val = selected_node
        .as_ref()
        .map(|n| format!("{:.1}", n.d))
        .unwrap_or_else(|| "1.0".into());

    let viewport_class = match (is_orbit, is_dragging) {
        (true, false) => "gizmo-viewport",
        (true, true) => "gizmo-viewport dragging",
        (false, false) => "gizmo-viewport position-mode",
        (false, true) => "gizmo-viewport position-mode dragging",
    };

    rsx! {
        div {
            class: "xyzd-controller-hud",
            id: "xyzd-gizmo-hud",
            onmousedown: move |e| e.stop_propagation(),
            div { class: "gizmo-card",
                // Mode Toggle Header
                div { class: "gizmo-header",
                    div { class: "gizmo-mode-toggle",
                        button {
                            r#type: "button",
                            class: if is_orbit { "gizmo-mode-btn active" } else { "gizmo-mode-btn" },
                            "data-mode": "orbit",
                            title: "Orbit Camera",
                            onclick: move |e| {
                                e.stop_propagation();
                                mode.set(GizmoMode::Orbit);
                            },
                            "🎥 Orbit"
                        }
                        button {
                            r#type: "button",
                            class: if !is_orbit { "gizmo-mode-btn active" } else { "gizmo-mode-btn" },
                            "data-mode": "position",
                            title: "Position Selected Container",
                            onclick: move |e| {
                                e.stop_propagation();
                                mode.set(GizmoMode::Position);
                            },
                            "📦 Position"
                        }
                    }
                }

                // Orbit Mode Quick Views (hidden in position mode)
                if is_orbit {
                    div { class: "gizmo-quick-views-bar", id: "gizmo-orbit-views",
                        button {
                            r#type: "button",
                            class: "gizmo-view-btn",
                            "data-view": "iso",
                            title: "Isometric",
                            onclick: move |e| {
                                e.stop_propagation();
                                snap_orbit(wb, 25.0, -35.0);
                            },
                            "ISO"
                        }
                        button {
                            r#type: "button",
                            class: "gizmo-view-btn",
                            "data-view": "top",
                            title: "Top (+Y)",
                            onclick: move |e| {
                                e.stop_propagation();
                                snap_orbit(wb, 89.0, 0.0);
                            },
                            "⬆"
                        }
                        button {
                            r#type: "button",
                            class: "gizmo-view-btn",
                            "data-view": "front",
                            title: "Front (+Z)",
                            onclick: move |e| {
                                e.stop_propagation();
                                snap_orbit(wb, 0.0, 0.0);
                            },
                            "⬤"
                        }
                        button {
                            r#type: "button",
                            class: "gizmo-view-btn",
                            "data-view": "right",
                            title: "Right (+X)",
                            onclick: move |e| {
                                e.stop_propagation();
                                snap_orbit(wb, 0.0, -90.0);
                            },
                            "⬤"
                        }
                        button {
                            r#type: "button",
                            class: "gizmo-view-btn",
                            "data-view": "reset",
                            title: "Reset",
                            onclick: move |e| {
                                e.stop_propagation();
                                snap_orbit(wb, 15.0, -25.0);
                            },
                            "↺"
                        }
                    }
                }

                // 3D Trackball Viewport
                div {
                    class: "{viewport_class}",
                    id: "gizmo-trackball-viewport",
                    title: "Drag to orbit · Scroll to adjust D-scale · Click axis label to snap",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        let c = e.data().client_coordinates();
                        let s = wb();
                        let (sx, sy) = s
                            .selected
                            .as_ref()
                            .and_then(|id| s.node(id))
                            .map(|n| (n.x, n.y))
                            .unwrap_or((0.0, 0.0));
                        drag.set(GizmoDrag {
                            active: true,
                            start_client_x: c.x,
                            start_client_y: c.y,
                            start_rx: s.orbit_rx,
                            start_ry: s.orbit_ry,
                            start_pos_x: sx,
                            start_pos_y: sy,
                        });
                    },
                    onmousemove: move |e| {
                        let d = drag();
                        if !d.active {
                            return;
                        }
                        e.stop_propagation();
                        let c = e.data().client_coordinates();
                        let dx = c.x - d.start_client_x;
                        let dy = c.y - d.start_client_y;
                        let mut s = wb();
                        if mode() == GizmoMode::Orbit {
                            s.orbit_ry = d.start_ry + dx * 0.7;
                            s.orbit_rx = (d.start_rx - dy * 0.7).clamp(-89.0, 89.0);
                            if s.dim == DimMode::D2 {
                                s.dim = DimMode::D3;
                            }
                        } else if let Some(sel_id) = s.selected.clone() {
                            let sensitivity = 3.0;
                            s.move_node(
                                &sel_id,
                                d.start_pos_x + dx * sensitivity,
                                d.start_pos_y + dy * sensitivity,
                            );
                        }
                        wb.set(s);
                    },
                    onmouseup: move |e| {
                        e.stop_propagation();
                        let mut d = drag();
                        d.active = false;
                        drag.set(d);
                    },
                    onmouseleave: move |_| {
                        let mut d = drag();
                        if d.active {
                            d.active = false;
                            drag.set(d);
                        }
                    },
                    onwheel: move |e| {
                        e.stop_propagation();
                        let raw_dy = match e.data().delta() {
                            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
                            dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 40.0,
                            dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 200.0,
                        };
                        let step = if raw_dy > 0.0 { -5.0 } else { 5.0 };
                        let mut s = wb();
                        if mode() == GizmoMode::Orbit {
                            s.manifold_d = (s.manifold_d + step / 100.0).clamp(0.5, 2.5);
                            if s.dim == DimMode::D2 {
                                s.dim = DimMode::D3;
                            }
                        } else if let Some(sel_id) = s.selected.clone() {
                            if let Some(node) = s.node_mut(&sel_id) {
                                node.z += step * 4.0;
                            }
                        }
                        wb.set(s);
                    },
                    svg {
                        class: "gizmo-svg",
                        view_box: "-60 -60 120 120",
                        xmlns: "http://www.w3.org/2000/svg",

                        // Wireframe latitude/longitude lines
                        g { class: "gizmo-wireframe", opacity: "0.12",
                            ellipse { cx: "0", cy: "0", rx: "40", ry: "40", fill: "none", stroke: "#fff", "stroke-width": "0.5" }
                            ellipse { cx: "0", cy: "0", rx: "40", ry: "14", fill: "none", stroke: "#fff", "stroke-width": "0.4" }
                            ellipse { cx: "0", cy: "0", rx: "14", ry: "40", fill: "none", stroke: "#fff", "stroke-width": "0.4" }
                        }

                        // Sphere body
                        circle { cx: "0", cy: "0", r: "40", fill: "rgba(20, 28, 42, 0.92)", stroke: "rgba(255,255,255,0.08)", "stroke-width": "0.8" }
                        circle { cx: "-8", cy: "-10", r: "22", fill: "rgba(140,180,255,0.10)" }

                        // D-Manifold outer ring (4th dimension)
                        circle {
                            cx: "0",
                            cy: "0",
                            r: "{d_ring_r:.0}",
                            fill: "none",
                            stroke: "var(--accent-violet, #a855f7)",
                            "stroke-width": "1.2",
                            "stroke-dasharray": "4 3",
                            opacity: "0.55",
                            class: "gizmo-d-ring",
                        }

                        // Projected axis lines sorted back-to-front
                        g { id: "gizmo-axes-group",
                            for item in axes.iter() {
                                if item.is_negative {
                                    {
                                        let depth_fade = (0.15 + (item.pz + AXIS_RADIUS) / (2.0 * AXIS_RADIUS) * 0.2)
                                            .clamp(0.08, 0.35);
                                        rsx! {
                                            line {
                                                key: "{item.key}",
                                                x1: "0",
                                                y1: "0",
                                                x2: "{item.px:.1}",
                                                y2: "{item.py:.1}",
                                                stroke: "{item.color}",
                                                "stroke-width": "1",
                                                "stroke-linecap": "round",
                                                opacity: "{depth_fade:.2}",
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let line_op = (0.4 + (item.pz + AXIS_RADIUS) / (2.0 * AXIS_RADIUS) * 0.6)
                                            .clamp(0.3, 1.0);
                                        let scale = (0.7 + (item.pz + AXIS_RADIUS) / (2.0 * AXIS_RADIUS) * 0.5)
                                            .clamp(0.55, 1.2);
                                        let tip_r = 7.0 * scale;
                                        let tip_op = (0.5 + (item.pz + AXIS_RADIUS) / (2.0 * AXIS_RADIUS) * 0.5)
                                            .clamp(0.35, 1.0);
                                        let lbl_op = (0.5 + (item.pz + AXIS_RADIUS) / (2.0 * AXIS_RADIUS) * 0.5)
                                            .clamp(0.3, 1.0);
                                        let axis_name = item.axis;
                                        rsx! {
                                            g { key: "{item.key}",
                                                line {
                                                    x1: "0",
                                                    y1: "0",
                                                    x2: "{item.px:.1}",
                                                    y2: "{item.py:.1}",
                                                    stroke: "{item.color}",
                                                    "stroke-width": "2",
                                                    "stroke-linecap": "round",
                                                    opacity: "{line_op:.2}",
                                                }
                                                circle {
                                                    cx: "{item.px:.1}",
                                                    cy: "{item.py:.1}",
                                                    r: "{tip_r:.1}",
                                                    fill: "{item.color}",
                                                    stroke: "#fff",
                                                    "stroke-width": "0.6",
                                                    opacity: "{tip_op:.2}",
                                                    class: "gizmo-axis-tip",
                                                    "data-axis": "{item.axis}",
                                                    onclick: move |e| {
                                                        e.stop_propagation();
                                                        on_axis_click(wb, mode(), axis_name);
                                                    },
                                                }
                                                text {
                                                    x: "{item.px:.1}",
                                                    y: "{item.py:.1}",
                                                    text_anchor: "middle",
                                                    "dominant-baseline": "central",
                                                    fill: "{item.text_fill}",
                                                    font_size: "7",
                                                    "font-weight": "800",
                                                    "font-family": "var(--font-mono, monospace)",
                                                    opacity: "{lbl_op:.2}",
                                                    class: "gizmo-axis-label",
                                                    "data-axis": "{item.axis}",
                                                    style: "cursor:pointer;",
                                                    onclick: move |e| {
                                                        e.stop_propagation();
                                                        on_axis_click(wb, mode(), axis_name);
                                                    },
                                                    "{item.label}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Central pivot
                            circle { cx: "0", cy: "0", r: "3", fill: "#fff", opacity: "0.9" }
                        }
                    }

                    // Mode indicator overlay
                    div {
                        class: if is_orbit { "gizmo-mode-indicator" } else { "gizmo-mode-indicator mode-position" },
                        id: "gizmo-mode-indicator",
                        if is_orbit { "ORBIT" } else { "POSITION" }
                    }
                }

                // Orbit Mode Footer: telemetry + D slider
                if is_orbit {
                    div { class: "gizmo-footer", id: "gizmo-orbit-footer",
                        div { class: "gizmo-axis-telemetry",
                            span { style: "color: #ef4444;",
                                "X: "
                                strong { id: "tele-x", "{w.orbit_rx.round() as i32}°" }
                            }
                            span { style: "color: #22c55e;",
                                "Y: "
                                strong { id: "tele-y", "{w.orbit_ry.round() as i32}°" }
                            }
                            span { style: "color: #06b6d4;",
                                "Z: "
                                strong { id: "tele-z", "{w.orbit_rz.round() as i32}°" }
                            }
                        }
                        div { class: "gizmo-d-control",
                            span { style: "color: var(--accent-violet, #a855f7); font-size: 10px; font-weight: 700;", "D:" }
                            input {
                                r#type: "range",
                                class: "gizmo-d-slider",
                                id: "gizmo-d-slider",
                                min: "50",
                                max: "250",
                                value: "{d_slider_val}",
                                title: "Adjust 4th-dimensional manifold aperture (D)",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f64>() {
                                        let mut s = wb();
                                        s.manifold_d = (v / 100.0).clamp(0.5, 2.5);
                                        if s.dim == DimMode::D2 {
                                            s.dim = DimMode::D3;
                                        }
                                        wb.set(s);
                                    }
                                },
                            }
                            span { class: "gizmo-d-value", id: "tele-d", "{w.manifold_d:.1}σ" }
                        }
                    }
                } else {
                    // Position Mode Footer: selected container name + XYZD numeric inputs
                    div { class: "gizmo-footer gizmo-position-footer", id: "gizmo-position-footer",
                        div { class: "gizmo-pos-target", id: "gizmo-pos-target",
                            if let Some(ref node) = selected_node {
                                span {
                                    class: "container-type-tag tag-{node.kind.id()}",
                                    style: "font-size: 9px; padding: 1px 5px;",
                                    "{node.kind.id()}"
                                }
                                span {
                                    class: "gizmo-pos-target-label",
                                    title: "{node.id}",
                                    "{node.title}"
                                }
                            } else {
                                span {
                                    class: "gizmo-pos-target-label",
                                    style: "opacity:0.5;",
                                    "No container selected"
                                }
                            }
                        }
                        div { class: "gizmo-pos-fields",
                            div { class: "gizmo-pos-row",
                                label { class: "gizmo-pos-label", style: "color: #ef4444;", "X" }
                                input {
                                    r#type: "number",
                                    class: "gizmo-pos-input",
                                    id: "gizmo-pos-x",
                                    step: "10",
                                    value: "{pos_x_val}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let mut s = wb();
                                            if let Some(sel_id) = s.selected.clone() {
                                                if let Some(n) = s.node_mut(&sel_id) {
                                                    n.x = v.max(0.0);
                                                }
                                            }
                                            wb.set(s);
                                        }
                                    },
                                }
                                span { class: "gizmo-pos-unit", "px" }
                            }
                            div { class: "gizmo-pos-row",
                                label { class: "gizmo-pos-label", style: "color: #22c55e;", "Y" }
                                input {
                                    r#type: "number",
                                    class: "gizmo-pos-input",
                                    id: "gizmo-pos-y",
                                    step: "10",
                                    value: "{pos_y_val}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let mut s = wb();
                                            if let Some(sel_id) = s.selected.clone() {
                                                if let Some(n) = s.node_mut(&sel_id) {
                                                    n.y = v.max(0.0);
                                                }
                                            }
                                            wb.set(s);
                                        }
                                    },
                                }
                                span { class: "gizmo-pos-unit", "px" }
                            }
                            div { class: "gizmo-pos-row",
                                label { class: "gizmo-pos-label", style: "color: #06b6d4;", "Z" }
                                input {
                                    r#type: "number",
                                    class: "gizmo-pos-input",
                                    id: "gizmo-pos-z",
                                    step: "10",
                                    value: "{pos_z_val}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let mut s = wb();
                                            if let Some(sel_id) = s.selected.clone() {
                                                if let Some(n) = s.node_mut(&sel_id) {
                                                    n.z = v;
                                                }
                                            }
                                            wb.set(s);
                                        }
                                    },
                                }
                                span { class: "gizmo-pos-unit", "px" }
                            }
                            div { class: "gizmo-pos-row",
                                label { class: "gizmo-pos-label", style: "color: var(--accent-violet, #a855f7);", "D" }
                                input {
                                    r#type: "number",
                                    class: "gizmo-pos-input",
                                    id: "gizmo-pos-d",
                                    step: "0.1",
                                    min: "0.1",
                                    max: "5.0",
                                    value: "{pos_d_val}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let mut s = wb();
                                            if let Some(sel_id) = s.selected.clone() {
                                                if let Some(n) = s.node_mut(&sel_id) {
                                                    n.d = v.clamp(0.1, 5.0);
                                                }
                                            }
                                            wb.set(s);
                                        }
                                    },
                                }
                                span { class: "gizmo-pos-unit", "σ" }
                            }
                        }
                        button {
                            r#type: "button",
                            class: "gizmo-pos-apply-btn",
                            id: "gizmo-pos-center",
                            title: "Center container in view",
                            onclick: move |e| {
                                e.stop_propagation();
                                let mut s = wb();
                                if let Some(sel_id) = s.selected.clone() {
                                    let zoom = s.zoom.max(0.1);
                                    if let Some(n) = s.node_mut(&sel_id) {
                                        n.x = ((640.0 / zoom) - n.width / 2.0).max(0.0);
                                        n.y = ((360.0 / zoom) - n.height / 2.0).max(0.0);
                                    }
                                    s.note(format!("Centered {sel_id} in viewport"));
                                }
                                wb.set(s);
                            },
                            "⊙ Center in View"
                        }
                    }
                }
            }
        }
    }
}

fn snap_orbit(mut wb: Signal<Workbench>, rx: f64, ry: f64) {
    let mut s = wb();
    s.orbit_rx = rx;
    s.orbit_ry = ry;
    if s.dim == DimMode::D2 {
        s.dim = DimMode::D3;
    }
    wb.set(s);
}

fn on_axis_click(mut wb: Signal<Workbench>, mode: GizmoMode, axis: &str) {
    match mode {
        GizmoMode::Orbit => {
            let (rx, ry) = match axis {
                "x" => (0.0, -90.0),
                "y" => (89.0, 0.0),
                "z" => (0.0, 0.0),
                _ => (0.0, 0.0),
            };
            snap_orbit(wb, rx, ry);
        }
        GizmoMode::Position => {
            let mut s = wb();
            if let Some(sel_id) = s.selected.clone() {
                if let Some(node) = s.node_mut(&sel_id) {
                    let nudge = 50.0;
                    match axis {
                        "x" => node.x += nudge,
                        "y" => node.y += nudge,
                        "z" => node.z += nudge,
                        _ => {}
                    }
                }
            }
            wb.set(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_rotation_preserves_axes() {
        let (x, y, z) = rotate_point_3d(36.0, 0.0, 0.0, 0.0, 0.0);
        assert!((x - 36.0).abs() < 1e-6);
        assert!(y.abs() < 1e-6);
        assert!(z.abs() < 1e-6);
    }

    #[test]
    fn projected_axes_are_sorted_back_to_front() {
        let axes = project_axes(15.0, -25.0);
        assert_eq!(axes.len(), 6);
        for pair in axes.windows(2) {
            assert!(pair[0].pz <= pair[1].pz);
        }
    }
}
