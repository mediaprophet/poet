//! Forensic Investigation, Evidence Link-Analysis, Role-Qualified Semantic Instruments,
//! CML Legal/Human-Rights Correlation, and Contradiction Audit Engine (`WP-POET-INV`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

use crate::kinds::{CanvasNode, ContainerKind, Epistemic, ManifoldId, Strata, Wire};
use crate::store::Workbench;
use crate::wordnet_q42_ontology::{deterministic_digest_hex, map_text_to_wordnet_and_ontology_triples};
use serde::{Deserialize, Serialize};

/// Role and epistemic authority of the creator of a Semantic Instrument or Evidence Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentAuthorRole {
    /// Attested by a qualified specialist (psychologist, physician, lawyer, social worker).
    QualifiedSpecialistAttested,
    /// Self-authored by the natural-person principal (primary lived-experience & selfhood testimony).
    PrincipalSelfAuthored,
    /// Co-attested with dual signatures by both the natural-person principal and a specialist.
    CollaborativeCoAttested,
    /// Disputed or contested external assertion — quarantined from positive symbolic inference.
    ContestedOrDisputedRecord,
}

impl InstrumentAuthorRole {
    pub const ALL: [Self; 4] = [
        Self::QualifiedSpecialistAttested,
        Self::PrincipalSelfAuthored,
        Self::CollaborativeCoAttested,
        Self::ContestedOrDisputedRecord,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::QualifiedSpecialistAttested => "Qualified Specialist Attested (VC + Clinical/Legal Authority)",
            Self::PrincipalSelfAuthored => "Principal Self-Authored (Natural Person Lived-Experience Testimony)",
            Self::CollaborativeCoAttested => "Collaborative Co-Attested (Dual Principal + Specialist Signatures)",
            Self::ContestedOrDisputedRecord => "Contested / Disputed Record (Quarantined from Positive Inference)",
        }
    }

    pub fn badge_color(self) -> &'static str {
        match self {
            Self::QualifiedSpecialistAttested => "#38bdf8",
            Self::PrincipalSelfAuthored => "#a78bfa",
            Self::CollaborativeCoAttested => "#34d399",
            Self::ContestedOrDisputedRecord => "#f87171",
        }
    }

    pub fn admitted_for_positive_symbolic_inference(self) -> bool {
        !matches!(self, Self::ContestedOrDisputedRecord)
    }
}

/// Category of node on the Poet Forensic Investigation & Link-Analysis Canvas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestigationNodeKind {
    /// Documentary or media evidence asset (Email, PDF, Clinical Letter, Audio/Video, Photo, Web Capture).
    EvidenceAsset,
    /// Role-qualified Semantic Instrument (ADHD/PTSD clinical determination, functional accommodation profile, self-report).
    SemanticInstrument,
    /// CML Legal or Human-Rights Norm (UDHR, ICESCR, UN CRPD, Administrative Duty of Inquiry).
    CmlLegalNorm,
    /// Natural person, specialist, agency, housing provider, or institution.
    ActorOrInstitution,
    /// Timestamped chronological event or administrative decision.
    TimelineEvent,
}

impl InvestigationNodeKind {
    pub const ALL: [Self; 5] = [
        Self::EvidenceAsset,
        Self::SemanticInstrument,
        Self::CmlLegalNorm,
        Self::ActorOrInstitution,
        Self::TimelineEvent,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::EvidenceAsset => "Evidence Asset (Email / PDF / Media)",
            Self::SemanticInstrument => "Semantic Instrument (Clinical / Selfhood VC)",
            Self::CmlLegalNorm => "CML Legal & Human-Rights Norm",
            Self::ActorOrInstitution => "Actor / Specialist / Institution",
            Self::TimelineEvent => "Chronological Timeline Event",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::EvidenceAsset => "📎",
            Self::SemanticInstrument => "🩺",
            Self::CmlLegalNorm => "⚖️",
            Self::ActorOrInstitution => "🏛️",
            Self::TimelineEvent => "⏱️",
        }
    }
}

