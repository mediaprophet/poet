//! Top bar rendering: menubar + canvas control bar.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use web_sys::{Document, Element};
use crate::tool_chest::core::registry::ManifoldSeed;

/// Build the top menubar (File/Edit/View + brand + fiduciary badge).
pub fn build_top_menubar(document: &Document) -> Element {
    let bar = document.create_element("div").unwrap();
    bar.set_class_name("top-menubar");

    // Left: brand + menus
    let left = document.create_element("div").unwrap();
    left.set_class_name("menu-items-group");

    let brand = document.create_element("div").unwrap();
    brand.set_class_name("brand-icon");
    brand.set_text_content(Some("\u{1F30C}"));
    let brand_text = document.create_element("span").unwrap();
    brand_text.set_class_name("brand-text");
    brand_text.set_text_content(Some("QualiaDB"));
    brand.append_child(&brand_text).unwrap();
    left.append_child(&brand).unwrap();

    for label in &["File", "Edit", "View", "Insert", "Help"] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("menu-btn");
        btn.set_text_content(Some(label));
        left.append_child(&btn).unwrap();
    }
    bar.append_child(&left).unwrap();

    // Right: fiduciary badge
    let right = document.create_element("div").unwrap();
    right.set_class_name("menu-items-group");
    let badge = document.create_element("span").unwrap();
    badge.set_class_name("fiduciary-badge");
    badge.set_text_content(Some("\u{1F6E1} Fiduciary L3"));
    right.append_child(&badge).unwrap();
    bar.append_child(&right).unwrap();

    bar
}

/// Build the canvas control bar (manifold pager + strata + epistemic + dimension).
pub fn build_canvas_control_bar(document: &Document, seeds: &[ManifoldSeed]) -> Element {
    let bar = document.create_element("div").unwrap();
    bar.set_class_name("canvas-control-bar");

    // Title box
    let title_box = document.create_element("div").unwrap();
    title_box.set_class_name("canvas-title-box");
    let title = document.create_element("span").unwrap();
    title.set_class_name("canvas-title-input");
    title.set_text_content(Some("Social Governance"));
    title_box.append_child(&title).unwrap();

    let graph_badge = document.create_element("span").unwrap();
    graph_badge.set_class_name("graph-address-badge");
    graph_badge.set_text_content(Some("graph:manifold:social_01"));
    title_box.append_child(&graph_badge).unwrap();
    bar.append_child(&title_box).unwrap();

    // Pager
    let pager = document.create_element("div").unwrap();
    pager.set_class_name("virtual-desktop-pager");
    let desktops = document.create_element("div").unwrap();
    desktops.set_class_name("pager-desktops-list");

    for (i, seed) in seeds.iter().enumerate() {
        let tab = document.create_element("button").unwrap();
        tab.set_class_name("desktop-tab-btn");
        if i == 0 {
            tab.class_list().add_1("active").unwrap();
        }
        tab.set_attribute("data-manifold", &seed.id).unwrap();

        let num = document.create_element("span").unwrap();
        num.set_class_name("desktop-num");
        num.set_text_content(Some(&(i + 1).to_string()));

        let label = document.create_element("span").unwrap();
        label.set_text_content(Some(&format!(" {} {}", seed.icon, seed.label)));

        tab.append_child(&num).unwrap();
        tab.append_child(&label).unwrap();
        desktops.append_child(&tab).unwrap();
    }
    pager.append_child(&desktops).unwrap();
    bar.append_child(&pager).unwrap();

    // Strata selector
    let strata = document.create_element("div").unwrap();
    strata.set_class_name("strata-deck-selector");
    for (label, key, active) in &[
        ("All", "all", true), ("Social", "social", false), ("Legal", "legal", false),
    ] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("strata-deck-btn");
        btn.set_attribute("data-strata", key).unwrap();
        if *active { btn.class_list().add_1("active").unwrap(); }
        btn.set_text_content(Some(label));
        strata.append_child(&btn).unwrap();
    }
    bar.append_child(&strata).unwrap();

    // Epistemic lens
    let lens = document.create_element("div").unwrap();
    lens.set_class_name("epistemic-lens-selector");
    for (label, key, active) in &[
        ("All", "all", true), ("Obj", "objective", false),
        ("Subj", "subjective", false), ("Inter", "intersubjective", false),
    ] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("epistemic-btn");
        btn.set_attribute("data-epistemic", key).unwrap();
        if *active { btn.class_list().add_1("active").unwrap(); }
        btn.set_text_content(Some(label));
        lens.append_child(&btn).unwrap();
    }
    bar.append_child(&lens).unwrap();

    // Dimension switch
    let dim = document.create_element("div").unwrap();
    dim.set_class_name("dimension-switch");
    for (label, active) in &[("2D", true), ("3D", false), ("4D", false)] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("dim-btn");
        if *active { btn.class_list().add_1("active").unwrap(); }
        btn.set_text_content(Some(label));
        dim.append_child(&btn).unwrap();
    }
    bar.append_child(&dim).unwrap();

    bar
}
