//! Poet HyperCanvas — Standalone Aether UI Workbench (`POET-SPEC-001..023`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use aether::prelude::*;

pub mod bodies;
mod chest;
mod chrome;
mod containers;
pub mod engine;
pub mod etalon_bridge;
pub mod evidentiary_cold_archiver_qar;
mod gpu_frame;
pub mod host;
pub mod icons;
mod inspector;
pub mod instrument_bay;
pub mod investigation_link_analysis;
pub mod kinds;
pub mod lexicon_bay;
pub mod lexicon_catalog;
mod manifolds;
pub mod mindmap_links;
pub mod qap_container_v1;
pub mod qap_space_multi_device_wm;
mod radial_menu;
pub mod semantic_instruments;
mod stage;
pub mod store;
mod styles;
pub mod synth_modular_containers;
pub mod telemetry_drawer;
pub mod toolchain_deck;
mod tools;
pub mod verifiable_pdf_printer;
mod vibe_console;
pub mod wordnet_q42_ontology;
mod workbench;
pub mod xyzd_gizmo;

pub use icons::IconBadge;
pub use kinds::{ContainerKind, ManifoldId, ToolboxId};
pub use radial_menu::{RadialActionRing, RadialState};
pub use workbench::PoetWorkbench;
pub use xyzd_gizmo::{GizmoMode, XyzdGizmo};

#[component]
/// Aether `PoetHarness` component.
pub fn PoetHarness() -> Element {
    rsx! { PoetWorkbench {} }
}

#[cfg(test)]
mod poet_overhaul_tests {
    use crate::investigation_link_analysis::{
        InstrumentAuthorRole, InvestigationCaseBoard, InvestigationNodeKind,
        InvestigationRelationKind,
    };
    use crate::kinds::ManifoldId;
    use crate::store::Workbench;
    use crate::verifiable_pdf_printer::{
        generate_verifiable_pdf_dossier, verify_pdf_dossier_signatures, RecipientReportProfile,
    };
    use crate::wordnet_q42_ontology::{
        compile_ontology_to_q42_pack, map_text_to_wordnet_and_ontology_triples,
        search_wordnet_synsets, OntologySourcePreset, WordNetPos,
    };

    #[test]
    fn wordnet_multilingual_lexicon_and_q42_compiler_roundtrip() {
        let adhd_hits = search_wordnet_synsets("ADHD", Some(WordNetPos::Noun));
        assert!(!adhd_hits.is_empty());
        assert_eq!(adhd_hits[0].ili_id, "i112410");
        assert!(adhd_hits[0]
            .mapped_ontology_iris
            .contains(&"http://purl.obolibrary.org/obo/DOID_1094".to_string()));

        // Multilingual lookup (French TDAH / TSPT)
        let fr_hits = search_wordnet_synsets("TDAH", None);
        assert_eq!(fr_hits.len(), 1);
        assert_eq!(fr_hits[0].synset_id, "wn31:114391926-n");

        // Natural-language text to WordNet + N3 + SHACL mapping
        let report = map_text_to_wordnet_and_ontology_triples(
            "Psychologist ADHD and PTSD attestation establishes duty of inquiry and housing sanctuary to protect human dignity.",
        );
        assert!(report.matched_synsets.len() >= 4);
        assert!(report
            .generated_turtle_n3
            .contains("qualia:LexicalGrounding"));
        assert!(report.generated_shacl_shapes.contains("sh:NodeShape"));

        // Compile all 8 ontology presets into binary Q42\0 v4 containers
        for preset in OntologySourcePreset::ALL {
            let compiled = compile_ontology_to_q42_pack(
                preset.slug(),
                preset.title(),
                preset.default_turtle_source(),
                true,
            )
            .expect("preset compiles into valid .q42");
            assert_eq!(&compiled.binary_payload_bytes[0..4], b"Q42\0");
            assert_eq!(compiled.format_version, 4);
            assert!(compiled.nquin48_triple_count >= 2);
            assert!(compiled.prolog_sentinel_42mib_ok);
            assert!(compiled
                .permissive_commons_qcdn_uri
                .as_deref()
                .unwrap_or("")
                .starts_with("qcdn://permissive-commons/ontologies/"));
        }
    }

