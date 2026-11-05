//! Packaged Poet `.vibe` workflow scripts embedded from `vibe/`.
//!
//! Mirrors the `wasm/src/vibe_host.rs` catalog for native CLI and Studio access.

/// Metadata and embedded source for a packaged Poet `.vibe` workflow script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackagedVibeScript {
    /// Stable script identifier (e.g. `"poet_document_cml"`).
    pub id: &'static str,
    /// File name under `vibe/` (e.g. `"poet_document_cml.vibe"`).
    pub filename: &'static str,
    /// Target Poet manifold / studio domain.
    pub manifold: &'static str,
    /// Alias for `manifold` domain classification.
    pub domain: &'static str,
    /// Human-readable workflow title.
    pub title: &'static str,
    /// Required capability namespaces declared via `using` in this workflow.
    pub required_capabilities: &'static [&'static str],
    /// Embedded `.vibe` source text.
    pub source: &'static str,
}

/// Static catalog of packaged Poet `.vibe` workflow scripts embedded from `vibe/`.
pub const PACKAGED_VIBE_SCRIPTS: &[PackagedVibeScript; 4] = &[
    PackagedVibeScript {
        id: "poet_document_cml",
        filename: "poet_document_cml.vibe",
        manifold: "document",
        domain: "document",
        title: "CML Document Grounding & Gazetteer Workflow",
        required_capabilities: &["Document", "NLP", "Inference", "GraphDatabase"],
        source: include_str!("../../vibe/poet_document_cml.vibe"),
    },
    PackagedVibeScript {
        id: "poet_numeric_sheet",
        filename: "poet_numeric_sheet.vibe",
        manifold: "sheet",
        domain: "sheet",
        title: "Reactive Sheet Formula & Tensor Evaluation Workflow",
        required_capabilities: &["Sheet", "Statistics", "LinearAlgebra"],
        source: include_str!("../../vibe/poet_numeric_sheet.vibe"),
    },
    PackagedVibeScript {
        id: "poet_spatial_authoring",
        filename: "poet_spatial_authoring.vibe",
        manifold: "spatial",
        domain: "spatial",
        title: "Spatial, Botanical & Terrain Authoring Workflow",
        required_capabilities: &[
            "BotanicalAnatomy",
            "Terrain",
            "PoetGeo",
            "Topology",
            "SceneGraph10D",
        ],
        source: include_str!("../../vibe/poet_spatial_authoring.vibe"),
    },
    PackagedVibeScript {
        id: "poet_research_manifold",
        filename: "poet_research_manifold.vibe",
        manifold: "research",
        domain: "research",
        title: "Research Manifold & SHACL Verification Workflow",
        required_capabilities: &[
            "Research",
            "SHACL",
            "EpistemicLogic",
            "DeonticLogic",
            "Poet",
        ],
        source: include_str!("../../vibe/poet_research_manifold.vibe"),
    },
];

/// Returns all packaged Poet `.vibe` workflow scripts bundled with the crate.
pub fn packaged_vibe_scripts() -> &'static [PackagedVibeScript] {
    PACKAGED_VIBE_SCRIPTS
}

/// Looks up a packaged `.vibe` workflow script by its `id` or `filename`.
pub fn find_packaged_vibe_script(id_or_name: &str) -> Option<&'static PackagedVibeScript> {
    PACKAGED_VIBE_SCRIPTS
        .iter()
        .find(|script| script.id == id_or_name || script.filename == id_or_name)
}

/// Alias matching `wasm/src/vibe_host.rs` `packaged_vibe_script(id_or_filename)`.
pub fn packaged_vibe_script(id_or_filename: &str) -> Option<&'static PackagedVibeScript> {
    find_packaged_vibe_script(id_or_filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packaged_vibe_scripts_parse_and_typecheck_cleanly() {
        let scripts = packaged_vibe_scripts();
        assert_eq!(scripts.len(), 4);
        for script in scripts {
            let program = vibe::parse_program(script.source).unwrap_or_else(|diag| {
                panic!(
                    "packaged script {} failed to parse: {}",
                    script.filename, diag.message
                )
            });
            vibe::check_program(&program).unwrap_or_else(|diag| {
                panic!(
                    "packaged script {} failed to typecheck: {}",
                    script.filename, diag.message
                )
            });
            assert_eq!(find_packaged_vibe_script(script.id), Some(script));
            assert_eq!(find_packaged_vibe_script(script.filename), Some(script));
            assert_eq!(packaged_vibe_script(script.id), Some(script));
        }
    }
}
