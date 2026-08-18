//! Contextual RDF popover — when the user selects text inside a document
//! container, a popover appears offering to annotate the selection with
//! an entity type. The annotation is rendered as a `<q-entity>` tag
//! wrapping the selected text, with a `data-entity-type` attribute.
//!
//! This is a UI-level mock of the NLP gazetteer annotation pipeline.
//! The actual NLP extraction (entity recognition, triple generation,
//! provenance tracking) is engine wiring pending.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, MouseEvent};

/// Entity types offered in the popover, aligned with
/// `ontologies/document.n3` §4 (Context Markup) markup types.
const ENTITY_TYPES: &[(&str, &str, &str)] = &[
    ("term", "\u{1F539}", "Term"),
    ("entity", "\u{1F4CD}", "Named Entity"),
    ("claimedFact", "\u{1F4AF}", "Claimed Fact"),
    ("statement", "\u{1F4DD}", "Statement"),
    ("statistic", "\u{1F4CA}", "Statistic"),
    ("citation", "\u{1F4D1}", "Citation"),
    ("definition", "\u{1F4D8}", "Definition"),
    ("quote", "\u{1F4AC}", "Quote"),
];

/// Wire the contextual RDF popover on all contenteditable elements.
/// Call this after the document is rendered and whenever new document
/// containers are placed.
pub fn wire_contextual_popover(document: &Document) {
    let editors = document
        .query_selector_all(".doc-editor[contenteditable=\"true\"]")
        .unwrap();
    for i in 0..editors.length() {
        let editor = editors.get(i).unwrap();
        let editor_el: Element = editor.dyn_into().unwrap();

        let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
            let window = web_sys::window().unwrap();
            let doc = window.document().unwrap();
            let sel = match window.get_selection() {
                Ok(Some(s)) => s,
                _ => return,
            };

            // Check if there's a non-empty text selection
            let text: String = sel.to_string().as_string().unwrap_or_default();
            if text.trim().is_empty() {
                hide_popover(&doc);
                return;
            }

            // Check that the selection is within this contenteditable
            let anchor_node = match sel.anchor_node() {
                Some(n) => n,
                None => return,
            };
            let anchor_el = match anchor_node.parent_element() {
                Some(e) => e,
                None => return,
            };
            if !anchor_el.closest(".doc-editor").unwrap().is_some() {
                hide_popover(&doc);
                return;
            }

            // Position the popover near the mouse
            show_popover(&doc, e.client_x() as i32, e.client_y() as i32, &text);
        }) as Box<dyn FnMut(MouseEvent)>);

        editor_el
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // Hide popover when clicking outside
    let doc_clone = document.clone();
    let dismiss_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
        let target: Element = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
            Some(t) => t,
            None => return,
        };
        // Don't hide if clicking inside the popover
        if target.closest("#rdf-popover").unwrap().is_some() {
            return;
        }
        hide_popover(&doc_clone);
    }) as Box<dyn FnMut(MouseEvent)>);
    document
        .add_event_listener_with_callback("mousedown", dismiss_closure.as_ref().unchecked_ref())
        .unwrap();
    dismiss_closure.forget();
}