    #[test]
    fn forensic_investigation_audit_and_workbench_canvas_projection() {
        let mut board = InvestigationCaseBoard::default();
        let audit = board.run_investigation_audit();
        assert_eq!(audit.total_nodes, 5);
        assert_eq!(audit.total_edges, 5);
        assert_eq!(audit.admitted_instrument_count, 2);
        assert_eq!(audit.quarantined_contested_count, 1);
        assert_eq!(audit.contradiction_count, 1);
        assert_eq!(audit.unfulfilled_duty_count, 1);

        // Add a supplemental co-attested GP exhibit
        board.add_node(
            "EXH-GP-06",
            InvestigationNodeKind::SemanticInstrument,
            "GP Clinical Co-Attestation for PTSD Sanctuary Housing",
            InstrumentAuthorRole::CollaborativeCoAttested,
            "did:web:clinic.example.au:gp",
            "2026-10-25T09:00:00Z",
            "qecp://sanctuary/instruments/gp-ptsd-2026.q42",
            "Co-attested GP clinical summary confirming PTSD and ADHD accommodation requirements.",
            Vec::new(),
            320,
            420,
        );
        board.add_edge(
            "EDGE-06",
            "EXH-GP-06",
            "EXH-CLIN-01",
            InvestigationRelationKind::Corroborates,
            "GP summary corroborates psychologist diagnosis.",
            "2026-10-25T09:00:00Z",
        );

        let mut wb = Workbench::new();
        board.project_onto_workbench_canvas(&mut wb, ManifoldId::Rights);
        assert_eq!(wb.active, ManifoldId::Rights);
        let projected_nodes = wb
            .nodes
            .iter()
            .filter(|n| n.id.starts_with("inv-"))
            .count();
        let projected_wires = wb
            .wires
            .iter()
            .filter(|w| w.id.starts_with("inv-wire-"))
            .count();
        assert_eq!(projected_nodes, 6);
        assert_eq!(projected_wires, 6);
    }

    #[test]
    fn verifiable_pdf_dossier_printer_and_qecp_signature_schedule_verification() {
        let board = InvestigationCaseBoard::default();
        let ontologies = vec![
            compile_ontology_to_q42_pack(
                OntologySourcePreset::Bio2RdfClinicalNeuro.slug(),
                OntologySourcePreset::Bio2RdfClinicalNeuro.title(),
                OntologySourcePreset::Bio2RdfClinicalNeuro.default_turtle_source(),
                true,
            )
            .unwrap(),
            compile_ontology_to_q42_pack(
                OntologySourcePreset::UnCrpdHumanRights.slug(),
                OntologySourcePreset::UnCrpdHumanRights.title(),
                OntologySourcePreset::UnCrpdHumanRights.default_turtle_source(),
                true,
            )
            .unwrap(),
        ];

        for profile in RecipientReportProfile::ALL {
            let pdf = generate_verifiable_pdf_dossier(
                &board,
                &ontologies,
                profile,
                "did:q42:person:timothy-holborn",
                "Timothy Charles Holborn (Natural Person Principal)",
            );
            assert!(pdf.pdf_bytes.starts_with(b"%PDF-1.7"));
            assert_eq!(pdf.page_count, 2);
            assert_eq!(pdf.exhibit_signatures.len(), 5);
            assert_eq!(pdf.compiled_q42_digests.len(), 2);

            let verification = verify_pdf_dossier_signatures(&pdf.pdf_bytes);
            assert!(verification.valid_pdf_header);
            assert!(verification.qecp_envelope_present);
            assert!(verification.master_signature_valid);
            assert_eq!(verification.verified_exhibit_count, 5);
            assert_eq!(verification.verified_q42_ontology_count, 2);
        }
    }

