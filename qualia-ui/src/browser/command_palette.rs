//! Command palette / omnibox — Ctrl+K palette for quick navigation.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement, KeyboardEvent};

/// Build the command palette overlay (hidden by default).
pub fn build_command_palette(document: &Document) -> Element {
    let overlay = document.create_element("div").unwrap();
    overlay.set_id("command-palette");
    let overlay_el: HtmlElement = overlay.clone().dyn_into().unwrap();
    overlay_el.style().set_css_text(
        "position: fixed; top: 0; left: 0; width: 100%; height: 100%; \
         background: rgba(0,0,0,0.6); z-index: 10000; display: none; \
         align-items: flex-start; justify-content: center; padding-top: 120px;"
    );

    let panel = document.create_element("div").unwrap();
    panel.set_class_name("cmd-palette-panel");
    let panel_el: HtmlElement = panel.clone().dyn_into().unwrap();
    panel_el.style().set_css_text(
        "width: 560px; max-height: 400px; background: var(--glass-bg); \
         border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); \
         backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); \
         box-shadow: 0 8px 32px rgba(0,0,0,0.4); overflow: hidden; \
         display: flex; flex-direction: column;"
    );

    // Search input
    let input = document.create_element("input").unwrap();
    let input_el: HtmlInputElement = input.clone().dyn_into().unwrap();
    input_el.set_placeholder("Search commands, manifolds, containers\u{2026}");
    input.set_id("cmd-palette-input");
    input.set_attribute("style",
        "width: 100%; box-sizing: border-box; padding: 14px 16px; \
         background: transparent; border: none; border-bottom: 1px solid var(--border-subtle); \
         color: var(--text-primary); font-size: 14px; font-family: var(--font-mono); \
         outline: none;"
    ).unwrap();
    panel.append_child(&input).unwrap();

    // Results list
    let results = document.create_element("div").unwrap();
    results.set_id("cmd-palette-results");
    let results_el: HtmlElement = results.clone().dyn_into().unwrap();
    results_el.style().set_css_text(
        "flex: 1; overflow-y: auto; padding: 4px 0;"
    );

    let commands = build_command_list();
    for (idx, cmd) in commands.iter().enumerate() {
        let item = document.create_element("div").unwrap();
        item.set_class_name("cmd-palette-item");
        if idx == 0 {
            item.class_list().add_1("selected").unwrap();
        }
        let item_el: HtmlElement = item.clone().dyn_into().unwrap();
        item_el.style().set_css_text(
            "padding: 8px 16px; cursor: pointer; display: flex; \
             align-items: center; gap: 8px; font-size: 12px; \
             color: var(--text-secondary); font-family: var(--font-mono);"
        );

        let icon = document.create_element("span").unwrap();
        icon.set_text_content(Some(cmd.icon));
        let icon_el: HtmlElement = icon.clone().dyn_into().unwrap();
        icon_el.style().set_css_text("color: var(--accent-cyan); width: 20px; text-align: center;");
        item.append_child(&icon).unwrap();

        let label = document.create_element("span").unwrap();
        label.set_text_content(Some(cmd.label));
        item.append_child(&label).unwrap();

        let shortcut = document.create_element("span").unwrap();
        shortcut.set_text_content(Some(cmd.shortcut));
        let sc_el: HtmlElement = shortcut.clone().dyn_into().unwrap();
        sc_el.style().set_css_text("margin-left: auto; color: var(--text-muted); font-size: 10px;");
        item.append_child(&shortcut).unwrap();

        results.append_child(&item).unwrap();
    }

    panel.append_child(&results).unwrap();
    overlay.append_child(&panel).unwrap();

    overlay
}

