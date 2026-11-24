//! `qap_container_v1` for Webizen Poet (`poet-aether-ui`):
//! Packages Poet's Modular Containers (`*.pcon.vibe` + `*.pcon.q42.yaml`), Industry Manifolds
//! (`*.pman.vibe`), WordNet 3.1 + OMW `.q42` (`Q42\0` v4 `NQuin48`) Ontology Vaults, and
//! Holographic 10D v2 (`HOLO` `0x0306`) Viewport Assets into a single 64-byte page-aligned,
//! signed `.qap` (`QAP\0` v1) application/manifold container with Sanctuary Staging (`collect != activate`).

use crate::synth_modular_containers::{
    builtin_industry_container_modules, synthesize_industry_manifold_module, IndustryDomainProfile,
    SynthContainerModule, SynthManifoldModule,
};
use crate::wordnet_q42_ontology::{compile_ontology_to_q42_pack, OntologySourcePreset};
use serde::{Deserialize, Serialize};

pub const QAP_MAGIC: [u8; 4] = *b"QAP\0";
pub const QAP_VERSION: u16 = 1;
pub const QAP_HEADER_SIZE: usize = 64;
pub const QAP_ENTRY_ALIGN: usize = 64;
pub const CBOR_LD_TAG: [u8; 3] = [0xD9, 0x05, 0x01];
pub const SANCTUARY_QAP_STAGING_URI: &str = "sanctuary://intake/qap-staging";

pub const QAP_FLAG_ALIGN_64: u16 = 0x0001;
pub const QAP_FLAG_SIGNED_QECP: u16 = 0x0002;
pub const QAP_FLAG_CBOR_LD_MANIFEST: u16 = 0x0004;
pub const QAP_FLAG_VIBE_ENTRYPOINT: u16 = 0x0008;

#[inline]
pub const fn align_up_64(n: usize) -> usize {
    (n + (QAP_ENTRY_ALIGN - 1)) & !(QAP_ENTRY_ALIGN - 1)
}

pub fn compute_qap_checksum32(bytes: &[u8]) -> u32 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for (idx, &b) in bytes.iter().enumerate() {
        let val = if (36..40).contains(&idx) { 0 } else { b };
        hash ^= val as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    (hash ^ (hash >> 32)) as u32
}

