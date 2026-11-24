//! Synth-Authored Modular Containers (`.pcon.json`) & Manifolds (`.pman.json`) Engine (`WP-POET-SYNTH-MOD`).
//!
//! Enables Webizen AI Synth (and human authors) to synthesize, store, load, share, and mount
//! file-backed container modules and complete industry manifolds (Sound Production, Film Mastering,
//! Colour Grading & Spectral LookDev, Medical & Clinical Diagnostics, Engineering CAD/CAE, and
//! Legal & Human-Rights Forensics) directly into Poet without recompiling the host.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

use crate::engine::{self, PoetEvalResult};
use crate::host;
use crate::kinds::{CanvasNode, ContainerKind, Epistemic, ManifoldId, Strata, Wire};
use crate::qap_space_multi_device_wm::{
    ContainerDisplayRoute, DeviceRole, DisplayTransportKind, QapSpaceMultiDeviceManager,
};
use crate::store::Workbench;
use crate::wordnet_q42_ontology::deterministic_digest_hex;
use aether::prelude::*;
use serde::{Deserialize, Serialize};

/// Industry or domain specialization for a Synth-authored Container (`.pcon`) or Manifold (`.pman`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndustryDomainProfile {
    /// Multi-track DAW, Channel Strip, Triad Formant Synth, Spatial HRTF Ambisonics & LUFS Meter Bridge.
    SoundAndSpatialAudio,
    /// Non-Linear Film Mastering, EDL/OTIO Timeline, 10D v2 Camera Rig & 4K TV Reference Monitor Out.
    FilmAndMotionPicture,
    /// 3-Way Lift/Gamma/Gain Color Wheels, CIE 1931 Spectral / ACEScg Parade, Vectorscope & EMF v4 LookDev.
    ColourGradingAndLookDev,
    /// 10D v2 Anatomical SDFB Cutaways, rPPG / Clinical Risk, ICD-11/DOID/HP & Role-Qualified Semantic Instruments.
    MedicalAndClinical,
    /// Parametric B-Rep CAD, NURBS Constraints, FEA/CFD/XPBD Simulation & QUDT Engineering BOM.
    EngineeringAndCadCae,
    /// CML Legal & Human-Rights Annotator, Hohfeldian Deontic Matrix, Paraconsistent Audit & PAdES/QECP PDF.
    LegalAndHumanRights,
    /// Custom user- or Synth-authored domain module.
    CustomSynthDomain,
}

impl IndustryDomainProfile {
    pub const ALL: [Self; 7] = [
        Self::SoundAndSpatialAudio,
        Self::FilmAndMotionPicture,
        Self::ColourGradingAndLookDev,
        Self::MedicalAndClinical,
        Self::EngineeringAndCadCae,
        Self::LegalAndHumanRights,
        Self::CustomSynthDomain,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Self::SoundAndSpatialAudio => "sound-spatial-audio",
            Self::FilmAndMotionPicture => "film-motion-picture",
            Self::ColourGradingAndLookDev => "colour-grading-lookdev",
            Self::MedicalAndClinical => "medical-clinical-diagnostics",
            Self::EngineeringAndCadCae => "engineering-cad-cae",
            Self::LegalAndHumanRights => "legal-human-rights-forensics",
            Self::CustomSynthDomain => "custom-synth-domain",
        }
    }

    pub fn from_slug(slug: &str) -> Self {
        match slug {
            "sound-spatial-audio" => Self::SoundAndSpatialAudio,
            "film-motion-picture" => Self::FilmAndMotionPicture,
            "colour-grading-lookdev" => Self::ColourGradingAndLookDev,
            "medical-clinical-diagnostics" => Self::MedicalAndClinical,
            "engineering-cad-cae" => Self::EngineeringAndCadCae,
            "legal-human-rights-forensics" => Self::LegalAndHumanRights,
            _ => Self::CustomSynthDomain,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::SoundAndSpatialAudio => "🎙️ Sound Production & Spatial HRTF Mastering",
            Self::FilmAndMotionPicture => "🎬 Film NLE Timeline & 4K Reference Out",
            Self::ColourGradingAndLookDev => "🎨 Colour Grading, ACEScg Scopes & Spectral LookDev",
            Self::MedicalAndClinical => "🩺 Medical Anatomy 10D & Clinical Diagnostics",
            Self::EngineeringAndCadCae => "⚙️ Engineering B-Rep CAD, FEA/CFD & QUDT Simulation",
            Self::LegalAndHumanRights => "⚖️ Legal & Human-Rights Forensic Case Studio",
            Self::CustomSynthDomain => "✨ Custom Synth-Authored Module",
        }
    }

    pub fn accent_color(self) -> &'static str {
        match self {
            Self::SoundAndSpatialAudio => "#38bdf8",
            Self::FilmAndMotionPicture => "#f59e0b",
            Self::ColourGradingAndLookDev => "#ec4899",
            Self::MedicalAndClinical => "#34d399",
            Self::EngineeringAndCadCae => "#60a5fa",
            Self::LegalAndHumanRights => "#a78bfa",
            Self::CustomSynthDomain => "#2dd4bf",
        }
    }

    pub fn default_manifold(self) -> ManifoldId {
        match self {
            Self::SoundAndSpatialAudio
            | Self::FilmAndMotionPicture
            | Self::ColourGradingAndLookDev => ManifoldId::Media,
            Self::MedicalAndClinical => ManifoldId::Research,
            Self::EngineeringAndCadCae => ManifoldId::Projects,
            Self::LegalAndHumanRights => ManifoldId::Rights,
            Self::CustomSynthDomain => ManifoldId::Vibe,
        }
    }
}

