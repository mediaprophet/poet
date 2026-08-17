//! Sanctuary manifold seed — vault, notes, pulse.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::super::core::registry::{
    ManifoldSeed, SeedContainer, SeedConnection, SeedPanel, DockPosition,
};

pub fn sanctuary_manifold_seed() -> ManifoldSeed {
    ManifoldSeed {
        id: "sanctuary".into(),
        label: "Sanctuary".into(),
        icon: "sanctuary".into(),
        ontology_prefix: "sanctuary".into(),
        description: "Vault lock, protected notes, pulse stream. Zero-motion mode.".into(),
        containers: vec![
            SeedContainer {
                container_type: "doc".into(),
                title: "Vault".into(),
                x: 30.0, y: 30.0, width: 400.0, height: 300.0,
                z: 1.0,
                honesty: "present".into(),
            },
            SeedContainer {
                container_type: "pulse".into(),
                title: "Pulse Stream".into(),
                x: 450.0, y: 30.0, width: 400.0, height: 300.0,
                z: 1.0,
                honesty: "present".into(),
            },
        ],
        connections: vec![],
        panels: vec![
            SeedPanel {
                panel_type: "aura".into(),
                dock: DockPosition::Right,
            },
        ],
    }
}