/// Show the RDF annotation popover at the given coordinates.
fn show_popover(document: &Document, x: i32, y: i32, selected_text: &str) {
    hide_popover(document);

    let popover = document.create_element("div").unwrap();
    popover.set_id("rdf-popover");
    let pop_el: HtmlElement = popover.clone().dyn_into().unwrap();

    // Position above and to the right of the selection point
    let px = x + 8;
    let py = y - 8;
    pop_el.style().set_css_text(&format!(
        "position: fixed; left: {}px; top: {}px; \
         background: var(--surface-glass-heavy); backdrop-filter: blur(20px); \
         border: 1px solid var(--border-medium); border-radius: var(--radius-sm); \
         box-shadow: var(--shadow-lg); z-index: 8000; \
         display: flex; flex-direction: column; gap: 0; \
         min-width: 200px; max-width: 280px; overflow: hidden; \
         transform: translateY(-100%);",
        px, py
    ));

    // Header
    let header = document.create_element("div").unwrap();
    let h_el: HtmlElement = header.clone().dyn_into().unwrap();
    h_el.style().set_css_text(
        "padding: 6px 10px; font-size: 9px; font-weight: 700; \
         color: var(--accent-cyan); text-transform: uppercase; \
         letter-spacing: 0.5px; border-bottom: 1px solid var(--border-subtle); \
         font-family: var(--font-mono);",
    );
    header.set_text_content(Some("\u{1F50D} Annotate Selection"));
    popover.append_child(&header).unwrap();

    // Selected text preview
    let preview = document.create_element("div").unwrap();
    let p_el: HtmlElement = preview.clone().dyn_into().unwrap();
    p_el.style().set_css_text(
        "padding: 4px 10px; font-size: 10px; color: var(--text-secondary); \
         font-style: italic; border-bottom: 1px solid var(--border-subtle); \
         max-height: 40px; overflow: hidden; font-family: var(--font-mono);",
    );
    let preview_text = if selected_text.len() > 60 {
        &selected_text[..60]
    } else {
        selected_text
    };
    preview.set_text_content(Some(&format!(
        "\u{201C}{}\u{2026}\u{201D}",
        preview_text.trim()
    )));
    popover.append_child(&preview).unwrap();

    // Entity type buttons
    for (type_id, icon, label) in ENTITY_TYPES {
        let btn = document.create_element("button").unwrap();
        btn.set_class_name("rdf-popover-btn");
        btn.set_attribute("data-entity-type", type_id).unwrap();
        let b_el: HtmlElement = btn.clone().dyn_into().unwrap();
        b_el.style().set_css_text(
            "padding: 6px 10px; background: transparent; border: none; \
             color: var(--text-secondary); font-size: 11px; font-family: var(--font-mono); \
             cursor: pointer; display: flex; align-items: center; gap: 8px; \
             text-align: left; transition: var(--trans-fast); width: 100%;",
        );

        let ic = document.create_element("span").unwrap();
        ic.set_text_content(Some(icon));
        let ic_el: HtmlElement = ic.clone().dyn_into().unwrap();
        ic_el
            .style()
            .set_css_text("width: 16px; text-align: center; font-size: 12px;");
        btn.append_child(&ic).unwrap();

        let lbl = document.create_element("span").unwrap();
        lbl.set_text_content(Some(label));
        btn.append_child(&lbl).unwrap();

        // Wire click to annotate
        let type_id_str = type_id.to_string();
        let label_str = label.to_string();
        let click_closure = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let doc = web_sys::window().unwrap().document().unwrap();
            annotate_selection(&doc, &type_id_str, &label_str);
        }) as Box<dyn FnMut(MouseEvent)>);
        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .unwrap();
        click_closure.forget();

        // Hover effect
        let btn_clone = btn.clone();
        let hover_closure = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let b: HtmlElement = btn_clone.clone().dyn_into().unwrap();
            b.style()
                .set_property("background", "rgba(0,255,170,0.06)")
                .unwrap();
            b.style()
                .set_property("color", "var(--text-primary)")
                .unwrap();
        }) as Box<dyn FnMut(MouseEvent)>);
        btn.add_event_listener_with_callback("mouseenter", hover_closure.as_ref().unchecked_ref())
            .unwrap();
        hover_closure.forget();

        let btn_clone2 = btn.clone();
        let leave_closure = Closure::wrap(Box::new(move |_e: MouseEvent| {
            let b: HtmlElement = btn_clone2.clone().dyn_into().unwrap();
            b.style().set_property("background", "transparent").unwrap();
            b.style()
                .set_property("color", "var(--text-secondary)")
                .unwrap();
        }) as Box<dyn FnMut(MouseEvent)>);
        btn.add_event_listener_with_callback("mouseleave", leave_closure.as_ref().unchecked_ref())
            .unwrap();
        leave_closure.forget();

        popover.append_child(&btn).unwrap();
    }

    // Honesty note
    let note = document.create_element("div").unwrap();
    let n_el: HtmlElement = note.clone().dyn_into().unwrap();
    n_el.style().set_css_text(
        "padding: 4px 10px; font-size: 8px; color: var(--text-muted); \
         border-top: 1px solid var(--border-subtle); font-family: var(--font-mono);",
    );
    note.set_text_content(Some(
        "UI annotation \u{2014} NLP extraction, triple generation, \
         and provenance tracking are engine wiring pending.",
    ));
    popover.append_child(&note).unwrap();

    if let Some(body) = document.body() {
        body.append_child(&popover).unwrap();
    }
}

