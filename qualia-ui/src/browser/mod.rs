//! Browser UI shell — renders the tool-chest in a browser for UX testing.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! Principal / inventor: Timothy Charles Holborn <timothy.holborn@gmail.com>
//! Assignment: COPYRIGHT.md  Licence: LICENSE (CC BY-NC-ND 4.0)
//!
//! This module renders the manifold pager, container layout, and
//! tool-chest sidebar using DOM manipulation via web-sys. It is
//! intended for UX testing prior to full QualiaDB integration.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

use crate::tool_chest::manifolds::all_seeds;
use crate::tool_chest::core::registry::{ManifoldSeed, SeedContainer};

// ---------------------------------------------------------------------------
// CSS (embedded as a string — no external stylesheet needed for preview)
// ---------------------------------------------------------------------------

const CSS: &str = r#"
:root {
  --bg: #07090e;
  --bg-panel: #0c1017;
  --bg-container: #111722;
  --bg-elevated: #161d2b;
  --border: #1e2738;
  --text: #c5cdd9;
  --text-dim: #6b7689;
  --accent: #00f2a9;
  --accent-dim: #00a978;
  --warn: #f5a623;
  --danger: #e85d75;
  --radius: 6px;
  --font: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --mono: 'JetBrains Mono', 'SF Mono', 'Cascadia Code', monospace;
}
* { margin: 0; padding: 0; box-sizing: border-box; }
body { background: var(--bg); color: var(--text); font-family: var(--font); overflow: hidden; }
.app { display: flex; flex-direction: column; height: 100vh; }

/* Top bar */
.topbar {
  display: flex; align-items: center; gap: 0;
  background: var(--bg-panel); border-bottom: 1px solid var(--border);
  padding: 0 12px; height: 44px; flex-shrink: 0;
}
.brand { font-size: 14px; font-weight: 700; color: var(--accent); margin-right: 16px; white-space: nowrap; }
.manifold-tabs { display: flex; gap: 2px; }
.manifold-tab {
  background: transparent; border: none; color: var(--text-dim);
  padding: 6px 14px; font-size: 12px; cursor: pointer; border-radius: 4px;
  transition: all 0.15s; display: flex; align-items: center; gap: 6px;
}
.manifold-tab:hover { color: var(--text); background: var(--bg-elevated); }
.manifold-tab.active { color: var(--accent); background: var(--bg-container); }

/* Main layout */
.main { display: flex; flex: 1; overflow: hidden; }

/* Sidebar (tool-chest) */
.sidebar {
  width: 220px; background: var(--bg-panel); border-right: 1px solid var(--border);
  display: flex; flex-direction: column; flex-shrink: 0; overflow-y: auto;
}
.sidebar-section { padding: 8px 0; border-bottom: 1px solid var(--border); }
.sidebar-header {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-dim); padding: 4px 12px; font-weight: 600;
}
.sidebar-item {
  display: flex; align-items: center; gap: 8px; padding: 6px 12px;
  font-size: 12px; color: var(--text); cursor: pointer; border-radius: 4px;
  margin: 1px 4px; transition: background 0.15s;
}
.sidebar-item:hover { background: var(--bg-elevated); }
.sidebar-item .icon { font-size: 14px; width: 18px; text-align: center; }
.honesty-badge {
  font-size: 9px; padding: 1px 5px; border-radius: 3px; margin-left: auto;
  text-transform: uppercase; letter-spacing: 0.05em;
}
.honesty-live { background: rgba(0,242,169,0.15); color: var(--accent); }
.honesty-partial { background: rgba(245,166,35,0.15); color: var(--warn); }
.honesty-present { background: rgba(107,118,137,0.15); color: var(--text-dim); }
.honesty-missing { background: rgba(232,93,117,0.1); color: var(--danger); }

/* Manifold canvas */
.canvas {
  flex: 1; position: relative; overflow: hidden;
  background:
    radial-gradient(circle at 50% 50%, rgba(0,210,255,0.02) 0%, transparent 60%),
    var(--bg);
}
.canvas-grid {
  position: absolute; inset: 0;
  background-image:
    linear-gradient(rgba(0,210,255,0.04) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0,210,255,0.04) 1px, transparent 1px);
  background-size: 40px 40px;
  pointer-events: none;
}

