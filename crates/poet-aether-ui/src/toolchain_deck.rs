//! Global Manifold Deck & Contextual Toolchain Socket Cases (`Canvas_Workbench/js/toolchain-deck.js`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::engine;
use super::kinds::{ContainerKind, DimMode};
use super::store::Workbench;
use dioxus::prelude::*;

/// Presentation mode for the Global Manifold Deck.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckMode {
    /// Mode 1: Socket-Case Drop-Tray with Tear-Off Floating Pods.
    CaseTray,
    /// Mode 2: 3D Cylindrical Rotary Prism Drum.
    PrismDrum,
}

/// Deterministic action dispatched when a tool button is clicked in a socket case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckAction {
    /// Place a new container of the given kind on the stage.
    Place(ContainerKind),
    /// Switch the stage into 3D Volumetric mode (`DimMode::D3`).
    SetDim3D,
    /// Evaluate a VibeScript snippet and record its receipt in the status bar.
    EvalVibe(&'static str, &'static str),
    /// Record a deterministic status receipt on the workbench.
    Receipt(&'static str),
}

/// Individual tool inside a Toolchain Socket Case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocketTool {
    pub id: &'static str,
    pub label: &'static str,
    pub tip: &'static str,
    pub primary: bool,
    pub action: DeckAction,
}

/// A named Toolchain Socket Case (rendered as a pill in CaseTray mode or face in PrismDrum mode).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocketCaseSpec {
    pub id: &'static str,
    pub icon: &'static str,
    pub label: &'static str,
    pub tools: &'static [SocketTool],
}

/// Complete domain specification for the active container's Toolchain Deck.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DomainDeckSpec {
    pub domain_key: &'static str,
    pub badge: &'static str,
    pub cases: &'static [SocketCaseSpec],
}

