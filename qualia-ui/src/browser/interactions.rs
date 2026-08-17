//! Canvas interactions: pan/zoom, container dragging, selection, wire rendering.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, MouseEvent};

use crate::tool_chest::core::registry::{ManifoldSeed, SeedConnection};

/// SVG namespace URI for creating SVG elements.
const SVG_NS: &str = "http://www.w3.org/2000/svg";

/// Render connection wires as SVG bezier curves between containers.
pub fn render_wires(document: &Document, canvas: &Element, seed: &ManifoldSeed) {
    if seed.connections.is_empty() {
        return;
    }

    let svg = document.create_element_ns(Some(SVG_NS), "svg").unwrap();
    let svg_el = svg.clone();
    svg_el.set_attribute("class", "wire-overlay").unwrap();
    svg_el.set_attribute("width", "100%").unwrap();
    svg_el.set_attribute("height", "100%").unwrap();

    for conn in &seed.connections {
        if conn.from >= seed.containers.len() || conn.to >= seed.containers.len() {
            continue;
        }
        let src = &seed.containers[conn.from];
        let tgt = &seed.containers[conn.to];

        let start_x = src.x + src.width;
        let start_y = src.y + src.height / 2.0;
        let end_x = tgt.x;
        let end_y = tgt.y + tgt.height / 2.0;
        let dx = ((end_x - start_x).abs()) * 0.5;
        let path_d = format!(
            "M {} {} C {} {}, {} {}, {} {}",
            start_x, start_y,
            start_x + dx, start_y,
            end_x - dx, end_y,
            end_x, end_y
        );

        let path = document.create_element_ns(Some(SVG_NS), "path").unwrap();
        path.set_attribute("d", &path_d).unwrap();
        path.set_attribute("class", &format!("wire-{}", conn.wire_type)).unwrap();
        path.set_attribute("data-id", &conn.id).unwrap();
        svg_el.append_child(&path).unwrap();

        // Midpoint label
        let mid_x = (start_x + end_x) / 2.0;
        let mid_y = (start_y + end_y) / 2.0 - 6.0;

        let text = document.create_element_ns(Some(SVG_NS), "text").unwrap();
        text.set_attribute("x", &mid_x.to_string()).unwrap();
        text.set_attribute("y", &mid_y.to_string()).unwrap();
        text.set_attribute("class", "wire-label-text").unwrap();
        text.set_text_content(Some(&conn.label));
        svg_el.append_child(&text).unwrap();
    }

    canvas.append_child(&svg).unwrap();
}