/* Container */
.container {
  position: absolute; background: var(--bg-container); border: 1px solid var(--border);
  border-radius: var(--radius); display: flex; flex-direction: column; overflow: hidden;
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}
.container-header {
  display: flex; align-items: center; gap: 8px; padding: 6px 10px;
  background: var(--bg-elevated); border-bottom: 1px solid var(--border);
  font-size: 11px; font-weight: 600; cursor: move; user-select: none;
}
.container-header .icon { font-size: 13px; }
.container-header .title { flex: 1; color: var(--text); }
.container-body { flex: 1; padding: 10px; overflow-y: auto; font-size: 12px; }
.container-placeholder {
  color: var(--text-dim); font-size: 11px; text-align: center; padding: 20px;
}

/* Right dock */
.right-dock {
  width: 260px; background: var(--bg-panel); border-left: 1px solid var(--border);
  display: flex; flex-direction: column; flex-shrink: 0;
}
.dock-panel { border-bottom: 1px solid var(--border); display: flex; flex-direction: column; }
.dock-panel-header {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-dim); padding: 8px 12px; font-weight: 600;
  background: var(--bg-elevated);
}
.dock-panel-body { padding: 10px 12px; font-size: 11px; }

/* Bottom dock */
.bottom-dock {
  height: 120px; background: var(--bg-panel); border-top: 1px solid var(--border);
  display: flex; flex-direction: column; flex-shrink: 0;
}
.bottom-dock-header {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-dim); padding: 6px 12px; font-weight: 600;
  background: var(--bg-elevated);
}
.bottom-dock-body { flex: 1; padding: 8px 12px; overflow-y: auto; font-family: var(--mono); font-size: 10px; color: var(--text-dim); }

/* Connection request card */
.cr-card {
  background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 4px;
  padding: 10px; margin-bottom: 8px;
}
.cr-card .cr-header { display: flex; align-items: center; gap: 6px; margin-bottom: 6px; }
.cr-card .cr-status {
  font-size: 9px; padding: 1px 6px; border-radius: 3px; text-transform: uppercase;
}
.cr-status-pending { background: rgba(245,166,35,0.15); color: var(--warn); }
.cr-status-verifying { background: rgba(0,210,255,0.15); color: #00d2ff; }
.cr-status-accepted { background: rgba(0,242,169,0.15); color: var(--accent); }
.cr-status-blocked { background: rgba(232,93,117,0.15); color: var(--danger); }
.cr-card .cr-meta { font-size: 10px; color: var(--text-dim); margin-bottom: 4px; }
.cr-card .cr-actions { display: flex; gap: 6px; margin-top: 6px; }
.cr-btn {
  font-size: 10px; padding: 3px 10px; border: 1px solid var(--border);
  background: var(--bg-container); color: var(--text); border-radius: 3px; cursor: pointer;
}
.cr-btn:hover { border-color: var(--accent); color: var(--accent); }
.cr-btn.danger:hover { border-color: var(--danger); color: var(--danger); }

/* Risk indicator */
.risk-indicator {
  display: inline-block; font-size: 9px; padding: 1px 5px; border-radius: 3px;
  margin-right: 4px; background: var(--bg); border: 1px solid var(--border);
}
.risk-moderate { border-color: var(--warn); color: var(--warn); }
.risk-high { border-color: var(--danger); color: var(--danger); }
.risk-low { border-color: var(--accent-dim); color: var(--accent-dim); }

/* Protection policy card */
.pp-card {
  background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 4px;
  padding: 10px; margin-bottom: 8px;
}
.pp-card .pp-category {
  font-size: 10px; font-weight: 600; color: var(--accent); margin-bottom: 4px;
}
.pp-card .pp-row { font-size: 10px; color: var(--text-dim); margin: 2px 0; }
.pp-card .pp-mandatory {
  font-size: 9px; padding: 1px 5px; border-radius: 3px;
  background: rgba(232,93,117,0.1); color: var(--danger); margin-left: 6px;
}
"#;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// WASM entry point — called automatically by wasm-bindgen when the
/// module is instantiated.
#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();

    // Inject CSS
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(CSS));
    document.head().unwrap().append_child(&style).unwrap();

    // Build the app
    let app = build_app(&document);
    document.body().unwrap().append_child(&app).unwrap();

    // Remove loading indicator
    if let Some(loading) = document.get_element_by_id("loading") {
        loading.remove();
    }
}