    #[test]
    fn qap_space_window_manager_and_multi_device_tv_tablet_hid_mesh() {
        use crate::qap_space_multi_device_wm::{
            DisplayTransportKind, QapSpaceKind, QapSpaceMultiDeviceManager, WindowLayoutMode,
        };

        let mut wm = QapSpaceMultiDeviceManager::default();
        let mut wb = Workbench::new();
        assert_eq!(wm.active_qap_space, QapSpaceKind::PersonalSanctuary);
        assert!(!wm.active_qap_space.allows_public_qcdn_export());
        assert_eq!(wm.displays.len(), 5);
        assert_eq!(wm.devices.len(), 5);

        // Switch Qap Space to WellFair Casework, then Permissive Commons
        wm.switch_qap_space(QapSpaceKind::WellFairCasework);
        assert!(!wm.active_qap_space.allows_public_qcdn_export());
        wm.switch_qap_space(QapSpaceKind::PermissiveCommonsStudio);
        assert!(wm.active_qap_space.allows_public_qcdn_export());

        // Apply Preset 1: Stream Content to TV + Laptop Tools & HID Input
        wm.apply_multi_device_preset("laptop-hid-tv-stream", &mut wb);
        assert_eq!(wm.layout_mode, WindowLayoutMode::MultiDisplayAndDeviceSplit);
        assert!(!wm.container_routes.is_empty());
        assert_eq!(wm.container_routes[0].target_display_id, "DISP-3-TV");
        assert!(wm.container_routes[0].fullscreen_on_target);
        assert!(!wm.hid_forwarding_log.is_empty());

        // Cycle TV connection across Chromecast -> Miracast -> WebRTC -> HbbTV -> Wired HDMI 2.1
        assert_eq!(wm.tv_transport(), DisplayTransportKind::WirelessCastChromecast);
        wm.cycle_tv_transport();
        assert_eq!(wm.tv_transport(), DisplayTransportKind::WirelessCastMiracastAirPlay);
        wm.cycle_tv_transport();
        assert_eq!(wm.tv_transport(), DisplayTransportKind::WebRtcLocalLanQuicStream);
        wm.cycle_tv_transport();
        assert_eq!(wm.tv_transport(), DisplayTransportKind::HbbTvCompanionBridge);
        wm.cycle_tv_transport();
        assert_eq!(wm.tv_transport(), DisplayTransportKind::HdmiOrDisplayPortWired);

        // Apply Preset 2: Main Monitor + Tablet/Phone Tool Surface
        wm.apply_multi_device_preset("tablet-tools-main-monitor", &mut wb);
        assert_eq!(wm.container_routes[0].target_display_id, "DISP-1-MAIN");

        // Apply Preset 3: Multi-Monitor Span + Window Tiling & Minimize/Restore
        wm.apply_multi_device_preset("triple-monitor-span", &mut wb);
        let first_id = wb.nodes[0].id.clone();
        wm.toggle_minimize_container(&first_id);
        assert!(wm.is_minimized(&first_id));
        wm.toggle_minimize_container(&first_id);
        assert!(!wm.is_minimized(&first_id));

        wm.tile_workbench_containers(&mut wb);
        assert_eq!(wm.layout_mode, WindowLayoutMode::TiledSplitWorkstation);
    }

