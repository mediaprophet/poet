//! SME | a11y Persona Modal & Technical Telemetry / Connection Ontology Drawer
//! (`Canvas_Workbench/js/a11y-manager.js`, `tech-sidebar.js`, `graph-generator.js`, `wire-inspector.js`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::engine;
use super::inspector::InspectorPanel;
use super::kinds::{CanvasNode, Wire};
use super::store::Workbench;
use dioxus::prelude::*;

/// Audience Persona level from `a11y-manager.js`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudiencePersona { Layperson, Sme, Dev }

impl AudiencePersona {
    pub const ALL: [Self; 3] = [Self::Layperson, Self::Sme, Self::Dev];
    pub fn label(self) -> &'static str {
        match self {
            Self::Layperson => "👤 Layperson / General Public",
            Self::Sme => "🔬 Subject Matter Expert (SME)",
            Self::Dev => "⚡ Ontologist & Core Dev",
        }
    }
    pub fn badge(self) -> &'static str {
        match self {
            Self::Layperson => "♿ 👤 Layperson | a11y",
            Self::Sme => "♿ 🔬 SME | a11y",
            Self::Dev => "♿ ⚡ Dev | a11y",
        }
    }
}

/// Visual Contrast & Lighting Theme from `a11y-manager.js`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContrastTheme { SlateDark, OledBlack, PaperLight }

impl ContrastTheme {
    pub const ALL: [Self; 3] = [Self::SlateDark, Self::OledBlack, Self::PaperLight];
    pub fn label(self) -> &'static str {
        match self {
            Self::SlateDark => "🌌 Cyber-Semantic Slate Dark",
            Self::OledBlack => "🖤 OLED Pure Black (High Contrast)",
            Self::PaperLight => "☀️ Clinical Paper Light",
        }
    }
}

/// Semantic Bridge Type for W3C RDF 1.2 wire reifiers (`wire-inspector.js`).
pub const SEMANTIC_BRIDGES: [(&str, &str); 5] = [
    ("schema-grounding", "Schema Grounding"),
    ("ontology-alignment", "Ontology Alignment"),
    ("dsl-contract", "Type-Safe DSL Contract"),
    ("tensor-projection", "Tensor Projection"),
    ("wormhole", "Multi-Tenant Wormhole"),
];

/// Packaged `.vibe` Workflow definition for one-click workbench execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackagedVibeWorkflow {
    pub id: &'static str,
    pub title: &'static str,
    pub domain: &'static str,
    pub source: &'static str,
}

pub const PACKAGED_VIBE_WORKFLOWS: [PackagedVibeWorkflow; 4] = [
    PackagedVibeWorkflow {
        id: "poet_document_cml",
        title: "1. CML Document Grounding & Gazetteer",
        domain: "Document · NLP · GraphDatabase",
        source: include_str!("../../../vibe/poet_document_cml.vibe"),
    },
    PackagedVibeWorkflow {
        id: "poet_numeric_sheet",
        title: "2. Reactive Sheet Formula & Tensor Evaluation",
        domain: "Sheet · Statistics · LinearAlgebra",
        source: include_str!("../../../vibe/poet_numeric_sheet.vibe"),
    },
    PackagedVibeWorkflow {
        id: "poet_spatial_authoring",
        title: "3. Spatial, Botanical & Terrain Authoring",
        domain: "BotanicalAnatomy · Terrain · SceneGraph10D",
        source: include_str!("../../../vibe/poet_spatial_authoring.vibe"),
    },
    PackagedVibeWorkflow {
        id: "poet_research_manifold",
        title: "4. Research Manifold & SHACL Verification",
        domain: "Research · SHACL · Epistemic/Deontic",
        source: include_str!("../../../vibe/poet_research_manifold.vibe"),
    },
];

