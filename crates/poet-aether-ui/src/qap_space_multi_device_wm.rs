//! Qap-Space Context Horizon Bar, 4-Mode Window Manager, Multi-Monitor Virtual Desktop,
//! Local Multi-Device Mesh (Desktop, Laptop, Tablet, Phone, TV), HDMI/Chromecast/Miracast/WebRTC
//! Display Streaming, and Human Input Device (HID) Forwarding Engine (`WP-POET-QAP-WM`).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//! License: CC BY-NC-ND 4.0.

use crate::store::Workbench;
use crate::wordnet_q42_ontology::deterministic_digest_hex;
use aether::prelude::*;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// 1. Active Qap-Space Context & Security Boundary
// ---------------------------------------------------------------------------

/// First-class Qap Space defining the active data vault, privacy posture,
/// capability lease boundary, and external sharing policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QapSpaceKind {
    /// Strictly private personal sanctuary — offline/LAN encrypted vault, zero external sharing.
    PersonalSanctuary,
    /// WellFair clinical, human-rights & advocacy casework — role-qualified Semantic Instruments & specialist leases.
    WellFairCasework,
    /// Cooperative multi-peer project space — Community Grounds, contribution ledgers & QualiaDB advancement.
    CooperativeGrounds,
    /// Public Permissive Commons studio — open ontologies (WordNet/Q42LEX), `.10d`/`.hmc` assets on Q-CDN.
    PermissiveCommonsStudio,
}

impl QapSpaceKind {
    pub const ALL: [Self; 4] = [
        Self::PersonalSanctuary,
        Self::WellFairCasework,
        Self::CooperativeGrounds,
        Self::PermissiveCommonsStudio,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "qap://space/personal-sanctuary",
            Self::WellFairCasework => "qap://space/wellfair-casework",
            Self::CooperativeGrounds => "qap://space/cooperative-grounds",
            Self::PermissiveCommonsStudio => "qap://space/permissive-commons",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "🛡️ Personal Sanctuary Space",
            Self::WellFairCasework => "🩺 WellFair Clinical & Advocacy Space",
            Self::CooperativeGrounds => "🤝 Cooperative Community Grounds",
            Self::PermissiveCommonsStudio => "🌐 Permissive Commons Studio",
        }
    }

    pub fn vault_filename(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "sanctuary-private.q42 (ML-KEM-1024 Encrypted)",
            Self::WellFairCasework => "wellfair-casework.q42 (Lease-Gated QECP)",
            Self::CooperativeGrounds => "cooperative-grounds.q42 (Co-Signed Multi-Peer)",
            Self::PermissiveCommonsStudio => "permissive-commons.q42 (Q-CDN Public Swarm)",
        }
    }

    pub fn network_posture(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "Local Sanctuary Mesh / WireGuard LAN Only",
            Self::WellFairCasework => "Strong-Auth Specialist Portal + Pocket Co-Signer",
            Self::CooperativeGrounds => "Authenticated Cooperative P2P Swarm",
            Self::PermissiveCommonsStudio => "Permissive Commons Q-CDN Distribution",
        }
    }

    pub fn orchestrator_tier(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "Tier-0 Sanctuary Priority · P0 Render / P1 Local Vault",
            Self::WellFairCasework => "Tier-1 Foreground Casework · P0 Render / P1 Symbolic Audit",
            Self::CooperativeGrounds => "Tier-1 Cooperative Studio · P1 Interactive / P2 Sync",
            Self::PermissiveCommonsStudio => "Tier-2 Commons & Elastic Burst · P1 Interactive / P3 Q-CDN",
        }
    }

    pub fn accent_color(self) -> &'static str {
        match self {
            Self::PersonalSanctuary => "#34d399",
            Self::WellFairCasework => "#a78bfa",
            Self::CooperativeGrounds => "#38bdf8",
            Self::PermissiveCommonsStudio => "#fbbf24",
        }
    }

    pub fn allows_public_qcdn_export(self) -> bool {
        matches!(
            self,
            Self::CooperativeGrounds | Self::PermissiveCommonsStudio
        )
    }
}

// ---------------------------------------------------------------------------
// 2. 4-Mode Window Manager Layout
// ---------------------------------------------------------------------------

/// Window arrangement mode for containers inside the active Qap Space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowLayoutMode {
    /// Infinite pan/zoom spatial canvas with draggable, resizable, port-wired containers.
    FreeformSpatialCanvas,
    /// Non-overlapping tiled split workstation (automatic 2-column / 4-quadrant grid).
    TiledSplitWorkstation,
    /// Active container maximized to 100% of stage with a top window-switcher tab bar.
    MaximizedFocusStage,
    /// Multi-display & multi-device split: main content routed to TV/Monitor while tools & HID run on Laptop/Tablet/Phone.
    MultiDisplayAndDeviceSplit,
}

impl WindowLayoutMode {
    pub const ALL: [Self; 4] = [
        Self::FreeformSpatialCanvas,
        Self::TiledSplitWorkstation,
        Self::MaximizedFocusStage,
        Self::MultiDisplayAndDeviceSplit,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::FreeformSpatialCanvas => "⊞ Freeform Canvas",
            Self::TiledSplitWorkstation => "◫ Tiled Split",
            Self::MaximizedFocusStage => "▣ Focus Maximized",
            Self::MultiDisplayAndDeviceSplit => "📺 Multi-Display & Device Cast",
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Multi-Monitor & Local Multi-Device Mesh (Phone, Tablet, Laptop, Desktop, TV)
// ---------------------------------------------------------------------------

/// Physical class of a personal device in the user's local mesh.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Tablet,
    Phone,
    TvOrProjector,
    Watch,
    HeadlessCompute,
}

impl DeviceType {
    pub const ALL: [Self; 7] = [
        Self::Desktop,
        Self::Laptop,
        Self::Tablet,
        Self::Phone,
        Self::TvOrProjector,
        Self::Watch,
        Self::HeadlessCompute,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop Workstation",
            Self::Laptop => "Laptop",
            Self::Tablet => "Tablet",
            Self::Phone => "Smartphone",
            Self::TvOrProjector => "TV / Large Display",
            Self::Watch => "Smartwatch",
            Self::HeadlessCompute => "Headless Compute Node",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Desktop => "🖥️",
            Self::Laptop => "💻",
            Self::Tablet => "📲",
            Self::Phone => "📱",
            Self::TvOrProjector => "📺",
            Self::Watch => "⌚",
            Self::HeadlessCompute => "🖧",
        }
    }
}