// ---------------------------------------------------------------------------
// App builder
// ---------------------------------------------------------------------------

fn build_app(document: &Document) -> HtmlElement {
    let app = document.create_element("div").unwrap();
    app.set_class_name("app");

    let seeds = all_seeds();

    // Top bar
    let topbar = build_topbar(document, &seeds);
    app.append_child(&topbar).unwrap();

    // Main layout
    let main = document.create_element("div").unwrap();
    main.set_class_name("main");

    // Sidebar
    let sidebar = build_sidebar(document, &seeds);
    main.append_child(&sidebar).unwrap();

    // Canvas (manifold view)
    let canvas = build_canvas(document, &seeds[0]); // Start with first manifold
    canvas.set_id("manifold-canvas");
    main.append_child(&canvas).unwrap();

    // Right dock
    let right_dock = build_right_dock(document);
    main.append_child(&right_dock).unwrap();

    app.append_child(&main).unwrap();

    // Bottom dock
    let bottom_dock = build_bottom_dock(document);
    app.append_child(&bottom_dock).unwrap();

    // Wire up manifold switching
    wire_manifold_tabs(document, &seeds);

    app.dyn_into::<HtmlElement>().unwrap()
}

fn build_topbar(document: &Document, seeds: &[ManifoldSeed]) -> Element {
    let topbar = document.create_element("div").unwrap();
    topbar.set_class_name("topbar");

    let brand = document.create_element("span").unwrap();
    brand.set_class_name("brand");
    brand.set_text_content(Some("🌌 QualiaDB"));
    topbar.append_child(&brand).unwrap();

    let tabs = document.create_element("div").unwrap();
    tabs.set_class_name("manifold-tabs");

    for (i, seed) in seeds.iter().enumerate() {
        let tab = document.create_element("button").unwrap();
        tab.set_class_name("manifold-tab");
        if i == 0 {
            tab.class_list().add_1("active").unwrap();
        }
        tab.set_attribute("data-manifold", &seed.id).unwrap();
        tab.set_text_content(Some(&format!("{} {}", seed.icon, seed.label)));
        tabs.append_child(&tab).unwrap();
    }

    topbar.append_child(&tabs).unwrap();
    topbar
}

fn build_sidebar(document: &Document, seeds: &[ManifoldSeed]) -> Element {
    let sidebar = document.create_element("div").unwrap();
    sidebar.set_class_name("sidebar");

    // Manifolds section
    let section = document.create_element("div").unwrap();
    section.set_class_name("sidebar-section");
    let header = document.create_element("div").unwrap();
    header.set_class_name("sidebar-header");
    header.set_text_content(Some("Manifolds"));
    section.append_child(&header).unwrap();

    for seed in seeds {
        let item = document.create_element("div").unwrap();
        item.set_class_name("sidebar-item");
        item.set_attribute("data-manifold", &seed.id).unwrap();
        let icon = document.create_element("span").unwrap();
        icon.set_class_name("icon");
        icon.set_text_content(Some(&seed.icon));
        let label = document.create_element("span").unwrap();
        label.set_text_content(Some(&seed.label));
        item.append_child(&icon).unwrap();
        item.append_child(&label).unwrap();
        section.append_child(&item).unwrap();
    }
    sidebar.append_child(&section).unwrap();

    // Toolboxes section (placeholder)
    let section2 = document.create_element("div").unwrap();
    section2.set_class_name("sidebar-section");
    let header2 = document.create_element("div").unwrap();
    header2.set_class_name("sidebar-header");
    header2.set_text_content(Some("Toolboxes"));
    section2.append_child(&header2).unwrap();

    for (icon, label, honesty) in &[
        ("💬", "Social", "live"),
        ("⚙", "Settings", "missing"),
        ("📡", "Communications", "missing"),
        ("📄", "Office", "live"),
        ("🔬", "Research", "live"),
        ("🎨", "Media", "live"),
        ("🔐", "Rights", "missing"),
    ] {
        let item = document.create_element("div").unwrap();
        item.set_class_name("sidebar-item");
        let ic = document.create_element("span").unwrap();
        ic.set_class_name("icon");
        ic.set_text_content(Some(icon));
        let lb = document.create_element("span").unwrap();
        lb.set_text_content(Some(label));
        let badge = document.create_element("span").unwrap();
        badge.set_class_name(&format!("honesty-badge honesty-{}", honesty));
        badge.set_text_content(Some(honesty));
        item.append_child(&ic).unwrap();
        item.append_child(&lb).unwrap();
        item.append_child(&badge).unwrap();
        section2.append_child(&item).unwrap();
    }
    sidebar.append_child(&section2).unwrap();

    sidebar
}

