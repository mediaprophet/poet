//! Browser UI shell — renders the tool-chest in a browser for UX testing.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! Principal / inventor: Timothy Charles Holborn <timothy.holborn@gmail.com>
//! Assignment: COPYRIGHT.md  Licence: LICENSE (CC BY-NC-ND 4.0)
//!
//! This module renders the manifold pager, container layout, and
//! tool-chest sidebar using DOM manipulation via web-sys. It is
//! intended for UX testing prior to full QualiaDB integration.
//!
//! Design system adapted from Canvas_Workbench — cyber-semantic
//! glassmorphism with strata, social, health & epistemic modalities.

pub mod css;
pub mod topbar;
pub mod containers;
pub mod docks;
pub mod interactions;
pub mod capabilities;
pub mod intent_bus;
pub mod registration;
pub mod container_views;
pub mod container_views_ext;
pub mod command_palette;
pub mod history;
pub mod theme;
pub mod diagnostics;
pub mod manifest;

use std::sync::atomic::{AtomicBool, Ordering};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

use crate::tool_chest::manifolds::all_seeds;
use crate::tool_chest::core::registry::ManifoldSeed;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Guard against double-initialisation. `main()` calls `start()` and the
/// HTML `TrunkApplicationStarted` listener also calls `start()` as a
/// fallback; the guard makes the second call a no-op.
static START_CALLED: AtomicBool = AtomicBool::new(false);

/// WASM entry point — called from main() in the binary target.
pub fn start() {
    if START_CALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    if let Err(msg) = try_start(&document) {
        web_sys::console::error_1(&format!("[qualia-ui] FATAL: {}", msg).into());
        show_fatal_error(&document, &msg);
    }
}

/// Show a visible error message in the DOM (replaces loading indicator).
fn show_fatal_error(document: &Document, msg: &str) {
    if let Some(loading) = document.get_element_by_id("loading") {
        loading.set_inner_html(&format!(
            r#"<div style="color:#ff4444;font-family:monospace;font-size:12px;padding:20px;max-width:600px;">
            <h3 style="margin:0 0 10px;">Qualia UI — Initialization Error</h3>
            <pre style="white-space:pre-wrap;word-break:break-all;">{}</pre>
            </div>"#,
            msg.replace('<', "&lt;").replace('>', "&gt;")
        ));
    }
}