    #[test]
    fn poet_capability_host_and_live_10d_holo_viewport_plumbing() {
        use crate::engine::{
            dispatch_poet_capability, synthesize_live_etalon_10d_holo_svg_uri,
        };

        let mut committed = 0usize;
        let empty_rec = vibe::Value::Record(std::collections::BTreeMap::new());
        for cap_id in [
            "Document.layout_paragraph",
            "Document.proof_check",
            "Sheet.sum_range",
            "Sheet.eval_formula",
            "Sheet.pivot_analyze",
            "GraphDatabase.lexicon_manifest",
            "GraphDatabase.volume_open",
            "GraphDatabase.sparql",
            "SHACL.extensions",
            "WgslForge.generate",
            "WgslForge.roofline",
            "ClinicalRisk.framingham",
            "ComputationalGeometry.convex_hull_2",
            "EngineeringAnalysis.kinematics",
            "Manifold.axes",
            "Econ.evaluate_peer",
            "Finance.npv",
            "Finance.portfolio_var",
            "commerce_settle",
            "Bookmarks.list",
            "Codecs.matrix",
            "Domain.info",
            "Git.status",
            "Job.list",
            "Net.peer_hash",
            "Social.lww",
            "Vibe.eval",
        ] {
            let res = dispatch_poet_capability(cap_id, &empty_rec, &mut committed)
                .expect("capability succeeds");
            if let vibe::Value::Record(m) = res {
                assert_eq!(m.get("ok"), Some(&vibe::Value::Bool(true)), "cap {cap_id} ok");
            } else {
                panic!("expected Record for {cap_id}");
            }
        }
        assert_eq!(committed, 27);

        let (uri, nodes, edges, faces) =
            synthesize_live_etalon_10d_holo_svg_uri("health", 720, 300, 45.0, 512.0);
        assert!(uri.starts_with("data:image/svg+xml;utf8,"));
        assert!(nodes > 0 && edges > 0 && faces > 0);
    }

    #[test]
    fn synth_modular_container_pcon_and_manifold_pman_file_roundtrip_and_mounting() {
        use crate::qap_space_multi_device_wm::QapSpaceMultiDeviceManager;
        use crate::synth_modular_containers::{
            builtin_industry_container_modules, mount_container_module_onto_workbench,
            mount_manifold_module_onto_workbench, synthesize_industry_manifold_module,
            IndustryDomainProfile, SynthContainerModule, SynthManifoldModule,
        };

        let modules = builtin_industry_container_modules();
        assert_eq!(modules.len(), 6);

        let mut wb = Workbench::new();
        let mut wm = QapSpaceMultiDeviceManager::default();

        for module in &modules {
            let vibe_src = module.to_pcon_vibe().expect("serializes to .pcon.vibe");
            let loaded = SynthContainerModule::from_pcon_vibe(&vibe_src)
                .expect("parses and evaluates from .pcon.vibe via vibe engine");
            assert_eq!(loaded.module_id, module.module_id);
            assert_eq!(loaded.content_digest_hex, module.content_digest_hex);
            assert!(!loaded.widgets.is_empty());

            let q42_yaml = module.to_q42_yaml();
            assert!(q42_yaml.contains("\"@context\":"));
            assert!(q42_yaml.contains("L6: \"L6QuantumPhysics\""));

            let node_id =
                mount_container_module_onto_workbench(&loaded, &mut wb, &mut wm, 40.0, 40.0);
            assert!(wb.node(&node_id).is_some());
            assert_eq!(
                wm.route_for_container(&node_id)
                    .map(|r| r.target_display_id.as_str()),
                Some(loaded.preferred_display_id.as_str())
            );
        }

        // Test full .pman.vibe manifold file roundtrip and mounting
        for ind in [
            IndustryDomainProfile::SoundAndSpatialAudio,
            IndustryDomainProfile::FilmAndMotionPicture,
            IndustryDomainProfile::ColourGradingAndLookDev,
            IndustryDomainProfile::MedicalAndClinical,
            IndustryDomainProfile::EngineeringAndCadCae,
            IndustryDomainProfile::LegalAndHumanRights,
        ] {
            let pman = synthesize_industry_manifold_module(ind);
            let pman_vibe = pman.to_pman_vibe().expect("serializes to .pman.vibe");
            let loaded_pman = SynthManifoldModule::from_pman_vibe(&pman_vibe)
                .expect("parses and evaluates from .pman.vibe via vibe engine");
            assert_eq!(loaded_pman.containers.len(), 2);
            assert_eq!(loaded_pman.wires.len(), 1);
            mount_manifold_module_onto_workbench(&loaded_pman, &mut wb, &mut wm);
            assert_eq!(wb.active, ind.default_manifold());
        }
    }