/// Visual display widget type rendered inside a modular `.pcon` container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerDisplayWidget {
    /// Interactive Holographic 10D v2 (`HOLO` `0x0306`) or CIE 1931 spectral SVG/GPU viewport.
    Holo10dAndSpectralViewport {
        scene_kind: String,
        wavelength_nm: f32,
        orbit_yaw_deg: f32,
    },
    /// Industry Scopes & Telemetry Meters (LUFS Audio Meter Bridge, RGB Parade, Vectorscope, or FEA Convergence).
    ScopesAndMeterBridge {
        scope_title: String,
        channels: Vec<(String, f32, String)>,
    },
    /// Interactive Parameter Control Surface (Faders, Color Wheels, Transport Scrubbers, or Parametric Knobs)
    /// that can be displayed on a Tablet / Laptop while content streams to a TV / Monitor.
    ControlSurfaceDeck {
        deck_title: String,
        parameters: Vec<(String, String, String)>,
    },
    /// Structured Semantic / QUDT / Clinical / Legal Table backed by `.q42` `NQuin48` triples.
    SemanticDataMatrix {
        matrix_title: String,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

impl ContainerDisplayWidget {
    pub fn to_vibe_expr(&self) -> String {
        match self {
            Self::Holo10dAndSpectralViewport {
                scene_kind,
                wavelength_nm,
                orbit_yaw_deg,
            } => format!(
                "{{ kind: \"holo_10d\", scene_kind: {scene_kind:?}, wavelength_nm: {wavelength_nm:.1}, orbit_yaw_deg: {orbit_yaw_deg:.1} }}"
            ),
            Self::ScopesAndMeterBridge {
                scope_title,
                channels,
            } => {
                let ch_items: Vec<String> = channels
                    .iter()
                    .map(|(n, v, u)| format!("{{ name: {n:?}, value: {v:.2}, unit: {u:?} }}"))
                    .collect();
                format!(
                    "{{ kind: \"scopes_meters\", scope_title: {scope_title:?}, channels: [{}] }}",
                    ch_items.join(", ")
                )
            }
            Self::ControlSurfaceDeck {
                deck_title,
                parameters,
            } => {
                let p_items: Vec<String> = parameters
                    .iter()
                    .map(|(n, v, c)| format!("{{ name: {n:?}, value: {v:?}, capability: {c:?} }}"))
                    .collect();
                format!(
                    "{{ kind: \"control_deck\", deck_title: {deck_title:?}, parameters: [{}] }}",
                    p_items.join(", ")
                )
            }
            Self::SemanticDataMatrix {
                matrix_title,
                headers,
                rows,
            } => {
                let h_items: Vec<String> = headers.iter().map(|h| format!("{h:?}")).collect();
                let r_items: Vec<String> = rows
                    .iter()
                    .map(|r| {
                        let cells: Vec<String> = r.iter().map(|c| format!("{c:?}")).collect();
                        format!("[{}]", cells.join(", "))
                    })
                    .collect();
                format!(
                    "{{ kind: \"semantic_matrix\", matrix_title: {matrix_title:?}, headers: [{}], rows: [{}] }}",
                    h_items.join(", "),
                    r_items.join(", ")
                )
            }
        }
    }

    pub fn from_vibe_value(val: &vibe::Value) -> Option<Self> {
        let vibe::Value::Record(m) = val else {
            return None;
        };
        let kind = m.get("kind")?.as_str()?;
        match kind {
            "holo_10d" => Some(Self::Holo10dAndSpectralViewport {
                scene_kind: m.get("scene_kind")?.as_str()?.to_string(),
                wavelength_nm: vibe_as_f64(m.get("wavelength_nm")?) as f32,
                orbit_yaw_deg: vibe_as_f64(m.get("orbit_yaw_deg")?) as f32,
            }),
            "scopes_meters" => {
                let scope_title = m.get("scope_title")?.as_str()?.to_string();
                let mut channels = Vec::new();
                if let Some(vibe::Value::List(list)) = m.get("channels") {
                    for item in list {
                        if let vibe::Value::Record(cm) = item {
                            let n = cm.get("name")?.as_str()?.to_string();
                            let v = vibe_as_f64(cm.get("value")?) as f32;
                            let u = cm.get("unit")?.as_str()?.to_string();
                            channels.push((n, v, u));
                        }
                    }
                }
                Some(Self::ScopesAndMeterBridge {
                    scope_title,
                    channels,
                })
            }
            "control_deck" => {
                let deck_title = m.get("deck_title")?.as_str()?.to_string();
                let mut parameters = Vec::new();
                if let Some(vibe::Value::List(list)) = m.get("parameters") {
                    for item in list {
                        if let vibe::Value::Record(pm) = item {
                            let n = pm.get("name")?.as_str()?.to_string();
                            let v = pm.get("value")?.as_str()?.to_string();
                            let c = pm.get("capability")?.as_str()?.to_string();
                            parameters.push((n, v, c));
                        }
                    }
                }
                Some(Self::ControlSurfaceDeck {
                    deck_title,
                    parameters,
                })
            }
            "semantic_matrix" => {
                let matrix_title = m.get("matrix_title")?.as_str()?.to_string();
                let mut headers = Vec::new();
                if let Some(vibe::Value::List(hlist)) = m.get("headers") {
                    for h in hlist {
                        headers.push(h.as_str()?.to_string());
                    }
                }
                let mut rows = Vec::new();
                if let Some(vibe::Value::List(rlist)) = m.get("rows") {
                    for r in rlist {
                        if let vibe::Value::List(clist) = r {
                            let mut row = Vec::new();
                            for c in clist {
                                row.push(c.as_str()?.to_string());
                            }
                            rows.push(row);
                        }
                    }
                }
                Some(Self::SemanticDataMatrix {
                    matrix_title,
                    headers,
                    rows,
                })
            }
            _ => None,
        }
    }
}

fn vibe_as_f64(val: &vibe::Value) -> f64 {
    match val {
        vibe::Value::F64(f) => *f,
        vibe::Value::I64(i) => *i as f64,
        vibe::Value::U64(u) => *u as f64,
        _ => 0.0,
    }
}

/// A portable, file-backed Poet Container Module (`.pcon.vibe`) authored in native VibeScript
/// that can be generated by Synth, saved to disk or `.q42`/Q-CDN, and mounted onto any Poet Manifold canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynthContainerModule {
    /// Format magic identifier `"POET-CONTAINER-MODULE-V1"`.
    pub format_magic: String,
    pub module_id: String,
    pub version: String,
    pub title: String,
    pub industry: IndustryDomainProfile,
    pub base_container_kind_id: String,
    pub strata_id: String,
    pub epistemic_id: String,
    /// Preferred target display (`"DISP-1-MAIN"`, `"DISP-2-SIDE"`, `"DISP-3-TV"`, `"DISP-4-LAPTOP"`, `"DISP-5-TABLET"`).
    pub preferred_display_id: String,
    /// Preferred companion control surface device (`"did:q42:device:tablet-03"`, `"did:q42:device:laptop-02"`).
    pub companion_control_device_id: String,
    pub default_width: f64,
    pub default_height: f64,
    /// Declarative display widgets inside this container module.
    pub widgets: Vec<ContainerDisplayWidget>,
    /// Embedded VibeScript morphism recipe executed when the user triggers this container's primary action.
    pub vibe_recipe_source: String,
    /// Referenced `.10d` / `.hmc` / `.q42` asset URIs produced by Synth.
    pub synth_asset_uris: Vec<String>,
    pub license_spdx: String,
    pub content_digest_hex: String,
}

impl SynthContainerModule {
    /// Serializes this container module to a deterministic, executable `.pcon.vibe` VibeScript source file.
    pub fn to_pcon_vibe(&self) -> Result<String, String> {
        let widgets_vibe: Vec<String> = self
            .widgets
            .iter()
            .map(|w| format!("            {}", w.to_vibe_expr()))
            .collect();
        let assets_vibe: Vec<String> = self
            .synth_asset_uris
            .iter()
            .map(|a| format!("{a:?}"))
            .collect();

        // Strip leading `requires [...]` from `vibe_recipe_source` when embedding so `requires` appears once at top of module
        let effect_body = self
            .vibe_recipe_source
            .lines()
            .filter(|l| !l.trim_start().starts_with("requires "))
            .collect::<Vec<_>>()
            .join("\n");

        let raw_vibe = format!(
            "// Webizen Poet & Synth Modular Container File (.pcon.vibe)\n\
             // License: {}\n\
             requires [ capability(\"capability.invoke\") ];\n\n\
             fn module_manifest() {{\n\
             \x20   return {{\n\
             \x20       format_magic: {:?},\n\
             \x20       module_id: {:?},\n\
             \x20       version: {:?},\n\
             \x20       title: {:?},\n\
             \x20       industry: {:?},\n\
             \x20       base_container_kind_id: {:?},\n\
             \x20       strata_id: {:?},\n\
             \x20       epistemic_id: {:?},\n\
             \x20       preferred_display_id: {:?},\n\
             \x20       companion_control_device_id: {:?},\n\
             \x20       default_width: {:.1},\n\
             \x20       default_height: {:.1},\n\
             \x20       widgets: [\n{}\n\
             \x20       ],\n\
             \x20       synth_asset_uris: [{}],\n\
             \x20       license_spdx: {:?}\n\
             \x20   }};\n\
             }}\n\n\
             {}\n",
            self.license_spdx,
            self.format_magic,
            self.module_id,
            self.version,
            self.title,
            self.industry.slug(),
            self.base_container_kind_id,
            self.strata_id,
            self.epistemic_id,
            self.preferred_display_id,
            self.companion_control_device_id,
            self.default_width,
            self.default_height,
            widgets_vibe.join(",\n"),
            assets_vibe.join(", "),
            self.license_spdx,
            effect_body.trim()
        );
        Ok(crate::engine::ensure_invoked_capabilities_in_requires(&raw_vibe))
    }