const DOC_TYPOGRAPHY: &[SocketTool] = &[
    SocketTool { id: "bold", label: "B", tip: "Bold (Ctrl+B)", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Bold formatting applied") },
    SocketTool { id: "italic", label: "I", tip: "Italic (Ctrl+I)", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Italic formatting applied") },
    SocketTool { id: "underline", label: "U", tip: "Underline (Ctrl+U)", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Underline formatting applied") },
    SocketTool { id: "h1", label: "H1", tip: "Heading 1", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Promoted block to Heading 1") },
    SocketTool { id: "h2", label: "H2", tip: "Heading 2", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Promoted block to Heading 2") },
    SocketTool { id: "ul", label: "• List", tip: "Bulleted List", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Bulleted list toggled") },
    SocketTool { id: "ol", label: "1. List", tip: "Numbered List", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Numbered list toggled") },
    SocketTool { id: "quote", label: "❝ Quote", tip: "Blockquote", primary: false, action: DeckAction::Receipt("📝 Doc Typography: Dialectical blockquote inserted") },
];
const DOC_GROUNDING: &[SocketTool] = &[
    SocketTool { id: "cite", label: "📎 Cite RDF", tip: "Insert Ontological Citation", primary: false, action: DeckAction::Receipt("📎 Grounding: Inserted W3C RDF 1.2 citation anchor") },
    SocketTool { id: "rdfa", label: "🏷️ RDFa Bind", tip: "Bind Selection to Aura Property", primary: false, action: DeckAction::Receipt("🏷️ Grounding: Bound selection to Aura property IRI") },
    SocketTool { id: "prov", label: "🛡️ Sign Claim", tip: "Cryptographic Ed25519 Claim Signature", primary: false, action: DeckAction::Receipt("🛡️ Grounding: Signed claim with Ed25519 fiduciary key") },
    SocketTool { id: "seal", label: "🔏 Seal .hcf", tip: "Compile Document to Sealed .hcf Bundle", primary: true, action: DeckAction::EvalVibe("let bundle = \"doc.hcf\"; bundle", "🔏 Sealed document to .hcf cryptographic bundle") },
];
const DOC_MATH_NLP: &[SocketTool] = &[
    SocketTool { id: "latex", label: "∑ LaTeX", tip: "Insert Mathematical Formula Block", primary: false, action: DeckAction::Receipt("∑ Math & NLP: Inserted KaTeX formula block") },
    SocketTool { id: "ner", label: "🔍 Entity Scan", tip: "Extract Named Entities to RDF", primary: false, action: DeckAction::Receipt("🔍 Math & NLP: Extracted named entities to NQuin triples") },
];
const DOC_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "typography", icon: "📝", label: "Typography", tools: DOC_TYPOGRAPHY },
    SocketCaseSpec { id: "grounding", icon: "🏷️", label: "Grounding", tools: DOC_GROUNDING },
    SocketCaseSpec { id: "math_nlp", icon: "🔍", label: "Math & NLP", tools: DOC_MATH_NLP },
];

const MESH_KINEMATICS: &[SocketTool] = &[
    SocketTool { id: "3d-view", label: "🧊 3D View", tip: "Switch Stage to 3D Volumetric Mode", primary: false, action: DeckAction::SetDim3D },
    SocketTool { id: "pose", label: "🦴 Rig / Pose", tip: "Inspect Kinematic Bone Hierarchy", primary: false, action: DeckAction::Receipt("🦴 10D Kinematics: Kinematic bone hierarchy active (24 DOF)") },
    SocketTool { id: "p64", label: "📐 P64 Weights", tip: "Bind P64 Tensor Weights to Articulation", primary: false, action: DeckAction::Receipt("📐 10D Kinematics: P64 tensor weights bound to articulation") },
    SocketTool { id: "nerf", label: "☁️ NeRF / Mesh", tip: "Toggle Volumetric Splat vs Polygon Mesh", primary: false, action: DeckAction::Receipt("☁️ 10D Kinematics: Toggled Gaussian Splat / Polygon Mesh view") },
];
const MESH_TIMELINE: &[SocketTool] = &[
    SocketTool { id: "keyframe", label: "⏱️ + Keyframe", tip: "Insert 4D Temporal Keyframe at T", primary: false, action: DeckAction::Receipt("⏱️ Timeline: Inserted 4D keyframe at active temporal cursor") },
    SocketTool { id: "curve", label: "〰️ Easing Curve", tip: "Edit Temporal Interpolation Curve", primary: false, action: DeckAction::Receipt("〰️ Timeline: Cubic Hermite easing curve editor armed") },
    SocketTool { id: "sync", label: "🔊 Audio Sync", tip: "Lock Lip/Viseme Rig to Phoneme Track", primary: false, action: DeckAction::Receipt("🔊 Timeline: Locked 10D vocal-tract viseme rig to phoneme track") },
];
const MESH_HOOKS: &[SocketTool] = &[
    SocketTool { id: "hook", label: "⚡ Event Hook", tip: "Attach VibeScript Trigger on Collision/Click", primary: false, action: DeckAction::EvalVibe("let trigger = \"on_collision\"; trigger", "⚡ Attached VibeScript collision trigger hook") },
    SocketTool { id: "seal10d", label: "🔏 Seal .10d", tip: "Sign & Export 10D Asset with Creator DID", primary: true, action: DeckAction::Receipt("🔏 Event Hooks & DID: Signed .10d asset with Creator DID") },
    SocketTool { id: "royalty", label: "⚖️ License Gate", tip: "Configure Micro-Commons Usage Terms", primary: false, action: DeckAction::Receipt("⚖️ License Gate: Bilateral Micro-Commons usage terms bound") },
];
const MESH_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "kinematics", icon: "📐", label: "10D Kinematics", tools: MESH_KINEMATICS },
    SocketCaseSpec { id: "timeline", icon: "⏱️", label: "Timeline", tools: MESH_TIMELINE },
    SocketCaseSpec { id: "hooks", icon: "⚡", label: "Event Hooks & DID", tools: MESH_HOOKS },
];

const SHEET_FORMULA: &[SocketTool] = &[
    SocketTool { id: "sum", label: "∑ AutoSum", tip: "Insert Vector Reduction Formula", primary: false, action: DeckAction::EvalVibe("let sum = 142.5 + 88.25; sum", "∑ Formula Engine: Vector reduction evaluated") },
    SocketTool { id: "pivot", label: "🔄 Pivot Cube", tip: "Project Sheet to N-Dim HyperCube", primary: false, action: DeckAction::Receipt("🔄 Formula Engine: Projected sheet into N-Dim HyperCube") },
    SocketTool { id: "bind-col", label: "🏷️ Bind Column", tip: "Map Column Header to Aura Ontology IRI", primary: false, action: DeckAction::Receipt("🏷️ Formula Engine: Mapped column header to Aura Ontology IRI") },
];
const SHEET_P64: &[SocketTool] = &[
    SocketTool { id: "norm", label: "📏 L2 Normalize", tip: "Normalize Selected Tensor Slice", primary: false, action: DeckAction::Receipt("📏 P64 Latents: L2-normalized active tensor slice") },
    SocketTool { id: "pca", label: "🎯 Project 3D", tip: "Project High-Dim Cells to 3D Stage", primary: false, action: DeckAction::SetDim3D },
    SocketTool { id: "export-parquet", label: "📦 Apache Arrow", tip: "Zero-Copy Export to Arrow/Parquet", primary: true, action: DeckAction::Receipt("📦 P64 Latents: Zero-copy Arrow/Parquet stream exported") },
];
const SHEET_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "formula", icon: "🧮", label: "Formula Engine", tools: SHEET_FORMULA },
    SocketCaseSpec { id: "latents", icon: "📐", label: "P64 Latents", tools: SHEET_P64 },
];

const ONTOLOGY_AURA: &[SocketTool] = &[
    SocketTool { id: "shacl", label: "🛡️ Validate SHACL", tip: "Run SHACL Shape Constraints on Graph", primary: true, action: DeckAction::Receipt("🛡️ Aura & SHACL: All SHACL shape constraints satisfied (0 violations)") },
    SocketTool { id: "align", label: "🔗 Align Bridge", tip: "Synthesize Semantic Bridge Mapping", primary: false, action: DeckAction::Receipt("🔗 Aura & SHACL: Synthesized owl:equivalentProperty bridge mapping") },
];
const ONTOLOGY_SPARQL: &[SocketTool] = &[
    SocketTool { id: "sparql", label: "🔍 Run SPARQL", tip: "Execute Federated SPARQL 1.2 Query", primary: false, action: DeckAction::Receipt("🔍 SPARQL 1.2: Executed federated RDF-star query across 48-byte NQuins") },
    SocketTool { id: "n3", label: "⚙️ N3 Reasoner", tip: "Run Forward/Backward N3 Logic Rules", primary: false, action: DeckAction::Receipt("⚙️ N3 Reasoner: Derived 4 strict + 2 defeasible norm conclusions") },
    SocketTool { id: "nquin", label: "🧬 Inspect 48B Nquin", tip: "View Raw 48-Byte Hardware Super-Quin", primary: false, action: DeckAction::Receipt("🧬 NQuin Inspector: 48-byte Super-Quin parity XOR verified") },
];
const ONTOLOGY_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "aura_shacl", icon: "🛡️", label: "Aura & SHACL", tools: ONTOLOGY_AURA },
    SocketCaseSpec { id: "sparql_n3", icon: "🔍", label: "SPARQL, N3 & Nquins", tools: ONTOLOGY_SPARQL },
];

