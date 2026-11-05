//! Poet CLI toolchain: REPL, formatter, linter, evaluator (T62).

mod format;
mod lint;
mod repl;
#[cfg(feature = "native-graphical")]
use poet_cli::graphical;
use poet_cli::packaged_vibe;

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
        "Poet CLI toolchain for VibeScript (v{})",
        vibe::LANGUAGE_VERSION
    );
    println!("\nUsage:");
    println!("  poet repl [--grant <capability>]...  Interactive REPL; default grants: none");
    println!("  poet format <file>  Format a .vibe file");
    println!("  poet lint <file>    Run static analysis checks on a .vibe file");
    println!("  poet eval <file> [--grant <capability>]...    Evaluate a .vibe file and print result");
    println!("  poet list-vibe      List packaged .vibe workflow scripts");
    println!("  poet eval-packaged <id> [--grant <capability>]...  Evaluate a packaged .vibe workflow script");
    println!("  poet help           Show this help message");
    #[cfg(feature = "native-graphical")]
    println!("  poet workbench      Launch the standalone HyperCanvas 10-Manifold Workbench");
    #[cfg(feature = "native-graphical")]
    println!(
        "  poet studio --diagnostic-new-source [--grant <capability>]...  Native Aether Studio"
    );
    #[cfg(feature = "native-graphical")]
    println!("  poet studio --native-project <directory> [--check-project]  Native enrollment, contributor and reviewer workspace");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        #[cfg(feature = "native-graphical")]
        "mcp" => {
            if args.len() != 2 {
                eprintln!(
                    "Usage: poet mcp; project admission is supplied only by a native embedding"
                );
                process::exit(1);
            }
            if let Err(error) = graphical::mcp::run_unbound_stdio() {
                eprintln!("Poet MCP: {error}");
                process::exit(1);
            }
        }
        #[cfg(feature = "native-graphical")]
        "studio" => {
            if let Err(error) = graphical::entry::run(&args[2..]) {
                eprintln!("Poet Studio: {error}");
                process::exit(1);
            }
        }
        #[cfg(feature = "native-graphical")]
        "workbench" | "hypercanvas" => {
            if let Err(error) = graphical::launch_workbench() {
                eprintln!("Poet Workbench: {error}");
                process::exit(1);
            }
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