/// Functional role assigned to a device in the distributed Poet workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceRole {
    /// Full manifold canvas + primary editing containers.
    PrimaryCanvas,
    /// Extended monitor canvas — secondary containers placed on Monitor 2 / 3.
    SecondaryExtendedDisplay,
    /// Tool Chest, Property Inspector, CML Aura Tray, Ontology Explorer & HID Controller (Laptop or Tablet).
    ToolAndControlSurface,
    /// Compact 8-Sector Command Orb (◉), Pocket Co-Signer, Manifold Switcher & Touch/Voice HID Remote (Phone).
    CompactRemoteAndHid,
    /// High-resolution content presentation stream (TV / Projector via HDMI or Wireless Cast).
    ContentPresentationDisplay,
    /// Headless GPU/NPU node executing heavy `.q42` reasoning, 10D raymarching, or NLP indexing.
    ComputeOffloadNode,
}

impl DeviceRole {
    pub const ALL: [Self; 6] = [
        Self::PrimaryCanvas,
        Self::SecondaryExtendedDisplay,
        Self::ToolAndControlSurface,
        Self::CompactRemoteAndHid,
        Self::ContentPresentationDisplay,
        Self::ComputeOffloadNode,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::PrimaryCanvas => "Primary Canvas & Host",
            Self::SecondaryExtendedDisplay => "Extended Monitor Canvas",
            Self::ToolAndControlSurface => "Tools, Inspector & HID Controller",
            Self::CompactRemoteAndHid => "Command Orb Remote & Pocket Co-Signer",
            Self::ContentPresentationDisplay => "Main Content Stream (TV / Monitor)",
            Self::ComputeOffloadNode => "Local GPU / NPU Compute Node",
        }
    }
}

/// Physical or wireless display transport connecting a monitor or TV.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayTransportKind {
    /// Direct wired HDMI 2.1 / DisplayPort / USB-C Alt-Mode connection.
    HdmiOrDisplayPortWired,
    /// Wireless Google Cast / Chromecast receiver on the local LAN.
    WirelessCastChromecast,
    /// Wireless Miracast / Wi-Fi Direct / AirPlay display stream.
    WirelessCastMiracastAirPlay,
    /// Webizen P2P WebRTC / QUIC encrypted local LAN stream (zero-cloud).
    WebRtcLocalLanQuicStream,
    /// HbbTV / Smart-TV companion web endpoint on the local LAN.
    HbbTvCompanionBridge,
}

impl DisplayTransportKind {
    pub const ALL: [Self; 5] = [
        Self::HdmiOrDisplayPortWired,
        Self::WirelessCastChromecast,
        Self::WirelessCastMiracastAirPlay,
        Self::WebRtcLocalLanQuicStream,
        Self::HbbTvCompanionBridge,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::HdmiOrDisplayPortWired => "Wired HDMI 2.1 / DisplayPort / USB-C",
            Self::WirelessCastChromecast => "Wireless Cast (Chromecast / Google Cast LAN)",
            Self::WirelessCastMiracastAirPlay => "Wireless Display (Miracast / AirPlay / Wi-Fi Direct)",
            Self::WebRtcLocalLanQuicStream => "Webizen WebRTC / QUIC Encrypted Local LAN Stream",
            Self::HbbTvCompanionBridge => "HbbTV / Smart-TV Local Companion Receiver",
        }
    }
}

/// Physical or virtual display attached to a device in the workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayEndpoint {
    pub display_id: String,
    pub device_id: String,
    pub label: String,
    pub width_px: u32,
    pub height_px: u32,
    pub scale_factor: f32,
    pub virtual_x: i32,
    pub virtual_y: i32,
    pub is_primary: bool,
    pub transport: DisplayTransportKind,
    pub privacy_shield_hide_operator_chrome: bool,
}

/// A paired personal device in the user's cryptographically authenticated local device graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedPersonalDevice {
    pub device_id: String,
    pub label: String,
    pub device_type: DeviceType,
    pub assigned_role: DeviceRole,
    pub transport: DisplayTransportKind,
    pub crypto_key_id: String,
    pub lan_endpoint: String,
    pub online: bool,
    pub forwards_hid_input: bool,
    pub displays_tool_chest_and_inspector: bool,
    pub receives_main_content_stream: bool,
}

/// Assignment of a specific Poet container (`node.id`) to a target display and/or personal device.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerDisplayRoute {
    pub container_id: String,
    pub target_device_id: String,
    pub target_display_id: String,
    pub target_role: DeviceRole,
    pub transport: DisplayTransportKind,
    pub fullscreen_on_target: bool,
}

/// Human Input Device (HID) event packet forwarded from a Laptop, Tablet, or Phone controller
/// to a container being displayed on a TV or secondary monitor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HidInputForwardingPacket {
    pub packet_id: String,
    pub source_device_id: String,
    pub target_display_id: String,
    pub target_container_id: String,
    pub input_kind: String,
    pub payload_summary: String,
    pub signature_hex: String,
}

/// Signed workspace delta for synchronizing multi-display and multi-device state over the local mesh.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedWorkspaceSyncDelta {
    pub version: u64,
    pub author_device_id: String,
    pub qap_space: QapSpaceKind,
    pub summary: String,
    pub delta_digest_hex: String,
}

/// Complete state of the Qap-Space Window Manager & Multi-Display / Multi-Device Local Mesh.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QapSpaceMultiDeviceManager {
    pub active_qap_space: QapSpaceKind,
    pub layout_mode: WindowLayoutMode,
    pub active_preset_name: String,
    pub focused_container_id: Option<String>,
    pub minimized_container_ids: Vec<String>,
    pub detached_popout_container_ids: Vec<String>,
    pub displays: Vec<DisplayEndpoint>,
    pub devices: Vec<PairedPersonalDevice>,
    pub container_routes: Vec<ContainerDisplayRoute>,
    pub hid_forwarding_log: Vec<HidInputForwardingPacket>,
    pub sync_deltas: Vec<SignedWorkspaceSyncDelta>,
    pub sync_version: u64,
    pub show_device_mesh_drawer: bool,
}

impl Default for QapSpaceMultiDeviceManager {
    fn default() -> Self {
        Self::seeded_local_mesh()
    }
}

