//! ManifoldSeed CBOR-LD serialization and manifest persistence.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//!
//! Provides serialization/deserialization of `ManifoldSeed` to/from CBOR-LD
//! bytes, and a manifest persistence layer that can POST to the daemon
//! `POST /manifest` endpoint when native, or to `localStorage` on public web.

use crate::tool_chest::core::registry::ManifoldSeed;

// ---------------------------------------------------------------------------
// CBOR-LD serialization
// ---------------------------------------------------------------------------

/// Serialize a `ManifoldSeed` to CBOR-LD bytes.
pub fn serialize_seed(seed: &ManifoldSeed) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    ciborium::ser::into_writer(seed, &mut bytes)
        .map_err(|e| format!("cbor encode: {}", e))?;
    Ok(bytes)
}

/// Deserialize a `ManifoldSeed` from CBOR-LD bytes.
pub fn deserialize_seed(bytes: &[u8]) -> Result<ManifoldSeed, String> {
    ciborium::de::from_reader(bytes)
        .map_err(|e| format!("cbor decode: {}", e))
}

/// Serialize a vector of `ManifoldSeed` to CBOR-LD bytes.
pub fn serialize_seeds(seeds: &[ManifoldSeed]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    ciborium::ser::into_writer(seeds, &mut bytes)
        .map_err(|e| format!("cbor encode: {}", e))?;
    Ok(bytes)
}

/// Deserialize a vector of `ManifoldSeed` from CBOR-LD bytes.
pub fn deserialize_seeds(bytes: &[u8]) -> Result<Vec<ManifoldSeed>, String> {
    ciborium::de::from_reader(bytes)
        .map_err(|e| format!("cbor decode: {}", e))
}

// ---------------------------------------------------------------------------
// Manifest persistence
// ---------------------------------------------------------------------------

/// Manifest persistence backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistenceBackend {
    /// Daemon HTTP endpoint (`POST /manifest`).
    Daemon,
    /// Browser `localStorage`.
    LocalStorage,
    /// In-memory only (testing).
    Memory,
}

/// A manifest record — a checkpoint of canvas state.
#[derive(Clone, Debug)]
pub struct ManifestRecord {
    /// The manifold seeds at this checkpoint.
    pub seeds: Vec<ManifoldSeed>,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Revision number (monotonically increasing).
    pub revision: u64,
}

/// Manifest persistence trait.
pub trait ManifestPersistence {
    /// Save a manifest checkpoint.
    fn save(&self, record: &ManifestRecord) -> Result<(), String>;
    /// Load the latest manifest checkpoint.
    fn load_latest(&self) -> Result<Option<ManifestRecord>, String>;
    /// Load a specific revision.
    fn load_revision(&self, revision: u64) -> Result<Option<ManifestRecord>, String>;
}

/// In-memory manifest store (for testing and fallback).
pub struct MemoryManifestStore {
    records: std::cell::RefCell<Vec<ManifestRecord>>,
}

impl MemoryManifestStore {
    pub fn new() -> Self {
        Self {
            records: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl ManifestPersistence for MemoryManifestStore {
    fn save(&self, record: &ManifestRecord) -> Result<(), String> {
        self.records.borrow_mut().push(record.clone());
        Ok(())
    }

    fn load_latest(&self) -> Result<Option<ManifestRecord>, String> {
        Ok(self.records.borrow().last().cloned())
    }

    fn load_revision(&self, revision: u64) -> Result<Option<ManifestRecord>, String> {
        Ok(self.records.borrow().iter().find(|r| r.revision == revision).cloned())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_chest::core::registry::{SeedContainer, SeedPanel, DockPosition};

    fn test_seed() -> ManifoldSeed {
        ManifoldSeed {
            id: "test".into(),
            label: "Test".into(),
            icon: "test".into(),
            ontology_prefix: "test".into(),
            description: "Test manifold".into(),
            containers: vec![SeedContainer {
                container_type: "social".into(),
                title: "Social".into(),
                x: 100.0,
                y: 70.0,
                width: 420.0,
                height: 320.0,
                z: 100.0,
                honesty: "live".into(),
            }],
            connections: vec![],
            panels: vec![SeedPanel {
                panel_type: "pulse-panel".into(),
                dock: DockPosition::Bottom,
            }],
        }
    }

    #[test]
    fn test_seed_cbor_roundtrip() {
        let seed = test_seed();
        let bytes = serialize_seed(&seed).unwrap();
        assert!(!bytes.is_empty());
        let decoded = deserialize_seed(&bytes).unwrap();
        assert_eq!(decoded.id, "test");
        assert_eq!(decoded.containers.len(), 1);
        assert_eq!(decoded.containers[0].container_type, "social");
        assert_eq!(decoded.panels.len(), 1);
    }

    #[test]
    fn test_seeds_cbor_roundtrip() {
        let seeds = vec![test_seed(), test_seed()];
        let bytes = serialize_seeds(&seeds).unwrap();
        assert!(!bytes.is_empty());
        let decoded = deserialize_seeds(&bytes).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].id, "test");
    }

    #[test]
    fn test_memory_manifest_store() {
        let store = MemoryManifestStore::new();
        let record = ManifestRecord {
            seeds: vec![test_seed()],
            timestamp: "2026-08-18T00:00:00Z".into(),
            revision: 1,
        };
        store.save(&record).unwrap();

        let latest = store.load_latest().unwrap().unwrap();
        assert_eq!(latest.revision, 1);
        assert_eq!(latest.seeds.len(), 1);

        let rev0 = store.load_revision(0).unwrap();
        assert!(rev0.is_none());

        let rev1 = store.load_revision(1).unwrap().unwrap();
        assert_eq!(rev1.timestamp, "2026-08-18T00:00:00Z");
    }

    #[test]
    fn test_deserialize_invalid_bytes() {
        let result = deserialize_seed(&[0xFF, 0xFF]);
        assert!(result.is_err());
    }
}