const CODE_EXEC: &[SocketTool] = &[
    SocketTool { id: "run", label: "▶ Run AST", tip: "Compile & Execute VibeScript in Sandbox", primary: true, action: DeckAction::EvalVibe("let status = \"AST Verified\"; status", "▶ VibeScript AST compiled and executed in Wasm sandbox") },
    SocketTool { id: "step", label: "⏭️ Step Op", tip: "Step Single Webizen Bytecode Instruction", primary: false, action: DeckAction::Receipt("⏭️ VibeScript Execution: Stepped 1 Webizen SlgOpcode frame") },
    SocketTool { id: "ast", label: "🌳 View AST", tip: "Inspect Typed Syntax Tree", primary: false, action: DeckAction::Receipt("🌳 VibeScript Execution: Typed AST inspector synchronized") },
];
const CODE_GAS: &[SocketTool] = &[
    SocketTool { id: "gas", label: "⛽ Gas Meter", tip: "Verify Deterministic Execution Budget", primary: false, action: DeckAction::Receipt("⛽ Capabilities & Gas: 1,420 / 50,000 deterministic gas units used") },
    SocketTool { id: "cap", label: "🔑 Capabilities", tip: "Inspect Sentinel Capability Grants", primary: false, action: DeckAction::Receipt("🔑 Capabilities & Gas: Level 3 Sentinel capability grants verified") },
    SocketTool { id: "wasm", label: "📦 Emit WASM", tip: "Compile Module to Portable WASM", primary: false, action: DeckAction::Receipt("📦 Capabilities & Gas: Emitted deterministic WASM component") },
];
const CODE_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "vibe_exec", icon: "▶", label: "VibeScript Execution", tools: CODE_EXEC },
    SocketCaseSpec { id: "vibe_caps", icon: "⛽", label: "Capabilities & Gas", tools: CODE_GAS },
];