impl QapSpaceMultiDeviceManager {
    /// Seeds a realistic multi-monitor + multi-device local mesh covering:
    /// 1. Primary Workstation Monitor (`DISP-1-MAIN`, 3840×2160)
    /// 2. Secondary Extended Monitor (`DISP-2-LEFT`, 2560×1440)
    /// 3. Living-Room / Office 4K TV (`DISP-3-TV`, 3840×2160, switchable between HDMI 2.1 and Chromecast / WebRTC Wireless Cast)
    /// 4. Personal Laptop (`DEV-LAPTOP-01` — Tool Surface, Inspector & Keyboard/Trackpad HID Controller)
    /// 5. Personal Tablet (`DEV-TABLET-02` — Touch Tool Chest, Aura Tray & Property Sheet Surface)
    /// 6. Personal Smartphone (`DEV-PHONE-03` — 8-Sector Command Orb `◉` Remote, Pocket Co-Signer & Voice/Touch HID)
    pub fn seeded_local_mesh() -> Self {
        let displays = vec![
            DisplayEndpoint {
                display_id: "DISP-1-MAIN".into(),
                device_id: "did:q42:device:workstation-01".into(),
                label: "Monitor 1 — Primary 4K Studio Display".into(),
                width_px: 3840,
                height_px: 2160,
                scale_factor: 1.5,
                virtual_x: 0,
                virtual_y: 0,
                is_primary: true,
                transport: DisplayTransportKind::HdmiOrDisplayPortWired,
                privacy_shield_hide_operator_chrome: false,
            },
            DisplayEndpoint {
                display_id: "DISP-2-SIDE".into(),
                device_id: "did:q42:device:workstation-01".into(),
                label: "Monitor 2 — Extended Side Monitor (QHD)".into(),
                width_px: 2560,
                height_px: 1440,
                scale_factor: 1.25,
                virtual_x: 3840,
                virtual_y: 0,
                is_primary: false,
                transport: DisplayTransportKind::HdmiOrDisplayPortWired,
                privacy_shield_hide_operator_chrome: false,
            },
            DisplayEndpoint {
                display_id: "DISP-3-TV".into(),
                device_id: "did:q42:device:livingroom-tv-04".into(),
                label: "TV Display — 4K Smart TV (HDMI / Chromecast / WebRTC)".into(),
                width_px: 3840,
                height_px: 2160,
                scale_factor: 1.0,
                virtual_x: -3840,
                virtual_y: 0,
                is_primary: false,
                transport: DisplayTransportKind::WirelessCastChromecast,
                privacy_shield_hide_operator_chrome: true,
            },
            DisplayEndpoint {
                display_id: "DISP-4-LAPTOP".into(),
                device_id: "did:q42:device:laptop-02".into(),
                label: "Laptop Built-in Screen (Tools & HID Controller)".into(),
                width_px: 2560,
                height_px: 1600,
                scale_factor: 1.5,
                virtual_x: 0,
                virtual_y: 2160,
                is_primary: false,
                transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                privacy_shield_hide_operator_chrome: false,
            },
            DisplayEndpoint {
                display_id: "DISP-5-TABLET".into(),
                device_id: "did:q42:device:tablet-03".into(),
                label: "Tablet Touch Surface (Tool Chest & Inspector)".into(),
                width_px: 2048,
                height_px: 1536,
                scale_factor: 2.0,
                virtual_x: 2560,
                virtual_y: 2160,
                is_primary: false,
                transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                privacy_shield_hide_operator_chrome: false,
            },
        ];

        let devices = vec![
            PairedPersonalDevice {
                device_id: "did:q42:device:workstation-01".into(),
                label: "Primary Desktop Workstation (Etalon GPU)".into(),
                device_type: DeviceType::Desktop,
                assigned_role: DeviceRole::PrimaryCanvas,
                transport: DisplayTransportKind::HdmiOrDisplayPortWired,
                crypto_key_id: "did:q42:key:ed25519-mldsa65-ws01".into(),
                lan_endpoint: "10.42.0.10:4242 (WireGuard LAN)".into(),
                online: true,
                forwards_hid_input: true,
                displays_tool_chest_and_inspector: true,
                receives_main_content_stream: true,
            },
            PairedPersonalDevice {
                device_id: "did:q42:device:laptop-02".into(),
                label: "Personal Laptop (Tools, Notes & Keyboard/Trackpad HID)".into(),
                device_type: DeviceType::Laptop,
                assigned_role: DeviceRole::ToolAndControlSurface,
                transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                crypto_key_id: "did:q42:key:ed25519-mldsa65-lap02".into(),
                lan_endpoint: "10.42.0.11:4242 (WireGuard LAN)".into(),
                online: true,
                forwards_hid_input: true,
                displays_tool_chest_and_inspector: true,
                receives_main_content_stream: false,
            },
            PairedPersonalDevice {
                device_id: "did:q42:device:tablet-03".into(),
                label: "Personal Tablet (Touch Tool Chest, Aura Tray & Inspector)".into(),
                device_type: DeviceType::Tablet,
                assigned_role: DeviceRole::ToolAndControlSurface,
                transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                crypto_key_id: "did:q42:key:ed25519-mldsa65-tab03".into(),
                lan_endpoint: "10.42.0.12:4242 (WireGuard LAN)".into(),
                online: true,
                forwards_hid_input: true,
                displays_tool_chest_and_inspector: true,
                receives_main_content_stream: false,
            },
            PairedPersonalDevice {
                device_id: "did:q42:device:livingroom-tv-04".into(),
                label: "4K Smart TV (Main Content Stream — HDMI / Chromecast)".into(),
                device_type: DeviceType::TvOrProjector,
                assigned_role: DeviceRole::ContentPresentationDisplay,
                transport: DisplayTransportKind::WirelessCastChromecast,
                crypto_key_id: "did:q42:key:ed25519-mldsa65-tv04".into(),
                lan_endpoint: "10.42.0.20:8009 (Local Cast / HDMI Bridge)".into(),
                online: true,
                forwards_hid_input: false,
                displays_tool_chest_and_inspector: false,
                receives_main_content_stream: true,
            },
            PairedPersonalDevice {
                device_id: "did:q42:device:phone-05".into(),
                label: "Smartphone (8-Sector Command Orb ◉ & Pocket Co-Signer)".into(),
                device_type: DeviceType::Phone,
                assigned_role: DeviceRole::CompactRemoteAndHid,
                transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                crypto_key_id: "did:q42:key:ed25519-mldsa65-phn05".into(),
                lan_endpoint: "10.42.0.15:4242 (AVF Sanctuary Mesh)".into(),
                online: true,
                forwards_hid_input: true,
                displays_tool_chest_and_inspector: true,
                receives_main_content_stream: false,
            },
        ];

        let mut mgr = Self {
            active_qap_space: QapSpaceKind::PersonalSanctuary,
            layout_mode: WindowLayoutMode::FreeformSpatialCanvas,
            active_preset_name: "Multi-Monitor Studio + Tablet/Phone Companion".into(),
            focused_container_id: None,
            minimized_container_ids: Vec::new(),
            detached_popout_container_ids: Vec::new(),
            displays,
            devices,
            container_routes: Vec::new(),
            hid_forwarding_log: Vec::new(),
            sync_deltas: Vec::new(),
            sync_version: 1,
            show_device_mesh_drawer: false,
        };
        mgr.record_sync_delta(
            "did:q42:device:workstation-01",
            "Initialized 5-device local mesh (Workstation, Laptop, Tablet, 4K TV, Phone) with signed DID key chain",
        );
        mgr
    }