fn build_canvas(document: &Document, seed: &ManifoldSeed) -> Element {
    let canvas = document.create_element("div").unwrap();
    canvas.set_class_name("canvas");

    // Grid background
    let grid = document.create_element("div").unwrap();
    grid.set_class_name("canvas-grid");
    canvas.append_child(&grid).unwrap();

    // Containers
    for container in &seed.containers {
        let el = build_container(document, container);
        canvas.append_child(&el).unwrap();
    }

    canvas
}

fn build_container(document: &Document, container: &SeedContainer) -> Element {
    let el = document.create_element("div").unwrap();
    el.set_class_name("container");
    el.set_attribute("data-container-type", &container.container_type).unwrap();

    let style = format!(
        "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {};",
        container.x as u32, container.y as u32,
        container.width as u32, container.height as u32,
        container.z as u32
    );
    el.set_attribute("style", &style).unwrap();

    // Header
    let header = document.create_element("div").unwrap();
    header.set_class_name("container-header");

    let icon = document.create_element("span").unwrap();
    icon.set_class_name("icon");
    icon.set_text_content(Some(&container_icon(&container.container_type)));

    let title = document.create_element("span").unwrap();
    title.set_class_name("title");
    title.set_text_content(Some(&container.title));

    let badge = document.create_element("span").unwrap();
    badge.set_class_name(&format!("honesty-badge honesty-{}", container.honesty));
    badge.set_text_content(Some(&container.honesty));

    header.append_child(&icon).unwrap();
    header.append_child(&title).unwrap();
    header.append_child(&badge).unwrap();
    el.append_child(&header).unwrap();

    // Body — render content based on container type
    let body = document.create_element("div").unwrap();
    body.set_class_name("container-body");

    match container.container_type.as_str() {
        "connection-requests" => {
            body.append_child(&build_connection_requests_view(document)).unwrap();
        }
        "protection-policies" => {
            body.append_child(&build_protection_policies_view(document)).unwrap();
        }
        "social" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some("Social graph visualisation — nodes and edges rendered here."));
            body.append_child(&ph).unwrap();
        }
        "reputation" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some("Reputation panels — trustworthiness, competence, integrity, conduct."));
            body.append_child(&ph).unwrap();
        }
        "capabilities" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some("Capability badges — active, suspended, revoked, expired."));
            body.append_child(&ph).unwrap();
        }
        "settings" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some("Preferences — accessibility, privacy, communication, workflow."));
            body.append_child(&ph).unwrap();
        }
        _ => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(&format!("{} — awaiting backend wiring.", container.title)));
            body.append_child(&ph).unwrap();
        }
    }

    el.append_child(&body).unwrap();
    el
}