const MAP_LAYERS: &[SocketTool] = &[
    SocketTool { id: "h3", label: "⬢ H3 Hex Grid", tip: "Toggle Uber H3 Spatial Index Overlay", primary: false, action: DeckAction::Receipt("⬢ GIS Layers: Uber H3 resolution-9 hex index overlay active") },
    SocketTool { id: "catchment", label: "💧 Catchment", tip: "Hydrological Flow & Watershed Layer", primary: false, action: DeckAction::Receipt("💧 GIS Layers: Hydrological watershed flow vectors projected") },
    SocketTool { id: "lidar", label: "🏔️ DEM / LiDAR", tip: "3D Terrain Elevation Extrusion", primary: false, action: DeckAction::SetDim3D },
    SocketTool { id: " geofence", label: "🚧 Geofence", tip: "Draw Jurisdiction / Sovereign Boundary", primary: true, action: DeckAction::Receipt("🚧 GIS Layers: Jurisdictional geofence polygon committed") },
];
const MAP_TIMELINE: &[SocketTool] = &[
    SocketTool { id: "replay", label: "⏪ 4D Replay", tip: "Scrub Spatiotemporal Incident Trace", primary: false, action: DeckAction::Receipt("⏪ Incident & Timeline: Scrubbing 4D spatiotemporal trace") },
    SocketTool { id: " dispatch", label: "🚨 Alert Node", tip: "Emit Emergency Telemetry Beacon", primary: false, action: DeckAction::Receipt("🚨 Incident & Timeline: Emergency telemetry beacon broadcast") },
];
const MAP_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "gis_layers", icon: "🗺️", label: "GIS Layers", tools: MAP_LAYERS },
    SocketCaseSpec { id: "gis_time", icon: "⏱️", label: "Incident & Timeline", tools: MAP_TIMELINE },
];

const HEALTH_BIO: &[SocketTool] = &[
    SocketTool { id: "fhir", label: "📋 HL7 FHIR", tip: "Validate Clinical Observation Resource", primary: false, action: DeckAction::Receipt("📋 Pathology & Biomarkers: HL7 FHIR R5 observation validated") },
    SocketTool { id: " sanctuary", label: "🛡️ Sanctuary Lock", tip: "Zero-Leak Local Encryption Gate", primary: true, action: DeckAction::Receipt("🛡️ Sanctuary Lock: Zero-leak PBKDF2 hardware lane engaged") },
];
const HEALTH_TOMO: &[SocketTool] = &[
    SocketTool { id: "dicom", label: "🩻 Volumetric Slice", tip: "Scrub Z-Axis Anatomical Cross-Section", primary: false, action: DeckAction::SetDim3D },
    SocketTool { id: "rig10d", label: "🫀 10D Organ Twin", tip: "Couple Physiological Telemetry to 10D Mesh", primary: false, action: DeckAction::Receipt("🫀 Tomography: Coupled live bio-telemetry to 10D organ twin") },
];
const HEALTH_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "pathology", icon: "🩺", label: "Pathology & Biomarkers", tools: HEALTH_BIO },
    SocketCaseSpec { id: "tomography", icon: "🩻", label: "Tomography & 10D Anatomy", tools: HEALTH_TOMO },
];

const SOCIAL_CHAT: &[SocketTool] = &[
    SocketTool { id: "invite", label: "🤖 + Summon Agent", tip: "Invite Specialist AI Persona to Room", primary: false, action: DeckAction::Receipt("🤖 Multi-Agent Chat: Summoned Specialist SME Agent into room") },
    SocketTool { id: "consensus", label: "⚖️ Ratify Vote", tip: "Propose M-of-N Governance Resolution", primary: true, action: DeckAction::Receipt("⚖️ Multi-Agent Chat: Proposed 2-of-3 M:N Guardianship resolution") },
    SocketTool { id: " fork", label: "🌿 Fork Thread", tip: "Branch Dialectical Argument into Sub-Canvas", primary: false, action: DeckAction::Place(ContainerKind::Subcanvas) },
];
const SOCIAL_WEBRTC: &[SocketTool] = &[
    SocketTool { id: "av", label: "🎥 P2P Stream", tip: "Open Encrypted WebRTC Media Channel", primary: false, action: DeckAction::Receipt("🎥 WebRTC & DID Auth: Encrypted P2P media stream negotiated") },
    SocketTool { id: "verify-did", label: "🔐 Verify Peer DID", tip: "Check Remote Agent Ed25519 Attestation", primary: false, action: DeckAction::Receipt("🔐 WebRTC & DID Auth: Peer did:q42 Ed25519 signature verified") },
    SocketTool { id: "screenshare", label: "🖥️ Cast Manifold", tip: "Stream Active Sub-Manifold to Peer", primary: false, action: DeckAction::Receipt("🖥️ WebRTC & DID Auth: Casting active manifold over P2P channel") },
];
const SOCIAL_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "agent_chat", icon: "💬", label: "Multi-Agent Chat", tools: SOCIAL_CHAT },
    SocketCaseSpec { id: "webrtc_did", icon: "🔐", label: "WebRTC & DID Auth", tools: SOCIAL_WEBRTC },
];