    /// Parses, typechecks, and evaluates a `.pcon.vibe` VibeScript container module file
    /// directly through the `vibe` engine (`module_manifest()`).
    pub fn from_pcon_vibe(vibe_source: &str) -> Result<Self, String> {
        let normalized = crate::engine::ensure_invoked_capabilities_in_requires(vibe_source);
        let prog = vibe::load_program(&normalized).map_err(|d| d.to_string())?;
        let mut host = crate::engine::PoetCapabilityHost::default();
        let mut env = vibe::Env::default();
        let val = vibe::eval_function(&prog, "module_manifest", Vec::new(), &mut host, &mut env)
            .map_err(|d| d.to_string())?;
        Self::from_manifest_record(&val, vibe_source)
    }

    pub fn from_manifest_record(val: &vibe::Value, full_vibe_source: &str) -> Result<Self, String> {
        let vibe::Value::Record(m) = val else {
            return Err("module_manifest() must return a VibeScript Record".into());
        };
        let get_str = |k: &str| -> Result<String, String> {
            m.get(k)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| format!("Missing string field `{k}` in .pcon.vibe manifest"))
        };
        let format_magic = get_str("format_magic")?;
        if format_magic != "POET-CONTAINER-MODULE-V1" {
            return Err(format!(
                "Invalid container module format_magic `{format_magic}` (expected POET-CONTAINER-MODULE-V1)"
            ));
        }

        let mut widgets = Vec::new();
        if let Some(vibe::Value::List(wlist)) = m.get("widgets") {
            for wval in wlist {
                if let Some(w) = ContainerDisplayWidget::from_vibe_value(wval) {
                    widgets.push(w);
                }
            }
        }

        let mut synth_asset_uris = Vec::new();
        if let Some(vibe::Value::List(alist)) = m.get("synth_asset_uris") {
            for aval in alist {
                if let Some(s) = aval.as_str() {
                    synth_asset_uris.push(s.to_string());
                }
            }
        }

        let industry = IndustryDomainProfile::from_slug(&get_str("industry")?);
        let mut module = Self {
            format_magic,
            module_id: get_str("module_id")?,
            version: get_str("version")?,
            title: get_str("title")?,
            industry,
            base_container_kind_id: get_str("base_container_kind_id")?,
            strata_id: get_str("strata_id")?,
            epistemic_id: get_str("epistemic_id")?,
            preferred_display_id: get_str("preferred_display_id")?,
            companion_control_device_id: get_str("companion_control_device_id")?,
            default_width: m.get("default_width").map(vibe_as_f64).unwrap_or(540.0),
            default_height: m.get("default_height").map(vibe_as_f64).unwrap_or(360.0),
            widgets,
            vibe_recipe_source: full_vibe_source.to_string(),
            synth_asset_uris,
            license_spdx: get_str("license_spdx")?,
            content_digest_hex: String::new(),
        };
        module.recompute_digest();
        Ok(module)
    }

    pub fn recompute_digest(&mut self) {
        let payload = format!(
            "PCON|{}|{}|{}|{:?}|{}",
            self.module_id,
            self.version,
            self.title,
            self.industry,
            self.widgets.len()
        );
        self.content_digest_hex = deterministic_digest_hex(payload.as_bytes());
    }

    /// Serializes this container module to a canonical `yaml-ld-q42` v2 manifest (`.pcon.q42.yaml`)
    /// declaring the 7-Layer Multi-Scale Computational Hierarchy (`L0`–`L6`, with `L6` = `L6QuantumPhysics`).
    pub fn to_q42_yaml(&self) -> String {
        let mut yaml = String::new();
        yaml.push_str("# yaml-ld-q42 v2 — Webizen Poet & Synth Modular Container Manifest (.pcon.q42.yaml)\n");
        yaml.push_str("\"@context\":\n");
        yaml.push_str("  qualia: \"https://ns.webcivics.net/qualia#\"\n");
        yaml.push_str("  poet: \"https://ns.webcivics.net/poet#\"\n");
        yaml.push_str("  q42: \"https://ns.webcivics.net/q42#\"\n");
        yaml.push_str(&format!("\"@id\": \"urn:q42:poet:container:{}\"\n", self.module_id));
        yaml.push_str("\"@type\": \"poet:ModularContainerManifest\"\n");
        yaml.push_str(&format!("format_magic: {:?}\n", self.format_magic));
        yaml.push_str(&format!("module_id: {:?}\n", self.module_id));
        yaml.push_str(&format!("version: {:?}\n", self.version));
        yaml.push_str(&format!("title: {:?}\n", self.title));
        yaml.push_str(&format!("industry: {:?}\n", self.industry.slug()));
        yaml.push_str(&format!("base_container_kind_id: {:?}\n", self.base_container_kind_id));
        yaml.push_str(&format!("strata_id: {:?}\n", self.strata_id));
        yaml.push_str(&format!("epistemic_id: {:?}\n", self.epistemic_id));
        yaml.push_str(&format!("preferred_display_id: {:?}\n", self.preferred_display_id));
        yaml.push_str(&format!("companion_control_device_id: {:?}\n", self.companion_control_device_id));
        yaml.push_str(&format!("default_width: {:.1}\n", self.default_width));
        yaml.push_str(&format!("default_height: {:.1}\n", self.default_height));
        yaml.push_str(&format!("license_spdx: {:?}\n", self.license_spdx));
        yaml.push_str(&format!("content_digest_hex: {:?}\n", self.content_digest_hex));
        yaml.push_str("computational_hierarchy:\n");
        yaml.push_str("  L0: \"L0Continuum\"\n");
        yaml.push_str("  L1: \"L1RigidArticulatedPgaXpbdSkl2\"\n");
        yaml.push_str("  L2: \"L2SoftBodyFluid\"\n");
        yaml.push_str("  L3: \"L3MolecularChemical\"\n");
        yaml.push_str("  L4: \"L4CellularBiological\"\n");
        yaml.push_str("  L5: \"L5EcologicalGeomorphic\"\n");
        yaml.push_str("  L6: \"L6QuantumPhysics\"\n");
        yaml.push_str("synth_asset_uris:\n");
        for uri in &self.synth_asset_uris {
            yaml.push_str(&format!("  - {uri:?}\n"));
        }
        yaml
    }
}

/// A portable, file-backed Poet Manifold Workspace Module (`.pman.vibe`) authored in native VibeScript
/// bundling a complete multi-container industry workstation layout, semantic wires, and multi-display/device routing map.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynthManifoldModule {
    /// Format magic identifier `"POET-MANIFOLD-MODULE-V1"`.
    pub format_magic: String,
    pub manifold_module_id: String,
    pub title: String,
    pub industry: IndustryDomainProfile,
    pub graph_iri: String,
    /// Containers placed on this manifold along with their `(x, y)` stage coordinates.
    pub containers: Vec<(SynthContainerModule, f64, f64)>,
    /// Semantic / Deontic / DataPipe wires connecting the containers (`(from_module_id, to_module_id, kind, predicate)`).
    pub wires: Vec<(String, String, String, String)>,
    pub license_spdx: String,
    pub content_digest_hex: String,
}

