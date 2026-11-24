//! 9-Tab WordNet 3.1 / Multilingual Lexicon & `.q42` Ontology Studio (`OntologyBody`),
//! Forensic Investigation & Asset Link-Analysis Bay (`ForensicInvestigationStudioBay`),
//! and Role-Tailored Verifiable PDF (`%PDF-1.7` + PAdES B-B / `QECP`) Report Printer Bay (`VerifiablePdfPrinterStudioBay`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

use crate::host;
use crate::investigation_link_analysis::{
    InstrumentAuthorRole, InvestigationCaseBoard, InvestigationNodeKind, InvestigationRelationKind,
};
use crate::kinds::ManifoldId;
use crate::store::Workbench;
use crate::verifiable_pdf_printer::{
    generate_verifiable_pdf_dossier, verify_pdf_dossier_signatures, RecipientReportProfile,
};
use crate::wordnet_q42_ontology::{
    compile_ontology_to_q42_pack, map_text_to_wordnet_and_ontology_triples, search_wordnet_synsets,
    OntologySourcePreset, OntologyWorkbenchTab, WordNetPos,
};
use aether::prelude::*;

const PANEL_BOX: &str = "background:#0f172a;border:1px solid #1e293b;border-radius:6px;padding:10px;display:grid;gap:8px;";
const INPUT_STYLE: &str = "background:#131c2e;border:1px solid #334155;color:#e2e8f0;border-radius:4px;padding:5px 8px;font-size:.74rem;";
const BTN_PRIMARY: &str = "background:#0284c7;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.73rem;font-weight:600;cursor:pointer;";
const BTN_ACCENT: &str = "background:#059669;color:#ffffff;border:none;border-radius:4px;padding:5px 10px;font-size:.73rem;font-weight:600;cursor:pointer;";
const BADGE_STYLE: &str = "background:#1e293b;color:#38bdf8;border:1px solid #334155;border-radius:4px;padding:2px 6px;font-size:.66rem;font-family:monospace;";