    #[test]
    fn poet_qap_container_v1_seals_and_zero_copy_loads_modular_pcon_and_pman_vibe() {
        use crate::qap_container_v1::{
            build_canonical_poet_qap_container, QapContainerReader, QAP_ENTRY_ALIGN, QAP_MAGIC,
        };

        let bytes = build_canonical_poet_qap_container().expect("builds poet.qap container");
        assert_eq!(&bytes[0..4], &QAP_MAGIC);
        assert_eq!(bytes.len() % QAP_ENTRY_ALIGN, 0);

        let reader = QapContainerReader::parse(&bytes).expect("parses poet.qap");
        assert_eq!(reader.entries.len(), 6);
        for e in &reader.entries {
            assert_eq!(e.offset as usize % QAP_ENTRY_ALIGN, 0);
        }

        // Zero-copy load .pcon.vibe modular container directly from the .qap byte slice
        let pcon = reader
            .load_pcon_vibe_module("vibe/containers/pcon_sound_spatial_hrtf.pcon.vibe")
            .expect("zero-copy loads and evaluates .pcon.vibe from .qap");
        assert_eq!(pcon.module_id, "pcon-sound-spatial-hrtf-01");

        // Zero-copy load .pman.vibe industry manifold directly from the .qap byte slice
        let pman = reader
            .load_pman_vibe_module("vibe/manifolds/pman_film_mastering.pman.vibe")
            .expect("zero-copy loads and evaluates .pman.vibe from .qap");
        assert_eq!(pman.containers.len(), 2);

        // Zero-copy verify embedded WordNet .q42 volume inside the .qap
        let q42_slice = reader
            .entry_slice("ontologies/wordnet31_omw_lexicon.q42")
            .expect("embedded .q42 slice");
        assert_eq!(&q42_slice[0..4], b"Q42\0");
    }

    #[test]
    fn poet_evidentiary_cold_archiver_qar_preserves_historical_pcon_and_pdf_versions() {
        use crate::evidentiary_cold_archiver_qar::{
            PreservedEvidentiaryTimestamps, QarContainerReader, QarEvidentiaryArchiver,
            QAR_ENTRY_ALIGN, QAR_MAGIC,
        };

        let mut archiver = QarEvidentiaryArchiver::new(
            "qar://cold-vault/poet-forensic-archive-2026.qar",
            "did:q42:person:timothy-holborn",
        );

        let v1_pcon = b"// Historical v0.0.40 .pcon.vibe modular container payload".to_vec();
        let v2_pcon = b"// Updated v0.0.41 .pcon.vibe modular container payload";

        let stub = archiver
            .supersede_and_archive_to_cold_storage(
                "poet:pcon:sound-spatial-hrtf",
                "pcon_sound_spatial_hrtf_v0.0.40.pcon.vibe",
                "vibe_container",
                "0.0.40",
                v1_pcon.clone(),
                v2_pcon,
                PreservedEvidentiaryTimestamps {
                    original_created_at_unix: 1_760_000_000,
                    original_modified_at_unix: 1_760_100_000,
                    sanctuary_ingested_at_unix: 1_760_100_500,
                    archived_superseded_at_unix: 1_763_960_000,
                },
                "did:q42:person:timothy-holborn",
                "Superseded by v0.0.41 spatial HRTF container; v0.0.40 archived in cold .qar",
            )
            .expect("archives v0.0.40 pcon");

        let qar_bytes = archiver
            .seal_cold_qar_container()
            .expect("seals poet .qar archive");
        assert_eq!(&qar_bytes[0..4], &QAR_MAGIC);
        assert_eq!(qar_bytes.len() % QAR_ENTRY_ALIGN, 0);
        assert_eq!(archiver.runtime_payload_bytes_freed, v1_pcon.len() as u64);

        let reader = QarContainerReader::parse(&qar_bytes).expect("verifies poet .qar");
        let cert = reader
            .recall_historical_version(&stub.archive_entry_id)
            .expect("recalls v0.0.40 pcon");
        assert_eq!(cert.recalled_payload_bytes, v1_pcon);
        assert!(cert.payload_digest_verified && cert.cumulative_chain_verified);
    }
}