    /// Switches the active Qap Space and records a signed workspace sync delta.
    pub fn switch_qap_space(&mut self, space: QapSpaceKind) {
        self.active_qap_space = space;
        self.record_sync_delta(
            "did:q42:device:workstation-01",
            &format!("Switched Qap Space to {} ({})", space.label(), space.vault_filename()),
        );
    }

    /// Applies one of the user's core multi-device / TV / tablet orchestration presets:
    /// 1. `"laptop-hid-tv-stream"`: Streams main content container(s) to the 4K TV (via HDMI or Chromecast/WebRTC)
    ///    while configuring the Laptop as the Tool Chest, Inspector, Notes & Keyboard/Trackpad HID controller!
    /// 2. `"tablet-tools-main-monitor"`: Places the main canvas on the Desktop Monitor while offloading
    ///    the Tool Chest, CML Aura Tray, and Inspector to the user's Tablet and Phone!
    /// 3. `"triple-monitor-span"`: Spans containers across Monitor 1 (Primary), Monitor 2 (Extended), and Monitor 3 / TV.
    pub fn apply_multi_device_preset(&mut self, preset_slug: &str, wb: &mut Workbench) {
        match preset_slug {
            "laptop-hid-tv-stream" => {
                self.layout_mode = WindowLayoutMode::MultiDisplayAndDeviceSplit;
                self.active_preset_name =
                    "TV Main Content Stream (HDMI/Cast) + Laptop Tools & HID Input".into();
                self.container_routes.clear();
                for (idx, node) in wb.nodes.iter().enumerate() {
                    if idx == 0 {
                        // Primary content container streams full-screen to the TV
                        self.container_routes.push(ContainerDisplayRoute {
                            container_id: node.id.clone(),
                            target_device_id: "did:q42:device:livingroom-tv-04".into(),
                            target_display_id: "DISP-3-TV".into(),
                            target_role: DeviceRole::ContentPresentationDisplay,
                            transport: self.tv_transport(),
                            fullscreen_on_target: true,
                        });
                    } else {
                        // Supporting ontology / notes / inspector containers stay on the Laptop control surface
                        self.container_routes.push(ContainerDisplayRoute {
                            container_id: node.id.clone(),
                            target_device_id: "did:q42:device:laptop-02".into(),
                            target_display_id: "DISP-4-LAPTOP".into(),
                            target_role: DeviceRole::ToolAndControlSurface,
                            transport: DisplayTransportKind::WebRtcLocalLanQuicStream,
                            fullscreen_on_target: false,
                        });
                    }
                }
                self.forward_hid_input(
                    "did:q42:device:laptop-02",
                    "DISP-3-TV",
                    wb.nodes
                        .first()
                        .map(|n| n.id.as_str())
                        .unwrap_or("main-stage"),
                    "Keyboard + Trackpad + Tool Selection HID Stream",
                    "Laptop HID controller bound to TV Main Content Stream (60fps low-latency LAN channel)",
                );
                wb.note("Activated TV Main Content Stream + Laptop Tools & HID Input Controller");
            }
            "tablet-tools-main-monitor" => {
                self.layout_mode = WindowLayoutMode::MultiDisplayAndDeviceSplit;
                self.active_preset_name =
                    "Main Monitor Content + Tablet/Phone Tool Chest & Inspector Companion".into();
                self.container_routes.clear();
                for node in &wb.nodes {
                    self.container_routes.push(ContainerDisplayRoute {
                        container_id: node.id.clone(),
                        target_device_id: "did:q42:device:workstation-01".into(),
                        target_display_id: "DISP-1-MAIN".into(),
                        target_role: DeviceRole::PrimaryCanvas,
                        transport: DisplayTransportKind::HdmiOrDisplayPortWired,
                        fullscreen_on_target: false,
                    });
                }
                self.forward_hid_input(
                    "did:q42:device:tablet-03",
                    "DISP-1-MAIN",
                    wb.selected.as_deref().unwrap_or("active-container"),
                    "Touch Stylus + Tool Palette + CML Aura Tagging HID",
                    "Tablet & Phone displaying Tool Chest, Property Sheet & 8-Sector Command Orb (◉) for Main Monitor",
                );
                wb.note("Activated Main Monitor Content + Tablet/Phone Tool & Inspector Surface");
            }
            "triple-monitor-span" => {
                self.layout_mode = WindowLayoutMode::FreeformSpatialCanvas;
                self.active_preset_name =
                    "Multi-Monitor Desktop Span (Monitor 1 + Monitor 2 + TV)".into();
                self.container_routes.clear();
                let display_cycle = ["DISP-1-MAIN", "DISP-2-SIDE", "DISP-3-TV"];
                for (idx, node) in wb.nodes.iter().enumerate() {
                    let target_disp = display_cycle[idx % display_cycle.len()];
                    let target_dev = if target_disp == "DISP-3-TV" {
                        "did:q42:device:livingroom-tv-04"
                    } else {
                        "did:q42:device:workstation-01"
                    };
                    self.container_routes.push(ContainerDisplayRoute {
                        container_id: node.id.clone(),
                        target_device_id: target_dev.into(),
                        target_display_id: target_disp.into(),
                        target_role: if idx == 0 {
                            DeviceRole::PrimaryCanvas
                        } else {
                            DeviceRole::SecondaryExtendedDisplay
                        },
                        transport: if target_disp == "DISP-3-TV" {
                            self.tv_transport()
                        } else {
                            DisplayTransportKind::HdmiOrDisplayPortWired
                        },
                        fullscreen_on_target: false,
                    });
                }
                wb.note("Spanned workspace containers across Monitor 1, Monitor 2, and TV Display");
            }
            _ => {}
        }
        self.record_sync_delta(
            "did:q42:device:workstation-01",
            &format!("Applied multi-device preset `{preset_slug}` ({} routes)", self.container_routes.len()),
        );
    }

    /// Toggles the TV connection transport between Wired HDMI 2.1, Wireless Chromecast,
    /// Wireless Miracast/AirPlay, WebRTC Local LAN Stream, and HbbTV Companion.
    pub fn cycle_tv_transport(&mut self) {
        let next = match self.tv_transport() {
            DisplayTransportKind::HdmiOrDisplayPortWired => {
                DisplayTransportKind::WirelessCastChromecast
            }
            DisplayTransportKind::WirelessCastChromecast => {
                DisplayTransportKind::WirelessCastMiracastAirPlay
            }
            DisplayTransportKind::WirelessCastMiracastAirPlay => {
                DisplayTransportKind::WebRtcLocalLanQuicStream
            }
            DisplayTransportKind::WebRtcLocalLanQuicStream => {
                DisplayTransportKind::HbbTvCompanionBridge
            }
            DisplayTransportKind::HbbTvCompanionBridge => {
                DisplayTransportKind::HdmiOrDisplayPortWired
            }
        };
        for d in &mut self.displays {
            if d.display_id == "DISP-3-TV" {
                d.transport = next;
            }
        }
        for dev in &mut self.devices {
            if dev.device_type == DeviceType::TvOrProjector {
                dev.transport = next;
            }
        }
        for route in &mut self.container_routes {
            if route.target_display_id == "DISP-3-TV" {
                route.transport = next;
            }
        }
        self.record_sync_delta(
            "did:q42:device:workstation-01",
            &format!("Switched TV Display transport to {}", next.label()),
        );
    }