#[component]
/// 9-Tab WordNet & `.q42` Ontology Studio (`OntologyBody`).
pub fn OntologyBody() -> Element {
    let mut active_tab = use_signal(|| OntologyWorkbenchTab::WordNetVocabMapper);
    let mut wn_query = use_signal(|| String::new());
    let mut wn_pos_filter = use_signal(|| Option::<WordNetPos>::None);
    let mut text_sample = use_signal(|| {
        String::from(
            "Clinical psychologist ADHD and PTSD diagnosis requires safe housing sanctuary, reasonable accommodation, and administrative duty of inquiry to protect human dignity.",
        )
    });
    let mut selected_preset = use_signal(|| OntologySourcePreset::Bio2RdfClinicalNeuro);
    let mut custom_turtle = use_signal(|| {
        OntologySourcePreset::Bio2RdfClinicalNeuro
            .default_turtle_source()
            .to_string()
    });
    let mut share_commons = use_signal(|| true);
    let mut compiled_artifacts = use_signal(|| {
        let mut initial = Vec::new();
        if let Ok(a1) = compile_ontology_to_q42_pack(
            OntologySourcePreset::WordNet31CoreLexicon.slug(),
            OntologySourcePreset::WordNet31CoreLexicon.title(),
            OntologySourcePreset::WordNet31CoreLexicon.default_turtle_source(),
            true,
        ) {
            initial.push(a1);
        }
        if let Ok(a2) = compile_ontology_to_q42_pack(
            OntologySourcePreset::Bio2RdfClinicalNeuro.slug(),
            OntologySourcePreset::Bio2RdfClinicalNeuro.title(),
            OntologySourcePreset::Bio2RdfClinicalNeuro.default_turtle_source(),
            true,
        ) {
            initial.push(a2);
        }
        initial
    });
    let mut status_msg = use_signal(|| {
        String::from("WordNet 3.1 + OMW Multilingual Lexicon & Q42\\0 v4 Ontology Compiler Ready")
    });

    let synsets = search_wordnet_synsets(&wn_query(), wn_pos_filter());
    let mapping_report = map_text_to_wordnet_and_ontology_triples(&text_sample());

    rsx! {
        div { style: "display:grid;gap:10px;padding:6px;color:#e2e8f0;font-size:.75rem;",
            // 9-Tab Header Bar
            div { style: "display:flex;gap:4px;flex-wrap:wrap;background:#090d16;padding:4px;border-radius:6px;border:1px solid #1e293b;",
                for tab in OntologyWorkbenchTab::ALL {
                    button {
                        r#type: "button",
                        style: if active_tab() == tab {
                            "background:#38bdf8;color:#0f172a;border:none;border-radius:4px;padding:4px 8px;font-size:.68rem;font-weight:700;cursor:pointer;"
                        } else {
                            "background:transparent;color:#94a3b8;border:none;border-radius:4px;padding:4px 8px;font-size:.68rem;cursor:pointer;"
                        },
                        onclick: move |_| active_tab.set(tab),
                        "{tab.title()}"
                    }
                }
            }

            div { style: "display:flex;justify-content:space-between;align-items:center;background:#131c2e;padding:6px 10px;border-radius:4px;border:1px solid #1e293b;",
                span { style: "color:#38bdf8;font-weight:600;font-size:.72rem;", "⚡ {status_msg}" }
                span { style: BADGE_STYLE, "Compiled .q42 Vaults: {compiled_artifacts().len()} · 42 MiB Sentinel OK" }
            }

            match active_tab() {
                OntologyWorkbenchTab::WordNetVocabMapper => rsx! {
                    div { style: PANEL_BOX,
                        div { style: "display:flex;gap:6px;flex-wrap:wrap;align-items:center;",
                            input {
                                r#type: "text",
                                style: "flex:1;min-width:180px;background:#131c2e;border:1px solid #334155;color:#e2e8f0;border-radius:4px;padding:5px 8px;font-size:.74rem;",
                                placeholder: "Search WordNet 3.1 synsets, multilingual lemmas (FR/ES/DE), ILI IDs, DOID, Bio2RDF, CRPD...",
                                value: "{wn_query}",
                                oninput: move |e| wn_query.set(e.value()),
                            }
                            button {
                                r#type: "button",
                                style: if wn_pos_filter().is_none() { BTN_PRIMARY } else { BADGE_STYLE },
                                onclick: move |_| wn_pos_filter.set(None),
                                "All POS"
                            }
                            for pos in WordNetPos::ALL {
                                button {
                                    r#type: "button",
                                    style: if wn_pos_filter() == Some(pos) { BTN_PRIMARY } else { BADGE_STYLE },
                                    onclick: move |_| wn_pos_filter.set(Some(pos)),
                                    "{pos.label()}"
                                }
                            }
                        }

                        div { style: "display:grid;gap:6px;max-height:220px;overflow:auto;",
                            for syn in synsets.iter() {
                                div {
                                    key: "{syn.synset_id}",
                                    style: "background:#131c2e;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:4px;",
                                    div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:6px;",
                                        strong { style: "color:#38bdf8;", "{syn.lemmas.join(\", \")}" }
                                        div { style: "display:flex;gap:4px;",
                                            span { style: BADGE_STYLE, "{syn.synset_id}" }
                                            span { style: BADGE_STYLE, "ILI: {syn.ili_id}" }
                                            span { style: BADGE_STYLE, "{syn.domain_category}" }
                                        }
                                    }
                                    p { style: "margin:0;color:#cbd5e1;font-size:.72rem;", "{syn.gloss}" }
                                    div { style: "color:#94a3b8;font-size:.68rem;",
                                        "Multilingual (OMW): "
                                        for m in syn.multilingual.iter() {
                                            span { style: "margin-right:8px;color:#a78bfa;", "[{m.lang_bcp47}] {m.lemmas.join(\"/\")}" }
                                        }
                                    }
                                    div { style: "color:#34d399;font-size:.68rem;font-family:monospace;",
                                        "Ontology IRIs: {syn.mapped_ontology_iris.join(\" · \")}"
                                    }
                                }
                            }
                        }

                        div { style: "border-top:1px solid #1e293b;padding-top:8px;display:grid;gap:6px;",
                            strong { style: "color:#a78bfa;", "Natural-Language Document → WordNet Synset & N3 / SHACL Grounding" }
                            textarea {
                                rows: "2",
                                style: INPUT_STYLE,
                                value: "{text_sample}",
                                oninput: move |e| text_sample.set(e.value()),
                            }
                            div { style: "display:flex;gap:8px;align-items:center;flex-wrap:wrap;",
                                span { style: BADGE_STYLE, "Tokens: {mapping_report.input_token_count}" }
                                span { style: BADGE_STYLE, "Matched WordNet Synsets: {mapping_report.matched_synsets.len()}" }
                                span { style: BADGE_STYLE, "Normative IRIs: {mapping_report.detected_normative_iris.len()}" }
                                button {
                                    r#type: "button",
                                    style: BTN_ACCENT,
                                    onclick: move |_| {
                                        let report = map_text_to_wordnet_and_ontology_triples(&text_sample());
                                        if let Ok(artifact) = compile_ontology_to_q42_pack(
                                            "wordnet-extracted-lexicon",
                                            "WordNet Extracted Natural-Language Grounding",
                                            &report.generated_turtle_n3,
                                            true,
                                        ) {
                                            status_msg.set(format!(
                                                "Compiled {} WordNet-grounded quads into .q42 ({})",
                                                artifact.nquin48_triple_count,
                                                &artifact.content_digest_hex[..16]
                                            ));
                                            let mut list = compiled_artifacts();
                                            list.push(artifact);
                                            compiled_artifacts.set(list);
                                        }
                                    },
                                    "Compile Extracted WordNet Triples to .q42 & Share Commons"
                                }
                            }
                        }
                    }
                },
                OntologyWorkbenchTab::LibraryAndQ42Compiler
                | OntologyWorkbenchTab::N3AndTurtleEditor
                | OntologyWorkbenchTab::PermissiveCommonsDistribution => rsx! {
                    div { style: PANEL_BOX,
                        div { style: "display:flex;gap:6px;flex-wrap:wrap;align-items:center;",
                            for preset in OntologySourcePreset::ALL {
                                button {
                                    r#type: "button",
                                    style: if selected_preset() == preset { BTN_PRIMARY } else { BADGE_STYLE },
                                    onclick: move |_| {
                                        selected_preset.set(preset);
                                        custom_turtle.set(preset.default_turtle_source().to_string());
                                    },
                                    "{preset.slug()}"
                                }
                            }
                        }
                        textarea {
                            rows: "7",
                            style: "background:#090d16;border:1px solid #334155;color:#34d399;border-radius:4px;padding:8px;font-family:monospace;font-size:.71rem;",
                            value: "{custom_turtle}",
                            oninput: move |e| custom_turtle.set(e.value()),
                        }
                        div { style: "display:flex;gap:10px;align-items:center;flex-wrap:wrap;",
                            button {
                                r#type: "button",
                                style: BTN_PRIMARY,
                                onclick: move |_| {
                                    let p = selected_preset();
                                    match compile_ontology_to_q42_pack(
                                        p.slug(),
                                        p.title(),
                                        &custom_turtle(),
                                        share_commons(),
                                    ) {
                                        Ok(artifact) => {
                                            status_msg.set(format!(
                                                "Sealed {} ({} NQuin48 records, {} bytes) → {}",
                                                artifact.ontology_id,
                                                artifact.nquin48_triple_count,
                                                artifact.prolog_arena_bytes_used,
                                                artifact.permissive_commons_qcdn_uri.as_deref().unwrap_or("Local Sanctuary Vault")
                                            ));
                                            let mut list = compiled_artifacts();
                                            list.retain(|x| x.ontology_id != artifact.ontology_id);
                                            list.push(artifact);
                                            compiled_artifacts.set(list);
                                        }
                                        Err(e) => status_msg.set(format!("Compile error: {e}")),
                                    }
                                },
                                "⚡ Compile Turtle/N3 to .q42 (Q42\\0 v4 NQuin48)"
                            }
                            button {
                                r#type: "button",
                                style: BADGE_STYLE,
                                onclick: move |_| share_commons.set(!share_commons()),
                                if share_commons() {
                                    "🌐 Permissive Commons Q-CDN Sharing: ENABLED"
                                } else {
                                    "🔒 Local Sanctuary Vault Only"
                                }
                            }
                        }
                        div { style: "display:grid;gap:6px;max-height:170px;overflow:auto;",
                            for art in compiled_artifacts().iter() {
                                div {
                                    key: "{art.ontology_id}-{art.content_digest_hex}",
                                    style: "background:#131c2e;border:1px solid #1e293b;border-radius:4px;padding:6px 10px;display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:6px;",
                                    div {
                                        strong { style: "color:#38bdf8;", "{art.title}" }
                                        div { style: "color:#94a3b8;font-size:.67rem;font-family:monospace;",
                                            "Magic: {art.magic_header} v{art.format_version} · {art.nquin48_triple_count} NQuin48 quads · Digest: {&art.content_digest_hex[..20]}…"
                                        }
                                    }
                                    if let Some(ref uri) = art.permissive_commons_qcdn_uri {
                                        span { style: "color:#34d399;font-size:.67rem;font-family:monospace;", "{uri}" }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { style: PANEL_BOX,
                        strong { style: "color:#38bdf8;", "{active_tab().title()} — Active WordNet & SHACL/ShEx Graph Projection" }
                        pre {
                            style: "background:#090d16;border:1px solid #1e293b;border-radius:4px;padding:8px;color:#a78bfa;font-size:.70rem;max-height:220px;overflow:auto;margin:0;",
                            "{mapping_report.generated_shacl_shapes}\n{mapping_report.generated_turtle_n3}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Forensic Investigation & Asset Link-Analysis Workbench Bay (`WP-POET-INV`).
pub fn ForensicInvestigationStudioBay(wb: Signal<Workbench>) -> Element {
    let mut board = use_signal(InvestigationCaseBoard::default);
    let mut new_id = use_signal(|| String::from("EXH-NEW-06"));
    let mut new_title = use_signal(|| String::from("Supplemental Specialist Housing & Sensory Accommodation Note"));
    let new_kind = use_signal(|| InvestigationNodeKind::SemanticInstrument);
    let mut new_role = use_signal(|| InstrumentAuthorRole::QualifiedSpecialistAttested);
    let new_author = use_signal(|| String::from("did:web:clinic.example.au:gp-dr-a-chen"));
    let new_summary = use_signal(|| {
        String::from(
            "General Practitioner clinical note corroborating ADHD and PTSD sensory accommodation requirements and housing stability necessity under ICESCR Art. 11 & UN CRPD Art. 19.",
        )
    });

    let current_board = board();
    let audit = current_board.run_investigation_audit();

    rsx! {
        div { style: "display:grid;gap:10px;padding:10px;background:#0b1120;color:#e2e8f0;font-size:.75rem;",
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;",
                div {
                    strong { style: "color:#38bdf8;font-size:.84rem;", "🔎 Forensic Investigation & Asset Link-Analysis Workbench ({current_board.case_id})" }
                    div { style: "color:#94a3b8;font-size:.70rem;", "{current_board.case_title} · Principal: {current_board.principal_did}" }
                }
                div { style: "display:flex;gap:6px;flex-wrap:wrap;",
                    span { style: BADGE_STYLE, "Exhibits: {audit.total_nodes}" }
                    span { style: BADGE_STYLE, "Semantic Wires: {audit.total_edges}" }
                    span { style: "background:#064e3b;color:#34d399;border:1px solid #059669;border-radius:4px;padding:2px 6px;font-size:.66rem;",
                        "Admitted Instruments: {audit.admitted_instrument_count}"
                    }
                    span { style: "background:#450a0a;color:#f87171;border:1px solid #dc2626;border-radius:4px;padding:2px 6px;font-size:.66rem;",
                        "Quarantined Contested: {audit.quarantined_contested_count} · Contradictions: {audit.contradiction_count}"
                    }
                    button {
                        r#type: "button",
                        style: BTN_PRIMARY,
                        onclick: move |_| {
                            let mut w = wb();
                            board().project_onto_workbench_canvas(&mut w, ManifoldId::Rights);
                            wb.set(w);
                        },
                        "↗ Project Investigation Graph onto Spatial Canvas (Rights Manifold)"
                    }
                }
            }

            // Exhibit & Instrument Grid
            div { style: "display:grid;grid-template-columns:repeat(auto-fit,minmax(270px,1fr));gap:8px;max-height:210px;overflow:auto;",
                for node in current_board.nodes.iter() {
                    div {
                        key: "{node.node_id}",
                        style: "background:#131c2e;border:1px solid #1e293b;border-radius:6px;padding:8px;display:grid;gap:4px;",
                        div { style: "display:flex;justify-content:space-between;align-items:center;gap:4px;",
                            strong { style: "color:#f8fafc;font-size:.74rem;", "{node.kind.icon()} [{node.node_id}] {node.title}" }
                        }
                        div {
                            style: format!("color:{};font-size:.67rem;font-weight:700;", node.author_role.badge_color()),
                            "Role: {node.author_role.label()}"
                        }
                        p { style: "margin:0;color:#cbd5e1;font-size:.70rem;", "{node.summary_or_quote}" }
                        div { style: "color:#64748b;font-size:.64rem;font-family:monospace;",
                            "SHA-256: {&node.sha256_digest_hex[..24]}… · {node.timestamp_iso}"
                        }
                    }
                }
            }

            // Add New Evidence Exhibit / Role-Qualified Semantic Instrument
            div { style: "background:#131c2e;border:1px solid #1e293b;border-radius:6px;padding:8px;display:flex;gap:6px;flex-wrap:wrap;align-items:center;",
                input {
                    r#type: "text",
                    style: "width:105px;background:#090d16;border:1px solid #334155;color:#e2e8f0;border-radius:4px;padding:4px 6px;font-size:.72rem;",
                    value: "{new_id}",
                    oninput: move |e| new_id.set(e.value()),
                }
                input {
                    r#type: "text",
                    style: "flex:1;min-width:180px;background:#090d16;border:1px solid #334155;color:#e2e8f0;border-radius:4px;padding:4px 6px;font-size:.72rem;",
                    value: "{new_title}",
                    oninput: move |e| new_title.set(e.value()),
                }
                button {
                    r#type: "button",
                    style: BADGE_STYLE,
                    onclick: move |_| {
                        let next = match new_role() {
                            InstrumentAuthorRole::QualifiedSpecialistAttested => InstrumentAuthorRole::PrincipalSelfAuthored,
                            InstrumentAuthorRole::PrincipalSelfAuthored => InstrumentAuthorRole::CollaborativeCoAttested,
                            InstrumentAuthorRole::CollaborativeCoAttested => InstrumentAuthorRole::ContestedOrDisputedRecord,
                            InstrumentAuthorRole::ContestedOrDisputedRecord => InstrumentAuthorRole::QualifiedSpecialistAttested,
                        };
                        new_role.set(next);
                    },
                    "Author Role: {new_role().label()}"
                }
                button {
                    r#type: "button",
                    style: BTN_ACCENT,
                    onclick: move |_| {
                        let mut b = board();
                        let id_str = new_id();
                        b.add_node(
                            &id_str,
                            new_kind(),
                            &new_title(),
                            new_role(),
                            &new_author(),
                            "2026-10-25T11:00:00Z",
                            "qecp://sanctuary/evidence/supplemental-exhibit.q42",
                            &new_summary(),
                            Vec::new(),
                            460,
                            460,
                        );
                        b.add_edge(
                            &format!("EDGE-{}", b.edges.len() + 1),
                            &id_str,
                            "EXH-CLIN-01",
                            InvestigationRelationKind::Corroborates,
                            "Supplemental exhibit corroborates primary clinical attestation.",
                            "2026-10-25T11:00:00Z",
                        );
                        board.set(b);
                    },
                    "+ Add Exhibit & Corroboration Wire"
                }
            }

            // Automated Forensic Audit Findings
            div { style: "background:#090d16;border:1px solid #1e293b;border-radius:6px;padding:8px;display:grid;gap:4px;",
                strong { style: "color:#f87171;font-size:.73rem;", "⚖️ Automated Paraconsistent Contradiction & Duty-of-Inquiry Audit Findings:" }
                for finding in audit.findings.iter() {
                    div {
                        key: "{finding.finding_id}",
                        style: "font-size:.70rem;color:#fca5a5;",
                        "• [{finding.severity}] {finding.finding_id}: {finding.summary}"
                    }
                }
            }
        }
    }
}

#[component]
/// Role-Tailored Verifiable PDF (`%PDF-1.7` + PAdES B-B & `QECP`) Report Printer Bay (`WP-PDF-QECP`).
pub fn VerifiablePdfPrinterStudioBay() -> Element {
    let mut selected_profile = use_signal(|| RecipientReportProfile::ClinicalDoctorAndPsychologist);
    let signer_did = use_signal(|| String::from("did:q42:person:timothy-holborn"));
    let signer_label = use_signal(|| String::from("Timothy Charles Holborn (Natural Person Principal & Sanctuary Custodian)"));
    let mut generated_pdf = use_signal(|| {
        let board = InvestigationCaseBoard::default();
        let ontologies = vec![
            compile_ontology_to_q42_pack(
                OntologySourcePreset::Bio2RdfClinicalNeuro.slug(),
                OntologySourcePreset::Bio2RdfClinicalNeuro.title(),
                OntologySourcePreset::Bio2RdfClinicalNeuro.default_turtle_source(),
                true,
            )
            .expect("valid bio2rdf q42"),
            compile_ontology_to_q42_pack(
                OntologySourcePreset::UnCrpdHumanRights.slug(),
                OntologySourcePreset::UnCrpdHumanRights.title(),
                OntologySourcePreset::UnCrpdHumanRights.default_turtle_source(),
                true,
            )
            .expect("valid crpd q42"),
        ];
        generate_verifiable_pdf_dossier(
            &board,
            &ontologies,
            RecipientReportProfile::ClinicalDoctorAndPsychologist,
            "did:q42:person:timothy-holborn",
            "Timothy Charles Holborn (Natural Person Principal & Sanctuary Custodian)",
        )
    });

    let artifact = generated_pdf();
    let verification = verify_pdf_dossier_signatures(&artifact.pdf_bytes);
    let pdf_preview_text = String::from_utf8_lossy(&artifact.pdf_bytes).to_string();

    rsx! {
        div { style: "display:grid;gap:10px;padding:10px;background:#0b1120;color:#e2e8f0;font-size:.75rem;",
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;",
                div {
                    strong { style: "color:#38bdf8;font-size:.84rem;", "🖨️ Verifiable PDF (%PDF-1.7 + PAdES B-B / QECP Signature Schedule) Dossier Printer" }
                    div { style: "color:#94a3b8;font-size:.70rem;",
                        "Tailors printable evidentiary reports for Doctors/Psychologists, Human-Rights Lawyers, Social Workers, and Protective Oversight."
                    }
                }
                span {
                    style: if verification.master_signature_valid {
                        "background:#064e3b;color:#34d399;border:1px solid #059669;border-radius:4px;padding:3px 8px;font-size:.70rem;font-weight:700;"
                    } else {
                        "background:#450a0a;color:#f87171;border:1px solid #dc2626;border-radius:4px;padding:3px 8px;font-size:.70rem;font-weight:700;"
                    },
                    "✓ {verification.diagnostic} ({artifact.pdf_bytes.len()} bytes · {artifact.page_count} pages)"
                }
            }

            // Recipient Role Profile Selector
            div { style: "display:flex;gap:6px;flex-wrap:wrap;",
                for profile in RecipientReportProfile::ALL {
                    button {
                        r#type: "button",
                        style: if selected_profile() == profile { BTN_PRIMARY } else { BADGE_STYLE },
                        onclick: move |_| {
                            selected_profile.set(profile);
                            let board = InvestigationCaseBoard::default();
                            let ontologies = vec![
                                compile_ontology_to_q42_pack(
                                    OntologySourcePreset::Bio2RdfClinicalNeuro.slug(),
                                    OntologySourcePreset::Bio2RdfClinicalNeuro.title(),
                                    OntologySourcePreset::Bio2RdfClinicalNeuro.default_turtle_source(),
                                    true,
                                )
                                .expect("valid bio2rdf q42"),
                                compile_ontology_to_q42_pack(
                                    OntologySourcePreset::UnCrpdHumanRights.slug(),
                                    OntologySourcePreset::UnCrpdHumanRights.title(),
                                    OntologySourcePreset::UnCrpdHumanRights.default_turtle_source(),
                                    true,
                                )
                                .expect("valid crpd q42"),
                            ];
                            generated_pdf.set(generate_verifiable_pdf_dossier(
                                &board,
                                &ontologies,
                                profile,
                                &signer_did(),
                                &signer_label(),
                            ));
                        },
                        "{profile.title()}"
                    }
                }
            }

            div { style: "display:flex;gap:8px;align-items:center;flex-wrap:wrap;background:#131c2e;padding:8px;border-radius:6px;border:1px solid #1e293b;",
                span { style: BADGE_STYLE, "File: {artifact.suggested_filename}" }
                span { style: BADGE_STYLE, "Body SHA-256: {&artifact.body_sha256_digest_hex[..20]}…" }
                span { style: BADGE_STYLE, "Master PAdES/QECP Sig: {&artifact.pades_qecp_master_signature_hex[..20]}…" }
                span { style: BADGE_STYLE, "Signed Exhibits: {artifact.exhibit_signatures.len()}" }
                button {
                    r#type: "button",
                    style: BTN_ACCENT,
                    onclick: move |_| {
                        let txt = String::from_utf8_lossy(&generated_pdf().pdf_bytes).to_string();
                        host::copy_to_clipboard(txt);
                    },
                    "📋 Copy Verifiable %PDF-1.7 + QECP Stream to Clipboard"
                }
            }

            pre {
                style: "background:#090d16;border:1px solid #1e293b;border-radius:6px;padding:10px;color:#34d399;font-family:monospace;font-size:.68rem;max-height:210px;overflow:auto;margin:0;",
                "{pdf_preview_text}"
            }
        }
    }
}
