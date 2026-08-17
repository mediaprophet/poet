//! Additional container body views: doc, sheet, graph, ontology, pulse, rights, wallet.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

/// Rich text document container — contenteditable with gazetteer placeholder.
pub fn build_doc_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 6px;");

    // Toolbar
    let toolbar = document.create_element("div").unwrap();
    toolbar.set_class_name("vibe-toolbar");
    for label in &["B", "I", "U", "Link", "Annotate", "Gazetteer"] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("vibe-run-btn");
        btn.set_text_content(Some(label));
        toolbar.append_child(&btn).unwrap();
    }
    wrapper.append_child(&toolbar).unwrap();

    // Editor area
    let editor = document.create_element("div").unwrap();
    let editor_el: HtmlElement = editor.clone().dyn_into().unwrap();
    editor_el.style().set_css_text(
        "flex: 1; background: var(--canvas-bg); border: 1px solid var(--border-subtle); \
         border-radius: var(--radius-xs); padding: 12px; color: var(--text-primary); \
         font-size: 13px; line-height: 1.6; overflow-y: auto; min-height: 120px; \
         contenteditable: true;"
    );
    editor.set_text_content(Some(
        "Rich text document container \u{2014} contenteditable with gazetteer annotation. \
         Select text to annotate with <q-entity> tags. Awaiting NLP engine wiring."
    ));
    wrapper.append_child(&editor).unwrap();

    wrapper
}

/// Spreadsheet container — formula bar + grid.
pub fn build_sheet_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 4px;");

    // Formula bar
    let formula = document.create_element("div").unwrap();
    formula.set_class_name("vibe-toolbar");
    let fx_label = document.create_element("span").unwrap();
    fx_label.set_text_content(Some("fx"));
    let fx_label_el: HtmlElement = fx_label.clone().dyn_into().unwrap();
    fx_label_el.style().set_css_text("color: var(--accent-cyan); font-family: var(--font-mono); font-size: 11px; font-weight: 700;");
    formula.append_child(&fx_label).unwrap();

    let input = document.create_element("input").unwrap();
    let input_el: web_sys::HtmlInputElement = input.clone().dyn_into().unwrap();
    input_el.set_placeholder("=SUM(A1:A10)");
    input.set_attribute("style", "flex: 1; background: var(--canvas-bg); border: 1px solid var(--border-subtle); border-radius: var(--radius-xs); padding: 4px 8px; color: var(--accent-emerald); font-family: var(--font-mono); font-size: 11px;").unwrap();
    formula.append_child(&input).unwrap();
    wrapper.append_child(&formula).unwrap();

    // Grid
    let grid = document.create_element("div").unwrap();
    let grid_el: HtmlElement = grid.clone().dyn_into().unwrap();
    grid_el.style().set_css_text(
        "flex: 1; overflow: auto; background: var(--canvas-bg); border: 1px solid var(--border-subtle); \
         border-radius: var(--radius-xs); font-family: var(--font-mono); font-size: 10px;"
    );

    // Header row
    let header_row = document.create_element("div").unwrap();
    let header_el: HtmlElement = header_row.clone().dyn_into().unwrap();
    header_el.style().set_css_text("display: flex; border-bottom: 1px solid var(--border-subtle);");
    for col in &["", "A", "B", "C", "D", "E"] {
        let cell = document.create_element("div").unwrap();
        let cell_el: HtmlElement = cell.clone().dyn_into().unwrap();
        cell_el.style().set_css_text("min-width: 60px; padding: 3px 6px; text-align: center; color: var(--text-muted); border-right: 1px solid var(--border-subtle);");
        cell.set_text_content(Some(col));
        header_row.append_child(&cell).unwrap();
    }
    grid.append_child(&header_row).unwrap();

    // Data rows
    for row_idx in 1..=5 {
        let row = document.create_element("div").unwrap();
        let row_el: HtmlElement = row.clone().dyn_into().unwrap();
        row_el.style().set_css_text("display: flex; border-bottom: 1px solid var(--border-subtle);");
        let row_label = document.create_element("div").unwrap();
        let rl_el: HtmlElement = row_label.clone().dyn_into().unwrap();
        rl_el.style().set_css_text("min-width: 60px; padding: 3px 6px; text-align: center; color: var(--text-muted); border-right: 1px solid var(--border-subtle);");
        row_label.set_text_content(Some(&row_idx.to_string()));
        row.append_child(&row_label).unwrap();
        for col_idx in 0..5 {
            let cell = document.create_element("div").unwrap();
            let cell_el: HtmlElement = cell.clone().dyn_into().unwrap();
            cell_el.style().set_css_text("min-width: 60px; padding: 3px 6px; color: var(--text-secondary); border-right: 1px solid var(--border-subtle);");
            if row_idx == 1 && col_idx == 0 { cell.set_text_content(Some("42")); }
            else if row_idx == 2 && col_idx == 0 { cell.set_text_content(Some("18")); }
            else if row_idx == 3 && col_idx == 0 { cell.set_text_content(Some("=A1+A2")); }
            row.append_child(&cell).unwrap();
        }
        grid.append_child(&row).unwrap();
    }
    wrapper.append_child(&grid).unwrap();

    wrapper
}