/// Annotate the current selection with a `<q-entity>` tag.
fn annotate_selection(document: &Document, entity_type: &str, entity_label: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let sel = match window.get_selection() {
        Ok(Some(s)) => s,
        _ => return,
    };

    if sel.range_count() == 0 {
        return;
    }

    let range = match sel.get_range_at(0) {
        Ok(r) => r,
        Err(_) => return,
    };

    // Create the q-entity wrapper element
    let q_entity = document.create_element("q-entity").unwrap();
    q_entity
        .set_attribute("data-entity-type", entity_type)
        .unwrap();
    q_entity
        .set_attribute("data-entity-label", entity_label)
        .unwrap();
    q_entity
        .set_attribute("title", &format!("{}: {}", entity_label, entity_type))
        .unwrap();

    // Style it as an annotated span
    let q_el: HtmlElement = q_entity.clone().dyn_into().unwrap();
    let color = match entity_type {
        "term" => "var(--accent-cyan)",
        "entity" => "var(--accent-violet)",
        "claimedFact" => "var(--accent-amber)",
        "statement" => "var(--accent-emerald)",
        "statistic" => "var(--accent-cyan)",
        "citation" => "var(--accent-violet)",
        "definition" => "var(--accent-emerald)",
        "quote" => "var(--accent-amber)",
        _ => "var(--accent-cyan)",
    };
    q_el.style().set_css_text(&format!(
        "border-bottom: 2px dotted {}; cursor: help; \
         background: {}22; border-radius: 2px; padding: 0 2px;",
        color, color
    ));

    // Surround the selection contents with the q-entity element
    range.surround_contents(&q_entity).unwrap();

    // Clear the selection
    sel.remove_all_ranges().unwrap();

    // Hide the popover
    hide_popover(document);

    // Show notification
    show_annotation_notification(document, entity_label);

    // Push a history frame
    super::history::push_current_frame("annotate entity");
}

/// Show a brief notification after annotation.
fn show_annotation_notification(document: &Document, entity_label: &str) {
    let notif = document.create_element("div").unwrap();
    let n_el: HtmlElement = notif.clone().dyn_into().unwrap();
    n_el.style().set_css_text(
        "position: fixed; bottom: 40px; right: 16px; \
         background: var(--surface-panel-elevated); \
         border: 1px solid var(--border-medium); border-radius: var(--radius-sm); \
         padding: 10px 14px; font-size: 12px; color: var(--text-primary); \
         box-shadow: var(--shadow-lg); z-index: 9000; max-width: 320px;",
    );
    notif.set_text_content(Some(&format!(
        "\u{1F50D} Annotated as {} \u{2014} <q-entity> tag inserted. \
         NLP extraction pending.",
        entity_label
    )));
    if let Some(body) = document.body() {
        body.append_child(&notif).unwrap();
    }
    let notif_clone = notif.clone();
    let timeout = Closure::wrap(Box::new(move || {
        notif_clone.remove();
    }) as Box<dyn FnMut()>);
    super::interactions::set_timeout(timeout.as_ref().unchecked_ref(), 2500);
    timeout.forget();
}

/// Hide the RDF popover if present.
fn hide_popover(document: &Document) {
    if let Some(existing) = document.get_element_by_id("rdf-popover") {
        existing.remove();
    }
}
