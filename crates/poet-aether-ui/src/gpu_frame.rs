//! Manifold canvas: live Holographic 10D v2 (`HOLO` `0x0306`) & Etalon GPU viewport (`WP-POET-ETALON-10D`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

use super::engine::{self, PoetRenderResult};
use super::etalon_bridge::EtalonStatusBadge;
use aether::prelude::*;

#[component]
/// Aether `PoetGpuFrame` component with interactive 10D v2 `HOLO` orbit & CIE 1931 spectral controls.
pub fn PoetGpuFrame(kind: &'static str, width: u32, height: u32) -> Element {
    let mut orbit_yaw = use_signal(|| 32.0_f32);
    let mut wavelength_nm = use_signal(|| 492.0_f32);
    let mut frame = use_signal(|| {
        let (uri, nodes, edges, faces) =
            engine::synthesize_live_etalon_10d_holo_svg_uri(kind, width, height, 32.0, 492.0);
        PoetRenderResult {
            ok: true,
            kind: kind.to_string(),
            honesty: "live".into(),
            node_count: nodes,
            edge_count: edges,
            face_count: faces,
            data_uri: Some(uri),
            diagnostic: None,
        }
    });

    let result = frame();
    let status = format!(
        "{} · {} nodes / {} edges / {} faces · {} · Yaw {:.0}° · λ {:.0}nm",
        result.kind,
        result.node_count,
        result.edge_count,
        result.face_count,
        result.honesty,
        orbit_yaw(),
        wavelength_nm()
    );

    rsx! {
        div {
            style: "position:relative;min-height:220px;border-radius:12px;overflow:hidden;background:#05070c;border:1px solid #1a2230;",
            if let Some(uri) = result.data_uri.clone() {
                img {
                    src: "{uri}",
                    alt: "Etalon Holographic 10D v2 frame for {kind}",
                    style: "display:block;width:100%;height:100%;object-fit:cover;min-height:220px;",
                }
            }
            EtalonStatusBadge { kind: kind, width: width, height: height }
            div {
                style: "position:absolute;right:10px;bottom:8px;display:flex;gap:4px;align-items:center;",
                button {
                    r#type: "button",
                    style: "background:rgba(15,23,42,0.85);color:#38bdf8;border:1px solid #334155;border-radius:4px;padding:2px 6px;font-size:10px;cursor:pointer;",
                    onclick: move |_| {
                        let next_yaw = (orbit_yaw() + 22.5) % 360.0;
                        orbit_yaw.set(next_yaw);
                        let (uri, n, e, f) = engine::synthesize_live_etalon_10d_holo_svg_uri(
                            kind,
                            width,
                            height,
                            next_yaw,
                            wavelength_nm(),
                        );
                        frame.set(PoetRenderResult {
                            ok: true,
                            kind: kind.to_string(),
                            honesty: "live".into(),
                            node_count: n,
                            edge_count: e,
                            face_count: f,
                            data_uri: Some(uri),
                            diagnostic: None,
                        });
                    },
                    "↻ Orbit 10D"
                }
                button {
                    r#type: "button",
                    style: "background:rgba(15,23,42,0.85);color:#a78bfa;border:1px solid #334155;border-radius:4px;padding:2px 6px;font-size:10px;cursor:pointer;",
                    onclick: move |_| {
                        let next_wl = if wavelength_nm() >= 640.0 {
                            440.0
                        } else {
                            wavelength_nm() + 35.0
                        };
                        wavelength_nm.set(next_wl);
                        let (uri, n, e, f) = engine::synthesize_live_etalon_10d_holo_svg_uri(
                            kind,
                            width,
                            height,
                            orbit_yaw(),
                            next_wl,
                        );
                        frame.set(PoetRenderResult {
                            ok: true,
                            kind: kind.to_string(),
                            honesty: "live".into(),
                            node_count: n,
                            edge_count: e,
                            face_count: f,
                            data_uri: Some(uri),
                            diagnostic: None,
                        });
                    },
                    "🌈 Spectral λ"
                }
            }
            div {
                style: "position:absolute;left:10px;bottom:8px;padding:3px 8px;border-radius:999px;background:rgba(7,9,14,.82);color:#38bdf8;font-size:.64rem;letter-spacing:.04em;font-family:monospace;",
                "HOLO 0x0306 · {status}"
            }
        }
    }
}