/// Graph/SPARQL explorer container.
pub fn build_graph_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 6px;");

    // SPARQL query input
    let query_bar = document.create_element("div").unwrap();
    query_bar.set_class_name("vibe-toolbar");
    let run_btn = document.create_element("button").unwrap();
    run_btn.set_class_name("vibe-run-btn");
    run_btn.set_text_content(Some("\u{25B6} Run SPARQL"));
    query_bar.append_child(&run_btn).unwrap();
    wrapper.append_child(&query_bar).unwrap();

    let editor = document.create_element("div").unwrap();
    editor.set_class_name("vibe-editor");
    editor.set_text_content(Some(
        "PREFIX soc: <https://qualiadb.org/ontology/social#>\n\
         SELECT ?peer ?modality WHERE {\n\
         \x20\x20?s soc:hasPeer ?peer .\n\
         \x20\x20?s soc:epistemicModality ?modality .\n\
         } LIMIT 10"
    ));
    wrapper.append_child(&editor).unwrap();

    // Results
    let results = document.create_element("div").unwrap();
    results.set_class_name("vibe-output");
    let line1 = document.create_element("div").unwrap();
    line1.set_class_name("vibe-out-line");
    line1.set_text_content(Some("peer=\u{2014}did:qualia:alice, modality=\u{2014}objective"));
    let line2 = document.create_element("div").unwrap();
    line2.set_class_name("vibe-out-line");
    line2.set_text_content(Some("peer=\u{2014}did:qualia:bob, modality=\u{2014}subjective"));
    let line3 = document.create_element("div").unwrap();
    line3.set_class_name("vibe-out-line");
    line3.set_text_content(Some("\u{2139}\u{FE0F} 2 results \u{00B7} 12ms \u{00B7} CBOR-LD: 48 bytes"));
    results.append_child(&line1).unwrap();
    results.append_child(&line2).unwrap();
    results.append_child(&line3).unwrap();
    wrapper.append_child(&results).unwrap();

    wrapper
}