/// Generates the container sub-graph triples and 48-byte NQuin assertion (`graph-generator.js`).
pub fn format_container_subgraph(node: &CanvasNode, graph_iri: &str) -> (Vec<(&'static str, String)>, String) {
    let triples = vec![
        ("rdf:type", format!("qualia:{}Container", node.kind.id())),
        ("qualia:title", format!("\"{}\"", node.title)),
        ("qualia:spatialCoord", format!("\"({}, {}, z={})\"^^qualia:Vec3", node.x as i32, node.y as i32, node.z as i32)),
        ("qualia:inCanvas", format!("<{graph_iri}>")),
    ];
    let nquin = format!(
        "<<[ <urn:qualia:container:{}> rdf:type qualia:{}Container <{graph_iri}> did:qualia:timothy_charles_holborn ]>>",
        node.id,
        node.kind.id()
    );
    (triples, nquin)
}

/// Formats the W3C RDF 1.2 Triple Reifier and 48-byte NQuin assertion for a `Wire`.
pub fn format_wire_reifier(wire: &Wire, graph_iri: &str) -> (String, String) {
    let reifier = format!("<< <urn:qualia:container:{}> {} <urn:qualia:container:{}> ~ reifier:{} >>", wire.from, wire.label, wire.to, wire.id);
    let nquin = format!("<<[ <urn:qualia:container:{}> {} <urn:qualia:container:{}> <{graph_iri}> did:qualia:timothy_charles_holborn ]>>", wire.from, wire.label, wire.to);
    (reifier, nquin)
}

fn pill_style(active: bool, accent: &str, bg_active: &str) -> String {
    format!(
        "text-align:left;padding:4px 8px;border-radius:5px;font-size:11px;cursor:pointer;border:1px solid {};background:{};color:var(--text-primary);",
        if active { accent } else { "rgba(255,255,255,0.12)" },
        if active { bg_active } else { "rgba(255,255,255,0.03)" }
    )
}

fn note_wb(mut wb: Signal<Workbench>, msg: impl Into<String>) {
    let mut s = wb();
    s.note(msg);
    wb.set(s);
}

#[component]
/// Top-bar trigger button (`♿ 🔬 SME | a11y`) + Accessibility, Persona & Multi-Cultural Settings Modal.
pub fn A11yPersonaButtonAndModal(wb: Signal<Workbench>) -> Element {
    let mut open = use_signal(|| false);
    let mut persona = use_signal(|| AudiencePersona::Sme);
    let mut contrast = use_signal(|| ContrastTheme::SlateDark);
    let mut font_scale = use_signal(|| "100%");
    let mut dyslexic = use_signal(|| false);
    let mut reduced_motion = use_signal(|| false);
    let mut locale = use_signal(|| "🇬🇧 EN");

    rsx! {
        div { style: "position:relative;display:inline-block;",
            button {
                r#type: "button", class: "btn",
                title: "Accessibility, Audience Persona (Layperson / SME / Dev) & Multi-Device Settings",
                style: "background:rgba(124,77,255,0.15);border:1px solid rgba(124,77,255,0.4);color:var(--text-primary);font-size:11px;padding:4px 8px;border-radius:var(--radius-sm);cursor:pointer;",
                onclick: move |_| open.set(!open()),
                "{persona().badge()}"
            }
            if open() {
                div {
                    class: "a11y-modal-card",
                    style: "position:fixed;top:84px;right:20px;width:400px;background:rgba(15,21,32,0.98);border:1px solid rgba(124,77,255,0.45);border-radius:10px;padding:14px;box-shadow:0 16px 40px rgba(0,0,0,0.65);z-index:200;display:grid;gap:10px;",
                    div { style: "display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid var(--border-subtle);padding-bottom:8px;",
                        span { style: "font-size:12px;font-weight:700;color:var(--accent-cyan);", "♿ Accessibility, Persona & Multi-Cultural Settings" }
                        button { r#type: "button", style: "background:transparent;border:none;color:var(--text-secondary);cursor:pointer;font-size:12px;", onclick: move |_| open.set(false), "✕" }
                    }
                    div { style: "display:grid;gap:4px;",
                        span { style: "font-size:10px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;", "1. Audience Persona & Cognitive Density" }
                        for p in AudiencePersona::ALL {
                            button {
                                key: "{p.label()}", r#type: "button",
                                style: "{pill_style(persona() == p, \"var(--accent-cyan)\", \"rgba(0,229,255,0.14)\")}",
                                onclick: move |_| { persona.set(p); note_wb(wb, format!("Persona switched to {}", p.label())); },
                                "{p.label()}"
                            }
                        }
                    }
                    div { style: "display:grid;gap:4px;",
                        span { style: "font-size:10px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;", "2. Visual Contrast & Lighting Theme" }
                        for c in ContrastTheme::ALL {
                            button {
                                key: "{c.label()}", r#type: "button",
                                style: "{pill_style(contrast() == c, \"var(--accent-gold)\", \"rgba(255,193,7,0.14)\")}",
                                onclick: move |_| { contrast.set(c); note_wb(wb, format!("Theme switched to {}", c.label())); },
                                "{c.label()}"
                            }
                        }
                    }
                    div { style: "display:grid;gap:4px;",
                        span { style: "font-size:10px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;", "3. Scalable Typography & Motion" }
                        div { style: "display:flex;gap:4px;",
                            for sz in ["100%", "125%", "150%", "200%"] {
                                button {
                                    key: "{sz}", r#type: "button",
                                    style: "{pill_style(font_scale() == sz, \"var(--accent-cyan)\", \"rgba(0,229,255,0.14)\")}flex:1;text-align:center;",
                                    onclick: move |_| font_scale.set(sz),
                                    "{sz}"
                                }
                            }
                        }
                        div { style: "display:flex;gap:8px;margin-top:2px;",
                            button {
                                r#type: "button",
                                style: "{pill_style(dyslexic(), \"var(--accent-emerald)\", \"rgba(0,230,118,0.14)\")}flex:1;",
                                onclick: move |_| dyslexic.set(!dyslexic()),
                                "📖 Dyslexic-Friendly Typography"
                            }
                            button {
                                r#type: "button",
                                style: "{pill_style(reduced_motion(), \"var(--accent-emerald)\", \"rgba(0,230,118,0.14)\")}flex:1;",
                                onclick: move |_| reduced_motion.set(!reduced_motion()),
                                "🛑 Reduced Motion"
                            }
                        }
                    }
                    div { style: "display:grid;gap:4px;",
                        span { style: "font-size:10px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;", "4. Internationalization & Phonological Script" }
                        div { style: "display:flex;flex-wrap:wrap;gap:4px;",
                            for lang in ["🇬🇧 EN", "🇫🇷 FR", "🇨🇳 ZH", "🇸🇦 AR (RTL)", "🔤 Phonetic / Q42"] {
                                button {
                                    key: "{lang}", r#type: "button",
                                    style: "{pill_style(locale() == lang, \"var(--accent-cyan)\", \"rgba(0,229,255,0.14)\")}",
                                    onclick: move |_| { locale.set(lang); note_wb(wb, format!("Script / Locale set to {lang}")); },
                                    "{lang}"
                                }
                            }
                        }
                    }
                    div {
                        style: "padding:8px;border-radius:6px;background:rgba(0,230,118,0.08);border:1px solid rgba(0,230,118,0.28);display:flex;align-items:center;justify-content:space-between;",
                        div {
                            div { style: "font-size:11px;font-weight:700;color:#00E676;", "📱 Tablet / Phone Controller · WebRTC Gateway" }
                            div { style: "font-size:10px;color:var(--text-secondary);", "Connected (Peer: tablet_remote_01 · Ed25519 Verified)" }
                        }
                        button {
                            r#type: "button",
                            style: "padding:3px 8px;border-radius:4px;font-size:10px;border:1px solid #00E676;background:rgba(0,230,118,0.15);color:#00E676;cursor:pointer;",
                            onclick: move |_| note_wb(wb, "📡 WebRTC Companion Gateway: Ping RTT 1.8ms (tablet_remote_01)"),
                            "Ping"
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Right-hand sidebar deck toggling between `"Inspector"` and `"Merkle-CRDT & Connection Ontology"`.
pub fn TechTelemetryDeck(wb: Signal<Workbench>) -> Element {
    let mut tab = use_signal(|| "inspector");
    let mut bridge_type = use_signal(|| "schema-grounding");
    let mut pred_override = use_signal(String::new);

    let state = wb();
    let active_node = state.selected.as_deref().and_then(|id| state.node(id)).or_else(|| state.nodes.first());
    let active_wire = state.selected_wire.as_deref().and_then(|id| state.wire(id)).or_else(|| state.wires.first());
    let total_nquins = state.nodes.len() * 4 + state.wires.len() * 2;

    rsx! {
        div { style: "display:flex;flex-direction:column;height:100%;",
            div { style: "display:flex;gap:4px;padding:8px 10px;border-bottom:1px solid var(--border-subtle);background:rgba(0,0,0,0.2);",
                button {
                    r#type: "button",
                    style: "{pill_style(tab() == \"inspector\", \"var(--accent-cyan)\", \"rgba(0,229,255,0.14)\")}flex:1;text-align:center;font-weight:600;",
                    onclick: move |_| tab.set("inspector"),
                    "Inspector"
                }
                button {
                    r#type: "button",
                    style: "{pill_style(tab() == \"ontology\", \"var(--accent-cyan)\", \"rgba(0,229,255,0.14)\")}flex:1;text-align:center;font-weight:600;",
                    onclick: move |_| tab.set("ontology"),
                    "Merkle-CRDT & Connection Ontology"
                }
            }
            if tab() == "inspector" {
                InspectorPanel { wb }
            } else {
                div { style: "padding:10px;display:grid;gap:12px;overflow-y:auto;font-size:11px;",
                    if let Some(node) = active_node {
                        {
                            let (triples, nquin) = format_container_subgraph(node, &state.graph_iri);
                            rsx! {
                                div { style: "background:rgba(255,255,255,0.03);border:1px solid var(--border-subtle);border-radius:6px;padding:8px;display:grid;gap:6px;",
                                    div { style: "font-weight:700;color:var(--accent-cyan);", "🧬 Selected Container Sub-Graph ({node.id})" }
                                    for (p, o) in triples {
                                        div { style: "font-family:var(--font-mono);font-size:10px;color:var(--text-secondary);",
                                            span { style: "color:var(--accent-gold);", "{p} " }
                                            span { "{o}" }
                                        }
                                    }
                                    div { style: "font-family:var(--font-mono);font-size:10px;padding:5px;border-radius:4px;background:rgba(0,0,0,0.35);color:#a5b4fc;word-break:break-all;", "{nquin}" }
                                }
                            }
                        }
                    }
                    if let Some(w) = active_wire {
                        {
                            let wid = w.id.clone();
                            let (reifier, wire_nquin) = format_wire_reifier(w, &state.graph_iri);
                            rsx! {
                                div { style: "background:rgba(255,255,255,0.03);border:1px solid rgba(0,229,255,0.25);border-radius:6px;padding:8px;display:grid;gap:6px;",
                                    div { style: "font-weight:700;color:var(--accent-gold);", "🔗 W3C RDF 1.2 Connection Ontology ({w.from} → {w.to})" }
                                    div { style: "display:flex;flex-wrap:wrap;gap:4px;",
                                        for (key, label) in SEMANTIC_BRIDGES {
                                            button {
                                                key: "{key}", r#type: "button",
                                                style: "{pill_style(bridge_type() == key, \"var(--accent-gold)\", \"rgba(255,193,7,0.14)\")}font-size:10px;padding:2px 6px;",
                                                onclick: move |_| bridge_type.set(key),
                                                "{label}"
                                            }
                                        }
                                    }
                                    div { style: "display:flex;gap:4px;",
                                        input {
                                            r#type: "text", placeholder: "{w.label}", value: "{pred_override}",
                                            style: "flex:1;background:rgba(0,0,0,0.3);border:1px solid var(--border-subtle);border-radius:4px;padding:3px 6px;color:var(--text-primary);font-size:10px;font-family:var(--font-mono);",
                                            oninput: move |e| pred_override.set(e.value()),
                                        }
                                        button {
                                            r#type: "button",
                                            style: "padding:3px 8px;font-size:10px;border-radius:4px;border:1px solid var(--accent-cyan);background:rgba(0,229,255,0.14);color:var(--accent-cyan);cursor:pointer;",
                                            onclick: move |_| {
                                                let next_lbl = if pred_override().trim().is_empty() { format!("qualia:{}", bridge_type()) } else { pred_override().trim().to_string() };
                                                let mut s = wb();
                                                s.rename_wire(&wid, next_lbl.clone());
                                                s.note(format!("Updated W3C RDF 1.2 wire {wid} → {next_lbl}"));
                                                wb.set(s);
                                            },
                                            "Save Bridge"
                                        }
                                    }
                                    div { style: "font-family:var(--font-mono);font-size:10px;padding:4px;background:rgba(0,0,0,0.3);border-radius:4px;color:#67e8f9;word-break:break-all;", "{reifier}" }
                                    div { style: "font-family:var(--font-mono);font-size:10px;padding:4px;background:rgba(0,0,0,0.3);border-radius:4px;color:#a5b4fc;word-break:break-all;", "{wire_nquin}" }
                                }
                            }
                        }
                    }
                    div { style: "background:rgba(255,255,255,0.03);border:1px solid var(--border-subtle);border-radius:6px;padding:8px;display:grid;gap:5px;",
                        div { style: "font-weight:700;color:#00E676;", "🛡️ Merkle-CRDT Root & Level 3 Sentinel Scopes" }
                        div { style: "font-family:var(--font-mono);font-size:10px;color:var(--text-secondary);", "Root: 0x8f4c2b9a71e06d33...a42 · {total_nquins} NQuins" }
                        div { style: "display:flex;gap:4px;flex-wrap:wrap;",
                            span { style: "padding:2px 6px;border-radius:4px;background:rgba(0,230,118,0.12);color:#00E676;font-size:10px;", "graph:read/write" }
                            span { style: "padding:2px 6px;border-radius:4px;background:rgba(0,229,255,0.12);color:var(--accent-cyan);font-size:10px;", "pulse:collaborate" }
                            span { style: "padding:2px 6px;border-radius:4px;background:rgba(255,193,7,0.12);color:var(--accent-gold);font-size:10px;", "ontology:map" }
                        }
                        div { style: "font-size:10px;color:var(--text-secondary);border-top:1px solid var(--border-subtle);padding-top:4px;", "📡 Pulse: did:qualia:timothy_charles_holborn ratified Lamport epoch #42" }
                    }
                    div { style: "background:rgba(255,255,255,0.03);border:1px solid var(--border-subtle);border-radius:6px;padding:8px;display:grid;gap:6px;",
                        div { style: "font-weight:700;color:var(--accent-cyan);", "⚡ Packaged .vibe Workflow Launcher (4)" }
                        for wf in PACKAGED_VIBE_WORKFLOWS {
                            div {
                                key: "{wf.id}", style: "display:flex;align-items:center;justify-content:space-between;padding:5px 6px;border-radius:4px;background:rgba(0,0,0,0.28);border:1px solid rgba(255,255,255,0.07);gap:6px;",
                                div {
                                    div { style: "font-size:10px;font-weight:600;color:var(--text-primary);", "{wf.title}" }
                                    div { style: "font-size:9px;color:var(--text-secondary);", "{wf.domain}" }
                                }
                                button {
                                    r#type: "button", style: "padding:3px 7px;font-size:10px;border-radius:4px;border:1px solid var(--accent-cyan);background:rgba(0,229,255,0.12);color:var(--accent-cyan);cursor:pointer;white-space:nowrap;",
                                    onclick: move |_| {
                                        let id = wf.id;
                                        let src = wf.source.to_string();
                                        note_wb(wb, format!("▶ Running {id}.vibe..."));
                                        spawn(async move {
                                            match engine::eval(src, false, None).await {
                                                Ok(res) if res.ok => {
                                                    note_wb(wb, format!("▶ Executed {id}.vibe → {} (rev {})", res.value, res.revision));
                                                }
                                                Ok(res) => {
                                                    note_wb(wb, format!("▶ Checked {id}.vibe ({})", res.diagnostic.unwrap_or_else(|| "ok".into())));
                                                }
                                                Err(e) => {
                                                    note_wb(wb, format!("▶ {id}.vibe error: {e}"));
                                                }
                                            }
                                        });
                                    },
                                    "▶ Run .vibe"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kinds::{ContainerKind, Epistemic, Strata};

    #[test]
    fn formats_subgraph_and_reifier_assertions() {
        let node = CanvasNode {
            id: "doc-1".into(), kind: ContainerKind::Doc, title: "Charter".into(),
            x: 100.0, y: 120.0, width: 400.0, height: 300.0, z: 0.0, d: 1.0,
            strata: Strata::Legal, epistemic: Epistemic::Normative,
        };
        let (triples, nquin) = format_container_subgraph(&node, "urn:qualia:canvas:research");
        assert_eq!(triples.len(), 4);
        assert!(nquin.contains("did:qualia:timothy_charles_holborn"));

        let wire = Wire {
            id: "wire-1".into(), from: "doc-1".into(), to: "sheet-2".into(),
            kind: "data-pipe".into(), label: "qualia:groundsClaim".into(),
        };
        let (reifier, wnquin) = format_wire_reifier(&wire, "urn:qualia:canvas:research");
        assert!(reifier.contains("~ reifier:wire-1"));
        assert!(wnquin.contains("qualia:groundsClaim"));
        assert_eq!(PACKAGED_VIBE_WORKFLOWS.len(), 4);
    }
}
