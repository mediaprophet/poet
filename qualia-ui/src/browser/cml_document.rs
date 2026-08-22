//! Context Markup Language (CML) HyperDoc Document Model & Serialization Engine.
//!
//! Provides the structured representation of documents annotated with CML
//! (<q-entity>, <q-relation>), bi-directional serialization (Visual HTML,
//! Markdown, RDF-Star), and live SHACL/Aura validation metrics.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use serde::{Deserialize, Serialize};

/// CML Entity Categories aligned with W3C RDF 1.2 & QualiaDB ontologies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CmlCategory {
    NamedEntity,
    Term,
    ClaimedFact,
    Hypothesis,
    Definition,
    Statistic,
    Citation,
    DeonticRule,
    EpistemicBelief,
    Metric,
    CodeSymbol,
    RelationSource,
}

impl CmlCategory {
    pub fn all() -> &'static [CmlCategory] {
        &[
            CmlCategory::NamedEntity,
            CmlCategory::Term,
            CmlCategory::ClaimedFact,
            CmlCategory::Hypothesis,
            CmlCategory::Definition,
            CmlCategory::Statistic,
            CmlCategory::Citation,
            CmlCategory::DeonticRule,
            CmlCategory::EpistemicBelief,
            CmlCategory::Metric,
            CmlCategory::CodeSymbol,
            CmlCategory::RelationSource,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            CmlCategory::NamedEntity => "entity",
            CmlCategory::Term => "term",
            CmlCategory::ClaimedFact => "claimedFact",
            CmlCategory::Hypothesis => "hypothesis",
            CmlCategory::Definition => "definition",
            CmlCategory::Statistic => "statistic",
            CmlCategory::Citation => "citation",
            CmlCategory::DeonticRule => "deonticRule",
            CmlCategory::EpistemicBelief => "epistemicBelief",
            CmlCategory::Metric => "metric",
            CmlCategory::CodeSymbol => "codeSymbol",
            CmlCategory::RelationSource => "relationSource",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            CmlCategory::NamedEntity => "Named Entity",
            CmlCategory::Term => "Terminology",
            CmlCategory::ClaimedFact => "Claimed Fact",
            CmlCategory::Hypothesis => "Hypothesis",
            CmlCategory::Definition => "Definition",
            CmlCategory::Statistic => "Statistic",
            CmlCategory::Citation => "Citation",
            CmlCategory::DeonticRule => "Deontic Rule",
            CmlCategory::EpistemicBelief => "Epistemic Belief",
            CmlCategory::Metric => "Metric / Gauge",
            CmlCategory::CodeSymbol => "Code Symbol",
            CmlCategory::RelationSource => "Relation Source",
        }
    }

    pub fn glyph(&self) -> &'static str {
        match self {
            CmlCategory::NamedEntity => "\u{1F4CD}",      // 📍
            CmlCategory::Term => "\u{1F539}",             // 🔹
            CmlCategory::ClaimedFact => "\u{1F4AF}",       // 💯
            CmlCategory::Hypothesis => "\u{1F52C}",        // 🔬
            CmlCategory::Definition => "\u{1F4D8}",        // 📘
            CmlCategory::Statistic => "\u{1F4CA}",         // 📊
            CmlCategory::Citation => "\u{1F4D1}",          // 📑
            CmlCategory::DeonticRule => "\u{2696}\u{FE0F}", // ⚖️
            CmlCategory::EpistemicBelief => "\u{1F9E0}",   // 🧠
            CmlCategory::Metric => "\u{26A1}",            // ⚡
            CmlCategory::CodeSymbol => "\u{1F4BB}",        // 💻
            CmlCategory::RelationSource => "\u{1F517}",    // 🔗
        }
    }

    pub fn color_accent(&self) -> &'static str {
        match self {
            CmlCategory::NamedEntity => "var(--accent-primary, #6366f1)",
            CmlCategory::Term => "var(--accent-info, #0ea5e9)",
            CmlCategory::ClaimedFact => "var(--accent-success, #10b981)",
            CmlCategory::Hypothesis => "var(--accent-warning, #f59e0b)",
            CmlCategory::Definition => "var(--accent-secondary, #8b5cf6)",
            CmlCategory::Statistic => "var(--accent-cyan, #06b6d4)",
            CmlCategory::Citation => "var(--accent-muted, #64748b)",
            CmlCategory::DeonticRule => "var(--accent-danger, #ef4444)",
            CmlCategory::EpistemicBelief => "var(--accent-purple, #a855f7)",
            CmlCategory::Metric => "var(--accent-amber, #d97706)",
            CmlCategory::CodeSymbol => "var(--accent-emerald, #059669)",
            CmlCategory::RelationSource => "var(--accent-indigo, #4f46e5)",
        }
    }

    pub fn from_code(code: &str) -> Option<CmlCategory> {
        match code {
            "entity" => Some(CmlCategory::NamedEntity),
            "term" => Some(CmlCategory::Term),
            "claimedFact" => Some(CmlCategory::ClaimedFact),
            "hypothesis" => Some(CmlCategory::Hypothesis),
            "definition" => Some(CmlCategory::Definition),
            "statistic" => Some(CmlCategory::Statistic),
            "citation" => Some(CmlCategory::Citation),
            "deonticRule" => Some(CmlCategory::DeonticRule),
            "epistemicBelief" => Some(CmlCategory::EpistemicBelief),
            "metric" => Some(CmlCategory::Metric),
            "codeSymbol" => Some(CmlCategory::CodeSymbol),
            "relationSource" => Some(CmlCategory::RelationSource),
            _ => None,
        }
    }
}