/// Wire up container selection — clicking a container selects it.
pub fn wire_container_selection(document: &Document) {
    let containers = document.query_selector_all(".canvas-container-node").unwrap();
    for i in 0..containers.length() {
        let node = containers.get(i).unwrap();
        let el: Element = node.dyn_into().unwrap();
        let el_clone = el.clone();

        let closure = Closure::wrap(Box::new(move |e: Event| {
            let me: MouseEvent = e.dyn_into().unwrap();
            me.stop_propagation();

            // Deselect all
            let doc = web_sys::window().unwrap().document().unwrap();
            let all = doc.query_selector_all(".canvas-container-node").unwrap();
            for j in 0..all.length() {
                let n = all.get(j).unwrap();
                let ne: Element = n.dyn_into().unwrap();
                ne.class_list().remove_1("selected").unwrap();
            }

            // Select this
            el_clone.class_list().add_1("selected").unwrap();
        }) as Box<dyn FnMut(Event)>);

        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

/// Wire up container dragging via header mousedown.
pub fn wire_container_dragging(document: &Document) {
    let headers = document.query_selector_all(".container-header").unwrap();
    for i in 0..headers.length() {
        let header = headers.get(i).unwrap();
        let header_el: Element = header.dyn_into().unwrap();

        let closure = Closure::wrap(Box::new(move |e: Event| {
            let me: MouseEvent = e.dyn_into().unwrap();
            me.stop_propagation();

            let doc = web_sys::window().unwrap().document().unwrap();
            let target = me.current_target().unwrap();
            let header: Element = target.dyn_into().unwrap();
            let parent = header.parent_element().unwrap();

            parent.class_list().add_1("dragging").unwrap();

            let start_mx = me.client_x() as f32;
            let start_my = me.client_y() as f32;
            let style = parent.get_attribute("style").unwrap_or_default();
            let (orig_x, orig_y) = parse_position(&style);

            let parent_clone = parent.clone();
            let on_move = Closure::wrap(Box::new(move |ev: Event| {
                let mev: MouseEvent = ev.dyn_into().unwrap();
                let dx = mev.client_x() as f32 - start_mx;
                let dy = mev.client_y() as f32 - start_my;
                let new_x = orig_x + dx;
                let new_y = orig_y + dy;
                let new_style = update_position(&style, new_x, new_y);
                parent_clone.set_attribute("style", &new_style).unwrap();
            }) as Box<dyn FnMut(Event)>);

            let parent_clone2 = parent.clone();
            let on_up = Closure::wrap(Box::new(move |_ev: Event| {
                parent_clone2.class_list().remove_1("dragging").unwrap();
            }) as Box<dyn FnMut(Event)>);

            doc.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).unwrap();
            doc.add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref()).unwrap();
            on_move.forget();
            on_up.forget();
        }) as Box<dyn FnMut(Event)>);

        header_el.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

/// Wire up canvas pan (drag on empty canvas) and zoom (wheel).
pub fn wire_canvas_pan_zoom(document: &Document) {
    let canvas = match document.get_element_by_id("manifold-canvas") {
        Some(c) => c,
        None => return,
    };

    // Pan
    let canvas_clone = canvas.clone();
    let pan_closure = Closure::wrap(Box::new(move |e: Event| {
        let me: MouseEvent = e.dyn_into().unwrap();
        let target = me.target().unwrap();
        let el: Element = target.dyn_into().unwrap();
        let tag = el.tag_name().to_lowercase();
        if tag != "div" {
            return;
        }
        let cls = el.class_list();
        if !cls.contains("canvas-viewport-container") && !cls.contains("canvas-grid-svg") {
            return;
        }

        let start_mx = me.client_x() as f32;
        let start_my = me.client_y() as f32;
        let canvas_ref = canvas_clone.clone();

        let on_move = Closure::wrap(Box::new(move |ev: Event| {
            let mev: MouseEvent = ev.dyn_into().unwrap();
            let dx = mev.client_x() as f32 - start_mx;
            let dy = mev.client_y() as f32 - start_my;
            canvas_ref.set_scroll_left(((canvas_ref.scroll_left() as f32 - dx) as i32).into());
            canvas_ref.set_scroll_top(((canvas_ref.scroll_top() as f32 - dy) as i32).into());
        }) as Box<dyn FnMut(Event)>);

        let doc = web_sys::window().unwrap().document().unwrap();
        doc.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).unwrap();
        on_move.forget();

        let on_up = Closure::wrap(Box::new(move |_ev: Event| {
            // Listeners are forgotten; in a production app we'd track and remove them.
            // For the UX preview this is acceptable.
        }) as Box<dyn FnMut(Event)>);

        doc.add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref()).unwrap();
        on_up.forget();
    }) as Box<dyn FnMut(Event)>);

    canvas.add_event_listener_with_callback("mousedown", pan_closure.as_ref().unchecked_ref()).unwrap();
    pan_closure.forget();

    // Zoom (wheel)
    let canvas_clone2 = canvas.clone();
    let zoom_closure = Closure::wrap(Box::new(move |e: Event| {
        let me: MouseEvent = e.dyn_into().unwrap();
        me.prevent_default();
        let factor = if (me.client_y() as i64) & 1 == 0 { 1.1 } else { 0.9 };
        let el: Element = canvas_clone2.clone().dyn_into().unwrap();
        let cur = el.get_attribute("data-zoom").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1.0);
        let new_zoom = (cur * factor).max(0.3).min(3.0);
        el.set_attribute("data-zoom", &new_zoom.to_string()).unwrap();
    }) as Box<dyn FnMut(Event)>);

    canvas.add_event_listener_with_callback("wheel", zoom_closure.as_ref().unchecked_ref()).unwrap();
    zoom_closure.forget();
}

