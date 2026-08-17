//! Rights & wallet manifold seed — agreements, identity, wallet.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::super::core::registry::{
    ManifoldSeed, SeedContainer, SeedConnection, SeedPanel, DockPosition,
};

pub fn rights_manifold_seed() -> ManifoldSeed {
    ManifoldSeed {
        id: "rights".into(),
        label: "Rights & Wallet".into(),
        icon: "rights".into(),
        ontology_prefix: "rights".into(),
        description: "Agreements, DID signing, fiduciary obligations, wallet.".into(),
        containers: vec![
            SeedContainer {
                container_type: "rights".into(),
                title: "Rights & Agreements".into(),
                x: 30.0, y: 30.0, width: 420.0, height: 360.0,
                z: 1.0,
                honesty: "present".into(),
            },
            SeedContainer {
                container_type: "wallet".into(),
                title: "Wallet".into(),
                x: 470.0, y: 30.0, width: 380.0, height: 360.0,
                z: 1.0,
                honesty: "present".into(),
            },
        ],
        connections: vec![
            SeedConnection {
                id: "wire-rw1".into(),
                from: 0, to: 1,
                wire_type: "active".into(),
                label: "rights:hasWallet".into(),
            },
        ],
        panels: vec![
            SeedPanel {
                panel_type: "aura".into(),
                dock: DockPosition::Right,
            },
        ],
    }
}