/// Typed semantic/forensic link relation between two nodes in an investigation graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestigationRelationKind {
    /// `qualia:groundsObservation` — empirical corroboration.
    Corroborates,
    /// `qualia:derivesFrom` — formal derivation / clinical or legal proof.
    DerivesFrom,
    /// `qualia:obligates` (`0x10`) — deontic obligation / duty of inquiry / accommodation duty.
    Obligates,
    /// `qualia:permits` (`0x11`) — deontic permission / authorized disclosure.
    Permits,
    /// `qualia:forbids` (`0x12`) — deontic prohibition / breach of right or duty.
    ViolatesOrForbids,
    /// `qualia:contradicts` (`0x30`) — paraconsistent contradiction / conflicting claim.
    Contradicts,
    /// `qualia:tracksContribution` — authored, signed, or attested by actor.
    AttestedBy,
    /// `qualia:until` (`0x43`) — temporal precedence on the chronological timeline.
    PrecedesInTime,
}

impl InvestigationRelationKind {
    pub const ALL: [Self; 8] = [
        Self::Corroborates,
        Self::DerivesFrom,
        Self::Obligates,
        Self::Permits,
        Self::ViolatesOrForbids,
        Self::Contradicts,
        Self::AttestedBy,
        Self::PrecedesInTime,
    ];

    pub fn predicate_iri(self) -> &'static str {
        match self {
            Self::Corroborates => "qualia:groundsObservation",
            Self::DerivesFrom => "qualia:derivesFrom",
            Self::Obligates => "qualia:obligates",
            Self::Permits => "qualia:permits",
            Self::ViolatesOrForbids => "qualia:forbids",
            Self::Contradicts => "qualia:contradicts",
            Self::AttestedBy => "qualia:tracksContribution",
            Self::PrecedesInTime => "qualia:until",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Corroborates => "Corroborates (qualia:groundsObservation)",
            Self::DerivesFrom => "Derives From (qualia:derivesFrom)",
            Self::Obligates => "Obligates / Duty of Care (qualia:obligates 0x10)",
            Self::Permits => "Permits / Discloses (qualia:permits 0x11)",
            Self::ViolatesOrForbids => "Violates / Breaches Norm (qualia:forbids 0x12)",
            Self::Contradicts => "Contradicts (qualia:contradicts 0x30)",
            Self::AttestedBy => "Attested / Authored By (qualia:tracksContribution)",
            Self::PrecedesInTime => "Precedes In Time (qualia:until 0x43)",
        }
    }

    pub fn wire_kind_css(self) -> &'static str {
        match self {
            Self::Corroborates | Self::DerivesFrom | Self::AttestedBy => "semantic-rel",
            Self::Obligates | Self::Permits | Self::ViolatesOrForbids | Self::Contradicts => {
                "deontic-guard"
            }
            Self::PrecedesInTime => "causal-flow",
        }
    }
}

/// A node in the Poet Forensic Investigation Case Board.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationNode {
    pub node_id: String,
    pub kind: InvestigationNodeKind,
    pub title: String,
    pub author_role: InstrumentAuthorRole,
    pub author_did_or_name: String,
    pub timestamp_iso: String,
    pub source_uri_or_citation: String,
    pub summary_or_quote: String,
    pub ontology_iris: Vec<String>,
    pub sha256_digest_hex: String,
    pub qecp_signature_hex: String,
    pub canvas_x: i32,
    pub canvas_y: i32,
}

/// A directed semantic/forensic link between two `InvestigationNode` records.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationEdge {
    pub edge_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub relation: InvestigationRelationKind,
    pub evidentiary_note: String,
    pub timestamp_iso: String,
}

/// Automated finding produced by `run_investigation_audit`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForensicAuditFinding {
    pub finding_id: String,
    pub severity: String,
    pub category: String,
    pub summary: String,
    pub involved_node_ids: Vec<String>,
    pub applicable_norm_iris: Vec<String>,
}

/// Complete audit report over an `InvestigationCaseBoard`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationAuditReport {
    pub case_id: String,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub admitted_instrument_count: usize,
    pub quarantined_contested_count: usize,
    pub contradiction_count: usize,
    pub unfulfilled_duty_count: usize,
    pub findings: Vec<ForensicAuditFinding>,
}

