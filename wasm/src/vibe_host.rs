//! Thin `vibe-host-0.1` facade for Poet (G-A).
//!
//! Re-exports the frozen four-op surface + versions. No AST / Host trait
//! bleed into Poet UI — capability reach is invoke-only and is resolved
//! through Vibe's in-process catalog kernels.

pub use vibe::{
    check_cell, check_program, diagnose, parse_cell, parse_program, DiagCode, DiagnoseReport,
    Diagnostic, Span, Value, LANGUAGE_VERSION,
};

/// Host ABI stamp for the frozen four-op surface.
pub const HOST_VERSION: &str = "vibe-host-0.1";

/// Crate / branch stamp aligned with `0.0.40`.
pub const CRATE_STAMP: &str = "0.0.40";

/// Metadata and embedded source for a packaged Poet `.vibe` workflow script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackagedVibeScript {
    /// Stable script identifier (e.g. `"poet_document_cml"`).
    pub id: &'static str,
    /// File name under `vibe/` (e.g. `"poet_document_cml.vibe"`).
    pub filename: &'static str,
    /// Target Poet manifold / studio domain.
    pub manifold: &'static str,
    /// Human-readable workflow title.
    pub title: &'static str,
    /// Embedded `.vibe` source text.
    pub source: &'static str,
}

/// Static catalog of packaged Poet `.vibe` workflow scripts embedded from `vibe/`.
pub const PACKAGED_VIBE_SCRIPTS: &[PackagedVibeScript; 4] = &[
    PackagedVibeScript {
        id: "poet_document_cml",
        filename: "poet_document_cml.vibe",
        manifold: "document",
        title: "CML Document Grounding & Gazetteer Workflow",
        source: include_str!("../../vibe/poet_document_cml.vibe"),
    },
    PackagedVibeScript {
        id: "poet_numeric_sheet",
        filename: "poet_numeric_sheet.vibe",
        manifold: "sheet",
        title: "Reactive Sheet Formula & Tensor Evaluation Workflow",
        source: include_str!("../../vibe/poet_numeric_sheet.vibe"),
    },
    PackagedVibeScript {
        id: "poet_spatial_authoring",
        filename: "poet_spatial_authoring.vibe",
        manifold: "spatial",
        title: "Spatial, Botanical & Terrain Authoring Workflow",
        source: include_str!("../../vibe/poet_spatial_authoring.vibe"),
    },
    PackagedVibeScript {
        id: "poet_research_manifold",
        filename: "poet_research_manifold.vibe",
        manifold: "research",
        title: "Research Manifold & SHACL Verification Workflow",
        source: include_str!("../../vibe/poet_research_manifold.vibe"),
    },
];

/// Returns all packaged Poet `.vibe` workflow scripts bundled with the crate.
pub fn packaged_vibe_scripts() -> &'static [PackagedVibeScript] {
    PACKAGED_VIBE_SCRIPTS
}

/// Looks up a packaged `.vibe` workflow script by its `id` or `filename`.
pub fn packaged_vibe_script(id_or_filename: &str) -> Option<&'static PackagedVibeScript> {
    PACKAGED_VIBE_SCRIPTS
        .iter()
        .find(|script| script.id == id_or_filename || script.filename == id_or_filename)
}

/// Pin: invoke by id only. Does not expose the wide [`vibe::Host`] trait.
///
/// Catalog capabilities are executed by Vibe's bounded in-process kernels, so
/// Poet remains usable without a Webizen Studio or daemon dependency. Unknown
/// ids still fail closed with the catalog diagnostic.
pub fn capability_invoke(id: &str, args: &Value, span: Span) -> Result<Value, Diagnostic> {
    vibe::catalog::invoke_local(id, args, span)
}

/// Diagnose JSON string with native shape (`valid`, `kind`, `error_code`, …).
pub fn diagnose_json(src: &str) -> String {
    diagnose(src).to_json()
}

/// Returns the host version.
pub fn host_version() -> &'static str {
    HOST_VERSION
}

/// Returns the language version.
pub fn language_version() -> &'static str {
    LANGUAGE_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn catalog_capability_invocation_is_live_without_studio() {
        let mut args = BTreeMap::new();
        args.insert("family".into(), Value::String("spatial_kinematics".into()));
        args.insert("preset".into(), Value::String("orbit_spin".into()));
        args.insert("t".into(), Value::F64(0.5));
        let value = capability_invoke(
            "Animation.evaluate_preset",
            &Value::Record(args),
            Span::point(0),
        )
        .expect("catalog kernel should execute");
        assert!(matches!(value, Value::Record(_)));
    }

    #[test]
    fn unknown_capability_remains_an_explicit_error() {
        let error = capability_invoke("Unknown.capability", &Value::Null, Span::point(0))
            .expect_err("unknown ids must not silently succeed");
        assert_eq!(error.code, DiagCode::E100);
    }

    #[test]
    fn packaged_vibe_scripts_parse_and_diagnose_cleanly() {
        let scripts = packaged_vibe_scripts();
        assert_eq!(scripts.len(), 4);
        for script in scripts {
            let report = diagnose(script.source);
            assert!(
                report.valid,
                "packaged script {} failed diagnose: {:?}",
                script.filename, report.errors
            );
            assert_eq!(report.kind, "module");
            assert!(packaged_vibe_script(script.id).is_some());
            assert!(packaged_vibe_script(script.filename).is_some());
        }
    }
}