impl SynthManifoldModule {
    /// Serializes this manifold module into a deterministic, executable `.pman.vibe` VibeScript file.
    pub fn to_pman_vibe(&self) -> Result<String, String> {
        let mut container_entries = Vec::new();
        for (cmod, x, y) in &self.containers {
            let widgets_vibe: Vec<String> =
                cmod.widgets.iter().map(|w| w.to_vibe_expr()).collect();
            let assets_vibe: Vec<String> =
                cmod.synth_asset_uris.iter().map(|a| format!("{a:?}")).collect();
            container_entries.push(format!(
                "            {{\n\
                 \x20               x: {x:.1},\n\
                 \x20               y: {y:.1},\n\
                 \x20               container: {{\n\
                 \x20                   format_magic: {:?},\n\
                 \x20                   module_id: {:?},\n\
                 \x20                   version: {:?},\n\
                 \x20                   title: {:?},\n\
                 \x20                   industry: {:?},\n\
                 \x20                   base_container_kind_id: {:?},\n\
                 \x20                   strata_id: {:?},\n\
                 \x20                   epistemic_id: {:?},\n\
                 \x20                   preferred_display_id: {:?},\n\
                 \x20                   companion_control_device_id: {:?},\n\
                 \x20                   default_width: {:.1},\n\
                 \x20                   default_height: {:.1},\n\
                 \x20                   widgets: [{}],\n\
                 \x20                   synth_asset_uris: [{}],\n\
                 \x20                   license_spdx: {:?}\n\
                 \x20               }}\n\
                 \x20           }}",
                cmod.format_magic,
                cmod.module_id,
                cmod.version,
                cmod.title,
                cmod.industry.slug(),
                cmod.base_container_kind_id,
                cmod.strata_id,
                cmod.epistemic_id,
                cmod.preferred_display_id,
                cmod.companion_control_device_id,
                cmod.default_width,
                cmod.default_height,
                widgets_vibe.join(", "),
                assets_vibe.join(", "),
                cmod.license_spdx
            ));
        }

        let wire_entries: Vec<String> = self
            .wires
            .iter()
            .map(|(from, to, kind, pred)| {
                format!(
                    "            {{ from: {from:?}, to: {to:?}, kind: {kind:?}, predicate: {pred:?} }}"
                )
            })
            .collect();

        Ok(format!(
            "// Webizen Poet & Synth Modular Manifold Workstation File (.pman.vibe)\n\
             // License: {}\n\
             requires [ capability(\"capability.invoke\") ];\n\n\
             fn manifold_manifest() {{\n\
             \x20   return {{\n\
             \x20       format_magic: {:?},\n\
             \x20       manifold_module_id: {:?},\n\
             \x20       title: {:?},\n\
             \x20       industry: {:?},\n\
             \x20       graph_iri: {:?},\n\
             \x20       license_spdx: {:?},\n\
             \x20       containers: [\n{}\n\
             \x20       ],\n\
             \x20       wires: [\n{}\n\
             \x20       ]\n\
             \x20   }};\n\
             }}\n",
            self.license_spdx,
            self.format_magic,
            self.manifold_module_id,
            self.title,
            self.industry.slug(),
            self.graph_iri,
            self.license_spdx,
            container_entries.join(",\n"),
            wire_entries.join(",\n")
        ))
    }

    /// Parses, typechecks, and evaluates a `.pman.vibe` VibeScript manifold module file
    /// directly through the `vibe` engine (`manifold_manifest()`).
    pub fn from_pman_vibe(vibe_source: &str) -> Result<Self, String> {
        let prog = vibe::load_program(vibe_source).map_err(|d| d.to_string())?;
        let mut host = crate::engine::PoetCapabilityHost::default();
        let mut env = vibe::Env::default();
        let val = vibe::eval_function(
            &prog,
            "manifold_manifest",
            Vec::new(),
            &mut host,
            &mut env,
        )
        .map_err(|d| d.to_string())?;

        let vibe::Value::Record(m) = val else {
            return Err("manifold_manifest() must return a VibeScript Record".into());
        };
        let get_str = |k: &str| -> Result<String, String> {
            m.get(k)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| format!("Missing string field `{k}` in .pman.vibe manifest"))
        };

        let format_magic = get_str("format_magic")?;
        if format_magic != "POET-MANIFOLD-MODULE-V1" {
            return Err(format!(
                "Invalid manifold module format_magic `{format_magic}` (expected POET-MANIFOLD-MODULE-V1)"
            ));
        }

        let mut containers = Vec::new();
        if let Some(vibe::Value::List(clist)) = m.get("containers") {
            for item in clist {
                if let vibe::Value::Record(cm) = item {
                    let x = cm.get("x").map(vibe_as_f64).unwrap_or(36.0);
                    let y = cm.get("y").map(vibe_as_f64).unwrap_or(36.0);
                    let c_rec = cm
                        .get("container")
                        .ok_or_else(|| "Missing `container` in .pman.vibe entry".to_string())?;
                    let c_mod = SynthContainerModule::from_manifest_record(c_rec, vibe_source)?;
                    containers.push((c_mod, x, y));
                }
            }
        }

        let mut wires = Vec::new();
        if let Some(vibe::Value::List(wlist)) = m.get("wires") {
            for item in wlist {
                if let vibe::Value::Record(wm) = item {
                    let from = wm
                        .get("from")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let to = wm
                        .get("to")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let kind = wm
                        .get("kind")
                        .and_then(|v| v.as_str())
                        .unwrap_or("semantic-rel")
                        .to_string();
                    let pred = wm
                        .get("predicate")
                        .and_then(|v| v.as_str())
                        .unwrap_or("qualia:groundsObservation")
                        .to_string();
                    wires.push((from, to, kind, pred));
                }
            }
        }

        let industry = IndustryDomainProfile::from_slug(&get_str("industry")?);
        let payload = format!(
            "PMAN|{}|{}",
            industry.slug(),
            containers
                .iter()
                .map(|(c, _, _)| c.content_digest_hex.as_str())
                .collect::<Vec<_>>()
                .join("|")
        );
        let content_digest_hex = deterministic_digest_hex(payload.as_bytes());

        Ok(Self {
            format_magic,
            manifold_module_id: get_str("manifold_module_id")?,
            title: get_str("title")?,
            industry,
            graph_iri: get_str("graph_iri")?,
            containers,
            wires,
            license_spdx: get_str("license_spdx")?,
            content_digest_hex,
        })
    }
}

/// Synthesizes the 6 flagship industry container modules (`.pcon`) covering:
/// 1. **Sound Production & Spatial HRTF Mastering**
/// 2. **Film NLE Timeline & 4K TV Reference Out**
/// 3. **Colour Grading, ACEScg Scopes & Spectral LookDev**
/// 4. **Medical Anatomy 10D (`SDFB`) & Clinical Diagnostics**
/// 5. **Engineering B-Rep CAD, FEA/CFD & QUDT Simulation**
/// 6. **Legal & Human-Rights Forensic Case Studio**
pub fn builtin_industry_container_modules() -> Vec<SynthContainerModule> {
    vec![
        synthesize_industry_container_module(IndustryDomainProfile::SoundAndSpatialAudio, None),
        synthesize_industry_container_module(IndustryDomainProfile::FilmAndMotionPicture, None),
        synthesize_industry_container_module(IndustryDomainProfile::ColourGradingAndLookDev, None),
        synthesize_industry_container_module(IndustryDomainProfile::MedicalAndClinical, None),
        synthesize_industry_container_module(IndustryDomainProfile::EngineeringAndCadCae, None),
        synthesize_industry_container_module(IndustryDomainProfile::LegalAndHumanRights, None),
    ]
}

