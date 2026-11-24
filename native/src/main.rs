//! Webizen Poet Native Workbench & CLI:
//! - `poet workbench` (or `poet` with `native-graphical`): Launches the Aether Native HyperCanvas Workbench.
//! - `poet wordnet <query>`: Searches the WordNet 3.1 + OMW Multilingual Lexicon (`Q42LEX`).
//! - `poet compile-q42 <preset-slug> <out.q42>`: Compiles an ontology preset to a `.q42` (`Q42\0` v4 `NQuin48`) binary pack.
//! - `poet export-pdf <profile-slug> <out.pdf>`: Generates a role-tailored Verifiable `%PDF-1.7` + PAdES/QECP Dossier.
//! - `poet repl` / `eval` / `format` / `lint` / `list-vibe` / `eval-packaged`: VibeScript toolchain.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

mod format;
mod lint;
mod repl;
use poet_cli::packaged_vibe;

use poet_aether_ui::investigation_link_analysis::InvestigationCaseBoard;
use poet_aether_ui::synth_modular_containers::{
    synthesize_industry_container_module, synthesize_industry_manifold_module,
    IndustryDomainProfile,
};
use poet_aether_ui::verifiable_pdf_printer::{
    generate_verifiable_pdf_dossier, verify_pdf_dossier_signatures, RecipientReportProfile,
};
use poet_aether_ui::wordnet_q42_ontology::{
    compile_ontology_to_q42_pack, search_wordnet_synsets, OntologySourcePreset,
};
use std::env;
use std::fs;
use std::process;

fn eval_source_with_grants(content: &str, grant_args: &[String]) {
    let grants = match repl::parse_repl_grants(grant_args) {
        Ok(grants) => grants,
        Err(error) => {
            eprintln!("Eval option error: {error}");
            process::exit(1);
        }
    };
    let mut host = repl::sheet_host::ReplHost::new(grants.clone());
    if grants.is_empty() {
        let mut env = vibe::Env::default();
        match vibe::eval_cell(content, &mut host, &mut env) {
            Ok(val) => println!("{val:?}"),
            Err(diag) => {
                eprintln!("Eval error: {}", diag.message);
                process::exit(1);
            }
        }
    } else {
        let mut session = vibe::session::SourceSession::new(
            grants,
            vibe::session::SessionLimits::default(),
        );
        match repl::eval_session_line(content, &mut host, &mut session) {
            Ok(result) => println!("{:?}", result.value),
            Err(error) => {
                eprintln!("Eval error: {}", error.diagnostic.message);
                if error.host_reconciliation_required {
                    eprintln!("Host outcome requires reconciliation; no retry was attempted.");
                }
                process::exit(1);
            }
        }
    }
}