/// Wire up toolbox dock button selection.
pub fn wire_toolbox_dock(document: &Document) {
    let buttons = document.query_selector_all(".toolbox-dock-btn").unwrap();
    for i in 0..buttons.length() {
        let btn = buttons.get(i).unwrap();
        let btn_el: Element = btn.dyn_into().unwrap();

        let closure = Closure::wrap(Box::new(move |_e: Event| {
            let doc = web_sys::window().unwrap().document().unwrap();
            let all = doc.query_selector_all(".toolbox-dock-btn").unwrap();
            for j in 0..all.length() {
                let n = all.get(j).unwrap();
                let ne: Element = n.dyn_into().unwrap();
                ne.class_list().remove_1("active").unwrap();
            }
            let doc2 = web_sys::window().unwrap().document().unwrap();
            let all2 = doc2.query_selector_all(".toolbox-dock-btn").unwrap();
            for j in 0..all2.length() {
                let n = all2.get(j).unwrap();
                let ne: Element = n.dyn_into().unwrap();
                ne.class_list().add_1("active").unwrap();
                break;
            }
        }) as Box<dyn FnMut(Event)>);

        btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

/// Wire up strata/epistemic/dimension selector buttons.
pub fn wire_selector_buttons(document: &Document) {
    for selector_class in &[".strata-deck-btn", ".epistemic-btn", ".dim-btn"] {
        let buttons = document.query_selector_all(selector_class).unwrap();
        let group_class = selector_class.trim_start_matches('.');

        for i in 0..buttons.length() {
            let btn = buttons.get(i).unwrap();
            let btn_el: Element = btn.dyn_into().unwrap();
            let btn_clone = btn_el.clone();
            let gc = group_class.to_string();

            let closure = Closure::wrap(Box::new(move |_e: Event| {
                let doc = web_sys::window().unwrap().document().unwrap();
                let all = doc.query_selector_all(&format!(".{}", gc)).unwrap();
                for j in 0..all.length() {
                    let n = all.get(j).unwrap();
                    let ne: Element = n.dyn_into().unwrap();
                    ne.class_list().remove_1("active").unwrap();
                }
                btn_clone.class_list().add_1("active").unwrap();
            }) as Box<dyn FnMut(Event)>);

            btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }
    }
}

/// Wire up container resize handles (bottom-right grip).
pub fn wire_container_resize(document: &Document) {
    let handles = document.query_selector_all(".resize-handle").unwrap();
    for i in 0..handles.length() {
        let handle = handles.get(i).unwrap();
        let handle_el: Element = handle.dyn_into().unwrap();

        let closure = Closure::wrap(Box::new(move |e: Event| {
            let me: MouseEvent = e.dyn_into().unwrap();
            me.stop_propagation();

            let target = me.current_target().unwrap();
            let handle: Element = target.dyn_into().unwrap();
            let parent = handle.parent_element().unwrap();

            let start_mx = me.client_x() as f32;
            let start_my = me.client_y() as f32;
            let style = parent.get_attribute("style").unwrap_or_default();
            let (orig_w, orig_h) = parse_size(&style);

            let parent_clone = parent.clone();
            let on_move = Closure::wrap(Box::new(move |ev: Event| {
                let mev: MouseEvent = ev.dyn_into().unwrap();
                let dx = mev.client_x() as f32 - start_mx;
                let dy = mev.client_y() as f32 - start_my;
                let new_w = (orig_w + dx).max(200.0);
                let new_h = (orig_h + dy).max(150.0);
                let new_style = update_size(&style, new_w, new_h);
                parent_clone.set_attribute("style", &new_style).unwrap();
            }) as Box<dyn FnMut(Event)>);

            let doc = web_sys::window().unwrap().document().unwrap();
            doc.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).unwrap();
            on_move.forget();

            let on_up = Closure::wrap(Box::new(move |_ev: Event| {}) as Box<dyn FnMut(Event)>);
            doc.add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref()).unwrap();
            on_up.forget();
        }) as Box<dyn FnMut(Event)>);

        handle_el.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_position(style: &str) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    for part in style.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("left: ") {
            x = val.trim_end_matches("px").parse().unwrap_or(0.0);
        } else if let Some(val) = part.strip_prefix("top: ") {
            y = val.trim_end_matches("px").parse().unwrap_or(0.0);
        }
    }
    (x, y)
}