/// Synthesizes a ready-to-save `.pcon` container module for the specified `industry` (with optional custom title).
pub fn synthesize_industry_container_module(
    industry: IndustryDomainProfile,
    custom_title: Option<&str>,
) -> SynthContainerModule {
    let title = custom_title.unwrap_or_else(|| industry.title()).to_string();
    let (
        module_id,
        base_kind,
        strata_id,
        epistemic_id,
        pref_disp,
        companion_dev,
        widgets,
        vibe_recipe,
        assets,
    ) = match industry {
        IndustryDomainProfile::SoundAndSpatialAudio => (
            "pcon-sound-spatial-hrtf-01",
            "media",
            "technical",
            "empirical",
            "DISP-2-SIDE",
            "did:q42:device:tablet-03",
            vec![
                ContainerDisplayWidget::ScopesAndMeterBridge {
                    scope_title: "ITU-R BS.1770-4 LUFS Loudness & 32-Band Spectral Meter Bridge".into(),
                    channels: vec![
                        ("Master L/R Integrated".into(), -14.2, "LUFS (Target -14.0)".into()),
                        ("True Peak Max".into(), -1.1, "dBTP (Headroom OK)".into()),
                        ("Ambisonics HOA Order-3".into(), 98.4, "% Phase Coherence".into()),
                        ("Triad Formant Bus".into(), -18.5, "LUFS".into()),
                    ],
                },
                ContainerDisplayWidget::ControlSurfaceDeck {
                    deck_title: "Tablet Touch Fader & Spatial HRTF Azimuth/Elevation Console".into(),
                    parameters: vec![
                        ("HRTF Azimuth".into(), "+32.5°".into(), "Audio.spatial_hrtf".into()),
                        ("Reverb Decay RT60".into(), "1.45 s".into(), "Audio.convolution_ir".into()),
                        ("Formant F1 / F2".into(), "730 Hz / 1090 Hz".into(), "Audio.triad_formant".into()),
                        ("Codec Master".into(), "96kHz / 24-bit FLAC + .p64".into(), "Codecs.matrix".into()),
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("Codecs.matrix", { domain: "sound-spatial-hrtf", sample_rate: 96000, ambisonic_order: 3 });
}"#,
            vec![
                "hmc://synth/audio/spatial-hrtf-master-bus.hmc".into(),
                "q42://commons/ontologies/audio-acoustics-qudt.q42".into(),
            ],
        ),
        IndustryDomainProfile::FilmAndMotionPicture => (
            "pcon-film-nle-master-01",
            "media",
            "social",
            "empirical",
            "DISP-3-TV",
            "did:q42:device:laptop-02",
            vec![
                ContainerDisplayWidget::Holo10dAndSpectralViewport {
                    scene_kind: "film-4k-uhd-master".into(),
                    wavelength_nm: 555.0,
                    orbit_yaw_deg: 24.0,
                },
                ContainerDisplayWidget::ControlSurfaceDeck {
                    deck_title: "Laptop NLE Multi-Track Timeline, JKL Shuttle & Camera Lens Tween Deck".into(),
                    parameters: vec![
                        ("Timecode SMPTE".into(), "01:14:22:18 (24.00 fps)".into(), "Timeline.scrub".into()),
                        ("Camera Focal Length".into(), "35mm Anamorphic T2.0".into(), "Camera3D.lens_tween".into()),
                        ("4K TV Reference Stream".into(), "HDMI 2.1 / Chromecast 10-bit HDR".into(), "Display.cast_tv".into()),
                        ("Container Master".into(), ".hmc + ISOBMFF ProRes 4444".into(), "Studio.export".into()),
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("WgslForge.generate", { domain: "film-compositor-hdr10", fps: 24 });
}"#,
            vec![
                "hmc://synth/film/scene-01-anamorphic-master.hmc".into(),
                "10d://synth/film/camera-rig-trajectory.10d".into(),
            ],
        ),
        IndustryDomainProfile::ColourGradingAndLookDev => (
            "pcon-colour-grading-aces-01",
            "vector",
            "technical",
            "empirical",
            "DISP-3-TV",
            "did:q42:device:tablet-03",
            vec![
                ContainerDisplayWidget::ScopesAndMeterBridge {
                    scope_title: "ACEScg / Rec.2020 CIE 1931 Chromaticity, RGB Parade & Vectorscope".into(),
                    channels: vec![
                        ("CIE ΔE00 Spectral Parity".into(), 0.34, "≤ 1.5 Certified".into()),
                        ("HDR Peak Luminance".into(), 1000.0, "cd/m² (nits)".into()),
                        ("Display P3 / Rec.2020 Gamut".into(), 99.2, "% Coverage".into()),
                        ("SSIM Frame Fidelity".into(), 0.996, "≥ 0.985 Certified".into()),
                    ],
                },
                ContainerDisplayWidget::ControlSurfaceDeck {
                    deck_title: "Tablet 3-Way Color Wheels (Lift / Gamma / Gain) & EMF v4 BRDF Inspector".into(),
                    parameters: vec![
                        ("Lift (Shadows)".into(), "[-0.012, -0.004, +0.018]".into(), "Raster.cms_convert".into()),
                        ("Gamma (Midtones)".into(), "[+1.020, +1.000, +0.985]".into(), "Raster.tone_curve".into()),
                        ("Gain (Highlights)".into(), "[+1.045, +1.012, +0.960]".into(), "Raster.cms_convert".into()),
                        ("EMF v4 Spectral Companion".into(), "380–780nm 5nm SPD Bound".into(), "WgslForge.roofline".into()),
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("WgslForge.roofline", { domain: "colour-acescg-lut3d", gamut: "Rec2020" });
}"#,
            vec![
                "emf://synth/lookdev/spectral-skin-foliage-v4.emf".into(),
                "10d://synth/lookdev/holo-pbr-turntable.10d".into(),
            ],
        ),
        IndustryDomainProfile::MedicalAndClinical => (
            "pcon-medical-clinical-10d-01",
            "health",
            "environmental",
            "empirical",
            "DISP-1-MAIN",
            "did:q42:device:tablet-03",
            vec![
                ContainerDisplayWidget::Holo10dAndSpectralViewport {
                    scene_kind: "health".into(),
                    wavelength_nm: 512.0,
                    orbit_yaw_deg: 36.0,
                },
                ContainerDisplayWidget::SemanticDataMatrix {
                    matrix_title: "Clinical Diagnostic Grounding (ICD-11 / OBO DOID / HP / Bio2RDF / Semantic Instruments)".into(),
                    headers: vec![
                        "Instrument / Finding".into(),
                        "Ontology IRI".into(),
                        "Authority Role".into(),
                        "Accommodation / Clinical Action".into(),
                    ],
                    rows: vec![
                        vec![
                            "ADHD Executive Profile".into(),
                            "obo:DOID_1094 · ICD-11 6A05".into(),
                            "QualifiedSpecialistAttested".into(),
                            "Structured written scheduling & sensory sanctuary".into(),
                        ],
                        vec![
                            "PTSD Trauma Safeguard".into(),
                            "obo:DOID_2055 · ICD-11 6B40".into(),
                            "QualifiedSpecialistAttested".into(),
                            "Safe housing tenure (CRPD Art. 19) & trauma-informed care".into(),
                        ],
                        vec![
                            "10D Anatomy SDFB Cutaway".into(),
                            "10d:HOLO:0x0306 · 21 Foramina".into(),
                            "CollaborativeCoAttested".into(),
                            "Cavity containment φ <= -ε & vascular/neural continuity".into(),
                        ],
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("ClinicalRisk.framingham", { age: 52, systolic_bp: 124, total_cholesterol_mmol: 4.9, hdl_cholesterol_mmol: 1.4 });
}"#,
            vec![
                "10d://synth/anatomy/cervical-foramina-holo-v2.10d".into(),
                "q42://sanctuary/clinical/bio2rdf-doid-hp.q42".into(),
            ],
        ),
        IndustryDomainProfile::EngineeringAndCadCae => (
            "pcon-engineering-cad-cae-01",
            "cad",
            "technical",
            "empirical",
            "DISP-1-MAIN",
            "did:q42:device:laptop-02",
            vec![
                ContainerDisplayWidget::Holo10dAndSpectralViewport {
                    scene_kind: "engineering-cad-brep".into(),
                    wavelength_nm: 475.0,
                    orbit_yaw_deg: 45.0,
                },
                ContainerDisplayWidget::SemanticDataMatrix {
                    matrix_title: "Parametric B-Rep Assembly, FEA/XPBD Simulation & QUDT Bill of Materials".into(),
                    headers: vec![
                        "Part / Sub-Assembly".into(),
                        "Representation".into(),
                        "QUDT Mass / Stress".into(),
                        "Verification Gate".into(),
                    ],
                    rows: vec![
                        vec![
                            "Primary Load Frame".into(),
                            "HOLO 0x0306 + NURBS B-Rep".into(),
                            "14.2 kg · σ_vm = 118 MPa".into(),
                            "Euler χ = 2 · Safety Factor 2.4x".into(),
                        ],
                        vec![
                            "6-DOF Kinematic Actuator".into(),
                            "PGA Motor INST / AREF".into(),
                            "δS = 0.000 J·s · cond(J) = 1.42".into(),
                            "Hamiltonian Stationary Action OK".into(),
                        ],
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("EngineeringAnalysis.kinematics", { dof: 6, solver: "PGA-XPBD-Hamiltonian" });
}"#,
            vec![
                "10d://synth/engineering/parametric-assembly-v2.10d".into(),
                "q42://commons/ontologies/qudt-engineering-units.q42".into(),
            ],
        ),
        IndustryDomainProfile::LegalAndHumanRights | IndustryDomainProfile::CustomSynthDomain => (
            "pcon-legal-human-rights-01",
            "ontology",
            "legal",
            "normative",
            "DISP-1-MAIN",
            "did:q42:device:laptop-02",
            vec![
                ContainerDisplayWidget::SemanticDataMatrix {
                    matrix_title: "Hohfeldian Deontic Norms, CML Evidence Correlator & Paraconsistent Contradiction Matrix".into(),
                    headers: vec![
                        "Norm / Exhibit ID".into(),
                        "Deontic Modality".into(),
                        "Grounded Treatise / IRI".into(),
                        "Forensic Audit Status".into(),
                    ],
                    rows: vec![
                        vec![
                            "cml:DutyOfInquiry".into(),
                            "OBLIGATES (0x10)".into(),
                            "UN CRPD Art. 13/19/28 · ICESCR Art. 11".into(),
                            "BREACH FLAGGED on Contested Notice EXH-DISP-05".into(),
                        ],
                        vec![
                            "udhr:Article27_2".into(),
                            "INALIENABLE RIGHT".into(),
                            "UDHR Art. 27(2) · ICESCR Art. 15(1)(c)".into(),
                            "Moral & Material Author Interests Protected (CC BY-NC-ND 4.0)".into(),
                        ],
                    ],
                },
                ContainerDisplayWidget::ControlSurfaceDeck {
                    deck_title: "Verifiable Dossier & QECP Cryptographic Chain-of-Custody Controls".into(),
                    parameters: vec![
                        ("Dossier Standard".into(), "ISO %PDF-1.7 + PAdES B-B + QECP".into(), "Pdf.inspect_cos".into()),
                        ("Contested Quarantine".into(), "Paraconsistent Isolation (0x30)".into(), "Paraconsistent.route".into()),
                        ("Signing Posture".into(), "Ed25519 + ML-DSA-65 (FIPS 204)".into(), "SHACL.validate".into()),
                    ],
                },
            ],
            r#"requires [ capability("capability.invoke") ];
effect fn run_module() {
    return capability.invoke("SHACL.extensions", { domain: "legal-human-rights-cml" });
}"#,
            vec![
                "q42://sanctuary/legal/webcivics-cml-normative.q42".into(),
                "qecp://sanctuary/dossiers/case-wf-2026-001.pdf".into(),
            ],
        ),
    };

    let mut module = SynthContainerModule {
        format_magic: "POET-CONTAINER-MODULE-V1".into(),
        module_id: module_id.into(),
        version: "0.0.41".into(),
        title,
        industry,
        base_container_kind_id: base_kind.into(),
        strata_id: strata_id.into(),
        epistemic_id: epistemic_id.into(),
        preferred_display_id: pref_disp.into(),
        companion_control_device_id: companion_dev.into(),
        default_width: 540.0,
        default_height: 360.0,
        widgets,
        vibe_recipe_source: vibe_recipe.into(),
        synth_asset_uris: assets,
        license_spdx: "CC-BY-NC-ND-4.0".into(),
        content_digest_hex: String::new(),
    };
    module.recompute_digest();
    module
}

/// Synthesizes a complete multi-container `.pman.json` Manifold Workspace Module for an industry profile,
/// complete with pre-wired containers and multi-display/device routing!
pub fn synthesize_industry_manifold_module(industry: IndustryDomainProfile) -> SynthManifoldModule {
    let primary = synthesize_industry_container_module(industry, None);
    let mut companion_ont = synthesize_industry_container_module(
        IndustryDomainProfile::LegalAndHumanRights,
        Some(&format!("{} — WordNet/.q42 Ontology & Provenance Gate", industry.title())),
    );
    companion_ont.module_id = format!("{}-q42-gate", primary.module_id);
    companion_ont.preferred_display_id = "DISP-4-LAPTOP".into();
    companion_ont.recompute_digest();

    let primary_id = primary.module_id.clone();
    let companion_id = companion_ont.module_id.clone();

    let payload = format!(
        "PMAN|{}|{}|{}",
        industry.slug(),
        primary.content_digest_hex,
        companion_ont.content_digest_hex
    );
    let digest = deterministic_digest_hex(payload.as_bytes());

    SynthManifoldModule {
        format_magic: "POET-MANIFOLD-MODULE-V1".into(),
        manifold_module_id: format!("pman-{}-01", industry.slug()),
        title: format!("{} Manifold Workstation", industry.title()),
        industry,
        graph_iri: format!("urn:q42:manifold:{}", industry.slug()),
        containers: vec![(primary, 36.0, 36.0), (companion_ont, 610.0, 36.0)],
        wires: vec![(
            primary_id,
            companion_id,
            "semantic-rel".into(),
            "qualia:groundsObservation".into(),
        )],
        license_spdx: "CC-BY-NC-ND-4.0".into(),
        content_digest_hex: digest,
    }
}

/// Mounts a `SynthContainerModule` (`.pcon`) onto Poet's spatial `Workbench` canvas and automatically
/// registers its preferred display/TV/tablet route in `QapSpaceMultiDeviceManager`.
pub fn mount_container_module_onto_workbench(
    module: &SynthContainerModule,
    wb: &mut Workbench,
    wm: &mut QapSpaceMultiDeviceManager,
    x: f64,
    y: f64,
) -> String {
    let kind = ContainerKind::from_id(&module.base_container_kind_id).unwrap_or(ContainerKind::Media);
    let strata = Strata::from_id(&module.strata_id).unwrap_or(Strata::Technical);
    let epistemic = Epistemic::from_id(&module.epistemic_id).unwrap_or(Epistemic::Objective);

    let node_id = module.module_id.clone();
    wb.nodes.retain(|n| n.id != node_id);
    wb.nodes.push(CanvasNode {
        id: node_id.clone(),
        kind,
        title: module.title.clone(),
        x,
        y,
        width: module.default_width,
        height: module.default_height,
        z: 0.0,
        d: 1.0,
        strata,
        epistemic,
    });
    wb.selected = Some(node_id.clone());

    let transport = if module.preferred_display_id == "DISP-3-TV" {
        wm.tv_transport()
    } else if module.preferred_display_id == "DISP-4-LAPTOP"
        || module.preferred_display_id == "DISP-5-TABLET"
    {
        DisplayTransportKind::WebRtcLocalLanQuicStream
    } else {
        DisplayTransportKind::HdmiOrDisplayPortWired
    };

    wm.container_routes.retain(|r| r.container_id != node_id);
    wm.container_routes.push(ContainerDisplayRoute {
        container_id: node_id.clone(),
        target_device_id: module.companion_control_device_id.clone(),
        target_display_id: module.preferred_display_id.clone(),
        target_role: if module.preferred_display_id == "DISP-3-TV" {
            DeviceRole::ContentPresentationDisplay
        } else {
            DeviceRole::PrimaryCanvas
        },
        transport,
        fullscreen_on_target: module.preferred_display_id == "DISP-3-TV",
    });

    wb.note(format!(
        "Mounted Synth Container Module `{}` ({}) → routed to {}",
        module.title, module.module_id, module.preferred_display_id
    ));
    node_id
}

/// Mounts an entire `SynthManifoldModule` (`.pman`) onto Poet's `Workbench`, switching to its target
/// manifold, placing all bundled `.pcon` containers, connecting their semantic wires, and routing displays!
pub fn mount_manifold_module_onto_workbench(
    manifold_mod: &SynthManifoldModule,
    wb: &mut Workbench,
    wm: &mut QapSpaceMultiDeviceManager,
) {
    wb.switch(manifold_mod.industry.default_manifold());
    wb.title = manifold_mod.title.clone();
    wb.graph_iri = manifold_mod.graph_iri.clone();

    for (container_mod, x, y) in &manifold_mod.containers {
        mount_container_module_onto_workbench(container_mod, wb, wm, *x, *y);
    }

    for (idx, (from_id, to_id, kind, predicate)) in manifold_mod.wires.iter().enumerate() {
        let wire_id = format!("pman-wire-{}-{idx}", manifold_mod.manifold_module_id);
        wb.wires.retain(|w| w.id != wire_id);
        wb.wires.push(Wire {
            id: wire_id,
            from: from_id.clone(),
            to: to_id.clone(),
            kind: kind.clone(),
            label: predicate.clone(),
        });
    }

    wb.note(format!(
        "Loaded Synth Manifold Module `{}` ({} modular containers, {} wires)",
        manifold_mod.title,
        manifold_mod.containers.len(),
        manifold_mod.wires.len()
    ));
}

// ---------------------------------------------------------------------------
// Aether RSX UI: SynthModularContainerStudioBay
// ---------------------------------------------------------------------------

#[component]
/// Interactive Studio Bay for authoring, previewing, importing/exporting (`.pcon.vibe` / `.pcon.q42.yaml` / `.pman.vibe`),
/// and mounting Synth-generated industry containers and manifolds (Sound, Film, Colour, Medical, Engineering, Legal).
pub fn SynthModularContainerStudioBay(wb: Signal<Workbench>) -> Element {
    let wm_ctx = try_consume_context::<Signal<QapSpaceMultiDeviceManager>>();
    let mut selected_industry = use_signal(|| IndustryDomainProfile::SoundAndSpatialAudio);
    let mut active_module = use_signal(|| {
        synthesize_industry_container_module(IndustryDomainProfile::SoundAndSpatialAudio, None)
    });
    let mut source_format = use_signal(|| "vibe");
    let mut vibe_buffer = use_signal(|| {
        synthesize_industry_container_module(IndustryDomainProfile::SoundAndSpatialAudio, None)
            .to_pcon_vibe()
            .unwrap_or_default()
    });
    let mut eval_output = use_signal(|| {
        String::from("Synth Modular Container (.pcon.vibe / .pcon.q42.yaml) & Manifold (.pman.vibe) Factory Ready")
    });

    let current_mod = active_module();
    let accent = current_mod.industry.accent_color();

    rsx! {
        div { style: "display:grid;gap:10px;padding:10px;background:#0b1120;color:#e2e8f0;font-size:.75rem;",
            // Header & Industry Domain Selector Bar
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;",
                div {
                    strong { style: "color:#38bdf8;font-size:.84rem;",
                        "🧩 Synth Modular Containers (.pcon.vibe / .q42.yaml) & Industry Manifolds (.pman.vibe) Studio"
                    }
                    div { style: "color:#94a3b8;font-size:.70rem;",
                        "Synthesize, save/load as native VibeScript (.vibe) or yaml-ld-q42 (.q42.yaml) modules, and mount specialized industry displays & multi-device tool surfaces."
                    }
                }
                div { style: "display:flex;gap:6px;flex-wrap:wrap;",
                    button {
                        r#type: "button",
                        style: "background:#0284c7;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.72rem;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let m = active_module();
                            let mut w = wb();
                            if let Some(mut wm_sig) = wm_ctx {
                                let mut wm_val = wm_sig();
                                mount_container_module_onto_workbench(&m, &mut w, &mut wm_val, 60.0, 60.0);
                                wm_sig.set(wm_val);
                            }
                            wb.set(w);
                        },
                        "↗ Mount .pcon.vibe Container onto Canvas ({current_mod.preferred_display_id})"
                    }
                    button {
                        r#type: "button",
                        style: "background:#7c3aed;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.72rem;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let pman = synthesize_industry_manifold_module(selected_industry());
                            let mut w = wb();
                            if let Some(mut wm_sig) = wm_ctx {
                                let mut wm_val = wm_sig();
                                mount_manifold_module_onto_workbench(&pman, &mut w, &mut wm_val);
                                wm_sig.set(wm_val);
                            }
                            wb.set(w);
                        },
                        "🌌 Mount Complete Industry Manifold (.pman.vibe Workstation)"
                    }
                    button {
                        r#type: "button",
                        style: "background:#059669;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.72rem;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            if let Ok(vb) = active_module().to_pcon_vibe() {
                                host::copy_to_clipboard(vb.clone());
                                source_format.set("vibe");
                                vibe_buffer.set(vb);
                                eval_output.set(format!(
                                    "Exported {}.pcon.vibe to clipboard (SHA-256: {}…)",
                                    active_module().module_id,
                                    &active_module().content_digest_hex[..16]
                                ));
                            }
                        },
                        "💾 Export .pcon.vibe"
                    }
                    button {
                        r#type: "button",
                        style: "background:#0f766e;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.72rem;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let yml = active_module().to_q42_yaml();
                            host::copy_to_clipboard(yml.clone());
                            source_format.set("yaml-ld-q42");
                            vibe_buffer.set(yml);
                            eval_output.set(format!(
                                "Exported {}.pcon.q42.yaml (yaml-ld-q42 v2 L0..L6) to clipboard",
                                active_module().module_id
                            ));
                        },
                        "📋 Export yaml-ld-q42 (.q42.yaml)"
                    }
                }
            }

            // 6 Industry Profile Selector Pills
            div { style: "display:flex;gap:5px;flex-wrap:wrap;background:#070b14;padding:5px;border-radius:6px;border:1px solid #1e293b;",
                for ind in IndustryDomainProfile::ALL {
                    button {
                        r#type: "button",
                        style: if selected_industry() == ind {
                            format!(
                                "background:{};color:#090d16;border:none;border-radius:4px;padding:4px 9px;font-size:.70rem;font-weight:700;cursor:pointer;",
                                ind.accent_color()
                            )
                        } else {
                            "background:#111827;color:#94a3b8;border:1px solid #1e293b;border-radius:4px;padding:4px 9px;font-size:.70rem;cursor:pointer;".to_string()
                        },
                        onclick: move |_| {
                            selected_industry.set(ind);
                            let synthesized = synthesize_industry_container_module(ind, None);
                            if source_format() == "yaml-ld-q42" {
                                vibe_buffer.set(synthesized.to_q42_yaml());
                            } else if let Ok(vb) = synthesized.to_pcon_vibe() {
                                vibe_buffer.set(vb);
                            }
                            active_module.set(synthesized);
                        },
                        "{ind.title()}"
                    }
                }
            }

            // Live Rendered Preview of the Active Industry Container Module's Custom Display Widgets
            div {
                style: format!(
                    "background:#111827;border:1px solid {accent};border-radius:6px;padding:10px;display:grid;gap:8px;"
                ),
                div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:6px;",
                    div {
                        strong { style: format!("color:{accent};font-size:.82rem;"), "{current_mod.title}" }
                        div { style: "color:#94a3b8;font-size:.68rem;font-family:monospace;",
                            "Module: {current_mod.module_id}.pcon.vibe · Display Target: {current_mod.preferred_display_id} · Companion Surface: {current_mod.companion_control_device_id}"
                        }
                    }
                    button {
                        r#type: "button",
                        style: "background:#1e293b;color:#38bdf8;border:1px solid #334155;border-radius:4px;padding:4px 9px;font-size:.70rem;font-weight:600;cursor:pointer;",
                        onclick: move |_| {
                            let recipe = active_module().vibe_recipe_source.clone();
                            spawn(async move {
                                match engine::eval(recipe, false, Some("run_module".into())).await {
                                    Ok(PoetEvalResult { ok: true, value, .. }) => {
                                        eval_output.set(format!("✓ Synth Module Kernel Executed: {value}"));
                                    }
                                    Ok(r) => {
                                        eval_output.set(r.diagnostic.unwrap_or(r.value));
                                    }
                                    Err(e) => eval_output.set(e),
                                }
                            });
                        },
                        "⚡ Execute Module VibeScript Kernel"
                    }
                }

                // Render the module's declarative widgets
                div { style: "display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:8px;",
                    for (w_idx, widget) in current_mod.widgets.iter().enumerate() {
                        match widget {
                            ContainerDisplayWidget::Holo10dAndSpectralViewport { scene_kind, wavelength_nm, orbit_yaw_deg } => {
                                let (uri, n, e, f) = engine::synthesize_live_etalon_10d_holo_svg_uri(
                                    scene_kind,
                                    480,
                                    200,
                                    *orbit_yaw_deg,
                                    *wavelength_nm,
                                );
                                rsx! {
                                    div {
                                        key: "w-{w_idx}",
                                        style: "background:#070b14;border:1px solid #1e293b;border-radius:5px;padding:6px;display:grid;gap:4px;",
                                        img {
                                            src: "{uri}",
                                            alt: "Synth Module 10D v2 Viewport",
                                            style: "width:100%;height:150px;object-fit:cover;border-radius:4px;",
                                        }
                                        div { style: "color:#38bdf8;font-family:monospace;font-size:.66rem;",
                                            "HOLO 0x0306 ({scene_kind}) · {n} nodes / {e} edges / {f} faces · λ {wavelength_nm}nm"
                                        }
                                    }
                                }
                            }
                            ContainerDisplayWidget::ScopesAndMeterBridge { scope_title, channels } => rsx! {
                                div {
                                    key: "w-{w_idx}",
                                    style: "background:#070b14;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:6px;",
                                    strong { style: "color:#38bdf8;font-size:.72rem;", "📊 {scope_title}" }
                                    for (ch_name, ch_val, ch_unit) in channels.iter() {
                                        div {
                                            style: "display:flex;justify-content:space-between;align-items:center;background:#111827;padding:4px 8px;border-radius:4px;border-left:3px solid #38bdf8;",
                                            span { style: "color:#e2e8f0;font-size:.70rem;", "{ch_name}" }
                                            span { style: "color:#34d399;font-family:monospace;font-size:.70rem;font-weight:700;",
                                                "{ch_val:.2} {ch_unit}"
                                            }
                                        }
                                    }
                                }
                            },
                            ContainerDisplayWidget::ControlSurfaceDeck { deck_title, parameters } => rsx! {
                                div {
                                    key: "w-{w_idx}",
                                    style: "background:#070b14;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:6px;",
                                    strong { style: "color:#a78bfa;font-size:.72rem;", "🎛️ {deck_title}" }
                                    for (p_name, p_val, p_cap) in parameters.iter() {
                                        div {
                                            style: "display:flex;justify-content:space-between;align-items:center;background:#111827;padding:4px 8px;border-radius:4px;",
                                            span { style: "color:#cbd5e1;font-size:.70rem;", "{p_name}" }
                                            div { style: "display:flex;gap:6px;align-items:center;",
                                                span { style: "color:#fbbf24;font-family:monospace;font-size:.70rem;font-weight:600;", "{p_val}" }
                                                span { style: "background:#1e293b;color:#94a3b8;padding:1px 5px;border-radius:3px;font-family:monospace;font-size:.62rem;",
                                                    "{p_cap}"
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            ContainerDisplayWidget::SemanticDataMatrix { matrix_title, headers, rows } => rsx! {
                                div {
                                    key: "w-{w_idx}",
                                    style: "background:#070b14;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:6px;overflow:auto;",
                                    strong { style: "color:#34d399;font-size:.72rem;", "🧬 {matrix_title}" }
                                    div { style: "display:grid;gap:4px;",
                                        div { style: "display:grid;grid-template-columns:repeat(4,1fr);gap:4px;color:#38bdf8;font-weight:700;font-size:.66rem;border-bottom:1px solid #1e293b;padding-bottom:3px;",
                                            for h in headers.iter() {
                                                span { "{h}" }
                                            }
                                        }
                                        for row in rows.iter() {
                                            div { style: "display:grid;grid-template-columns:repeat(4,1fr);gap:4px;color:#cbd5e1;font-size:.66rem;background:#111827;padding:3px 5px;border-radius:3px;",
                                                for cell in row.iter() {
                                                    span { "{cell}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }

                div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:6px;color:#94a3b8;font-size:.68rem;font-family:monospace;",
                    span { "Synth Assets: {current_mod.synth_asset_uris.join(\" · \")}" }
                    span { style: "color:#34d399;", "SHA-256: {&current_mod.content_digest_hex[..24]}…" }
                }
            }

            // Editable .pcon.vibe / .pcon.q42.yaml File Module Inspector & Live Hot-Loader
            div { style: "background:#070b14;border:1px solid #1e293b;border-radius:6px;padding:8px;display:grid;gap:6px;",
                div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:6px;",
                    strong { style: "color:#38bdf8;font-size:.72rem;",
                        "📄 Portable Module Source (.pcon.vibe VibeScript & yaml-ld-q42 .q42.yaml)"
                    }
                    div { style: "display:flex;gap:6px;align-items:center;",
                        span { style: "color:#34d399;font-size:.68rem;font-family:monospace;", "{eval_output}" }
                        button {
                            r#type: "button",
                            style: "background:#0284c7;color:#ffffff;border:none;border-radius:4px;padding:3px 9px;font-size:.68rem;font-weight:700;cursor:pointer;",
                            onclick: move |_| {
                                match SynthContainerModule::from_pcon_vibe(&vibe_buffer()) {
                                    Ok(parsed) => {
                                        eval_output.set(format!(
                                            "✓ Evaluated & Verified `{}` via vibe engine ({} widgets, digest {}…)",
                                            parsed.title,
                                            parsed.widgets.len(),
                                            &parsed.content_digest_hex[..14]
                                        ));
                                        active_module.set(parsed);
                                    }
                                    Err(e) => {
                                        eval_output.set(format!("✗ .pcon.vibe evaluation error: {e}"));
                                    }
                                }
                            },
                            "🔄 Hot-Load Edited .pcon.vibe Module"
                        }
                    }
                }
                textarea {
                    rows: "6",
                    style: "background:#090d16;border:1px solid #334155;color:#34d399;border-radius:4px;padding:6px;font-family:monospace;font-size:.67rem;",
                    value: "{vibe_buffer}",
                    oninput: move |e| vibe_buffer.set(e.value()),
                }
            }
        }
    }
}