fn try_start(document: &Document) -> Result<(), String> {
    // Inject CSS
    let style = document.create_element("style")
        .map_err(|e| format!("create_element(style): {:?}", e))?;
    style.set_text_content(Some(css::CSS));
    document.head()
        .ok_or("head() returned None")?
        .append_child(&style)
        .map_err(|e| format!("append_child(style): {:?}", e))?;

    // Apply default theme
    let theme_state = theme::ThemeState::default();
    theme_state.apply(document);

    // Build the app
    let app = build_app(document);

    // Append to body
    document.body()
        .ok_or("body() returned None")?
        .append_child(&app)
        .map_err(|e| format!("append_child(app): {:?}", e))?;

    // Remove loading indicator
    if let Some(loading) = document.get_element_by_id("loading") {
        loading.remove();
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// App builder
// ---------------------------------------------------------------------------

fn build_app(document: &Document) -> HtmlElement {
    let app = document.create_element("div").unwrap();
    app.set_class_name("app");

    let seeds = all_seeds();

    // Top menubar
    let menubar = topbar::build_top_menubar(document);
    app.append_child(&menubar).unwrap();

    // Canvas control bar (manifold pager + strata + epistemic + dimension)
    let control_bar = topbar::build_canvas_control_bar(document, &seeds);
    app.append_child(&control_bar).unwrap();

    // Main workspace
    let workspace = document.create_element("div").unwrap();
    workspace.set_class_name("main-workspace");

    // Toolbox dock (left sidebar)
    let toolbox = docks::build_toolbox_dock(document);
    workspace.append_child(&toolbox).unwrap();

    // Canvas viewport
    let canvas = build_canvas(document, &seeds[0]);
    canvas.set_id("manifold-canvas");
    workspace.append_child(&canvas).unwrap();

    // Right dock (aura tray + pulse stream)
    let right_dock = docks::build_right_dock(document);
    workspace.append_child(&right_dock).unwrap();

    app.append_child(&workspace).unwrap();

    // Bottom status bar
    let statusbar = docks::build_bottom_statusbar(document);
    app.append_child(&statusbar).unwrap();

    // Command palette overlay
    let palette = command_palette::build_command_palette(document);
    app.append_child(&palette).unwrap();

    // Wire up manifold switching
    wire_manifold_tabs(document, &seeds);

    // Wire up canvas interactions
    interactions::wire_container_selection(document);
    interactions::wire_container_dragging(document);
    interactions::wire_container_resize(document);
    interactions::wire_canvas_pan_zoom(document);
    interactions::wire_toolbox_dock(document);
    interactions::wire_selector_buttons(document);

    // Wire up command palette (Ctrl+K)
    command_palette::wire_command_palette(document);

    app.dyn_into::<HtmlElement>().unwrap()
}

// ---------------------------------------------------------------------------
// Canvas
// ---------------------------------------------------------------------------

fn build_canvas(document: &Document, seed: &ManifoldSeed) -> Element {
    let canvas = document.create_element("div").unwrap();
    canvas.set_class_name("canvas-viewport-container");

    // Grid background
    let grid = document.create_element("div").unwrap();
    grid.set_class_name("canvas-grid-svg");
    canvas.append_child(&grid).unwrap();

    // Containers
    for container in &seed.containers {
        let el = containers::build_container(document, container);
        canvas.append_child(&el).unwrap();
    }

    // Connection wires
    interactions::render_wires(document, &canvas, seed);

    canvas
}

// ---------------------------------------------------------------------------
// Manifold switching
// ---------------------------------------------------------------------------

fn wire_manifold_tabs(document: &Document, seeds: &[ManifoldSeed]) {
    let tabs = document.query_selector_all(".desktop-tab-btn").unwrap();
    for i in 0..tabs.length() {
        let tab = tabs.get(i).unwrap();
        let tab_el: Element = tab.dyn_into().unwrap();
        let manifold_id = tab_el.get_attribute("data-manifold").unwrap();

        let seeds_clone: Vec<ManifoldSeed> = seeds.to_vec();
        let closure = Closure::wrap(Box::new(move || {
            switch_manifold(&manifold_id, &seeds_clone);
        }) as Box<dyn FnMut()>);

        tab_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

fn switch_manifold(manifold_id: &str, seeds: &[ManifoldSeed]) {
    let document = web_sys::window().unwrap().document().unwrap();

    // Update tab active states
    let tabs = document.query_selector_all(".desktop-tab-btn").unwrap();
    for i in 0..tabs.length() {
        let tab = tabs.get(i).unwrap();
        let tab_el: Element = tab.dyn_into().unwrap();
        let id = tab_el.get_attribute("data-manifold").unwrap();
        if id == manifold_id {
            tab_el.class_list().add_1("active").unwrap();
        } else {
            tab_el.class_list().remove_1("active").unwrap();
        }
    }

    // Update canvas title and graph badge
    if let Some(seed) = seeds.iter().find(|s| s.id == manifold_id) {
        if let Some(title) = document.query_selector(".canvas-title-input").unwrap() {
            title.set_text_content(Some(&seed.label));
        }
        if let Some(badge) = document.query_selector(".graph-address-badge").unwrap() {
            badge.set_text_content(Some(&format!("graph:manifold:{}", seed.id)));
        }

        // Replace canvas content
        if let Some(canvas) = document.get_element_by_id("manifold-canvas") {
            canvas.set_inner_html("");
            let grid = document.create_element("div").unwrap();
            grid.set_class_name("canvas-grid-svg");
            canvas.append_child(&grid).unwrap();

            for container in &seed.containers {
                let el = containers::build_container(&document, container);
                canvas.append_child(&el).unwrap();
            }

            // Re-render wires
            interactions::render_wires(&document, &canvas, seed);

            // Re-wire interactions for new containers
            interactions::wire_container_selection(&document);
            interactions::wire_container_dragging(&document);
            interactions::wire_container_resize(&document);
        }
    }
}