/// Wire up Ctrl+K to toggle the command palette.
pub fn wire_command_palette(document: &Document) {
    let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        if e.key() == "k" && (e.ctrl_key() || e.meta_key()) {
            e.prevent_default();
            let palette = document.get_element_by_id("command-palette");
            if let Some(p) = palette {
                let p_el: HtmlElement = p.dyn_into().unwrap();
                let display = p_el.style().get_property_value("display").unwrap_or_default();
                if display == "none" {
                    p_el.style().set_property("display", "flex").unwrap();
                    if let Some(input) = document.get_element_by_id("cmd-palette-input") {
                        let input_el: HtmlInputElement = input.dyn_into().unwrap();
                        input_el.focus().unwrap();
                    }
                } else {
                    p_el.style().set_property("display", "none").unwrap();
                }
            }
        }

        // Escape to close
        if e.key() == "Escape" {
            if let Some(p) = document.get_element_by_id("command-palette") {
                let p_el: HtmlElement = p.dyn_into().unwrap();
                p_el.style().set_property("display", "none").unwrap();
            }
        }

        // Ctrl+Z / Ctrl+Y — undo/redo (visual feedback only at this stage)
        if (e.key() == "z" || e.key() == "Z") && (e.ctrl_key() || e.meta_key()) && !e.shift_key() {
            e.prevent_default();
            if let Some(status) = document.get_element_by_id("status-bar") {
                let status_el: HtmlElement = status.dyn_into().unwrap();
                status_el.set_text_content(Some("\u{21A9} Undo \u{2014} history stack not yet wired to canvas state"));
            }
        }
        if (e.key() == "y" || e.key() == "Y") && (e.ctrl_key() || e.meta_key()) {
            e.prevent_default();
            if let Some(status) = document.get_element_by_id("status-bar") {
                let status_el: HtmlElement = status.dyn_into().unwrap();
                status_el.set_text_content(Some("\u{21AA} Redo \u{2014} history stack not yet wired to canvas state"));
            }
        }
        // Ctrl+Shift+Z also does redo
        if (e.key() == "z" || e.key() == "Z") && (e.ctrl_key() || e.meta_key()) && e.shift_key() {
            e.prevent_default();
            if let Some(status) = document.get_element_by_id("status-bar") {
                let status_el: HtmlElement = status.dyn_into().unwrap();
                status_el.set_text_content(Some("\u{21AA} Redo \u{2014} history stack not yet wired to canvas state"));
            }
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

// ---------------------------------------------------------------------------
// Command list
// ---------------------------------------------------------------------------

struct CommandEntry {
    icon: &'static str,
    label: &'static str,
    shortcut: &'static str,
}

fn build_command_list() -> Vec<CommandEntry> {
    vec![
        CommandEntry { icon: "\u{1F50C}", label: "New Document", shortcut: "Ctrl+N" },
        CommandEntry { icon: "\u{1F4CA}", label: "New Sheet", shortcut: "" },
        CommandEntry { icon: "\u{1F50D}", label: "Run SPARQL Query", shortcut: "" },
        CommandEntry { icon: "\u{1F3DB}", label: "Browse Ontology", shortcut: "" },
        CommandEntry { icon: "\u{1F4AC}", label: "Open Social Graph", shortcut: "" },
        CommandEntry { icon: "\u{1F4E1}", label: "Open Pulse Stream", shortcut: "" },
        CommandEntry { icon: "\u{1F50D}", label: "Open Aura (SHACL)", shortcut: "" },
        CommandEntry { icon: "\u{1F4CD}", label: "Open Map (GIS)", shortcut: "" },
        CommandEntry { icon: "\u{1F4A1}", label: "Open VibeScript Console", shortcut: "" },
        CommandEntry { icon: "\u{1F510}", label: "Open Rights & Agreements", shortcut: "" },
        CommandEntry { icon: "\u{1F4B0}", label: "Open Wallet", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Research", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Social", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Knowledge", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Projects", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Rights", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Sanctuary", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Media", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Communications", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Settings", shortcut: "" },
        CommandEntry { icon: "\u{2B07}", label: "Switch Manifold: Vibe", shortcut: "" },
    ]
}