/// Ontology browser container — tree view of classes and properties.
pub fn build_ontology_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    wrapper.set_class_name("ontology-tree");

    let tree = document.create_element("div").unwrap();
    let tree_el: HtmlElement = tree.clone().dyn_into().unwrap();
    tree_el.style().set_css_text("overflow-y: auto; flex: 1; padding: 8px;");

    let entries = &[
        ("soc:", "Social", "class", 0),
        ("  soc:hasPeer", "hasPeer", "prop", 1),
        ("  soc:epistemicModality", "epistemicModality", "prop", 1),
        ("  soc:requestsConnection", "requestsConnection", "prop", 1),
        ("health:", "Health", "class", 0),
        ("  health:hasCondition", "hasCondition", "prop", 1),
        ("  health:hasMedication", "hasMedication", "prop", 1),
        ("geo:", "Geospatial", "class", 0),
        ("  geo:hasGeometry", "hasGeometry", "prop", 1),
        ("  geo:withinHull", "withinHull", "prop", 1),
        ("vibe:", "VibeScript", "class", 0),
        ("  vibe:Intent", "Intent", "class", 1),
        ("  vibe:Receipt", "Receipt", "class", 1),
    ];

    for (prefix, label, kind, depth) in entries {
        let node = document.create_element("div").unwrap();
        node.set_class_name("ontology-tree-node");
        let node_el: HtmlElement = node.clone().dyn_into().unwrap();
        node_el.style().set_css_text(&format!("padding-left: {}px;", depth * 16));

        let pfx = document.create_element("span").unwrap();
        pfx.set_class_name("ot-prefix");
        pfx.set_text_content(Some(prefix));
        node.append_child(&pfx).unwrap();

        let cls = document.create_element("span").unwrap();
        cls.set_class_name(if *kind == "class" { "ot-class" } else { "ot-prop" });
        cls.set_text_content(Some(label));
        node.append_child(&cls).unwrap();

        tree.append_child(&node).unwrap();
    }

    wrapper.append_child(&tree).unwrap();
    wrapper
}

/// Pulse stream container — event log with publish allowlist.
pub fn build_pulse_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 4px;");

    // Publish bar
    let bar = document.create_element("div").unwrap();
    bar.set_class_name("vibe-toolbar");
    let label = document.create_element("span").unwrap();
    label.set_text_content(Some("topic:"));
    let label_el: HtmlElement = label.clone().dyn_into().unwrap();
    label_el.style().set_css_text("color: var(--text-muted); font-size: 10px; font-family: var(--font-mono);");
    bar.append_child(&label).unwrap();
    let input = document.create_element("input").unwrap();
    let input_el: web_sys::HtmlInputElement = input.clone().dyn_into().unwrap();
    input_el.set_placeholder("pulse:topic:event");
    input.set_attribute("style", "flex: 1; background: var(--canvas-bg); border: 1px solid var(--border-subtle); border-radius: var(--radius-xs); padding: 3px 6px; color: var(--accent-cyan); font-family: var(--font-mono); font-size: 10px;").unwrap();
    bar.append_child(&input).unwrap();
    let btn = document.create_element("button").unwrap();
    btn.set_class_name("vibe-run-btn");
    btn.set_text_content(Some("Publish"));
    bar.append_child(&btn).unwrap();
    wrapper.append_child(&bar).unwrap();

    // Event log
    let log = document.create_element("div").unwrap();
    log.set_class_name("vibe-output");
    let events = &[
        "12:04:01 \u{00B7} pulse:social:connect \u{00B7} did:qualia:alice \u{2192} did:qualia:bob",
        "12:03:45 \u{00B7} pulse:graph:mutate \u{00B7} quin.statement #a42",
        "12:03:12 \u{00B7} pulse:aura:validate \u{00B7} SHACL shape:ok",
        "12:02:58 \u{00B7} pulse:telemetry:tick \u{00B7} wavefunction collapse",
    ];
    for ev in events {
        let line = document.create_element("div").unwrap();
        line.set_class_name("vibe-out-line");
        line.set_text_content(Some(ev));
        log.append_child(&line).unwrap();
    }
    wrapper.append_child(&log).unwrap();

    wrapper
}

