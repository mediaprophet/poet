//! Dock rendering: toolbox sidebar, right dock (aura + pulse), bottom status bar.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use web_sys::{Document, Element, HtmlElement};
use wasm_bindgen::JsCast;

/// Build the toolbox dock (vertical icon sidebar).
pub fn build_toolbox_dock(document: &Document) -> Element {
    let dock = document.create_element("div").unwrap();
    dock.set_class_name("toolbox-dock");

    let toolboxes: &[(&str, &str, &str, bool)] = &[
        ("\u{1F4AC}", "Social Toolbox", "social", true),
        ("\u{2699}", "Settings Toolbox", "settings", false),
        ("\u{1F4E1}", "Communications Toolbox", "comm", false),
        ("\u{1F4C4}", "Office Toolbox", "office", false),
        ("\u{1F52C}", "Research Toolbox", "research", false),
        ("\u{1F3A8}", "Media Toolbox", "media", false),
    ];

    for (icon, label, id, active) in toolboxes {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("toolbox-dock-btn");
        if *active { btn.class_list().add_1("active").unwrap(); }
        btn.set_attribute("data-toolbox", id).unwrap();
        btn.set_text_content(Some(icon));

        let tooltip = document.create_element("span").unwrap();
        tooltip.set_class_name("dock-tooltip");
        tooltip.set_text_content(Some(label));
        btn.append_child(&tooltip).unwrap();

        dock.append_child(&btn).unwrap();
    }

    // Divider
    let divider = document.create_element("div").unwrap();
    divider.set_class_name("dock-divider");
    dock.append_child(&divider).unwrap();

    // Rights toolbox
    let rights = document.create_element("button").unwrap();
    rights.set_class_name("toolbox-dock-btn");
    rights.set_text_content(Some("\u{1F512}"));
    let rt = document.create_element("span").unwrap();
    rt.set_class_name("dock-tooltip");
    rt.set_text_content(Some("Rights Toolbox"));
    rights.append_child(&rt).unwrap();
    dock.append_child(&rights).unwrap();

    dock
}

/// Build the right dock (aura tray + pulse stream).
pub fn build_right_dock(document: &Document) -> Element {
    let dock = document.create_element("div").unwrap();
    dock.set_class_name("right-dock");

    // Aura Tray — wired to diagnostics module
    let aura = document.create_element("div").unwrap();
    aura.set_class_name("dock-panel");
    let aura_header = document.create_element("div").unwrap();
    aura_header.set_class_name("dock-panel-header");
    aura_header.set_text_content(Some("Aura Tray"));
    aura.append_child(&aura_header).unwrap();

    let shacl_results = super::diagnostics::default_shacl_results();
    let aura_body = super::diagnostics::render_aura_tray(document, &shacl_results);
    aura.append_child(&aura_body).unwrap();
    dock.append_child(&aura).unwrap();

    // Pulse Stream — wired to diagnostics module
    let pulse = document.create_element("div").unwrap();
    pulse.set_class_name("dock-panel");
    let pulse_el: HtmlElement = pulse.clone().dyn_into().unwrap();
    pulse_el.style().set_css_text("flex: 1; overflow: hidden; display: flex; flex-direction: column;");

    let pulse_header = document.create_element("div").unwrap();
    pulse_header.set_class_name("dock-panel-header");
    pulse_header.set_text_content(Some("Pulse Stream"));
    pulse.append_child(&pulse_header).unwrap();

    let pulse_events = super::diagnostics::default_pulse_events();
    let pulse_body = super::diagnostics::render_pulse_stream(document, &pulse_events);
    pulse.append_child(&pulse_body).unwrap();
    dock.append_child(&pulse).unwrap();

    // Job Center — background job queue
    let jobs = super::diagnostics::default_jobs();
    let job_panel = super::diagnostics::render_job_center(document, &jobs);
    dock.append_child(&job_panel).unwrap();

    dock
}

/// Build the bottom status bar.
pub fn build_bottom_statusbar(document: &Document) -> Element {
    let bar = document.create_element("div").unwrap();
    bar.set_class_name("bottom-statusbar");

    // Left section
    let left = document.create_element("div").unwrap();
    left.set_class_name("statusbar-section");

    let graph = document.create_element("div").unwrap();
    graph.set_class_name("statusbar-item");
    let g_label = document.create_element("span").unwrap();
    g_label.set_class_name("statusbar-label");
    g_label.set_text_content(Some("Graph:"));
    let g_val = document.create_element("span").unwrap();
    g_val.set_class_name("statusbar-value");
    g_val.set_text_content(Some("catchment_sites"));
    graph.append_child(&g_label).unwrap();
    graph.append_child(&g_val).unwrap();
    left.append_child(&graph).unwrap();

    let merkle = document.create_element("div").unwrap();
    merkle.set_class_name("statusbar-item");
    let m_label = document.create_element("span").unwrap();
    m_label.set_class_name("statusbar-label");
    m_label.set_text_content(Some("Merkle:"));
    let m_val = document.create_element("span").unwrap();
    m_val.set_class_name("statusbar-value");
    m_val.set_text_content(Some("0x8f...a42"));
    merkle.append_child(&m_label).unwrap();
    merkle.append_child(&m_val).unwrap();
    left.append_child(&merkle).unwrap();

    bar.append_child(&left).unwrap();

    // Right section
    let right = document.create_element("div").unwrap();
    right.set_class_name("statusbar-section");

    let gas = document.create_element("div").unwrap();
    gas.set_class_name("statusbar-item");
    let g_label = document.create_element("span").unwrap();
    g_label.set_class_name("statusbar-label");
    g_label.set_text_content(Some("Gas:"));
    let g_val = document.create_element("span").unwrap();
    g_val.set_class_name("statusbar-gas");
    g_val.set_text_content(Some("984,500 / 1M"));
    gas.append_child(&g_label).unwrap();
    gas.append_child(&g_val).unwrap();
    right.append_child(&gas).unwrap();

    let strata = document.create_element("div").unwrap();
    strata.set_class_name("statusbar-item");
    let s_label = document.create_element("span").unwrap();
    s_label.set_class_name("statusbar-label");
    s_label.set_text_content(Some("Strata:"));
    let s_val = document.create_element("span").unwrap();
    s_val.set_class_name("statusbar-value");
    s_val.set_text_content(Some("social, legal"));
    strata.append_child(&s_label).unwrap();
    strata.append_child(&s_val).unwrap();
    right.append_child(&strata).unwrap();

    bar.append_child(&right).unwrap();
    bar
}
