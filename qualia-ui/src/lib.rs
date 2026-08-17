//! QualiaDB UI — Tool-Chest, Manifolds, and Browser Preview.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! Principal / inventor: Timothy Charles Holborn <timothy.holborn@gmail.com>
//! Assignment: COPYRIGHT.md  Licence: LICENSE (CC BY-NC-ND 4.0)

pub mod tool_chest {
    pub mod core;
    pub mod manifolds;
    pub mod payloads;
}

pub mod browser;

pub use tool_chest::core::{
    ActionType, ContextRef, IntentBus, IntentReceipt, IntentStatus,
    Provenance, TargetIdentifier, TargetKind, VibeScriptPayload,
    Manifest, ManifestChain, ManifestTool,
    OntologyClass, OntologyModule, OntologyProperty, OntologyRegistry,
    DockPosition, ManifoldSeed, Registry, SeedConnection, SeedContainer, SeedPanel,
    SimpleTool, Tool, ToolKind, ToolMetadata, build_payload,
    ToolChain, ToolChainMetadata,
    Toolbox, ToolboxMetadata,
};