fn print_usage() {
    println!(
        "Webizen Poet Native Workbench & CLI (VibeScript v{})",
        vibe::LANGUAGE_VERSION
    );
    println!("\nUsage:");
    #[cfg(feature = "native-graphical")]
    println!("  poet workbench                     Launch the native Aether HyperCanvas 10-Manifold Workbench");
    println!("  poet wordnet <query>               Search WordNet 3.1 + OMW multilingual synsets & ontology IRIs");
    println!("  poet compile-q42 <preset> <out>    Compile ontology preset to .q42 (Q42\\0 v4 NQuin48) binary vault");
    println!("  poet export-pdf <profile> <out>    Print verifiable %PDF-1.7 + PAdES/QECP dossier (clinical|legal|housing|oversight)");
    println!("  poet synth-container <ind> <out>   Synthesize modular industry container (.pcon.vibe or .pcon.q42.yaml)");
    println!("  poet synth-manifold <ind> <out>    Synthesize modular industry manifold (.pman.vibe)");
    println!("  poet repl [--grant <cap>]...       Interactive VibeScript REPL");
    println!("  poet format <file>                 Format a .vibe file");
    println!("  poet lint <file>                   Run static analysis checks on a .vibe file");
    println!("  poet eval <file> [--grant <cap>]   Evaluate a .vibe file and print result");
    println!("  poet list-vibe                     List packaged .vibe workflow scripts");
    println!("  poet eval-packaged <id>            Evaluate a packaged .vibe workflow script");
    println!("  poet help                          Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        #[cfg(feature = "native-graphical")]
        "workbench" | "hypercanvas" => {
            if let Err(error) = poet_cli::launch_workbench() {
                eprintln!("Poet Workbench: {error}");
                process::exit(1);
            }
        }
        "wordnet" => {
            let query = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let hits = search_wordnet_synsets(query, None);
            println!("WordNet 3.1 + OMW Multilingual hits for '{query}': {}", hits.len());
            for syn in hits {
                println!(
                    "  [{}] (ILI: {}) {} — {}",
                    syn.synset_id,
                    syn.ili_id,
                    syn.lemmas.join(", "),
                    syn.gloss
                );
                println!("    IRIs: {}", syn.mapped_ontology_iris.join(", "));
            }
        }
        "compile-q42" => {
            if args.len() < 4 {
                eprintln!("Usage: poet compile-q42 <preset-slug> <output.q42>");
                process::exit(1);
            }
            let slug = &args[2];
            let out_path = &args[3];
            let preset = OntologySourcePreset::ALL
                .into_iter()
                .find(|p| p.slug() == slug)
                .unwrap_or(OntologySourcePreset::Bio2RdfClinicalNeuro);
            match compile_ontology_to_q42_pack(
                preset.slug(),
                preset.title(),
                preset.default_turtle_source(),
                true,
            ) {
                Ok(art) => {
                    if let Err(e) = fs::write(out_path, &art.binary_payload_bytes) {
                        eprintln!("Failed to write {out_path}: {e}");
                        process::exit(1);
                    }
                    println!(
                        "Compiled {} ({} NQuin48 quads, {} bytes, digest {}) -> {}",
                        art.ontology_id,
                        art.nquin48_triple_count,
                        art.prolog_arena_bytes_used,
                        art.content_digest_hex,
                        out_path
                    );
                }
                Err(e) => {
                    eprintln!("Compile error: {e}");
                    process::exit(1);
                }
            }
        }
        "export-pdf" => {
            if args.len() < 4 {
                eprintln!("Usage: poet export-pdf <clinical|legal|housing|oversight> <output.pdf>");
                process::exit(1);
            }
            let prof_arg = args[2].to_ascii_lowercase();
            let out_path = &args[3];
            let profile = if prof_arg.contains("legal") || prof_arg.contains("right") {
                RecipientReportProfile::HumanRightsAndLegalAdvocacy
            } else if prof_arg.contains("hous") || prof_arg.contains("social") {
                RecipientReportProfile::SocialWorkAndHousingSupport
            } else if prof_arg.contains("over") || prof_arg.contains("police") || prof_arg.contains("law") {
                RecipientReportProfile::LawEnforcementAndProtectiveOversight
            } else {
                RecipientReportProfile::ClinicalDoctorAndPsychologist
            };
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
            let artifact = generate_verifiable_pdf_dossier(
                &board,
                &ontologies,
                profile,
                "did:q42:person:timothy-holborn",
                "Timothy Charles Holborn (Natural Person Principal)",
            );
            let verify = verify_pdf_dossier_signatures(&artifact.pdf_bytes);
            if !verify.master_signature_valid {
                eprintln!("PDF signature verification failed: {}", verify.diagnostic);
                process::exit(1);
            }
            if let Err(e) = fs::write(out_path, &artifact.pdf_bytes) {
                eprintln!("Failed to write {out_path}: {e}");
                process::exit(1);
            }
            println!(
                "Exported verifiable %PDF-1.7 dossier ({}, {} bytes, master sig {}) -> {}",
                artifact.profile.slug(),
                artifact.pdf_bytes.len(),
                artifact.pades_qecp_master_signature_hex,
                out_path
            );
        }
        "synth-container" => {
            if args.len() < 4 {
                eprintln!("Usage: poet synth-container <sound|film|colour|medical|engineering|legal> <out.pcon.vibe | out.pcon.q42.yaml>");
                process::exit(1);
            }
            let ind_arg = args[2].to_ascii_lowercase();
            let out_path = &args[3];
            let ind = if ind_arg.contains("sound") || ind_arg.contains("audio") {
                IndustryDomainProfile::SoundAndSpatialAudio
            } else if ind_arg.contains("film") || ind_arg.contains("video") {
                IndustryDomainProfile::FilmAndMotionPicture
            } else if ind_arg.contains("col") || ind_arg.contains("look") {
                IndustryDomainProfile::ColourGradingAndLookDev
            } else if ind_arg.contains("med") || ind_arg.contains("clin") {
                IndustryDomainProfile::MedicalAndClinical
            } else if ind_arg.contains("eng") || ind_arg.contains("cad") {
                IndustryDomainProfile::EngineeringAndCadCae
            } else {
                IndustryDomainProfile::LegalAndHumanRights
            };
            let module = synthesize_industry_container_module(ind, None);
            let content_str = if out_path.ends_with(".yaml") || out_path.ends_with(".yml") {
                module.to_q42_yaml()
            } else {
                module.to_pcon_vibe().expect("valid .pcon.vibe")
            };
            if let Err(e) = fs::write(out_path, &content_str) {
                eprintln!("Failed to write {out_path}: {e}");
                process::exit(1);
            }
            println!(
                "Synthesized modular container {} ({}, {} widgets, display {}, digest {}) -> {}",
                module.module_id,
                module.industry.slug(),
                module.widgets.len(),
                module.preferred_display_id,
                module.content_digest_hex,
                out_path
            );
        }
        "synth-manifold" => {
            if args.len() < 4 {
                eprintln!("Usage: poet synth-manifold <sound|film|colour|medical|engineering|legal> <out.pman.vibe>");
                process::exit(1);
            }
            let ind_arg = args[2].to_ascii_lowercase();
            let out_path = &args[3];
            let ind = if ind_arg.contains("sound") || ind_arg.contains("audio") {
                IndustryDomainProfile::SoundAndSpatialAudio
            } else if ind_arg.contains("film") || ind_arg.contains("video") {
                IndustryDomainProfile::FilmAndMotionPicture
            } else if ind_arg.contains("col") || ind_arg.contains("look") {
                IndustryDomainProfile::ColourGradingAndLookDev
            } else if ind_arg.contains("med") || ind_arg.contains("clin") {
                IndustryDomainProfile::MedicalAndClinical
            } else if ind_arg.contains("eng") || ind_arg.contains("cad") {
                IndustryDomainProfile::EngineeringAndCadCae
            } else {
                IndustryDomainProfile::LegalAndHumanRights
            };
            let pman = synthesize_industry_manifold_module(ind);
            let vibe_str = pman.to_pman_vibe().expect("valid .pman.vibe");
            if let Err(e) = fs::write(out_path, &vibe_str) {
                eprintln!("Failed to write {out_path}: {e}");
                process::exit(1);
            }
            println!(
                "Synthesized modular manifold {} ({}, {} containers, {} wires, digest {}) -> {}",
                pman.manifold_module_id,
                pman.industry.slug(),
                pman.containers.len(),
                pman.wires.len(),
                pman.content_digest_hex,
                out_path
            );
        }
        "list-vibe" | "vibe-list" => {
            println!(
                "{:<24} {:<10} {:<48} {}",
                "ID", "DOMAIN", "TITLE", "REQUIRED CAPABILITIES"
            );
            for script in packaged_vibe::packaged_vibe_scripts() {
                println!(
                    "{:<24} {:<10} {:<48} {}",
                    script.id,
                    script.domain,
                    script.title,
                    script.required_capabilities.join(", ")
                );
            }
        }
        "eval-packaged" => {
            if args.len() < 3 {
                eprintln!("Error: missing script id argument for eval-packaged");
                process::exit(1);
            }
            let id = &args[2];
            match packaged_vibe::find_packaged_vibe_script(id) {
                Some(script) => eval_source_with_grants(script.source, &args[3..]),
                None => {
                    eprintln!("Error: unknown packaged .vibe script '{id}'");
                    process::exit(1);
                }
            }
        }
        "repl" => {
            let grants = match repl::parse_repl_grants(&args[2..]) {
                Ok(grants) => grants,
                Err(error) => {
                    eprintln!("REPL option error: {error}");
                    process::exit(1);
                }
            };
            if let Err(e) = repl::run_repl_with_grants(grants) {
                eprintln!("REPL error: {e}");
                process::exit(1);
            }
        }
        "format" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for format");
                process::exit(1);
            }
            let path = &args[2];
            match fs::read_to_string(path) {
                Ok(content) => {
                    let formatted = format::format_source(&content);
                    if let Err(e) = fs::write(path, &formatted) {
                        eprintln!("Error writing formatted file: {e}");
                        process::exit(1);
                    }
                    println!("Formatted {}", path);
                }
                Err(e) => {
                    eprintln!("Error reading file '{path}': {e}");
                    process::exit(1);
                }
            }
        }
        "lint" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for lint");
                process::exit(1);
            }
            let path = &args[2];
            match fs::read_to_string(path) {
                Ok(content) => {
                    let issues = lint::lint_source(&content);
                    if issues.is_empty() {
                        println!("No issues found in {}", path);
                    } else {
                        for issue in &issues {
                            println!(
                                "{}:{}: {:?}: {}",
                                path, issue.line, issue.severity, issue.message
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{path}': {e}");
                    process::exit(1);
                }
            }
        }
        "eval" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for eval");
                process::exit(1);
            }
            let path = &args[2];
            match fs::read_to_string(path) {
                Ok(content) => eval_source_with_grants(&content, &args[3..]),
                Err(e) => {
                    eprintln!("Error reading file '{path}': {e}");
                    process::exit(1);
                }
            }
        }
        "help" | "-h" | "--help" => {
            print_usage();
        }
        other => {
            eprintln!("Unknown command: '{other}'");
            print_usage();
            process::exit(1);
        }
    }
}
