//! Poet HyperCanvas — Standalone Aether UI Workbench (`POET-SPEC-001..023`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use aether::prelude::*;

mod bodies;
mod chest;
mod chrome;
mod containers;
pub mod engine;
pub mod etalon_bridge;
mod gpu_frame;
pub mod host;
pub mod icons;
mod inspector;
pub mod instrument_bay;
pub mod kinds;
pub mod lexicon_bay;
pub mod lexicon_catalog;
mod manifolds;
pub mod mindmap_links;
mod radial_menu;
pub mod semantic_instruments;
mod stage;
pub mod store;
mod styles;
pub mod telemetry_drawer;
pub mod toolchain_deck;
mod tools;
mod vibe_console;
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