    pub fn tv_transport(&self) -> DisplayTransportKind {
        self.displays
            .iter()
            .find(|d| d.display_id == "DISP-3-TV")
            .map(|d| d.transport)
            .unwrap_or(DisplayTransportKind::WirelessCastChromecast)
    }

    /// Routes a single container (`container_id`) to the next display in the mesh
    /// (`DISP-1-MAIN` -> `DISP-2-SIDE` -> `DISP-3-TV` -> `DISP-4-LAPTOP` -> `DISP-5-TABLET`).
    pub fn cycle_container_display_route(&mut self, container_id: &str) -> ContainerDisplayRoute {
        let display_ids = [
            ("DISP-1-MAIN", "did:q42:device:workstation-01", DeviceRole::PrimaryCanvas),
            ("DISP-2-SIDE", "did:q42:device:workstation-01", DeviceRole::SecondaryExtendedDisplay),
            ("DISP-3-TV", "did:q42:device:livingroom-tv-04", DeviceRole::ContentPresentationDisplay),
            ("DISP-4-LAPTOP", "did:q42:device:laptop-02", DeviceRole::ToolAndControlSurface),
            ("DISP-5-TABLET", "did:q42:device:tablet-03", DeviceRole::ToolAndControlSurface),
        ];

        let current_idx = self
            .container_routes
            .iter()
            .find(|r| r.container_id == container_id)
            .and_then(|r| {
                display_ids
                    .iter()
                    .position(|(d_id, _, _)| *d_id == r.target_display_id)
            })
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % display_ids.len();
        let (disp_id, dev_id, role) = display_ids[next_idx];
        let transport = self
            .displays
            .iter()
            .find(|d| d.display_id == disp_id)
            .map(|d| d.transport)
            .unwrap_or(DisplayTransportKind::HdmiOrDisplayPortWired);

        let route = ContainerDisplayRoute {
            container_id: container_id.to_string(),
            target_device_id: dev_id.to_string(),
            target_display_id: disp_id.to_string(),
            target_role: role,
            transport,
            fullscreen_on_target: disp_id == "DISP-3-TV",
        };
        self.container_routes
            .retain(|r| r.container_id != container_id);
        self.container_routes.push(route.clone());
        self.record_sync_delta(
            "did:q42:device:workstation-01",
            &format!("Routed container `{container_id}` to display `{disp_id}` ({})", transport.label()),
        );
        route
    }

    pub fn route_for_container(&self, container_id: &str) -> Option<&ContainerDisplayRoute> {
        self.container_routes
            .iter()
            .find(|r| r.container_id == container_id)
    }

    /// Minimizes or restores a container window in the active Qap Space taskbar shelf.
    pub fn toggle_minimize_container(&mut self, container_id: &str) {
        if let Some(pos) = self
            .minimized_container_ids
            .iter()
            .position(|id| id == container_id)
        {
            self.minimized_container_ids.remove(pos);
        } else {
            self.minimized_container_ids.push(container_id.to_string());
        }
    }

    pub fn is_minimized(&self, container_id: &str) -> bool {
        self.minimized_container_ids
            .iter()
            .any(|id| id == container_id)
    }

    /// Toggles detached native multi-window popout state for a container.
    pub fn toggle_popout_container(&mut self, container_id: &str) {
        if let Some(pos) = self
            .detached_popout_container_ids
            .iter()
            .position(|id| id == container_id)
        {
            self.detached_popout_container_ids.remove(pos);
        } else {
            self.detached_popout_container_ids
                .push(container_id.to_string());
        }
    }

    pub fn is_popped_out(&self, container_id: &str) -> bool {
        self.detached_popout_container_ids
            .iter()
            .any(|id| id == container_id)
    }

    /// Tiles all visible (non-minimized) containers on the active Workbench into a clean
    /// side-by-side 2-column workstation grid without overlapping windows.
    pub fn tile_workbench_containers(&mut self, wb: &mut Workbench) {
        self.layout_mode = WindowLayoutMode::TiledSplitWorkstation;
        let visible_ids: Vec<String> = wb
            .nodes
            .iter()
            .filter(|n| !self.is_minimized(&n.id))
            .map(|n| n.id.clone())
            .collect();
        for (idx, id) in visible_ids.iter().enumerate() {
            let col = idx % 2;
            let row = idx / 2;
            if let Some(node) = wb.nodes.iter_mut().find(|n| &n.id == id) {
                node.x = 24.0 + (col as f64) * 560.0;
                node.y = 24.0 + (row as f64) * 380.0;
                node.width = 536.0;
                node.height = 356.0;
            }
        }
        wb.pan_x = 0.0;
        wb.pan_y = 0.0;
        wb.zoom = 1.0;
        wb.note(format!(
            "Tiled {} windows in 2-column split workstation layout",
            visible_ids.len()
        ));
    }

    /// Records a signed Human Input Device (HID) forwarding packet from a personal device
    /// (Laptop, Tablet, Phone) controlling a container on another screen (such as a TV or Monitor).
    pub fn forward_hid_input(
        &mut self,
        source_device_id: &str,
        target_display_id: &str,
        target_container_id: &str,
        input_kind: &str,
        payload_summary: &str,
    ) -> HidInputForwardingPacket {
        let packet_id = format!("HID-{:04}", self.hid_forwarding_log.len() + 1);
        let canonical = format!(
            "{packet_id}|{source_device_id}|{target_display_id}|{target_container_id}|{input_kind}|{payload_summary}"
        );
        let signature_hex = deterministic_digest_hex(canonical.as_bytes());
        let pkt = HidInputForwardingPacket {
            packet_id,
            source_device_id: source_device_id.to_string(),
            target_display_id: target_display_id.to_string(),
            target_container_id: target_container_id.to_string(),
            input_kind: input_kind.to_string(),
            payload_summary: payload_summary.to_string(),
            signature_hex,
        };
        self.hid_forwarding_log.push(pkt.clone());
        pkt
    }

    fn record_sync_delta(&mut self, author_device_id: &str, summary: &str) {
        self.sync_version += 1;
        let payload = format!(
            "DELTA|v{}|{}|{:?}|{}",
            self.sync_version, author_device_id, self.active_qap_space, summary
        );
        let delta_digest_hex = deterministic_digest_hex(payload.as_bytes());
        self.sync_deltas.push(SignedWorkspaceSyncDelta {
            version: self.sync_version,
            author_device_id: author_device_id.to_string(),
            qap_space: self.active_qap_space,
            summary: summary.to_string(),
            delta_digest_hex,
        });
    }
}