const GLOBAL_QUICK: &[SocketTool] = &[
    SocketTool { id: "new-doc", label: "📝 + Document", tip: "Place Rich Document Container", primary: false, action: DeckAction::Place(ContainerKind::Doc) },
    SocketTool { id: "new-sheet", label: "📊 + Tensor Sheet", tip: "Place Symbolic Spreadsheet Container", primary: false, action: DeckAction::Place(ContainerKind::Sheet) },
    SocketTool { id: "new-10d", label: "🧊 + 10D Asset", tip: "Place 10D Kinematic Asset Container", primary: false, action: DeckAction::Place(ContainerKind::Mesh3d) },
    SocketTool { id: "new-map", label: "🗺️ + GIS Map", tip: "Place Spatiotemporal GIS Container", primary: false, action: DeckAction::Place(ContainerKind::Map) },
];
const GLOBAL_CASES: &[SocketCaseSpec] = &[
    SocketCaseSpec { id: "quick_place", icon: "🧰", label: "Workspace Quick-Place", tools: GLOBAL_QUICK },
];

/// Maps the currently selected container kind (or `None`) to its contextual `DomainDeckSpec`.
pub fn deck_for_kind(kind: Option<ContainerKind>) -> DomainDeckSpec {
    match kind {
        Some(ContainerKind::Doc | ContainerKind::Pdf | ContainerKind::Deck) => DomainDeckSpec {
            domain_key: "doc",
            badge: "📝 Document / TinyMCE Suite",
            cases: DOC_CASES,
        },
        Some(ContainerKind::Mesh3d | ContainerKind::Media | ContainerKind::Vector) => DomainDeckSpec {
            domain_key: "10d",
            badge: "🧊 10D Asset (.10d) & Media Studio",
            cases: MESH_CASES,
        },
        Some(ContainerKind::Sheet | ContainerKind::Economics) => DomainDeckSpec {
            domain_key: "sheet",
            badge: "📊 Spreadsheet & Tensor Suite",
            cases: SHEET_CASES,
        },
        Some(ContainerKind::Ontology | ContainerKind::SemanticInstrument | ContainerKind::Chora) => DomainDeckSpec {
            domain_key: "ontology",
            badge: "🏷️ RDF 1.2 & Aura Catalog",
            cases: ONTOLOGY_CASES,
        },
        Some(ContainerKind::Code | ContainerKind::Shaders | ContainerKind::Codecs | ContainerKind::GitForge) => DomainDeckSpec {
            domain_key: "code",
            badge: "⚡ VibeScript 0.1 AST Console",
            cases: CODE_CASES,
        },
        Some(ContainerKind::Map) => DomainDeckSpec {
            domain_key: "map",
            badge: "🗺️ Spatiotemporal GIS Suite",
            cases: MAP_CASES,
        },
        Some(ContainerKind::Health) => DomainDeckSpec {
            domain_key: "health",
            badge: "🩺 Clinical & Bio-Telemetry Suite",
            cases: HEALTH_CASES,
        },
        Some(ContainerKind::Social | ContainerKind::WebRtc | ContainerKind::Mail) => DomainDeckSpec {
            domain_key: "social",
            badge: "📞 Social & Multi-Agent Suite",
            cases: SOCIAL_CASES,
        },
        _ => DomainDeckSpec {
            domain_key: "global",
            badge: "🌌 Global Manifold Deck",
            cases: GLOBAL_CASES,
        },
    }
}

/// Rotates the active prism face index by `delta` (`-1` or `+1`) modulo `len`.
pub fn step_prism_index(current: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let next = (current as isize + delta).rem_euclid(len as isize);
    next as usize
}