/// A span annotation representing an interactive `<q-entity>` within a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmlSpan {
    pub id: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub category: CmlCategory,
    pub iri: String,
    pub label: String,
    pub certainty: u8, // 0..=100
    pub provenance: String,
}

/// A directional semantic relation connecting two CML entity spans (`<q-relation>`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmlRelation {
    pub id: String,
    pub subject_id: String,
    pub predicate_iri: String,
    pub object_id: String,
    pub label: String,
    pub certainty: u8,
}

/// An RDF-Star triple extracted from CML annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdfStarTriple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub provenance: String,
    pub confidence: f32,
}

/// Live SHACL & Epistemic Aura Validation Report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaclValidationReport {
    pub conforms: bool,
    pub warnings: usize,
    pub errors: usize,
    pub quin_count: usize,
    pub status_label: String,
}

/// Structured representation of a CML HyperDoc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmlDocument {
    pub id: String,
    pub title: String,
    pub author_did: String,
    pub sensitivity_class: u8,
    pub raw_text: String,
    pub spans: Vec<CmlSpan>,
    pub relations: Vec<CmlRelation>,
}

impl Default for CmlDocument {
    fn default() -> Self {
        let text = "Poet HyperDoc Authoring Subsystem provides native Context Markup Language (CML) \
                    integration with QualiaDB. The 42MB Prolog Sentinel verifies inalienable custody, \
                    while VibeScript executes reactive cells within metered gas bounds.";
        
        let spans = vec![
            CmlSpan {
                id: "ent_poet".into(),
                start_offset: 0,
                end_offset: 4,
                category: CmlCategory::NamedEntity,
                iri: "did:qualia:poet:shell".into(),
                label: "Poet".into(),
                certainty: 98,
                provenance: "Gazetteer:SystemCore".into(),
            },
            CmlSpan {
                id: "ent_cml".into(),
                start_offset: 40,
                end_offset: 71,
                category: CmlCategory::Term,
                iri: "qualia:ontology#ContextMarkupLanguage".into(),
                label: "Context Markup Language (CML)".into(),
                certainty: 95,
                provenance: "Lexicon:W3C_RDF".into(),
            },
            CmlSpan {
                id: "ent_sentinel".into(),
                start_offset: 93,
                end_offset: 114,
                category: CmlCategory::DeonticRule,
                iri: "qualia:sentinel:slg_arena".into(),
                label: "42MB Prolog Sentinel".into(),
                certainty: 100,
                provenance: "Sentinel:CoreProof".into(),
            },
            CmlSpan {
                id: "ent_vibe".into(),
                start_offset: 153,
                end_offset: 163,
                category: CmlCategory::CodeSymbol,
                iri: "qualia:vibe:runtime".into(),
                label: "VibeScript".into(),
                certainty: 92,
                provenance: "AST:Lexer".into(),
            },
        ];

        let relations = vec![
            CmlRelation {
                id: "rel_1".into(),
                subject_id: "ent_poet".into(),
                predicate_iri: "qualia:implements".into(),
                object_id: "ent_cml".into(),
                label: "implements".into(),
                certainty: 96,
            },
            CmlRelation {
                id: "rel_2".into(),
                subject_id: "ent_sentinel".into(),
                predicate_iri: "qualia:enforcesOn".into(),
                object_id: "ent_vibe".into(),
                label: "enforces bounds on".into(),
                certainty: 99,
            },
        ];

        Self {
            id: "doc_default_01".into(),
            title: "Poet Architecture Dossier".into(),
            author_did: "did:q42:author:timothy".into(),
            sensitivity_class: 0,
            raw_text: text.into(),
            spans,
            relations,
        }
    }
}

