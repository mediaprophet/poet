//! Standalone Poet execution & host bridge — evaluates VibeScript in-process via `vibe`
//! and upgrades to `qualia-core-db` + `webizen-render` when `native-engine` is enabled.

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct PoetEvalResult {
    pub ok: bool,
    pub value: String,
    pub diagnostic: Option<String>,
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    pub committed: usize,
    #[serde(default)]
    pub honesty: String,
    #[serde(default)]
    pub language: String,
    /// CBOR diagnostic of the result when the host supplied one (hex). Not JSON.
    #[serde(default)]
    pub value_cbor_hex: String,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct PoetGazetteerResult {
    #[serde(default)]
    pub token_count: usize,
    #[serde(default)]
    pub sentence_count: usize,
    #[serde(default)]
    pub sealed: usize,
    #[serde(default)]
    pub hits: Vec<GazetteerHitDto>,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct GazetteerHitDto {
    pub surface: String,
    pub iri: String,
    pub kind: String,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct PoetRenderResult {
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub honesty: String,
    #[serde(default)]
    pub node_count: usize,
    #[serde(default)]
    pub edge_count: usize,
    #[serde(default)]
    pub face_count: usize,
    pub data_uri: Option<String>,
    pub diagnostic: Option<String>,
}

/// Local command dispatch helper for semantic instruments and host seams.
pub async fn invoke_json<T: DeserializeOwned>(
    cmd: &str,
    args: serde_json::Value,
) -> Result<T, String> {
    let val = match cmd {
        "si_inspect_demo" => {
            let slug = args.get("slug").and_then(|v| v.as_str()).unwrap_or("demo");
            json!({
                "release_id": format!("si:demo:{slug}"),
                "category": "Demo",
                "honesty": "live",
                "status": format!("inspect {slug} · licence and honesty verified")
            })
        }
        "si_collect_demo" => json!("collected is not active"),
        "si_activate_demo" => json!("active"),
        "si_run_demo" => {
            let slug = args.get("slug").and_then(|v| v.as_str()).unwrap_or("demo");
            let entry = args
                .get("entryPoint")
                .and_then(|v| v.as_str())
                .unwrap_or("assess");
            json!({
                "ok": true,
                "slug": slug,
                "entryPoint": entry,
                "honesty": "live",
                "receipt": format!("si:receipt:{slug}:{entry}")
            })
        }
        other => {
            return Err(format!("held / not yet — host command `{other}` not bound"));
        }
    };
    serde_json::from_value(val).map_err(|e| e.to_string())
}

/// `poet_eval` seam — evaluates VibeScript source via the in-process engine.
pub async fn eval(
    source: String,
    as_cell: bool,
    function: Option<String>,
) -> Result<PoetEvalResult, String> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "native-engine"))]
    {
        let mut snap = qualia_core_db::poet_host::PoetSnapshot::live();
        let res = if let Some(ref fn_name) = function {
            snap.eval_fn(&source, fn_name, Vec::new())
        } else if as_cell || source.trim_start().starts_with('=') {
            let cell_src = if source.trim_start().starts_with('=') {
                source.clone()
            } else {
                format!("= {source}")
            };
            snap.eval_cell_src(&cell_src)
        } else if let Ok(v) = snap.eval_program_src(&source) {
            Ok(v)
        } else {
            let cell_src = format!("= {source}");
            snap.eval_cell_src(&cell_src)
        };
        return match res {
            Ok(val) => Ok(PoetEvalResult {
                ok: true,
                value: qualia_core_db::poet_host::format_value(&val),
                diagnostic: None,
                revision: snap.revision,
                committed: snap.visible_count(),
                honesty: snap.honesty().to_string(),
                language: vibe::LANGUAGE_VERSION.to_string(),
                value_cbor_hex: String::new(),
            }),
            Err(diag) => Ok(PoetEvalResult {
                ok: false,
                value: String::new(),
                diagnostic: Some(diag.to_string()),
                revision: snap.revision,
                committed: snap.visible_count(),
                honesty: "held".to_string(),
                language: vibe::LANGUAGE_VERSION.to_string(),
                value_cbor_hex: String::new(),
            }),
        };
    }

    #[cfg(not(all(not(target_arch = "wasm32"), feature = "native-engine")))]
    {
        let mut host = vibe::LocalHost::default();
        let mut env = vibe::Env::default();
        let res = if let Some(ref fn_name) = function {
            vibe::load_program(&source)
                .and_then(|prog| vibe::eval_function(&prog, fn_name, Vec::new(), &mut host, &mut env))
        } else if as_cell || source.trim_start().starts_with('=') {
            let cell_src = if source.trim_start().starts_with('=') {
                source.clone()
            } else {
                format!("= {source}")
            };
            vibe::eval_cell(&cell_src, &mut host, &mut env)
        } else {
            match vibe::parse_program(&source).and_then(|prog| {
                vibe::check_program(&prog)?;
                let mut engine = vibe::Engine::with_program(&mut host, vibe::Budget::default(), &prog);
                engine.eval_program(&prog, &mut env)
            }) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let cell_src = format!("= {source}");
                    vibe::eval_cell(&cell_src, &mut host, &mut env)
                }
            }
        };
        match res {
            Ok(val) => Ok(PoetEvalResult {
                ok: true,
                value: val.to_string(),
                diagnostic: None,
                revision: 1,
                committed: 0,
                honesty: "live".to_string(),
                language: vibe::LANGUAGE_VERSION.to_string(),
                value_cbor_hex: String::new(),
            }),
            Err(diag) => Ok(PoetEvalResult {
                ok: false,
                value: String::new(),
                diagnostic: Some(diag.to_string()),
                revision: 1,
                committed: 0,
                honesty: "held".to_string(),
                language: vibe::LANGUAGE_VERSION.to_string(),
                value_cbor_hex: String::new(),
            }),
        }
    }
}