fn build_connection_requests_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();

    // Demo connection request card
    let card = document.create_element("div").unwrap();
    card.set_class_name("cr-card");

    let header = document.create_element("div").unwrap();
    header.set_class_name("cr-header");

    let status = document.create_element("span").unwrap();
    status.set_class_name("cr-status cr-status-verifying");
    status.set_text_content(Some("verifying"));
    header.append_child(&status).unwrap();

    let name = document.create_element("span").unwrap();
    name.set_text_content(Some(" did:qualia:alice"));
    name.set_attribute("style", "font-family: var(--mono); font-size: 10px; color: var(--text-dim);").unwrap();
    header.append_child(&name).unwrap();

    card.append_child(&header).unwrap();

    let meta = document.create_element("div").unwrap();
    meta.set_class_name("cr-meta");
    meta.set_text_content(Some("Requested: soc:friendship · ZKP: age proof, identity uniqueness"));
    card.append_child(&meta).unwrap();

    // Risk indicators
    let risk = document.create_element("div").unwrap();
    risk.set_class_name("cr-meta");
    let ri1 = document.create_element("span").unwrap();
    ri1.set_class_name("risk-indicator risk-low");
    ri1.set_text_content(Some("new-account"));
    let ri2 = document.create_element("span").unwrap();
    ri2.set_class_name("risk-indicator risk-moderate");
    ri2.set_text_content(Some("no-shared-contacts"));
    risk.append_child(&ri1).unwrap();
    risk.append_child(&ri2).unwrap();
    risk.append_child(&document.create_text_node(" Risk: low")).unwrap();
    card.append_child(&risk).unwrap();

    // Actions
    let actions = document.create_element("div").unwrap();
    actions.set_class_name("cr-actions");
    let accept = document.create_element("button").unwrap();
    accept.set_class_name("cr-btn");
    accept.set_text_content(Some("Accept"));
    let decline = document.create_element("button").unwrap();
    decline.set_class_name("cr-btn danger");
    decline.set_text_content(Some("Decline"));
    actions.append_child(&accept).unwrap();
    actions.append_child(&decline).unwrap();
    card.append_child(&actions).unwrap();

    wrapper.append_child(&card).unwrap();

    // Second demo card — blocked
    let card2 = document.create_element("div").unwrap();
    card2.set_class_name("cr-card");
    let h2 = document.create_element("div").unwrap();
    h2.set_class_name("cr-header");
    let s2 = document.create_element("span").unwrap();
    s2.set_class_name("cr-status cr-status-blocked");
    s2.set_text_content(Some("blocked"));
    h2.append_child(&s2).unwrap();
    let n2 = document.create_element("span").unwrap();
    n2.set_text_content(Some(" did:qualia:unknown"));
    n2.set_attribute("style", "font-family: var(--mono); font-size: 10px; color: var(--text-dim);").unwrap();
    h2.append_child(&n2).unwrap();
    card2.append_child(&h2).unwrap();

    let m2 = document.create_element("div").unwrap();
    m2.set_class_name("cr-meta");
    m2.set_text_content(Some("Requested: soc:friendship · Auto-blocked: critical risk"));
    card2.append_child(&m2).unwrap();

    let r2 = document.create_element("div").unwrap();
    r2.set_class_name("cr-meta");
    let ri3 = document.create_element("span").unwrap();
    ri3.set_class_name("risk-indicator risk-high");
    ri3.set_text_content(Some("grooming-pattern"));
    let ri4 = document.create_element("span").unwrap();
    ri4.set_class_name("risk-indicator risk-high");
    ri4.set_text_content(Some("identity-mismatch"));
    r2.append_child(&ri3).unwrap();
    r2.append_child(&ri4).unwrap();
    r2.append_child(&document.create_text_node(" Risk: critical")).unwrap();
    card2.append_child(&r2).unwrap();

    wrapper.append_child(&card2).unwrap();

    wrapper
}

fn build_protection_policies_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();

    // Demo: minor protection
    let card = document.create_element("div").unwrap();
    card.set_class_name("pp-card");

    let cat = document.create_element("div").unwrap();
    cat.set_class_name("pp-category");
    cat.set_text_content(Some("Minor Protection"));
    let mand = document.create_element("span").unwrap();
    mand.set_class_name("pp-mandatory");
    mand.set_text_content(Some("mandatory"));
    cat.append_child(&mand).unwrap();
    card.append_child(&cat).unwrap();

    let rows = [
        ("Approval mode", "always-required"),
        ("Max disclosure", "acquaintance"),
        ("Monitoring", "active"),
        ("Alert triggers", "grooming, bullying, adult-contact, isolation"),
    ];
    for (k, v) in &rows {
        let row = document.create_element("div").unwrap();
        row.set_class_name("pp-row");
        row.set_text_content(Some(&format!("{}: {}", k, v)));
        card.append_child(&row).unwrap();
    }
    wrapper.append_child(&card).unwrap();

    // Demo: DV survivor protection
    let card2 = document.create_element("div").unwrap();
    card2.set_class_name("pp-card");
    let cat2 = document.create_element("div").unwrap();
    cat2.set_class_name("pp-category");
    cat2.set_text_content(Some("DV Survivor Protection"));
    let opt = document.create_element("span").unwrap();
    opt.set_class_name("honesty-badge honesty-partial");
    opt.set_text_content(Some("opt-in"));
    cat2.append_child(&opt).unwrap();
    card2.append_child(&cat2).unwrap();

    let rows2 = [
        ("Approval mode", "network-based"),
        ("Max disclosure", "personal"),
        ("Monitoring", "passive"),
        ("Alert triggers", "coercive-control, location-sharing, isolation"),
    ];
    for (k, v) in &rows2 {
        let row = document.create_element("div").unwrap();
        row.set_class_name("pp-row");
        row.set_text_content(Some(&format!("{}: {}", k, v)));
        card2.append_child(&row).unwrap();
    }
    wrapper.append_child(&card2).unwrap();

    wrapper
}