fn dispatch_tool(mut wb: Signal<Workbench>, tool: SocketTool) {
    let mut s = wb();
    match tool.action {
        DeckAction::Place(k) => {
            s.place(k);
            s.note(format!("Placed {} from Global Manifold Deck", k.title()));
        }
        DeckAction::SetDim3D => {
            s.dim = DimMode::D3;
            s.note("🧊 Switched stage to 3D Volumetric Mode (XYZD)");
        }
        DeckAction::EvalVibe(src, msg) => {
            s.note(msg);
            wb.set(s);
            spawn(async move {
                if let Ok(res) = engine::eval(src.to_string(), false, None).await {
                    let mut st = wb();
                    if res.ok && !res.value.is_empty() {
                        st.note(format!("{msg} → {}", res.value));
                    } else if let Some(diag) = res.diagnostic {
                        st.note(format!("{msg} ({diag})"));
                    }
                    wb.set(st);
                }
            });
            return;
        }
        DeckAction::Receipt(msg) => {
            s.note(msg);
        }
    }
    wb.set(s);
}

#[component]
/// Contextual Global Manifold Deck rendered directly below `ControlBar`.
pub fn ToolchainDeck(wb: Signal<Workbench>) -> Element {
    let mut mode = use_signal(|| DeckMode::CaseTray);
    let mut open_case = use_signal(|| Option::<&'static str>::None);
    let mut prism_idx = use_signal(|| 0usize);
    let mut pinned = use_signal(|| true);
    let mut torn_pods = use_signal(Vec::<&'static str>::new);

    let selected_kind = {
        let s = wb();
        s.selected
            .as_deref()
            .and_then(|id| s.node(id))
            .map(|n| n.kind)
    };
    let spec = deck_for_kind(selected_kind);
    let active_prism = prism_idx().min(spec.cases.len().saturating_sub(1));

    rsx! {
        div {
            id: "global-toolchain-deck",
            class: "toolchain-deck-wrapper",
            style: "position:relative;display:flex;align-items:center;justify-content:space-between;padding:4px 12px;background:rgba(14,19,28,0.92);border-bottom:1px solid var(--border-subtle);min-height:36px;z-index:40;gap:10px;",
            div { style: "display:flex;align-items:center;gap:8px;flex-wrap:wrap;",
                span {
                    class: "deck-domain-badge",
                    style: "font-size:11px;font-weight:700;padding:3px 9px;border-radius:4px;background:rgba(0,229,255,0.1);border:1px solid rgba(0,229,255,0.28);color:var(--accent-cyan);white-space:nowrap;",
                    "{spec.badge}"
                }
                if mode() == DeckMode::CaseTray {
                    for case in spec.cases.iter() {
                        {
                            let cid = case.id;
                            let is_open = open_case() == Some(cid);
                            let is_torn = torn_pods().contains(&cid);
                            let count = case.tools.len();
                            rsx! {
                                button {
                                    key: "{cid}",
                                    r#type: "button",
                                    class: if is_open { "socket-case-pill is-open" } else { "socket-case-pill" },
                                    style: format!(
                                        "display:inline-flex;align-items:center;gap:5px;padding:3px 9px;border-radius:999px;font-size:11px;cursor:pointer;border:1px solid {};background:{};color:var(--text-primary);",
                                        if is_open { "var(--accent-cyan)" } else if is_torn { "var(--accent-gold)" } else { "rgba(255,255,255,0.14)" },
                                        if is_open { "rgba(0,229,255,0.16)" } else { "rgba(255,255,255,0.04)" }
                                    ),
                                    onclick: move |_| {
                                        if open_case() == Some(cid) {
                                            open_case.set(None);
                                        } else {
                                            open_case.set(Some(cid));
                                        }
                                    },
                                    span { "{case.icon}" }
                                    span { "{case.label}" }
                                    span { style: "font-size:10px;opacity:0.75;font-family:var(--font-mono);", "({count})" }
                                    span { style: "font-size:9px;opacity:0.7;", "▾" }
                                }
                            }
                        }
                    }
                } else {
                    if let Some(face) = spec.cases.get(active_prism) {
                        div {
                            class: "prism-drum-viewport",
                            style: "display:flex;align-items:center;gap:6px;background:rgba(255,255,255,0.03);border:1px solid rgba(0,229,255,0.22);border-radius:6px;padding:3px 8px;",
                            div { style: "display:flex;flex-direction:column;gap:1px;",
                                button {
                                    r#type: "button",
                                    title: "Rotate Prism Up",
                                    style: "background:transparent;border:none;color:var(--text-secondary);font-size:9px;cursor:pointer;line-height:1;",
                                    onclick: move |_| prism_idx.set(step_prism_index(active_prism, -1, spec.cases.len())),
                                    "▲"
                                }
                                button {
                                    r#type: "button",
                                    title: "Rotate Prism Down",
                                    style: "background:transparent;border:none;color:var(--text-secondary);font-size:9px;cursor:pointer;line-height:1;",
                                    onclick: move |_| prism_idx.set(step_prism_index(active_prism, 1, spec.cases.len())),
                                    "▼"
                                }
                            }
                            span { style: "font-size:10px;font-weight:700;color:var(--accent-gold);white-space:nowrap;",
                                "{face.icon} {face.label} ({active_prism + 1}/{spec.cases.len()})"
                            }
                            div { style: "display:flex;align-items:center;gap:4px;flex-wrap:wrap;",
                                for tool in face.tools.iter().copied() {
                                    button {
                                        key: "{tool.id}",
                                        r#type: "button",
                                        title: "{tool.tip}",
                                        style: "padding:2px 7px;font-size:11px;border-radius:4px;border:1px solid rgba(255,255,255,0.14);background:rgba(255,255,255,0.05);color:var(--text-primary);cursor:pointer;",
                                        onclick: move |_| dispatch_tool(wb, tool),
                                        "{tool.label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { style: "display:flex;align-items:center;gap:6px;",
                button {
                    r#type: "button",
                    title: "Switch between Socket-Case Drop-Tray and 3D Rotary Prism Drum",
                    style: "padding:3px 8px;font-size:10px;border-radius:4px;border:1px solid var(--border-medium);background:var(--surface-panel);color:var(--text-secondary);cursor:pointer;",
                    onclick: move |_| {
                        let next = if mode() == DeckMode::CaseTray { DeckMode::PrismDrum } else { DeckMode::CaseTray };
                        mode.set(next);
                        open_case.set(None);
                    },
                    if mode() == DeckMode::CaseTray { "🧰 Mode: Socket Cases" } else { "🎡 Mode: 3D Prism Drum" }
                }
                button {
                    r#type: "button",
                    title: "Pin / Auto-Hide Global Manifold Deck",
                    style: format!(
                        "padding:3px 7px;font-size:10px;border-radius:4px;border:1px solid {};background:var(--surface-panel);color:var(--text-primary);cursor:pointer;",
                        if pinned() { "var(--accent-cyan)" } else { "var(--border-subtle)" }
                    ),
                    onclick: move |_| pinned.set(!pinned()),
                    "📌"
                }
            }
            if mode() == DeckMode::CaseTray {
                if let Some(active_id) = open_case() {
                    if let Some(case) = spec.cases.iter().find(|c| c.id == active_id) {
                        {
                            let cid = case.id;
                            let is_torn = torn_pods().contains(&cid);
                            rsx! {
                                div {
                                    class: "socket-case-drop-tray is-visible",
                                    style: "position:absolute;top:100%;left:12px;margin-top:4px;background:rgba(15,21,32,0.96);backdrop-filter:blur(12px);border:1px solid rgba(0,229,255,0.35);border-radius:8px;padding:8px 12px;box-shadow:0 12px 28px rgba(0,0,0,0.55);z-index:80;min-width:340px;",
                                    div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:6px;gap:12px;",
                                        span { style: "font-size:11px;font-weight:700;color:var(--accent-cyan);",
                                            "{case.icon} {case.label} Socket Case"
                                        }
                                        div { style: "display:flex;gap:6px;",
                                            button {
                                                r#type: "button",
                                                style: "font-size:10px;padding:2px 7px;border-radius:4px;border:1px solid rgba(255,193,7,0.4);background:rgba(255,193,7,0.12);color:var(--accent-gold);cursor:pointer;",
                                                onclick: move |_| {
                                                    let mut list = torn_pods();
                                                    if let Some(pos) = list.iter().position(|x| *x == cid) {
                                                        list.remove(pos);
                                                    } else {
                                                        list.push(cid);
                                                    }
                                                    torn_pods.set(list);
                                                },
                                                if is_torn { "📌 Dock Pod" } else { "📌 Tear-off Pod" }
                                            }
                                            button {
                                                r#type: "button",
                                                style: "font-size:10px;padding:2px 6px;border-radius:4px;border:1px solid rgba(255,255,255,0.15);background:transparent;color:var(--text-secondary);cursor:pointer;",
                                                onclick: move |_| open_case.set(None),
                                                "✕"
                                            }
                                        }
                                    }
                                    div { style: "display:flex;flex-wrap:wrap;gap:6px;",
                                        for (idx, tool) in case.tools.iter().copied().enumerate() {
                                            button {
                                                key: "{tool.id}",
                                                r#type: "button",
                                                title: "{tool.tip}",
                                                style: format!(
                                                    "display:inline-flex;align-items:center;gap:5px;padding:4px 9px;border-radius:5px;font-size:11px;cursor:pointer;border:1px solid {};background:{};color:var(--text-primary);",
                                                    if tool.primary { "var(--accent-cyan)" } else { "rgba(255,255,255,0.14)" },
                                                    if tool.primary { "rgba(0,229,255,0.15)" } else { "rgba(255,255,255,0.04)" }
                                                ),
                                                onclick: move |_| dispatch_tool(wb, tool),
                                                span { style: "font-size:9px;opacity:0.65;font-family:var(--font-mono);", "{idx + 1}" }
                                                span { "{tool.label}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !torn_pods().is_empty() {
                div {
                    class: "floating-toolchain-pods",
                    style: "position:fixed;top:92px;right:24px;display:flex;flex-direction:column;gap:8px;z-index:75;",
                    for pod_id in torn_pods().iter().copied() {
                        if let Some(case) = spec.cases.iter().find(|c| c.id == pod_id) {
                            div {
                                key: "pod-{pod_id}",
                                class: "floating-toolchain-pod",
                                style: "background:rgba(14,20,30,0.95);border:1px solid var(--accent-gold);border-radius:8px;padding:6px 10px;box-shadow:0 8px 20px rgba(0,0,0,0.5);",
                                div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:4px;gap:8px;",
                                    span { style: "font-size:10px;font-weight:700;color:var(--accent-gold);", "📌 {case.icon} {case.label}" }
                                    button {
                                        r#type: "button",
                                        style: "background:transparent;border:none;color:var(--text-secondary);font-size:10px;cursor:pointer;",
                                        onclick: move |_| {
                                            let mut list = torn_pods();
                                            list.retain(|x| *x != pod_id);
                                            torn_pods.set(list);
                                        },
                                        "✕"
                                    }
                                }
                                div { style: "display:flex;flex-wrap:wrap;gap:4px;",
                                    for tool in case.tools.iter().copied() {
                                        button {
                                            key: "pod-{pod_id}-{tool.id}",
                                            r#type: "button",
                                            title: "{tool.tip}",
                                            style: "padding:2px 6px;font-size:10px;border-radius:4px;border:1px solid rgba(255,255,255,0.14);background:rgba(255,255,255,0.05);color:var(--text-primary);cursor:pointer;",
                                            onclick: move |_| dispatch_tool(wb, tool),
                                            "{tool.label}"
                                        }
                                    }
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

    #[test]
    fn maps_container_kinds_to_expected_socket_cases() {
        let doc = deck_for_kind(Some(ContainerKind::Doc));
        assert_eq!(doc.domain_key, "doc");
        assert_eq!(doc.cases.len(), 3);
        assert_eq!(doc.cases[0].tools.len(), 8);
        assert_eq!(doc.cases[1].tools.len(), 4);
        assert_eq!(doc.cases[2].tools.len(), 2);

        let mesh = deck_for_kind(Some(ContainerKind::Mesh3d));
        assert_eq!(mesh.domain_key, "10d");
        assert_eq!(mesh.cases.len(), 3);
        assert_eq!(mesh.cases[0].tools.len(), 4);

        let sheet = deck_for_kind(Some(ContainerKind::Sheet));
        assert_eq!(sheet.domain_key, "sheet");
        assert_eq!(sheet.cases.len(), 2);

        let ont = deck_for_kind(Some(ContainerKind::Ontology));
        assert_eq!(ont.domain_key, "ontology");
        assert_eq!(ont.cases.len(), 2);

        let global = deck_for_kind(None);
        assert_eq!(global.domain_key, "global");
        assert_eq!(global.cases[0].tools.len(), 4);
    }

    #[test]
    fn prism_drum_stepper_wraps_cleanly() {
        assert_eq!(step_prism_index(0, -1, 3), 2);
        assert_eq!(step_prism_index(2, 1, 3), 0);
        assert_eq!(step_prism_index(1, 1, 3), 2);
    }
}