// ---------------------------------------------------------------------------
// 4. Aether RSX Components: QapSpaceHorizonBar, MultiDeviceMeshDrawer & WindowTaskbarShelf
// ---------------------------------------------------------------------------

#[component]
/// Always-visible Qap-Space Context Horizon Bar, Breadcrumb Trail, 4-Mode Window Manager Switcher,
/// and Multi-Display / Multi-Device Local Mesh Bar mounted at the top of Poet.
pub fn QapSpaceHorizonBar(
    wb: Signal<Workbench>,
    wm: Signal<QapSpaceMultiDeviceManager>,
) -> Element {
    let current_wm = wm();
    let current_wb = wb();
    let space = current_wm.active_qap_space;
    let accent = space.accent_color();
    let selected_title = current_wb
        .selected
        .as_ref()
        .and_then(|id| current_wb.node(id))
        .map(|n| n.title.clone())
        .unwrap_or_else(|| "All Manifold Windows".to_string());

    rsx! {
        div {
            class: "qap-space-horizon-bar",
            "data-qap-space": "{space.slug()}",
            style: "background:#070b14;border-bottom:1px solid #1e293b;padding:5px 12px;display:flex;flex-direction:column;gap:5px;font-size:11px;color:#e2e8f0;",

            // Row 1: Qap-Space Switcher + Active Vault & Privacy Boundary + Window Layout Mode + Multi-Device Cast Toggle
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;",
                // Left: Qap-Space Selector Pills
                div { style: "display:flex;align-items:center;gap:4px;flex-wrap:wrap;",
                    span { style: "color:#94a3b8;font-weight:700;text-transform:uppercase;font-size:10px;letter-spacing:0.05em;margin-right:4px;",
                        "QAP SPACE:"
                    }
                    for sp in QapSpaceKind::ALL {
                        button {
                            r#type: "button",
                            style: if space == sp {
                                format!(
                                    "background:{};color:#090d16;border:none;border-radius:4px;padding:3px 8px;font-size:11px;font-weight:700;cursor:pointer;",
                                    sp.accent_color()
                                )
                            } else {
                                "background:#111827;color:#94a3b8;border:1px solid #1e293b;border-radius:4px;padding:3px 8px;font-size:11px;cursor:pointer;".to_string()
                            },
                            onclick: move |_| {
                                let mut m = wm();
                                m.switch_qap_space(sp);
                                wm.set(m);
                                let mut w = wb();
                                w.note(format!("Entered {} · Vault: {}", sp.label(), sp.vault_filename()));
                                wb.set(w);
                            },
                            "{sp.label()}"
                        }
                    }
                }

                // Center: Vault & Network Boundary Pill
                div { style: "display:flex;align-items:center;gap:6px;flex-wrap:wrap;",
                    span {
                        style: format!(
                            "background:#0f172a;color:{accent};border:1px solid {accent};border-radius:4px;padding:2px 8px;font-family:monospace;font-size:10px;"
                        ),
                        "Vault: {space.vault_filename()}"
                    }
                    span {
                        style: "background:#0f172a;color:#38bdf8;border:1px solid #1e293b;border-radius:4px;padding:2px 8px;font-size:10px;",
                        "{space.network_posture()}"
                    }
                }

                // Right: 4-Mode Window Manager & Multi-Device Mesh Drawer Toggle
                div { style: "display:flex;align-items:center;gap:4px;flex-wrap:wrap;",
                    for mode in WindowLayoutMode::ALL {
                        button {
                            r#type: "button",
                            style: if current_wm.layout_mode == mode {
                                "background:#38bdf8;color:#090d16;border:none;border-radius:4px;padding:3px 8px;font-size:10px;font-weight:700;cursor:pointer;"
                            } else {
                                "background:#111827;color:#cbd5e1;border:1px solid #1e293b;border-radius:4px;padding:3px 8px;font-size:10px;cursor:pointer;"
                            },
                            onclick: move |_| {
                                let mut m = wm();
                                m.layout_mode = mode;
                                if mode == WindowLayoutMode::TiledSplitWorkstation {
                                    let mut w = wb();
                                    m.tile_workbench_containers(&mut w);
                                    wb.set(w);
                                } else if mode == WindowLayoutMode::MaximizedFocusStage {
                                    let mut w = wb();
                                    if m.focused_container_id.is_none() {
                                        m.focused_container_id = w.selected.clone().or_else(|| w.nodes.first().map(|n| n.id.clone()));
                                    }
                                    w.note("Entered Focus Maximized Window Mode");
                                    wb.set(w);
                                } else if mode == WindowLayoutMode::MultiDisplayAndDeviceSplit {
                                    m.show_device_mesh_drawer = true;
                                }
                                wm.set(m);
                            },
                            "{mode.label()}"
                        }
                    }
                    button {
                        r#type: "button",
                        style: if current_wm.show_device_mesh_drawer {
                            "background:#a78bfa;color:#090d16;border:none;border-radius:4px;padding:3px 9px;font-size:10px;font-weight:700;cursor:pointer;"
                        } else {
                            "background:#1e1b4b;color:#c4b5fd;border:1px solid #4c1d95;border-radius:4px;padding:3px 9px;font-size:10px;font-weight:600;cursor:pointer;"
                        },
                        onclick: move |_| {
                            let mut m = wm();
                            m.show_device_mesh_drawer = !m.show_device_mesh_drawer;
                            wm.set(m);
                        },
                        "🖥️📱📺 Screens & Devices ({current_wm.displays.len()} displays · {current_wm.devices.len()} devices)"
                    }
                }
            }

            // Row 2: Breadcrumb Horizon Trail & Active Multi-Device Preset Indicator
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;color:#94a3b8;font-size:10px;",
                div { style: "display:flex;align-items:center;gap:5px;flex-wrap:wrap;",
                    span { style: "color:#64748b;", "HORIZON:" }
                    span { style: "color:#cbd5e1;font-weight:600;", "Webizen OS" }
                    span { "▸" }
                    span { style: format!("color:{accent};font-weight:700;"), "{space.label()}" }
                    span { "▸" }
                    span { style: "color:#38bdf8;font-weight:600;", "Poet HyperCanvas" }
                    span { "▸" }
                    span { style: "color:#e2e8f0;", "{current_wb.active.icon()} {current_wb.active.title()}" }
                    span { "▸" }
                    span { style: "color:#fbbf24;font-weight:600;", "{selected_title}" }
                }
                div { style: "display:flex;align-items:center;gap:8px;",
                    span { style: "font-family:monospace;color:#34d399;",
                        "Mesh Sync v{current_wm.sync_version} · {current_wm.active_preset_name}"
                    }
                    span { style: "font-family:monospace;color:#64748b;",
                        "{space.orchestrator_tier()}"
                    }
                }
            }

            // Optional Expandable Multi-Monitor, TV Casting & Personal Device Mesh Drawer
            if current_wm.show_device_mesh_drawer {
                MultiDisplayAndDeviceMeshDrawer { wb, wm }
            }
        }
    }
}