fn build_right_dock(document: &Document) -> Element {
    let dock = document.create_element("div").unwrap();
    dock.set_class_name("right-dock");

    // Inspector panel
    let panel = document.create_element("div").unwrap();
    panel.set_class_name("dock-panel");
    let header = document.create_element("div").unwrap();
    header.set_class_name("dock-panel-header");
    header.set_text_content(Some("Inspector"));
    panel.append_child(&header).unwrap();
    let body = document.create_element("div").unwrap();
    body.set_class_name("dock-panel-body");
    body.set_text_content(Some("Select a container to inspect its semantic structure, provenance, and ontology context."));
    panel.append_child(&body).unwrap();
    dock.append_child(&panel).unwrap();

    // Aura tray
    let panel2 = document.create_element("div").unwrap();
    panel2.set_class_name("dock-panel");
    let header2 = document.create_element("div").unwrap();
    header2.set_class_name("dock-panel-header");
    header2.set_text_content(Some("Aura Tray"));
    panel2.append_child(&header2).unwrap();
    let body2 = document.create_element("div").unwrap();
    body2.set_class_name("dock-panel-body");
    body2.set_text_content(Some("SHACL shapes and validation results for the active container."));
    panel2.append_child(&body2).unwrap();
    dock.append_child(&panel2).unwrap();

    dock
}

fn build_bottom_dock(document: &Document) -> Element {
    let dock = document.create_element("div").unwrap();
    dock.set_class_name("bottom-dock");

    let header = document.create_element("div").unwrap();
    header.set_class_name("bottom-dock-header");
    header.set_text_content(Some("Pulse"));
    dock.append_child(&header).unwrap();

    let body = document.create_element("div").unwrap();
    body.set_class_name("bottom-dock-body");
    body.set_text_content(Some("[pulse] topic:social · notification · connection request received from did:qualia:alice\n[pulse] topic:social · telemetry · risk assessment complete: low\n[pulse] topic:protection · notification · grooming-pattern alert suppressed (no active policy)"));
    dock.append_child(&body).unwrap();

    dock
}

fn container_icon(container_type: &str) -> String {
    match container_type {
        "social" => "👥",
        "connection-requests" => "🔗",
        "reputation" => "⭐",
        "settings" => "⚙",
        "capabilities" => "🔐",
        "protection-policies" => "🛡",
        "conversations" => "💬",
        "channels" => "📡",
        "presence" => "🟢",
        "map" => "🗺",
        "media" => "🎬",
        "code" => "⌨",
        "doc" => "📄",
        _ => "📦",
    }
    .into()
}

fn wire_manifold_tabs(document: &Document, seeds: &[ManifoldSeed]) {
    let tabs = document.query_selector_all(".manifold-tab").unwrap();
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

    // Also wire sidebar items
    let items = document.query_selector_all(".sidebar-item[data-manifold]").unwrap();
    for i in 0..items.length() {
        let item = items.get(i).unwrap();
        let item_el: Element = item.dyn_into().unwrap();
        let manifold_id = item_el.get_attribute("data-manifold").unwrap();

        let seeds_clone: Vec<ManifoldSeed> = seeds.to_vec();
        let closure = Closure::wrap(Box::new(move || {
            switch_manifold(&manifold_id, &seeds_clone);
        }) as Box<dyn FnMut()>);

        item_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

fn switch_manifold(manifold_id: &str, seeds: &[ManifoldSeed]) {
    let document = web_sys::window().unwrap().document().unwrap();

    // Update tab active states
    let tabs = document.query_selector_all(".manifold-tab").unwrap();
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

    // Replace canvas content
    let seed = seeds.iter().find(|s| s.id == manifold_id);
    if let Some(seed) = seed {
        if let Some(canvas) = document.get_element_by_id("manifold-canvas") {
            canvas.set_inner_html("");
            let grid = document.create_element("div").unwrap();
            grid.set_class_name("canvas-grid");
            canvas.append_child(&grid).unwrap();

            for container in &seed.containers {
                let el = build_container(&document, container);
                canvas.append_child(&el).unwrap();
            }
        }
    }
}