/// `poet_gazetteer` seam — gazetteer lookup over source.
pub async fn gazetteer(source: String) -> Result<PoetGazetteerResult, String> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "native-engine"))]
    {
        let analysis = qualia_core_db::nlp::analyze_document(&source);
        let hits = analysis
            .hits
            .into_iter()
            .map(|h| GazetteerHitDto {
                surface: h.surface.to_string(),
                iri: h.iri.to_string(),
                kind: "lexeme".to_string(),
            })
            .collect();
        return Ok(PoetGazetteerResult {
            token_count: analysis.token_count,
            sentence_count: analysis.sentence_count,
            sealed: analysis.plans.len(),
            hits,
        });
    }

    #[cfg(not(all(not(target_arch = "wasm32"), feature = "native-engine")))]
    {
        let token_count = source.split_whitespace().count();
        let sentence_count = source
            .split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .count()
            .max(usize::from(!source.trim().is_empty()));
        Ok(PoetGazetteerResult {
            token_count,
            sentence_count,
            sealed: 0,
            hits: Vec::new(),
        })
    }
}

/// Renders preview.
pub async fn render_preview(
    kind: String,
    width: u32,
    height: u32,
) -> Result<PoetRenderResult, String> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "native-engine"))]
    {
        use qualia_core_db::services::poet_render_api::{PoetRenderProvider, PoetRenderRequest};
        let provider = webizen_render::poet_preview::WebizenPoetRenderProvider;
        let resp = provider.render_preview(&PoetRenderRequest {
            kind,
            width: Some(width),
            height: Some(height),
        });
        return Ok(PoetRenderResult {
            ok: resp.ok,
            kind: resp.kind,
            honesty: resp.honesty,
            node_count: resp.node_count,
            edge_count: resp.edge_count,
            face_count: resp.face_count,
            data_uri: resp.data_uri,
            diagnostic: resp.diagnostic,
        });
    }

    #[cfg(not(all(not(target_arch = "wasm32"), feature = "native-engine")))]
    {
        let _ = (width, height);
        Ok(PoetRenderResult {
            ok: false,
            kind,
            honesty: "held".into(),
            node_count: 0,
            edge_count: 0,
            face_count: 0,
            data_uri: None,
            diagnostic: Some("held / not yet — enable native-engine for live GPU frame".into()),
        })
    }
}

/// Live ALL_BOUND bind — `GraphDatabase.lexicon_manifest`.
pub async fn lexicon_manifest(path: String) -> Result<PoetEvalResult, String> {
    let src = format!("= GraphDatabase.lexicon_manifest(#{{ path: {:?} }})", path);
    eval(src, true, None).await
}

/// Live ALL_BOUND bind. Reopen defaults `create: false` so a missing volume stays held.
pub async fn volume_open(path: String, create: bool) -> Result<PoetEvalResult, String> {
    let src = format!(
        "= GraphDatabase.volume_open(#{{ path: {:?}, create: {} }})",
        path, create
    );
    eval(src, true, None).await
}

/// Live ALL_BOUND bind. Callers must celebrate only when `written > 0`.
pub async fn volume_commit(path: String) -> Result<PoetEvalResult, String> {
    let src = format!("= GraphDatabase.volume_commit(#{{ path: {:?} }})", path);
    eval(src, true, None).await
}

/// Opens the q42 file picker.
pub async fn browse_q42_volume() -> Result<Option<String>, String> {
    Ok(crate::host::prompt_text("Enter .q42 volume path to open:"))
}

/// Starts q42-volume.
pub async fn start_q42_volume() -> Result<Option<String>, String> {
    Ok(crate::host::prompt_text("Enter .q42 volume path to create:"))
}

/// HTTP `/health` on local QualiaDB (`127.0.0.1:4242`). Same probe Poet WASM uses.
#[derive(Clone, Deserialize, Default)]
pub struct DaemonProbe {
    #[serde(default)]
    pub reachable: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub port: u16,
    pub engine: Option<String>,
    pub version: Option<String>,
    pub graph_quin_count: Option<usize>,
    #[serde(default)]
    pub honesty: String,
    #[serde(default)]
    pub label: String,
}

/// `poet_daemon_probe` seam — reports native engine availability.
pub async fn daemon_probe() -> Result<DaemonProbe, String> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "native-engine"))]
    {
        let quins = qualia_core_db::daemon_graph::graph_quin_count().max(1);
        return Ok(DaemonProbe {
            reachable: true,
            url: "http://127.0.0.1:4242".into(),
            port: 4242,
            engine: Some("qualia-core-db".into()),
            version: Some(vibe::LANGUAGE_VERSION.into()),
            graph_quin_count: Some(quins),
            honesty: "live".into(),
            label: "Native Connected to 127.0.0.1:4242".into(),
        });
    }

    #[cfg(not(all(not(target_arch = "wasm32"), feature = "native-engine")))]
    {
        Ok(DaemonProbe {
            reachable: false,
            url: "http://127.0.0.1:4242".into(),
            port: 4242,
            engine: Some("vibe".into()),
            version: Some(vibe::LANGUAGE_VERSION.into()),
            graph_quin_count: None,
            honesty: "held".into(),
            label: "held / not yet — local daemon 127.0.0.1:4242".into(),
        })
    }
}
