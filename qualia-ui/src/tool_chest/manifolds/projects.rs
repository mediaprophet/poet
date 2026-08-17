//! Projects manifold seed — sheet, graph, pulse for collaborative projects.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use super::super::core::registry::{
    ManifoldSeed, SeedContainer, SeedConnection, SeedPanel, DockPosition,
};

pub fn projects_manifold_seed() -> ManifoldSeed {
    ManifoldSeed {
        id: "projects".into(),
        label: "Projects".into(),
        icon: "projects".into(),
        ontology_prefix: "coop".into(),
        description: "Collaborative projects — sheets, graph, pulse stream.".into(),
        containers: vec![
            SeedContainer {
                container_type: "sheet".into(),
                title: "Project Sheet".into(),
                x: 30.0, y: 30.0, width: 420.0, height: 340.0,
                z: 1.0,
                honesty: "present".into(),
            },
            SeedContainer {
                container_type: "graph".into(),
                title: "Project Graph".into(),
                x: 470.0, y: 30.0, width: 380.0, height: 340.0,
                z: 1.0,
                honesty: "present".into(),
            },
            SeedContainer {
                container_type: "pulse".into(),
                title: "Activity Pulse".into(),
                x: 30.0, y: 390.0, width: 820.0, height: 200.0,
                z: 1.0,
                honesty: "present".into(),
            },
        ],
        connections: vec![
            SeedConnection {
                id: "wire-p1".into(),
                from: 0, to: 1,
                wire_type: "active".into(),
                label: "coop:hasGraph".into(),
            },
            SeedConnection {
                id: "wire-p2".into(),
                from: 1, to: 2,
                wire_type: "event".into(),
                label: "coop:emitsPulse".into(),
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