fn update_position(style: &str, new_x: f32, new_y: f32) -> String {
    let mut result = String::new();
    for part in style.split(';') {
        let part = part.trim();
        if part.starts_with("left: ") {
            result.push_str(&format!("left: {}px; ", new_x as u32));
        } else if part.starts_with("top: ") {
            result.push_str(&format!("top: {}px; ", new_y as u32));
        } else if !part.is_empty() {
            result.push_str(part);
            result.push_str("; ");
        }
    }
    if !result.contains("left:") {
        result.push_str(&format!("left: {}px; ", new_x as u32));
    }
    if !result.contains("top:") {
        result.push_str(&format!("top: {}px; ", new_y as u32));
    }
    result
}

fn parse_size(style: &str) -> (f32, f32) {
    let mut w = 400.0;
    let mut h = 300.0;
    for part in style.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("width: ") {
            w = val.trim_end_matches("px").parse().unwrap_or(400.0);
        } else if let Some(val) = part.strip_prefix("height: ") {
            h = val.trim_end_matches("px").parse().unwrap_or(300.0);
        }
    }
    (w, h)
}

fn update_size(style: &str, new_w: f32, new_h: f32) -> String {
    let mut result = String::new();
    for part in style.split(';') {
        let part = part.trim();
        if part.starts_with("width: ") {
            result.push_str(&format!("width: {}px; ", new_w as u32));
        } else if part.starts_with("height: ") {
            result.push_str(&format!("height: {}px; ", new_h as u32));
        } else if !part.is_empty() {
            result.push_str(part);
            result.push_str("; ");
        }
    }
    if !result.contains("width:") {
        result.push_str(&format!("width: {}px; ", new_w as u32));
    }
    if !result.contains("height:") {
        result.push_str(&format!("height: {}px; ", new_h as u32));
    }
    result
}

// ---------------------------------------------------------------------------
// Grid snapping
// ---------------------------------------------------------------------------

/// Default grid size in pixels.
const GRID_SIZE: f32 = 16.0;

/// Snap a value to the nearest grid point.
pub fn snap_to_grid(value: f32) -> f32 {
    (value / GRID_SIZE).round() * GRID_SIZE
}

/// Snap a (x, y) pair to the grid.
pub fn snap_point(x: f32, y: f32) -> (f32, f32) {
    (snap_to_grid(x), snap_to_grid(y))
}

/// Clamp a value between min and max.
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min { min } else if value > max { max } else { value }
}

/// Snap and clamp a position within canvas bounds.
pub fn snap_clamp_position(x: f32, y: f32, canvas_w: f32, canvas_h: f32, elem_w: f32, elem_h: f32) -> (f32, f32) {
    let sx = snap_to_grid(x);
    let sy = snap_to_grid(y);
    let cx = clamp(sx, 0.0, (canvas_w - elem_w).max(0.0));
    let cy = clamp(sy, 0.0, (canvas_h - elem_h).max(0.0));
    (cx, cy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_to_grid() {
        assert_eq!(snap_to_grid(0.0), 0.0);
        assert_eq!(snap_to_grid(15.0), 16.0);
        assert_eq!(snap_to_grid(17.0), 16.0);
        assert_eq!(snap_to_grid(24.0), 32.0);
        assert_eq!(snap_to_grid(-3.0), 0.0);
        assert_eq!(snap_to_grid(48.0), 48.0);
    }

    #[test]
    fn test_snap_point() {
        let (x, y) = snap_point(15.0, 33.0);
        assert_eq!(x, 16.0);
        assert_eq!(y, 32.0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_snap_clamp_position() {
        let (x, y) = snap_clamp_position(15.0, 33.0, 800.0, 600.0, 200.0, 150.0);
        assert_eq!(x, 16.0);
        assert_eq!(y, 32.0);

        let (x2, _) = snap_clamp_position(700.0, 0.0, 800.0, 600.0, 200.0, 150.0);
        assert_eq!(x2, 600.0);
    }
}