impl CmlDocument {
    /// Render structured text with inline interactive `<q-entity>` tags.
    pub fn to_cml_html(&self) -> String {
        let mut sorted_spans = self.spans.clone();
        sorted_spans.sort_by_key(|s| s.start_offset);

        let mut output = String::with_capacity(self.raw_text.len() * 2);
        let mut last_idx = 0;

        for span in &sorted_spans {
            let start = span.start_offset.min(self.raw_text.len());
            let end = span.end_offset.min(self.raw_text.len());

            if start > last_idx && last_idx < self.raw_text.len() {
                output.push_str(&html_escape(&self.raw_text[last_idx..start]));
            }

            if start < end && end <= self.raw_text.len() {
                let inner_text = &self.raw_text[start..end];
                output.push_str(&format!(
                    "<q-entity class=\"cml-entity-tag\" data-id=\"{}\" data-category=\"{}\" data-iri=\"{}\" data-certainty=\"{}\" data-provenance=\"{}\" style=\"border-color: {};\">{}<span class=\"cml-entity-badge\" style=\"background: {};\">{}</span></q-entity>",
                    span.id,
                    span.category.code(),
                    span.iri,
                    span.certainty,
                    span.provenance,
                    span.category.color_accent(),
                    html_escape(inner_text),
                    span.category.color_accent(),
                    span.category.glyph()
                ));
            }

            last_idx = end;
        }

        if last_idx < self.raw_text.len() {
            output.push_str(&html_escape(&self.raw_text[last_idx..]));
        }

        output
    }

    /// Render markdown representation with inline CML annotations.
    pub fn to_markdown(&self) -> String {
        let mut sorted_spans = self.spans.clone();
        sorted_spans.sort_by_key(|s| s.start_offset);

        let mut output = format!("# {}\n\n*Author: `{}` | Sensitivity: Level {}*\n\n", 
            self.title, self.author_did, self.sensitivity_class);
        
        let mut last_idx = 0;
        for span in &sorted_spans {
            let start = span.start_offset.min(self.raw_text.len());
            let end = span.end_offset.min(self.raw_text.len());

            if start > last_idx && last_idx < self.raw_text.len() {
                output.push_str(&self.raw_text[last_idx..start]);
            }

            if start < end && end <= self.raw_text.len() {
                let inner_text = &self.raw_text[start..end];
                output.push_str(&format!(
                    "[{}]({} \"type:{}, cert:{}%\")",
                    inner_text, span.iri, span.category.code(), span.certainty
                ));
            }

            last_idx = end;
        }

        if last_idx < self.raw_text.len() {
            output.push_str(&self.raw_text[last_idx..]);
        }

        if !self.relations.is_empty() {
            output.push_str("\n\n### Semantic Relations (`<q-relation>`)\n");
            for rel in &self.relations {
                output.push_str(&format!(
                    "- `{}` --[{}]--> `{}` (Certainty: {}%)\n",
                    rel.subject_id, rel.label, rel.object_id, rel.certainty
                ));
            }
        }

        output
    }