#[component]
/// Interactive drawer for managing Multiple Monitors, Wired HDMI / Wireless Chromecast / Miracast / WebRTC TV Casting,
/// Tablet/Phone Tool Chest & Inspector Companions, and Laptop HID Input Forwarding.
pub fn MultiDisplayAndDeviceMeshDrawer(
    wb: Signal<Workbench>,
    wm: Signal<QapSpaceMultiDeviceManager>,
) -> Element {
    let current_wm = wm();
    let tv_transport = current_wm.tv_transport();

    rsx! {
        div {
            style: "background:#0d1322;border:1px solid #334155;border-radius:6px;padding:10px;margin-top:4px;display:grid;gap:10px;",
            div { style: "display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;",
                div {
                    strong { style: "color:#38bdf8;font-size:12px;",
                        "🖥️📱📺 Multi-Monitor, Personal Device Mesh & TV Stream Orchestrator (Local WireGuard / WebRTC / HDMI / Cast)"
                    }
                    div { style: "color:#94a3b8;font-size:10px;",
                        "Route main content to a TV or primary monitor while using your Laptop, Tablet, or Phone for authoring tools, inspector panels, and human input (HID)."
                    }
                }
                div { style: "display:flex;gap:6px;flex-wrap:wrap;",
                    button {
                        r#type: "button",
                        style: "background:#0284c7;color:#ffffff;border:none;border-radius:4px;padding:4px 9px;font-size:10px;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let mut m = wm();
                            let mut w = wb();
                            m.apply_multi_device_preset("laptop-hid-tv-stream", &mut w);
                            wm.set(m);
                            wb.set(w);
                        },
                        "📺+💻 Preset 1: Stream Content to TV + Laptop Tools & HID"
                    }
                    button {
                        r#type: "button",
                        style: "background:#7c3aed;color:#ffffff;border:none;border-radius:4px;padding:4px 9px;font-size:10px;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let mut m = wm();
                            let mut w = wb();
                            m.apply_multi_device_preset("tablet-tools-main-monitor", &mut w);
                            wm.set(m);
                            wb.set(w);
                        },
                        "🖥️+📲 Preset 2: Main Monitor Content + Tablet/Phone Tool Surface"
                    }
                    button {
                        r#type: "button",
                        style: "background:#059669;color:#ffffff;border:none;border-radius:4px;padding:4px 9px;font-size:10px;font-weight:700;cursor:pointer;",
                        onclick: move |_| {
                            let mut m = wm();
                            let mut w = wb();
                            m.apply_multi_device_preset("triple-monitor-span", &mut w);
                            wm.set(m);
                            wb.set(w);
                        },
                        "🖥️🖥️📺 Preset 3: Span Across Monitor 1 + Monitor 2 + TV"
                    }
                    button {
                        r#type: "button",
                        style: "background:#1e293b;color:#fbbf24;border:1px solid #475569;border-radius:4px;padding:4px 9px;font-size:10px;font-weight:600;cursor:pointer;",
                        onclick: move |_| {
                            let mut m = wm();
                            m.cycle_tv_transport();
                            wm.set(m);
                        },
                        "🔄 TV Connection: {tv_transport.label()}"
                    }
                }
            }

            // Physical Displays & TV Endpoints Map
            div { style: "display:grid;grid-template-columns:repeat(auto-fit,minmax(215px,1fr));gap:8px;",
                for disp in current_wm.displays.iter() {
                    {
                        let disp_id = disp.display_id.clone();
                        let routed_count = current_wm
                            .container_routes
                            .iter()
                            .filter(|r| r.target_display_id == disp.display_id)
                            .count();
                        rsx! {
                            div {
                                key: "{disp.display_id}",
                                style: "background:#131c2e;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:3px;",
                                div { style: "display:flex;justify-content:space-between;align-items:center;",
                                    strong { style: "color:#38bdf8;font-size:11px;", "{disp.label}" }
                                    span { style: "background:#090d16;color:#34d399;padding:1px 5px;border-radius:3px;font-family:monospace;font-size:9px;",
                                        "{disp.width_px}×{disp.height_px}"
                                    }
                                }
                                div { style: "color:#94a3b8;font-size:10px;", "Transport: {disp.transport.label()}" }
                                div { style: "display:flex;justify-content:space-between;align-items:center;margin-top:2px;",
                                    span { style: "color:#cbd5e1;font-size:10px;",
                                        "Routed Windows: {routed_count} · Pos: ({disp.virtual_x}, {disp.virtual_y})"
                                    }
                                    button {
                                        r#type: "button",
                                        style: "background:#1e293b;color:#38bdf8;border:1px solid #334155;border-radius:3px;padding:2px 6px;font-size:9px;cursor:pointer;",
                                        onclick: move |_| {
                                            let mut m = wm();
                                            let mut w = wb();
                                            if let Some(sel) = w.selected.clone().or_else(|| w.nodes.first().map(|n| n.id.clone())) {
                                                m.container_routes.retain(|r| r.container_id != sel);
                                                m.container_routes.push(ContainerDisplayRoute {
                                                    container_id: sel.clone(),
                                                    target_device_id: "did:q42:device:workstation-01".into(),
                                                    target_display_id: disp_id.clone(),
                                                    target_role: DeviceRole::ContentPresentationDisplay,
                                                    transport: m.tv_transport(),
                                                    fullscreen_on_target: disp_id == "DISP-3-TV",
                                                });
                                                w.note(format!("Sent window `{sel}` to `{disp_id}`"));
                                                wm.set(m);
                                                wb.set(w);
                                            }
                                        },
                                        "Send Active Window Here"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Paired Personal Devices & HID Input Forwarding Row
            div { style: "display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:8px;",
                for dev in current_wm.devices.iter() {
                    {
                        let dev_id = dev.device_id.clone();
                        let dev_label = dev.label.clone();
                        rsx! {
                            div {
                                key: "{dev.device_id}",
                                style: "background:#111827;border:1px solid #1e293b;border-radius:5px;padding:8px;display:grid;gap:3px;",
                                div { style: "display:flex;justify-content:space-between;align-items:center;",
                                    strong { style: "color:#e2e8f0;font-size:11px;", "{dev.device_type.icon()} {dev.label}" }
                                    span { style: "color:#34d399;font-size:9px;font-family:monospace;", "● ONLINE" }
                                }
                                div { style: "color:#a78bfa;font-size:10px;font-weight:600;", "Role: {dev.assigned_role.label()}" }
                                div { style: "color:#64748b;font-size:9px;font-family:monospace;", "{dev.lan_endpoint} · {dev.crypto_key_id}" }
                                div { style: "display:flex;gap:4px;margin-top:2px;flex-wrap:wrap;",
                                    if dev.displays_tool_chest_and_inspector {
                                        span { style: "background:#1e1b4b;color:#c4b5fd;padding:1px 5px;border-radius:3px;font-size:9px;",
                                            "🧰 Displays Tool Chest & Inspector"
                                        }
                                    }
                                    if dev.receives_main_content_stream {
                                        span { style: "background:#064e3b;color:#34d399;padding:1px 5px;border-radius:3px;font-size:9px;",
                                            "📺 Receives Main Content Stream"
                                        }
                                    }
                                    if dev.forwards_hid_input {
                                        button {
                                            r#type: "button",
                                            style: "background:#0f172a;color:#38bdf8;border:1px solid #334155;border-radius:3px;padding:1px 6px;font-size:9px;cursor:pointer;",
                                            onclick: move |_| {
                                                let mut m = wm();
                                                let mut w = wb();
                                                let target_node = w.selected.clone().or_else(|| w.nodes.first().map(|n| n.id.clone())).unwrap_or_else(|| "stage".into());
                                                let pkt = m.forward_hid_input(
                                                    &dev_id,
                                                    "DISP-3-TV",
                                                    &target_node,
                                                    "HID Pointer/Keyboard/Touch/Command-Orb Event",
                                                    &format!("Forwarded human input from {dev_label} to `{target_node}` on TV/Main Display"),
                                                );
                                                w.note(format!("HID Packet {} forwarded from {} (sig {}…)", pkt.packet_id, dev_label, &pkt.signature_hex[..10]));
                                                wm.set(m);
                                                wb.set(w);
                                            },
                                            "⌨️👆 Test HID Input → TV/Stage"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Bottom Window Taskbar Shelf for restoring minimized windows, switching focused windows in
/// `MaximizedFocusStage`, and viewing per-window display/TV routing badges.
pub fn WindowTaskbarShelf(
    wb: Signal<Workbench>,
    wm: Signal<QapSpaceMultiDeviceManager>,
) -> Element {
    let current_wb = wb();
    let current_wm = wm();

    rsx! {
        div {
            class: "window-taskbar-shelf",
            style: "background:#070b14;border-top:1px solid #1e293b;padding:4px 12px;display:flex;align-items:center;gap:6px;flex-wrap:wrap;font-size:11px;color:#cbd5e1;",
            span { style: "color:#64748b;font-weight:700;font-size:10px;text-transform:uppercase;",
                "WINDOWS ({current_wb.nodes.len()}):"
            }
            for node in current_wb.nodes.iter() {
                {
                    let id = node.id.clone();
                    let id_min = node.id.clone();
                    let id_route = node.id.clone();
                    let is_sel = current_wb.selected.as_deref() == Some(node.id.as_str());
                    let is_min = current_wm.is_minimized(&node.id);
                    let route_badge = current_wm
                        .route_for_container(&node.id)
                        .map(|r| r.target_display_id.clone())
                        .unwrap_or_else(|| "DISP-1-MAIN".into());
                    rsx! {
                        div {
                            key: "shelf-{id}",
                            style: if is_sel {
                                "display:flex;align-items:center;gap:4px;background:#1e293b;border:1px solid #38bdf8;border-radius:4px;padding:2px 6px;"
                            } else if is_min {
                                "display:flex;align-items:center;gap:4px;background:#090d16;border:1px dashed #475569;border-radius:4px;padding:2px 6px;opacity:0.75;"
                            } else {
                                "display:flex;align-items:center;gap:4px;background:#111827;border:1px solid #1e293b;border-radius:4px;padding:2px 6px;"
                            },
                            button {
                                r#type: "button",
                                style: "background:transparent;border:none;color:#e2e8f0;font-size:10px;cursor:pointer;padding:0;",
                                onclick: move |_| {
                                    let mut m = wm();
                                    if m.is_minimized(&id) {
                                        m.toggle_minimize_container(&id);
                                    }
                                    m.focused_container_id = Some(id.clone());
                                    wm.set(m);
                                    let mut w = wb();
                                    w.selected = Some(id.clone());
                                    wb.set(w);
                                },
                                "[{node.kind.id()}] {node.title}"
                            }
                            button {
                                r#type: "button",
                                title: "Cycle Target Display / TV / Tablet for this Window",
                                style: "background:#0f172a;color:#38bdf8;border:1px solid #334155;border-radius:3px;padding:1px 4px;font-size:9px;font-family:monospace;cursor:pointer;",
                                onclick: move |_| {
                                    let mut m = wm();
                                    let route = m.cycle_container_display_route(&id_route);
                                    wm.set(m);
                                    let mut w = wb();
                                    w.note(format!(
                                        "Window `{}` routed to {} ({})",
                                        id_route,
                                        route.target_display_id,
                                        route.transport.label()
                                    ));
                                    wb.set(w);
                                },
                                "📺 {route_badge}"
                            }
                            button {
                                r#type: "button",
                                title: if is_min { "Restore Window" } else { "Minimize Window to Shelf" },
                                style: "background:transparent;border:none;color:#94a3b8;font-size:10px;cursor:pointer;padding:0 2px;",
                                onclick: move |_| {
                                    let mut m = wm();
                                    m.toggle_minimize_container(&id_min);
                                    wm.set(m);
                                },
                                if is_min { "🔼" } else { "🗕" }
                            }
                        }
                    }
                }
            }
            div { style: "margin-left:auto;display:flex;align-items:center;gap:6px;",
                button {
                    r#type: "button",
                    style: "background:#111827;color:#38bdf8;border:1px solid #1e293b;border-radius:4px;padding:2px 8px;font-size:10px;cursor:pointer;",
                    onclick: move |_| {
                        let mut m = wm();
                        let mut w = wb();
                        m.tile_workbench_containers(&mut w);
                        wm.set(m);
                        wb.set(w);
                    },
                    "◫ Tile All Windows"
                }
                if !current_wm.minimized_container_ids.is_empty() {
                    button {
                        r#type: "button",
                        style: "background:#111827;color:#34d399;border:1px solid #1e293b;border-radius:4px;padding:2px 8px;font-size:10px;cursor:pointer;",
                        onclick: move |_| {
                            let mut m = wm();
                            m.minimized_container_ids.clear();
                            wm.set(m);
                        },
                        "Restore All ({current_wm.minimized_container_ids.len()})"
                    }
                }
            }
        }
    }
}