/// Forensic Investigation & Link-Analysis Case Board.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationCaseBoard {
    pub case_id: String,
    pub case_title: String,
    pub principal_did: String,
    pub principal_display_name: String,
    pub nodes: Vec<InvestigationNode>,
    pub edges: Vec<InvestigationEdge>,
}

impl Default for InvestigationCaseBoard {
    fn default() -> Self {
        Self::seeded_human_rights_and_wellfair_case()
    }
}

impl InvestigationCaseBoard {
    /// Creates a rich, realistic default WellFair & Human-Rights Protection case board demonstrating:
    /// - Specialist-attested ADHD + PTSD Semantic Instruments,
    /// - Principal self-authored lived-experience functional impact instrument,
    /// - Contemporaneous email & PDF evidence exhibits,
    /// - CML Legal & Human-Rights norms (UN CRPD Art. 13/19/28, ICESCR Art. 11/12, Administrative Duty of Inquiry),
    /// - A contested institutional claim quarantined via `ContestedOrDisputedRecord` and flagged by `Contradicts`.
    pub fn seeded_human_rights_and_wellfair_case() -> Self {
        let mut board = Self {
            case_id: "CASE-WF-2026-001".into(),
            case_title: "Human-Rights Protection, Clinical Continuity & Housing Sanctuary Dossier".into(),
            principal_did: "did:q42:person:timothy-holborn".into(),
            principal_display_name: "Natural Person Principal (Sanctuary Custodian)".into(),
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        board.add_node(
            "EXH-CLIN-01",
            InvestigationNodeKind::SemanticInstrument,
            "Clinical Psychologist Diagnostic Attestation (ADHD & PTSD)",
            InstrumentAuthorRole::QualifiedSpecialistAttested,
            "did:web:clinic.example.au:psychologist-dr-m-vance",
            "2026-10-08T10:30:00Z",
            "qecp://sanctuary/instruments/psych-adhd-ptsd-2026.q42",
            "Formal psychological diagnosis confirming ADHD (ICD-11 6A05 / DOID:1094) and PTSD (ICD-11 6B40 / DOID:2055). Recommends quiet housing sanctuary, trauma-informed advocacy, and structured written communications.",
            vec![
                "http://purl.obolibrary.org/obo/DOID_1094".into(),
                "http://purl.obolibrary.org/obo/DOID_2055".into(),
                "https://bio2rdf.org/mesh:D001289".into(),
                "https://bio2rdf.org/mesh:D013313".into(),
            ],
            80,
            60,
        );

        board.add_node(
            "EXH-SELF-02",
            InvestigationNodeKind::SemanticInstrument,
            "Principal Self-Authored Functional Impact & Sanctuary Statement",
            InstrumentAuthorRole::PrincipalSelfAuthored,
            "did:q42:person:timothy-holborn",
            "2026-10-10T14:15:00Z",
            "qecp://sanctuary/instruments/self-functional-impact-2026.q42",
            "First-person lived-experience declaration detailing sensory overload triggers, executive-function scheduling requirements, and the necessity of stable housing tenure for health and work continuity.",
            vec![
                "http://purl.obolibrary.org/obo/HP_0007018".into(),
                "http://purl.obolibrary.org/obo/HP_0033676".into(),
                "https://ns.webcivics.net/rights/udhr#Article1_HumanDignity".into(),
            ],
            80,
            280,
        );

        board.add_node(
            "EXH-DOC-03",
            InvestigationNodeKind::EvidenceAsset,
            "Email & Attached GP Clinical Summary Sent to Housing/Welfare Agency",
            InstrumentAuthorRole::CollaborativeCoAttested,
            "did:q42:person:timothy-holborn + General Practice Clinic",
            "2026-10-14T09:05:00Z",
            "qecp://sanctuary/evidence/email-agency-notice-2026-10-14.pdf",
            "Signed email delivery receipt and PDF medical summary notifying the agency of diagnosed ADHD and PTSD and requesting reasonable procedural and housing accommodations.",
            vec![
                "https://ns.webcivics.net/cml#EvidentiaryChainOfCustody".into(),
                "https://ns.webcivics.net/rights/icescr#Article12_HighestAttainableHealth".into(),
            ],
            460,
            60,
        );

        board.add_node(
            "NORM-CML-04",
            InvestigationNodeKind::CmlLegalNorm,
            "UN CRPD Art. 19 & 28 + Administrative Duty of Inquiry (CML Norm)",
            InstrumentAuthorRole::QualifiedSpecialistAttested,
            "https://ns.webcivics.net/cml#DutyOfInquiry",
            "2026-01-01T00:00:00Z",
            "https://ns.webcivics.net/rights/crpd#Article19_IndependentLivingAndSanctuary",
            "Mandatory legal obligation requiring agencies and providers to actively inquire into disclosed disability/clinical evidence and provide reasonable housing and procedural accommodations prior to any adverse step.",
            vec![
                "https://ns.webcivics.net/cml#DutyOfInquiry".into(),
                "https://ns.webcivics.net/rights/crpd#Article19_IndependentLivingAndSanctuary".into(),
                "https://ns.webcivics.net/rights/crpd#Article28_AdequateStandardOfLiving".into(),
                "https://ns.webcivics.net/rights/icescr#Article11_AdequateHousing".into(),
            ],
            460,
            280,
        );

        board.add_node(
            "EXH-DISP-05",
            InvestigationNodeKind::EvidenceAsset,
            "Contested Agency Notice Claiming 'No Clinical Documentation on File'",
            InstrumentAuthorRole::ContestedOrDisputedRecord,
            "External Administrative Officer (Contested Record)",
            "2026-10-22T16:40:00Z",
            "qecp://sanctuary/contested/agency-form-letter-2026-10-22.pdf",
            "Form letter asserting that no clinical or accommodation evidence was received prior to 22 October 2026 — directly contradicted by the 14 October 2026 delivery receipt and GP/Psychologist attestations.",
            vec!["https://ns.webcivics.net/qualia#contradicts".into()],
            840,
            170,
        );

        board.add_edge(
            "EDGE-01",
            "EXH-CLIN-01",
            "EXH-SELF-02",
            InvestigationRelationKind::Corroborates,
            "Clinical psychologist assessment empirically grounds and corroborates the principal's self-authored functional impact testimony.",
            "2026-10-10T14:20:00Z",
        );
        board.add_edge(
            "EDGE-02",
            "EXH-DOC-03",
            "EXH-CLIN-01",
            InvestigationRelationKind::DerivesFrom,
            "14 October notice package transmits and derives directly from the specialist clinical attestation.",
            "2026-10-14T09:05:00Z",
        );
        board.add_edge(
            "EDGE-03",
            "EXH-DOC-03",
            "NORM-CML-04",
            InvestigationRelationKind::Obligates,
            "Receipt of clinical notice on 14 October triggered the agency's affirmative Duty of Inquiry and UN CRPD Art. 19/28 accommodation obligation.",
            "2026-10-14T09:05:00Z",
        );
        board.add_edge(
            "EDGE-04",
            "EXH-DISP-05",
            "EXH-DOC-03",
            InvestigationRelationKind::Contradicts,
            "22 October letter claiming 'no documentation on file' is chronologically and factually refuted by the 14 October signed delivery receipt.",
            "2026-10-22T16:40:00Z",
        );
        board.add_edge(
            "EDGE-05",
            "EXH-DISP-05",
            "NORM-CML-04",
            InvestigationRelationKind::ViolatesOrForbids,
            "Issuing an unaccommodated determination without reviewing the 14 October clinical submission breaches the administrative Duty of Inquiry and UN CRPD Art. 19/28.",
            "2026-10-22T16:40:00Z",
        );

        board
    }

    /// Adds a node to the case board, automatically enriching its ontology IRIs via WordNet
    /// and computing deterministic SHA-256 + QECP signature digests.
    #[allow(clippy::too_many_arguments)]
    pub fn add_node(
        &mut self,
        node_id: &str,
        kind: InvestigationNodeKind,
        title: &str,
        author_role: InstrumentAuthorRole,
        author_did_or_name: &str,
        timestamp_iso: &str,
        source_uri_or_citation: &str,
        summary_or_quote: &str,
        mut ontology_iris: Vec<String>,
        canvas_x: i32,
        canvas_y: i32,
    ) {
        let wn_report = map_text_to_wordnet_and_ontology_triples(&format!("{title} {summary_or_quote}"));
        for iri in wn_report.detected_normative_iris {
            if !ontology_iris.contains(&iri) {
                ontology_iris.push(iri);
            }
        }

        let canonical_payload = format!(
            "NODE|{}|{:?}|{}|{:?}|{}|{}|{}|{}",
            node_id,
            kind,
            title,
            author_role,
            author_did_or_name,
            timestamp_iso,
            source_uri_or_citation,
            summary_or_quote
        );
        let sha256_digest_hex = deterministic_digest_hex(canonical_payload.as_bytes());
        let sig_payload = format!("QECP-SIG-V1|{}|{}", self.principal_did, sha256_digest_hex);
        let qecp_signature_hex = deterministic_digest_hex(sig_payload.as_bytes());

        self.nodes.push(InvestigationNode {
            node_id: node_id.to_string(),
            kind,
            title: title.to_string(),
            author_role,
            author_did_or_name: author_did_or_name.to_string(),
            timestamp_iso: timestamp_iso.to_string(),
            source_uri_or_citation: source_uri_or_citation.to_string(),
            summary_or_quote: summary_or_quote.to_string(),
            ontology_iris,
            sha256_digest_hex,
            qecp_signature_hex,
            canvas_x,
            canvas_y,
        });
    }

    /// Adds a directed semantic/forensic edge between two nodes on the case board.
    pub fn add_edge(
        &mut self,
        edge_id: &str,
        from_node_id: &str,
        to_node_id: &str,
        relation: InvestigationRelationKind,
        evidentiary_note: &str,
        timestamp_iso: &str,
    ) {
        self.edges.push(InvestigationEdge {
            edge_id: edge_id.to_string(),
            from_node_id: from_node_id.to_string(),
            to_node_id: to_node_id.to_string(),
            relation,
            evidentiary_note: evidentiary_note.to_string(),
            timestamp_iso: timestamp_iso.to_string(),
        });
    }

    /// Runs a deterministic Forensic Contradiction, Paraconsistent Isolation & Duty-of-Inquiry Audit.
    pub fn run_investigation_audit(&self) -> InvestigationAuditReport {
        let mut findings = Vec::new();
        let mut admitted_instrument_count = 0usize;
        let mut quarantined_contested_count = 0usize;
        let mut contradiction_count = 0usize;
        let mut unfulfilled_duty_count = 0usize;

        for node in &self.nodes {
            if !node.author_role.admitted_for_positive_symbolic_inference() {
                quarantined_contested_count += 1;
                findings.push(ForensicAuditFinding {
                    finding_id: format!("AUDIT-QUAR-{}", node.node_id),
                    severity: "PARACONSISTENT-QUARANTINE".into(),
                    category: "Contested Record Isolation".into(),
                    summary: format!(
                        "Exhibit `{}` ({}) is marked as a Contested/Disputed Record and is quarantined from positive symbolic inference while preserved for contradiction cross-examination.",
                        node.node_id, node.title
                    ),
                    involved_node_ids: vec![node.node_id.clone()],
                    applicable_norm_iris: node.ontology_iris.clone(),
                });
            } else if node.kind == InvestigationNodeKind::SemanticInstrument {
                admitted_instrument_count += 1;
            }
        }

        for edge in &self.edges {
            match edge.relation {
                InvestigationRelationKind::Contradicts => {
                    contradiction_count += 1;
                    let from_title = self
                        .nodes
                        .iter()
                        .find(|n| n.node_id == edge.from_node_id)
                        .map(|n| n.title.as_str())
                        .unwrap_or(&edge.from_node_id);
                    let to_title = self
                        .nodes
                        .iter()
                        .find(|n| n.node_id == edge.to_node_id)
                        .map(|n| n.title.as_str())
                        .unwrap_or(&edge.to_node_id);
                    findings.push(ForensicAuditFinding {
                        finding_id: format!("AUDIT-CONTRA-{}", edge.edge_id),
                        severity: "HIGH-EVIDENTIARY-CONTRADICTION".into(),
                        category: "Paraconsistent Contradiction (0x30)".into(),
                        summary: format!(
                            "`{}` ({}) directly contradicts `{}` ({}): {}",
                            edge.from_node_id,
                            from_title,
                            edge.to_node_id,
                            to_title,
                            edge.evidentiary_note
                        ),
                        involved_node_ids: vec![edge.from_node_id.clone(), edge.to_node_id.clone()],
                        applicable_norm_iris: vec![
                            "https://ns.webcivics.net/qualia#contradicts".into(),
                            "https://ns.webcivics.net/cml#EvidentiaryChainOfCustody".into(),
                        ],
                    });
                }
                InvestigationRelationKind::ViolatesOrForbids => {
                    unfulfilled_duty_count += 1;
                    findings.push(ForensicAuditFinding {
                        finding_id: format!("AUDIT-BREACH-{}", edge.edge_id),
                        severity: "CRITICAL-DEONTIC-BREACH".into(),
                        category: "Breach of Legal / Human-Rights Obligation (0x12)".into(),
                        summary: format!(
                            "`{}` breaches norm `{}`: {}",
                            edge.from_node_id, edge.to_node_id, edge.evidentiary_note
                        ),
                        involved_node_ids: vec![edge.from_node_id.clone(), edge.to_node_id.clone()],
                        applicable_norm_iris: vec![
                            "https://ns.webcivics.net/cml#DutyOfInquiry".into(),
                            "https://ns.webcivics.net/rights/crpd#Article19_IndependentLivingAndSanctuary".into(),
                            "https://ns.webcivics.net/rights/crpd#Article28_AdequateStandardOfLiving".into(),
                        ],
                    });
                }
                _ => {}
            }
        }

        InvestigationAuditReport {
            case_id: self.case_id.clone(),
            total_nodes: self.nodes.len(),
            total_edges: self.edges.len(),
            admitted_instrument_count,
            quarantined_contested_count,
            contradiction_count,
            unfulfilled_duty_count,
            findings,
        }
    }

    /// Projects the entire `InvestigationCaseBoard` (nodes and semantic wires) directly onto
    /// Poet's spatial `Workbench` canvas (`ManifoldId::Rights` or the target manifold)
    /// so the user can interactively drag, inspect, and re-wire evidence containers on `CanvasStage`.
    pub fn project_onto_workbench_canvas(&self, wb: &mut Workbench, target_manifold: ManifoldId) {
        wb.switch(target_manifold);
        for node in &self.nodes {
            let canvas_id = format!("inv-{}", node.node_id.to_ascii_lowercase());
            wb.nodes.retain(|n| n.id != canvas_id);
            let container_kind = match node.kind {
                InvestigationNodeKind::EvidenceAsset => ContainerKind::Doc,
                InvestigationNodeKind::SemanticInstrument => ContainerKind::SemanticInstrument,
                InvestigationNodeKind::CmlLegalNorm => ContainerKind::Ontology,
                InvestigationNodeKind::ActorOrInstitution => ContainerKind::Social,
                InvestigationNodeKind::TimelineEvent => ContainerKind::Sheet,
            };
            wb.nodes.push(CanvasNode {
                id: canvas_id,
                kind: container_kind,
                title: format!("{} {} ({})", node.kind.icon(), node.title, node.node_id),
                x: node.canvas_x as f64,
                y: node.canvas_y as f64,
                width: 330.0,
                height: 190.0,
                z: 0.0,
                d: 0.0,
                strata: Strata::Legal,
                epistemic: Epistemic::All,
            });
        }

        for edge in &self.edges {
            let wire_id = format!("inv-wire-{}", edge.edge_id.to_ascii_lowercase());
            wb.wires.retain(|w| w.id != wire_id);
            wb.wires.push(Wire {
                id: wire_id,
                from: format!("inv-{}", edge.from_node_id.to_ascii_lowercase()),
                to: format!("inv-{}", edge.to_node_id.to_ascii_lowercase()),
                kind: edge.relation.wire_kind_css().to_string(),
                label: edge.relation.predicate_iri().to_string(),
            });
        }

        wb.note(format!(
            "Projected {} investigation exhibits & {} forensic wires onto {} manifold",
            self.nodes.len(),
            self.edges.len(),
            target_manifold.title()
        ));
    }
}