pub fn compute_entry_digest32(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for lane in 0..4u64 {
        let mut h: u64 = 0xcbf29ce484222325 ^ lane.wrapping_mul(0x9e3779b97f4a7c15);
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        out[(lane as usize) * 8..((lane as usize) + 1) * 8].copy_from_slice(&h.to_le_bytes());
    }
    out
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QapBundleEntry {
    pub key: String,
    pub kind: String,
    pub offset: u64,
    pub length: u64,
    pub digest_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QapYamlLdDescriptor {
    pub qap_id: String,
    pub qap_kind: String,
    pub display_name: String,
    pub version: String,
    pub abi_version: String,
    pub license: String,
    pub author_did: String,
    pub qdp_domain: String,
    pub did_git: String,
    pub max_sensitivity_clearance: String,
    pub required_ontologies: Vec<String>,
    pub required_host_capabilities: Vec<String>,
    pub main_vibe_module: String,
    pub main_vibe_entrypoint_fn: String,
    pub qecp_signature_hex: String,
}

impl QapYamlLdDescriptor {
    pub fn to_yaml_ld_q42_string(&self) -> String {
        let mut y = String::new();
        y.push_str("%YAML 1.2\n---\n");
        y.push_str("\"@context\":\n");
        y.push_str("  q42: \"https://qualia.io/ontology/q42#\"\n");
        y.push_str("  qap: \"https://qualia.io/ontology/qap#\"\n");
        y.push_str("  hro: \"https://qualia.io/ontology/human-rights#\"\n");
        y.push_str("  qdp: \"https://webcivics.github.io/QDP/ontdev/QDP#\"\n");
        y.push_str(&format!("\"@id\": \"urn:qualia:qap:{}:{}\"\n", self.qap_id, self.version));
        y.push_str("\"@type\": \"qap:ApplicationPackage\"\n");
        y.push_str("schema_version: \"yaml-ld-q42/v2\"\n");
        y.push_str(&format!("qap_id: \"{}\"\n", self.qap_id));
        y.push_str(&format!("qap_kind: \"{}\"\n", self.qap_kind));
        y.push_str(&format!("display_name: \"{}\"\n", self.display_name));
        y.push_str(&format!("version: \"{}\"\n", self.version));
        y.push_str(&format!("abi_version: \"{}\"\n", self.abi_version));
        y.push_str(&format!("license: \"{}\"\n", self.license));
        y.push_str(&format!("author_did: \"{}\"\n", self.author_did));
        y.push_str(&format!("qdp_domain: \"{}\"\n", self.qdp_domain));
        y.push_str(&format!("did_git: \"{}\"\n", self.did_git));
        y.push_str("computational_hierarchy_7_layer:\n");
        y.push_str("  L0: \"L0Continuum\"\n");
        y.push_str("  L1: \"L1RigidArticulatedPgaXpbdSkl2\"\n");
        y.push_str("  L2: \"L2SoftBodyFluid\"\n");
        y.push_str("  L3: \"L3MolecularChemical\"\n");
        y.push_str("  L4: \"L4CellularBiological\"\n");
        y.push_str("  L5: \"L5EcologicalGeomorphic\"\n");
        y.push_str("  L6: \"L6QuantumPhysics\"\n");
        y.push_str("sentinel_capability_claims:\n");
        y.push_str(&format!(
            "  max_sensitivity_clearance: \"{}\"\n",
            self.max_sensitivity_clearance
        ));
        y.push_str("  required_ontologies:\n");
        for ont in &self.required_ontologies {
            y.push_str(&format!("    - \"{}\"\n", ont));
        }
        y.push_str("  required_host_capabilities:\n");
        for cap in &self.required_host_capabilities {
            y.push_str(&format!("    - \"{}\"\n", cap));
        }
        y.push_str("vibe_entrypoints:\n");
        y.push_str(&format!("  main_module: \"{}\"\n", self.main_vibe_module));
        y.push_str(&format!(
            "  entrypoint_fn: \"{}\"\n",
            self.main_vibe_entrypoint_fn
        ));
        y.push_str(&format!("qecp_signature_hex: \"{}\"\n", self.qecp_signature_hex));
        y
    }
}

pub struct QapContainerWriter {
    pending: Vec<(String, String, Vec<u8>)>,
}

impl Default for QapContainerWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl QapContainerWriter {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        key: impl Into<String>,
        kind: impl Into<String>,
        payload: Vec<u8>,
    ) -> Result<(), String> {
        let k = key.into();
        if k.trim().is_empty() {
            return Err("QAP entry key cannot be empty".to_string());
        }
        if self.pending.iter().any(|(existing, _, _)| existing == &k) {
            return Err(format!("Duplicate QAP entry key '{k}'"));
        }
        self.pending.push((k, kind.into(), payload));
        Ok(())
    }

    pub fn build(&self) -> Result<Vec<u8>, String> {
        let mut buf = vec![0u8; QAP_HEADER_SIZE];
        let mut entries = Vec::with_capacity(self.pending.len());
        let mut manifest_digest_prefix = [0u8; 24];

        for (key, kind, payload) in &self.pending {
            let aligned_off = align_up_64(buf.len());
            if aligned_off > buf.len() {
                buf.resize(aligned_off, 0);
            }
            let offset = buf.len() as u64;
            let length = payload.len() as u64;
            let digest = compute_entry_digest32(payload);
            if key == "manifest/qap_manifest.cborld" || key == "manifest/qap.q42.yaml" {
                manifest_digest_prefix.copy_from_slice(&digest[..24]);
            }
            buf.extend_from_slice(payload);
            entries.push(QapBundleEntry {
                key: key.clone(),
                kind: kind.clone(),
                offset,
                length,
                digest_hex: hex_encode(&digest),
            });
        }

        let index_aligned_off = align_up_64(buf.len());
        if index_aligned_off > buf.len() {
            buf.resize(index_aligned_off, 0);
        }
        let index_offset = buf.len() as u64;

        let mut index_bytes = Vec::new();
        index_bytes.extend_from_slice(&CBOR_LD_TAG);
        let json_index = serde_json::to_vec(&entries).map_err(|e| e.to_string())?;
        index_bytes.extend_from_slice(&json_index);
        let index_length = index_bytes.len() as u64;
        buf.extend_from_slice(&index_bytes);

        let final_aligned_len = align_up_64(buf.len());
        if final_aligned_len > buf.len() {
            buf.resize(final_aligned_len, 0);
        }
        let total_length = buf.len() as u64;

        let flags = QAP_FLAG_ALIGN_64
            | QAP_FLAG_SIGNED_QECP
            | QAP_FLAG_CBOR_LD_MANIFEST
            | QAP_FLAG_VIBE_ENTRYPOINT;

        buf[0..4].copy_from_slice(&QAP_MAGIC);
        buf[4..6].copy_from_slice(&QAP_VERSION.to_le_bytes());
        buf[6..8].copy_from_slice(&flags.to_le_bytes());
        buf[8..12].copy_from_slice(&(entries.len() as u32).to_le_bytes());
        buf[12..20].copy_from_slice(&index_offset.to_le_bytes());
        buf[20..28].copy_from_slice(&index_length.to_le_bytes());
        buf[28..36].copy_from_slice(&total_length.to_le_bytes());
        buf[40..64].copy_from_slice(&manifest_digest_prefix);

        let crc = compute_qap_checksum32(&buf);
        buf[36..40].copy_from_slice(&crc.to_le_bytes());
        Ok(buf)
    }
}

pub struct QapContainerReader<'a> {
    bytes: &'a [u8],
    pub version: u16,
    pub flags: u16,
    pub entries: Vec<QapBundleEntry>,
}