    /// Extract RDF-Star triples from entities and relations.
    pub fn to_rdf_star_triples(&self) -> Vec<RdfStarTriple> {
        let mut triples = Vec::new();

        // Entity occurrence triples
        for span in &self.spans {
            triples.push(RdfStarTriple {
                subject: format!("did:qualia:doc#{}", self.id),
                predicate: "qualia:hasAnnotation".into(),
                object: format!("<< {} rdf:type qualia:{} >>", span.iri, span.category.code()),
                provenance: span.provenance.clone(),
                confidence: span.certainty as f32 / 100.0,
            });
        }

        // Semantic relation triples
        for rel in &self.relations {
            let subj_iri = self.spans.iter()
                .find(|s| s.id == rel.subject_id)
                .map(|s| s.iri.as_str())
                .unwrap_or(&rel.subject_id);
            let obj_iri = self.spans.iter()
                .find(|s| s.id == rel.object_id)
                .map(|s| s.iri.as_str())
                .unwrap_or(&rel.object_id);

            triples.push(RdfStarTriple {
                subject: subj_iri.to_string(),
                predicate: rel.predicate_iri.clone(),
                object: obj_iri.to_string(),
                provenance: format!("DocRelation:{}", rel.id),
                confidence: rel.certainty as f32 / 100.0,
            });
        }

        triples
    }

    /// Live SHACL & Epistemic Aura Validation computation.
    pub fn validate_shacl(&self) -> ShaclValidationReport {
        let total_triples = self.to_rdf_star_triples().len();
        let quin_count = total_triples * 2; // Parity super-quin expansion
        let mut errors = 0;
        let mut warnings = 0;

        for span in &self.spans {
            if span.certainty < 60 {
                warnings += 1;
            }
            if span.iri.is_empty() {
                errors += 1;
            }
        }

        let conforms = errors == 0;
        let status_label = if conforms && warnings == 0 {
            "SHACL: Full Conformance (100%)".into()
        } else if conforms {
            format!("SHACL: Conforming ({} Warning{})", warnings, if warnings > 1 { "s" } else { "" })
        } else {
            format!("SHACL: Non-Conforming ({} Error{})", errors, if errors > 1 { "s" } else { "" })
        };

        ShaclValidationReport {
            conforms,
            warnings,
            errors,
            quin_count,
            status_label,
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cml_html_rendering() {
        let doc = CmlDocument::default();
        let html = doc.to_cml_html();
        assert!(html.contains("<q-entity class=\"cml-entity-tag\""));
        assert!(html.contains("data-iri=\"did:qualia:poet:shell\""));
        assert!(html.contains("data-category=\"entity\""));
        assert!(html.contains("data-certainty=\"98\""));
        assert!(html.contains("Poet"));
    }

    #[test]
    fn test_cml_markdown_export() {
        let doc = CmlDocument::default();
        let md = doc.to_markdown();
        assert!(md.contains("# Poet Architecture Dossier"));
        assert!(md.contains("[Poet](did:qualia:poet:shell \"type:entity, cert:98%\")"));
        assert!(md.contains("### Semantic Relations (`<q-relation>`)"));
        assert!(md.contains("- `ent_poet` --[implements]--> `ent_cml`"));
    }

    #[test]
    fn test_rdf_star_extraction() {
        let doc = CmlDocument::default();
        let triples = doc.to_rdf_star_triples();
        assert_eq!(triples.len(), 6); // 4 entity occurrences + 2 relations
        let poet_cml_rel = triples.iter().find(|t| t.predicate == "qualia:implements");
        assert!(poet_cml_rel.is_some());
        assert_eq!(poet_cml_rel.unwrap().subject, "did:qualia:poet:shell");
    }

    #[test]
    fn test_shacl_validation() {
        let doc = CmlDocument::default();
        let report = doc.validate_shacl();
        assert!(report.conforms);
        assert_eq!(report.errors, 0);
        assert_eq!(report.quin_count, 12);
        assert!(report.status_label.contains("SHACL: Full Conformance"));
    }
}
