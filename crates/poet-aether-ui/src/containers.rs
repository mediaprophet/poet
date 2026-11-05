//! Container interiors on the HyperCanvas stage.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::bodies::{
    BookmarksBody, CadStudioBody, CodecsBody, DeckStudioBody, DocBody, DomainsBody, EconomicsBody,
    GitBody, HealthBody, IdeBody, JobCenterBody, MapBody, MediaBody, OntologyBody, PdfStudioBody,
    ShadersBody, ShedToolboxRunner, SheetBody, SocialBody, SolidBody, SubmanifoldBody,
    VectorRasterStudioBody,
};
use super::kinds::{ContainerKind, ManifoldId};
use super::store::Workbench;
use super::vibe_console::VibeConsole;
use aether::prelude::*;

/// Live miniature SVG node-and-wire topological preview card with one-click manifold jump buttons.
#[component]
pub fn ManifoldPortalMiniPreview(#[props(default = "Nested Sub-Manifold Portal")] label: &'static str) -> Element {
    let wb_ctx = try_consume_context::<Signal<Workbench>>();
    let jumps: [(ManifoldId, &str); 5] = [
        (ManifoldId::Research, "Research"),
        (ManifoldId::Knowledge, "Knowledge"),
        (ManifoldId::Projects, "Projects"),
        (ManifoldId::Media, "Media"),
        (ManifoldId::Vibe, "Vibe"),
    ];

    rsx! {
        div {
            class: "manifold-portal-mini-preview",
            style: "background:rgba(10,15,24,0.88);border:1px solid rgba(0,229,255,0.28);border-radius:8px;padding:8px 10px;display:grid;gap:6px;",
            div { style: "display:flex;align-items:center;justify-content:space-between;font-size:10px;",
                span { style: "font-weight:700;color:var(--accent-cyan);", "🌀 {label}" }
                span { style: "font-family:var(--font-mono);color:#00E676;", "L3 Fiduciary · Verified · DAG 0x9B2C4F18" }
            }
            svg {
                view_box: "0 0 260 74",
                style: "width:100%;height:74px;background:rgba(4,8,14,0.75);border:1px solid rgba(255,255,255,0.07);border-radius:6px;",
                path {
                    d: "M 68 26 C 95 26, 95 22, 112 22",
                    stroke: "#00E5FF",
                    stroke_width: "1.5",
                    fill: "none",
                    stroke_dasharray: "3 2",
                }
                path {
                    d: "M 164 22 C 180 22, 182 48, 196 48",
                    stroke: "#FFC107",
                    stroke_width: "1.5",
                    fill: "none",
                }
                path {
                    d: "M 68 32 C 120 58, 155 54, 196 52",
                    stroke: "#00E676",
                    stroke_width: "1.2",
                    fill: "none",
                    stroke_dasharray: "2 2",
                }
                g {
                    rect { x: "12", y: "14", width: "56", height: "24", rx: "4", fill: "#131c2b", stroke: "#00E5FF", stroke_width: "1" }
                    text { x: "18", y: "29", fill: "#e2e8f0", font_size: "9", font_family: "monospace", "📝 Doc" }
                }
                g {
                    rect { x: "112", y: "10", width: "52", height: "24", rx: "4", fill: "#131c2b", stroke: "#a855f7", stroke_width: "1" }
                    text { x: "118", y: "25", fill: "#e2e8f0", font_size: "9", font_family: "monospace", "🧊 10D" }
                }
                g {
                    rect { x: "196", y: "36", width: "54", height: "24", rx: "4", fill: "#131c2b", stroke: "#00E676", stroke_width: "1" }
                    text { x: "201", y: "51", fill: "#e2e8f0", font_size: "9", font_family: "monospace", "📊 Sheet" }
                }
            }
            div { style: "display:flex;align-items:center;justify-content:space-between;gap:4px;flex-wrap:wrap;",
                span { style: "font-size:9px;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.04em;", "Jump Manifold:" }
                div { style: "display:flex;gap:4px;flex-wrap:wrap;",
                    for (target_id, short_lbl) in jumps {
                        button {
                            key: "{short_lbl}",
                            r#type: "button",
                            style: "padding:2px 7px;font-size:10px;border-radius:4px;border:1px solid rgba(0,229,255,0.35);background:rgba(0,229,255,0.1);color:var(--accent-cyan);cursor:pointer;",
                            onclick: move |_| {
                                if let Some(mut wb) = wb_ctx {
                                    let mut s = wb();
                                    s.switch(target_id);
                                    s.note(format!("🌀 Wormhole Portal jumped to {} manifold", target_id.title()));
                                    wb.set(s);
                                }
                            },
                            "{short_lbl}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Aether `NodeBody` component.
pub fn NodeBody(kind: ContainerKind) -> Element {
    rsx! {
        match kind {
            ContainerKind::Code => rsx! {
                div { style: "display:grid;gap:8px;padding:4px;",
                    IdeBody {}
                    VibeConsole {}
                }
            },
            ContainerKind::Doc => rsx! { DocBody {} },
            ContainerKind::Sheet => rsx! { SheetBody {} },
            ContainerKind::Ontology => rsx! { OntologyBody {} },
            ContainerKind::Map => rsx! { MapBody {} },
            ContainerKind::Media => rsx! { MediaBody {} },
            ContainerKind::Social => rsx! { SocialBody {} },
            ContainerKind::Health => rsx! { HealthBody {} },
            ContainerKind::Subcanvas => rsx! {
                div { style: "display:grid;gap:8px;padding:6px;",
                    ManifoldPortalMiniPreview { label: "Sub-Manifold Live Topological Preview" }
                    SubmanifoldBody {}
                    ShedToolboxRunner { target_id: "submanifold-shed".to_string(), wb: None }
                }
            },
            ContainerKind::Mesh3d => rsx! { CadStudioBody {} },
            ContainerKind::WebRtc => rsx! {
                div { style: "padding:12px;",
                    h4 { style: "margin:0 0 6px;color:var(--accent-emerald);font-size:12px;", "WebRTC P2P Data Mesh" }
                    p { class: "held-bind-note", "held / not yet — mesh channel bind missing." }
                    p { style: muted(), "Zero-leak P2P direct data channel. Peer DID hash verified with Ed25519 root signatures." }
                }
            },
            ContainerKind::Webview => rsx! {
                div { style: "padding:12px;",
                    h4 { style: "margin:0 0 6px;color:var(--accent-purple);font-size:12px;", "Sandboxed Dialectical Webview" }
                    p { style: muted(), "Sandboxed web reader harvesting W3C RDFa and JSON-LD semantic claims." }
                }
            },
            ContainerKind::Portal => rsx! {
                div { style: "padding:10px;display:grid;gap:8px;",
                    div {
                        h4 { style: "margin:0 0 4px;color:var(--accent-gold);font-size:12px;", "Wormhole Portal" }
                        p { style: muted(), "Wormhole portal to inalienable commons network graph." }
                    }
                    ManifoldPortalMiniPreview { label: "Wormhole Portal Topological Preview" }
                }
            },
            ContainerKind::Mail => rsx! { DomainsBody {} },
            ContainerKind::Chora => rsx! {
                div { style: "padding:12px;display:grid;gap:8px;",
                    h4 { style: "margin:0 0 6px;color:var(--accent-teal);font-size:12px;", "Chora 4D Dialectical Reader" }
                    p { style: muted(), "4D spatio-temporal knowledge graph reader with dialectical trust weights." }
                    ShedToolboxRunner { target_id: "submanifold-shed".to_string(), wb: None }
                }
            },
            ContainerKind::ErpKanban => rsx! {
                div { style: "padding:10px;display:grid;grid-template-columns:repeat(3,1fr);gap:8px;",
                    div { style: "background:rgba(255,255,255,0.03);border:1px solid rgba(255,255,255,0.06);border-radius:6px;padding:6px;",
                        span { style: "font-size:10px;font-weight:700;color:#94a3b8;text-transform:uppercase;", "To Do (2)" }
                        div { style: "margin-top:6px;background:#141a23;padding:6px;border-radius:4px;font-size:11px;", "Task A: Epistemic Norms" }
                    }
                    div { style: "background:rgba(255,255,255,0.03);border:1px solid rgba(255,255,255,0.06);border-radius:6px;padding:6px;",
                        span { style: "font-size:10px;font-weight:700;color:var(--accent-cyan);text-transform:uppercase;", "In Progress (1)" }
                        div { style: "margin-top:6px;background:#141a23;padding:6px;border-radius:4px;font-size:11px;border-left:2px solid var(--accent-cyan);", "Sprint 1: Poet Shell" }
                    }
                    div { style: "background:rgba(255,255,255,0.03);border:1px solid rgba(255,255,255,0.06);border-radius:6px;padding:6px;",
                        span { style: "font-size:10px;font-weight:700;color:#00E676;text-transform:uppercase;", "Done (5)" }
                        div { style: "margin-top:6px;background:#141a23;padding:6px;border-radius:4px;font-size:11px;border-left:2px solid #00E676;", "Specs 00..23 Complete" }
                    }
                }
            },
            ContainerKind::GitForge => rsx! { GitBody {} },
            ContainerKind::SolidHub => rsx! { SolidBody {} },
            ContainerKind::Economics => rsx! { EconomicsBody {} },
            ContainerKind::JobCenter => rsx! { JobCenterBody {} },
            ContainerKind::Bookmarks => rsx! { BookmarksBody {} },
            ContainerKind::Shaders => rsx! { ShadersBody {} },
            ContainerKind::Codecs => rsx! { CodecsBody {} },
            ContainerKind::Domains => rsx! { DomainsBody {} },
            ContainerKind::Deck => rsx! { DeckStudioBody {} },
            ContainerKind::Pdf => rsx! { PdfStudioBody {} },
            ContainerKind::Vector => rsx! { VectorRasterStudioBody {} },
            ContainerKind::SemanticInstrument => rsx! { crate::instrument_bay::InstrumentBay {} },
        }
    }
}

fn muted() -> &'static str {
    "margin:0;color:var(--text-secondary);font-size:12px;line-height:1.45;"
}