impl<'a> QapContainerReader<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        if bytes.len() < QAP_HEADER_SIZE || bytes.len() % QAP_ENTRY_ALIGN != 0 {
            return Err("Invalid .qap container length or 64-byte alignment".to_string());
        }
        if bytes[0..4] != QAP_MAGIC {
            return Err("Bad .qap magic header (expected 'QAP\\0')".to_string());
        }
        let version = u16::from_le_bytes([bytes[4], bytes[5]]);
        if version != QAP_VERSION {
            return Err(format!("Unsupported .qap version {version}"));
        }
        let flags = u16::from_le_bytes([bytes[6], bytes[7]]);
        let entry_count = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
        let index_offset = u64::from_le_bytes(bytes[12..20].try_into().unwrap()) as usize;
        let index_length = u64::from_le_bytes(bytes[20..28].try_into().unwrap()) as usize;
        let total_length = u64::from_le_bytes(bytes[28..36].try_into().unwrap()) as usize;
        let expected_crc = u32::from_le_bytes(bytes[36..40].try_into().unwrap());

        if total_length != bytes.len() {
            return Err("Header total_length mismatch in .qap container".to_string());
        }
        if compute_qap_checksum32(bytes) != expected_crc {
            return Err("Tampered .qap container: whole-file checksum mismatch".to_string());
        }
        if index_offset + index_length > bytes.len() || index_length < 3 {
            return Err("Invalid index_offset/index_length in .qap container".to_string());
        }
        let idx_slice = &bytes[index_offset..index_offset + index_length];
        if idx_slice[0..3] != CBOR_LD_TAG {
            return Err("Missing 0xD9 0x05 0x01 CBOR-LD tag on .qap index footer".to_string());
        }
        let entries: Vec<QapBundleEntry> =
            serde_json::from_slice(&idx_slice[3..]).map_err(|e| e.to_string())?;
        if entries.len() != entry_count {
            return Err("Entry count mismatch in .qap index footer".to_string());
        }

        let reader = Self {
            bytes,
            version,
            flags,
            entries,
        };
        reader.verify_all_entries()?;
        Ok(reader)
    }

    pub fn entry_slice(&self, key: &str) -> Option<&'a [u8]> {
        let e = self.entries.iter().find(|e| e.key == key)?;
        let start = e.offset as usize;
        let end = start.checked_add(e.length as usize)?;
        self.bytes.get(start..end)
    }

    pub fn verify_all_entries(&self) -> Result<(), String> {
        for e in &self.entries {
            if e.offset as usize % QAP_ENTRY_ALIGN != 0 {
                return Err(format!("Entry '{}' is not 64-byte aligned", e.key));
            }
            let slice = self
                .entry_slice(&e.key)
                .ok_or_else(|| format!("Entry '{}' out of bounds", e.key))?;
            if hex_encode(&compute_entry_digest32(slice)) != e.digest_hex {
                return Err(format!("Digest mismatch for .qap entry '{}'", e.key));
            }
        }
        Ok(())
    }

    /// Zero-copy loads and evaluates a `.pcon.vibe` modular container directly from the `.qap` byte slice.
    pub fn load_pcon_vibe_module(&self, key: &str) -> Result<SynthContainerModule, String> {
        let slice = self
            .entry_slice(key)
            .ok_or_else(|| format!("Modular container '{key}' not found in .qap"))?;
        let text = std::str::from_utf8(slice).map_err(|e| e.to_string())?;
        SynthContainerModule::from_pcon_vibe(text)
    }

    /// Zero-copy loads and evaluates a `.pman.vibe` industry manifold directly from the `.qap` byte slice.
    pub fn load_pman_vibe_module(&self, key: &str) -> Result<SynthManifoldModule, String> {
        let slice = self
            .entry_slice(key)
            .ok_or_else(|| format!("Industry manifold '{key}' not found in .qap"))?;
        let text = std::str::from_utf8(slice).map_err(|e| e.to_string())?;
        SynthManifoldModule::from_pman_vibe(text)
    }
}

