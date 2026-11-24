//! Poet native authoring services, domain engines, and Aether native window launcher.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

pub mod domains;
pub mod packaged_vibe;

/// Launches the standalone Aether Native Poet HyperCanvas 10-Manifold Workbench.
#[cfg(feature = "native-graphical")]
pub fn launch_workbench() -> Result<(), String> {
    let attributes = aether_native::WindowAttributes::default()
        .with_title("Webizen Poet · HyperCanvas Workbench, Ontology Studio & Forensic Dossier")
        .with_surface_size(aether_native::LogicalSize::new(1600, 960));
    aether_native::launch_cfg(
        poet_aether_ui::PoetWorkbench,
        Vec::new(),
        vec![Box::new(attributes)],
    );
    Ok(())
}