/// Rights container — agreements, DID signing, fiduciary.
pub fn build_rights_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 8px;");

    // Agreement cards
    for (title, party, status) in &[
        ("Data Sharing Agreement", "did:qualia:alice", "active"),
        ("Fiduciary Obligation", "did:qualia:guardian", "active"),
        ("Consent Revocation", "did:qualia:bob", "pending"),
    ] {
        let card = document.create_element("div").unwrap();
        card.set_class_name("cr-card");
        let h = document.create_element("div").unwrap();
        h.set_class_name("cr-header");
        let title_el = document.create_element("span").unwrap();
        title_el.set_class_name("cr-name");
        title_el.set_text_content(Some(title));
        h.append_child(&title_el).unwrap();
        let badge = document.create_element("span").unwrap();
        badge.set_class_name(&format!("honesty-badge honesty-{}", status));
        badge.set_text_content(Some(status));
        h.append_child(&badge).unwrap();
        card.append_child(&h).unwrap();

        let meta = document.create_element("div").unwrap();
        meta.set_class_name("cr-meta");
        meta.set_text_content(Some(&format!("Party: {} \u{00B7} Deontic: obligation", party)));
        card.append_child(&meta).unwrap();

        wrapper.append_child(&card).unwrap();
    }

    // Sign button
    let sign_bar = document.create_element("div").unwrap();
    sign_bar.set_class_name("vibe-toolbar");
    let sign_btn = document.create_element("button").unwrap();
    sign_btn.set_class_name("vibe-run-btn");
    sign_btn.set_text_content(Some("\u{270D} DID Sign"));
    sign_bar.append_child(&sign_btn).unwrap();
    let fid_btn = document.create_element("button").unwrap();
    fid_btn.set_class_name("vibe-run-btn");
    fid_btn.set_text_content(Some("\u{1F4DC} Fiduciary Sign"));
    sign_bar.append_child(&fid_btn).unwrap();
    wrapper.append_child(&sign_bar).unwrap();

    wrapper
}

/// Wallet container — identity, balances, vault.
pub fn build_wallet_view(document: &Document) -> Element {
    let wrapper = document.create_element("div").unwrap();
    let wrapper_el: HtmlElement = wrapper.clone().dyn_into().unwrap();
    wrapper_el.style().set_css_text("display: flex; flex-direction: column; flex: 1; gap: 8px;");

    // Identity card
    let id_card = document.create_element("div").unwrap();
    id_card.set_class_name("cr-card");
    let id_h = document.create_element("div").unwrap();
    id_h.set_class_name("cr-header");
    let id_title = document.create_element("span").unwrap();
    id_title.set_class_name("cr-name");
    id_title.set_text_content(Some("Identity"));
    id_h.append_child(&id_title).unwrap();
    let id_badge = document.create_element("span").unwrap();
    id_badge.set_class_name("honesty-badge honesty-live");
    id_badge.set_text_content(Some("live"));
    id_h.append_child(&id_badge).unwrap();
    id_card.append_child(&id_h).unwrap();
    let id_meta = document.create_element("div").unwrap();
    id_meta.set_class_name("cr-meta");
    id_meta.set_text_content(Some("DID: did:qualia:timothy_charles_holborn \u{00B7} BIP39: 12 words"));
    id_card.append_child(&id_meta).unwrap();
    wrapper.append_child(&id_card).unwrap();

    // Balance table
    let balance = document.create_element("div").unwrap();
    balance.set_class_name("vibe-output");
    for (asset, amount) in &[("XEC", "1,250.00"), ("USDC", "340.12"), ("Q42", "8,000")] {
        let line = document.create_element("div").unwrap();
        line.set_class_name("vibe-out-line");
        line.set_text_content(Some(&format!("{}: {}", asset, amount)));
        balance.append_child(&line).unwrap();
    }
    wrapper.append_child(&balance).unwrap();

    // Actions
    let actions = document.create_element("div").unwrap();
    actions.set_class_name("vibe-toolbar");
    for label in &["Send", "Receive", "Vault Handshake", "Export"] {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("vibe-run-btn");
        btn.set_text_content(Some(label));
        actions.append_child(&btn).unwrap();
    }
    wrapper.append_child(&actions).unwrap();

    wrapper
}