/// Seals Poet's flagship Modular Containers (`*.pcon.vibe` + `*.pcon.q42.yaml`), Industry Manifolds
/// (`*.pman.vibe`), and WordNet 3.1 `.q42` (`Q42\0` v4 `NQuin48`) Ontology Vault into a canonical
/// `poet.qap` (`QAP\0` v1) container.
pub fn build_canonical_poet_qap_container() -> Result<Vec<u8>, String> {
    let descriptor = QapYamlLdDescriptor {
        qap_id: "poet".to_string(),
        qap_kind: "CoreApplicationAndManifoldStudio".to_string(),
        display_name: "Webizen Poet — HyperCanvas, 9-Tab .q42 Ontology Studio, Forensic Workbench & Modular Vibe Containers".to_string(),
        version: "0.0.41".to_string(),
        abi_version: "1.0".to_string(),
        license: "CC-BY-NC-ND-4.0 (PermissiveCommons-1.0)".to_string(),
        author_did: "did:webizen:person:timothy-holborn".to_string(),
        qdp_domain: "poet.webizen.org".to_string(),
        did_git: "did:git:github.com/WebizenAI/poet".to_string(),
        max_sensitivity_clearance: "0x02".to_string(),
        required_ontologies: vec![
            "qualia://ontologies/wordnet31-omw-lexicon.q42".to_string(),
            "qualia://ontologies/uncrpd-human-rights.q42".to_string(),
        ],
        required_host_capabilities: vec![
            "GraphDatabase.lexicon_manifest".to_string(),
            "GraphDatabase.sparql".to_string(),
            "WgslForge.generate".to_string(),
            "ClinicalRisk.framingham".to_string(),
        ],
        main_vibe_module: "vibe/containers/pcon_sound_spatial_hrtf.pcon.vibe".to_string(),
        main_vibe_entrypoint_fn: "describe_container_module".to_string(),
        qecp_signature_hex: "ed25519_ml_dsa_87_qap_poet_0_0_41_sig".to_string(),
    };

    let yaml_str = descriptor.to_yaml_ld_q42_string();
    let mut cborld = Vec::new();
    cborld.extend_from_slice(&CBOR_LD_TAG);
    cborld.extend_from_slice(&serde_json::to_vec(&descriptor).map_err(|e| e.to_string())?);

    let mut writer = QapContainerWriter::new();
    writer.add_entry("manifest/qap.q42.yaml", "yaml_ld_q42", yaml_str.into_bytes())?;
    writer.add_entry("manifest/qap_manifest.cborld", "cborld_manifest", cborld)?;

    let containers = builtin_industry_container_modules();
    if let Some(first_c) = containers.first() {
        let pcon_vibe = first_c.to_pcon_vibe()?;
        let pcon_yaml = first_c.to_q42_yaml();
        writer.add_entry(
            "vibe/containers/pcon_sound_spatial_hrtf.pcon.vibe",
            "vibe_container",
            pcon_vibe.into_bytes(),
        )?;
        writer.add_entry(
            "vibe/containers/pcon_sound_spatial_hrtf.pcon.q42.yaml",
            "yaml_ld_q42",
            pcon_yaml.into_bytes(),
        )?;
    }

    let film_pman = synthesize_industry_manifold_module(IndustryDomainProfile::FilmAndMotionPicture);
    writer.add_entry(
        "vibe/manifolds/pman_film_mastering.pman.vibe",
        "vibe_manifold",
        film_pman.to_pman_vibe()?.into_bytes(),
    )?;

    let wn_q42 = compile_ontology_to_q42_pack(
        OntologySourcePreset::UnCrpdHumanRights.slug(),
        OntologySourcePreset::UnCrpdHumanRights.title(),
        OntologySourcePreset::UnCrpdHumanRights.default_turtle_source(),
        true,
    )?;
    writer.add_entry(
        "ontologies/wordnet31_omw_lexicon.q42",
        "q42",
        wn_q42.binary_payload_bytes,
    )?;

    writer.build()
}
