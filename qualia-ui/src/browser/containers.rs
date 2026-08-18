//! Container rendering: glassmorphism containers with type tags, badges, ports.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

use crate::tool_chest::core::registry::SeedContainer;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

use super::container_inline_views::{
    build_connection_requests_view, build_conversations_view, build_gis_map_view,
    build_media_3d_view, build_protection_policies_view, build_social_chat_view,
    build_vibescript_console,
};

/// Build a single container node on the canvas.
pub fn build_container(document: &Document, container: &SeedContainer) -> Element {
    let el = document.create_element("div").unwrap();
    el.set_class_name("canvas-container-node");
    el.set_attribute("data-container-type", &container.container_type)
        .unwrap();

    // Set strata and epistemic data attributes for filtering
    let (strata, epistemic) = container_type_filter_attrs(&container.container_type);
    el.set_attribute("data-strata", strata).unwrap();
    el.set_attribute("data-epistemic", epistemic).unwrap();

    let style = format!(
        "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {};",
        container.x as u32,
        container.y as u32,
        container.width as u32,
        container.height as u32,
        container.z as u32
    );
    el.set_attribute("style", &style).unwrap();

    // Header
    let header = document.create_element("div").unwrap();
    header.set_class_name("container-header");

    let title_group = document.create_element("div").unwrap();
    title_group.set_class_name("container-title-group");

    // Type tag
    let tag = document.create_element("span").unwrap();
    let (tag_class, tag_label) = container_type_tag(&container.container_type);
    tag.set_class_name(&format!("container-type-tag {}", tag_class));
    tag.set_text_content(Some(tag_label));
    title_group.append_child(&tag).unwrap();

    // Title
    let title = document.create_element("span").unwrap();
    title.set_class_name("container-title");
    title.set_text_content(Some(&container.title));
    title_group.append_child(&title).unwrap();

    // Honesty badge
    let badge = document.create_element("span").unwrap();
    badge.set_class_name(&format!("honesty-badge honesty-{}", container.honesty));
    badge.set_text_content(Some(&container.honesty));
    title_group.append_child(&badge).unwrap();

    header.append_child(&title_group).unwrap();

    // Actions
    let actions = document.create_element("div").unwrap();
    actions.set_class_name("container-actions");
    // Settings button
    let settings_btn = document.create_element("button").unwrap();
    settings_btn.set_class_name("container-action-btn");
    settings_btn.set_text_content(Some("\u{2699}"));
    settings_btn
        .set_attribute("title", "Container settings")
        .unwrap();
    actions.append_child(&settings_btn).unwrap();
    // Delete button
    let delete_btn = document.create_element("button").unwrap();
    delete_btn.set_class_name("container-action-btn delete-btn");
    delete_btn.set_text_content(Some("\u{2715}"));
    delete_btn
        .set_attribute("title", "Delete container")
        .unwrap();
    actions.append_child(&delete_btn).unwrap();
    header.append_child(&actions).unwrap();
    el.append_child(&header).unwrap();

    // Body
    let body = document.create_element("div").unwrap();
    body.set_class_name("container-body");

    match container.container_type.as_str() {
        "social" => {
            body.append_child(&build_social_chat_view(document))
                .unwrap();
        }
        "connection-requests" => {
            body.append_child(&build_connection_requests_view(document))
                .unwrap();
        }
        // --- Project container types (Workstream A) ---
        "kanban" => {
            body.append_child(&super::project_views::kanban::build_kanban_view(document))
                .unwrap();
        }
        "project_sheet" => {
            body.append_child(
                &super::project_views::project_sheet::build_project_sheet_view(document),
            )
            .unwrap();
        }
        "budget" => {
            body.append_child(&super::project_views::budget::build_budget_view(document))
                .unwrap();
        }
        "cost_base" => {
            body.append_child(&super::project_views::cost_base::build_cost_base_view(
                document,
            ))
            .unwrap();
        }
        "deliverable" => {
            body.append_child(&super::project_views::deliverable::build_deliverable_view(
                document,
            ))
            .unwrap();
        }
        "review" => {
            body.append_child(&super::project_views::review::build_review_view(document))
                .unwrap();
        }
        "discussion" => {
            body.append_child(&super::project_views::discussion::build_discussion_view(
                document,
            ))
            .unwrap();
        }
        "roadmap" => {
            body.append_child(&super::project_views::roadmap::build_roadmap_view(document))
                .unwrap();
        }
        "commons" => {
            body.append_child(&super::project_views::commons::build_commons_view(document))
                .unwrap();
        }
        "agreement_builder" => {
            body.append_child(
                &super::agreement_views::agreement_builder::build_agreement_builder_view(document),
            )
            .unwrap();
        }
        "compensation_model" => {
            body.append_child(
                &super::agreement_views::compensation_model::build_compensation_model_view(
                    document,
                ),
            )
            .unwrap();
        }
        "contribution_ledger" => {
            body.append_child(
                &super::agreement_views::contribution_ledger::build_contribution_ledger_view(
                    document,
                ),
            )
            .unwrap();
        }
        "license_builder" => {
            body.append_child(
                &super::agreement_views::license_builder::build_license_builder_view(document),
            )
            .unwrap();
        }
        "obligation_tracker" => {
            body.append_child(
                &super::agreement_views::obligation_tracker::build_obligation_tracker_view(
                    document,
                ),
            )
            .unwrap();
        }
        "ip_registry" => {
            body.append_child(&super::project_views::ip_registry::build_ip_registry_view(
                document,
            ))
            .unwrap();
        }
        "data_sources" => {
            body.append_child(
                &super::project_views::data_sources::build_data_sources_view(document),
            )
            .unwrap();
        }
        "disputes" => {
            body.append_child(&super::governance_views::disputes::build_disputes_view(
                document,
            ))
            .unwrap();
        }
        "complaints" => {
            body.append_child(&super::governance_views::complaints::build_complaints_view(
                document,
            ))
            .unwrap();
        }
        "corrections" => {
            body.append_child(
                &super::governance_views::corrections::build_corrections_view(document),
            )
            .unwrap();
        }
        "governance_meetings" => {
            body.append_child(
                &super::governance_views::governance_meetings::build_governance_meetings_view(
                    document,
                ),
            )
            .unwrap();
        }
        "conflict_of_interest" => {
            body.append_child(
                &super::governance_views::conflict_of_interest::build_conflict_of_interest_view(
                    document,
                ),
            )
            .unwrap();
        }
        "onboarding" => {
            body.append_child(&super::project_views::onboarding::build_onboarding_view(
                document,
            ))
            .unwrap();
        }
        "bulk_import" => {
            body.append_child(&super::project_views::bulk_import::build_bulk_import_view(
                document,
            ))
            .unwrap();
        }
        "knowledge_base" => {
            body.append_child(
                &super::project_views::knowledge_base::build_knowledge_base_view(document),
            )
            .unwrap();
        }
        "agent_console" => {
            body.append_child(
                &super::project_views::agent_console::build_agent_console_view(document),
            )
            .unwrap();
        }
        "awards" => {
            body.append_child(&super::project_views::awards::build_awards_view(document))
                .unwrap();
        }
        "token_mgr" => {
            body.append_child(&super::project_views::token_mgr::build_token_mgr_view(
                document,
            ))
            .unwrap();
        }
        "dashboard" => {
            body.append_child(&super::project_views::dashboard::build_dashboard_view(
                document,
            ))
            .unwrap();
        }
        "wiki" => {
            body.append_child(&super::project_views::wiki::build_wiki_view(document))
                .unwrap();
        }
        "governance" => {
            body.append_child(&super::project_views::governance::build_governance_view(
                document,
            ))
            .unwrap();
        }
        "credentials" => {
            body.append_child(&super::project_views::credentials::build_credentials_view(
                document,
            ))
            .unwrap();
        }
        "gantt" => {
            body.append_child(&super::project_views::gantt::build_gantt_view(document))
                .unwrap();
        }
        "timeline" => {
            body.append_child(&super::project_views::timeline::build_timeline_view(
                document,
            ))
            .unwrap();
        }
        "calendar" => {
            body.append_child(&super::project_views::calendar::build_calendar_view(
                document,
            ))
            .unwrap();
        }
        "doc_mgmt" => {
            body.append_child(&super::project_views::doc_mgmt::build_doc_mgmt_view(
                document,
            ))
            .unwrap();
        }
        "resource_report" => {
            body.append_child(
                &super::project_views::resource_report::build_resource_report_view(document),
            )
            .unwrap();
        }
        "time_tracking" => {
            body.append_child(
                &super::project_views::time_tracking::build_time_tracking_view(document),
            )
            .unwrap();
        }
        "voting" => {
            body.append_child(&super::project_views::voting::build_voting_view(document))
                .unwrap();
        }
        "risk" => {
            body.append_child(&super::project_views::risk::build_risk_view(document))
                .unwrap();
        }
        "reputation" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(
                "Reputation panels \u{2014} trustworthiness, competence, integrity, conduct.",
            ));
            body.append_child(&ph).unwrap();
        }
        "protection-policies" => {
            body.append_child(&build_protection_policies_view(document))
                .unwrap();
        }
        "capabilities" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(
                "Capability badges \u{2014} active, suspended, revoked, expired.",
            ));
            body.append_child(&ph).unwrap();
        }
        "settings" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(
                "Preferences \u{2014} accessibility, privacy, communication, workflow.",
            ));
            body.append_child(&ph).unwrap();
        }
        "conversations" => {
            body.append_child(&build_conversations_view(document))
                .unwrap();
        }
        "channels" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(
                "Channels \u{2014} direct, topic, request-response, stream, group, federation.",
            ));
            body.append_child(&ph).unwrap();
        }
        "presence" => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(
                "Presence indicators \u{2014} online, away, busy, offline.",
            ));
            body.append_child(&ph).unwrap();
        }
        "map" => {
            body.append_child(&build_gis_map_view(document)).unwrap();
        }
        "media" => {
            body.append_child(&build_media_3d_view(document)).unwrap();
        }
        "code" => {
            body.append_child(&build_vibescript_console(document))
                .unwrap();
        }
        "doc" => {
            body.append_child(&super::container_views::build_doc_view(document))
                .unwrap();
        }
        "sheet" => {
            body.append_child(&super::container_views::build_sheet_view(document))
                .unwrap();
        }
        "graph" => {
            body.append_child(&super::container_views::build_graph_view(document))
                .unwrap();
        }
        "ontology" => {
            body.append_child(&super::container_views::build_ontology_view(document))
                .unwrap();
        }
        "pulse" => {
            body.append_child(&super::container_views::build_pulse_view(document))
                .unwrap();
        }
        "rights" => {
            body.append_child(&super::rights_views::build_rights_view(document))
                .unwrap();
        }
        "wallet" => {
            body.append_child(&super::rights_views::build_wallet_view(document))
                .unwrap();
        }
        "library" => {
            body.append_child(&super::container_views_ext::build_library_view(document))
                .unwrap();
        }
        "aura" => {
            body.append_child(&super::container_views_ext::build_aura_view(document))
                .unwrap();
        }
        "latex" => {
            body.append_child(&super::container_views_ext::build_latex_view(document))
                .unwrap();
        }
        "health" => {
            body.append_child(&super::container_views_ext::build_health_view(document))
                .unwrap();
        }
        "anatomy" => {
            body.append_child(&super::container_views_ext::build_anatomy_view(document))
                .unwrap();
        }
        "webview" => {
            body.append_child(&super::container_views_ext::build_webview_view(document))
                .unwrap();
        }
        "webrtc" => {
            body.append_child(&super::container_views_ext::build_webrtc_view(document))
                .unwrap();
        }
        "finance" => {
            body.append_child(&super::container_views_ext::build_finance_view(document))
                .unwrap();
        }
        "vision" => {
            body.append_child(&super::container_views_ext::build_vision_view(document))
                .unwrap();
        }
        "listen" => {
            body.append_child(&super::container_views_ext::build_listen_view(document))
                .unwrap();
        }
        "triad" => {
            body.append_child(&super::container_views_ext::build_triad_view(document))
                .unwrap();
        }
        "portal" => {
            body.append_child(&super::container_views_ext::build_portal_view(document))
                .unwrap();
        }
        "slide" => {
            body.append_child(&super::container_views_ext::build_slide_view(document))
                .unwrap();
        }
        "3d" => {
            body.append_child(&super::container_views_ext::build_3d_view(document))
                .unwrap();
        }
        "subcanvas" => {
            body.append_child(&super::container_views_ext::build_subcanvas_view(document))
                .unwrap();
        }
        // --- Workflow panel containers (see SAVE_ARCHITECTURE.md) ---
        "checkpoint-tray" => {
            body.append_child(&super::workflow_panels::build_checkpoint_tray_view(
                document,
            ))
            .unwrap();
        }
        "credential-inspector" => {
            body.append_child(&super::workflow_panels::build_credential_inspector_view(
                document,
            ))
            .unwrap();
        }
        "context-markup-editor" => {
            body.append_child(&super::workflow_panels::build_context_markup_editor_view(
                document,
            ))
            .unwrap();
        }
        "provenance-panel" => {
            body.append_child(&super::workflow_panels::build_provenance_panel_view(
                document,
            ))
            .unwrap();
        }
        "publication-workflow" => {
            body.append_child(&super::workflow_panels::build_publication_workflow_view(
                document,
            ))
            .unwrap();
        }
        "constituency-manager" => {
            body.append_child(&super::workflow_panels::build_constituency_manager_view(
                document,
            ))
            .unwrap();
        }
        // --- Widget containers ---
        "capability-badge" => {
            body.append_child(&super::workflow_panels::build_capability_badge_view(
                document,
            ))
            .unwrap();
        }
        "checkpoint-indicator" => {
            body.append_child(&super::workflow_panels::build_checkpoint_indicator_view(
                document,
            ))
            .unwrap();
        }
        "consent-indicator" => {
            body.append_child(&super::workflow_panels::build_consent_indicator_view(
                document,
            ))
            .unwrap();
        }
        _ => {
            let ph = document.create_element("div").unwrap();
            ph.set_class_name("container-placeholder");
            ph.set_text_content(Some(&format!(
                "{} \u{2014} awaiting backend wiring.",
                container.title
            )));
            body.append_child(&ph).unwrap();
        }
    }
    el.append_child(&body).unwrap();

    // Connection ports
    let port_in = document.create_element("div").unwrap();
    port_in.set_class_name("container-port port-in");
    port_in.set_attribute("data-port", "in").unwrap();
    el.append_child(&port_in).unwrap();

    let port_out = document.create_element("div").unwrap();
    port_out.set_class_name("container-port port-out");
    port_out.set_attribute("data-port", "out").unwrap();
    el.append_child(&port_out).unwrap();

    // Resize handle
    let resizer = document.create_element("div").unwrap();
    resizer.set_class_name("container-resizer");
    el.append_child(&resizer).unwrap();

    el
}

/// Map container type to CSS tag class and label.
/// Map container type to strata and epistemic categories for filtering.
/// Used by the Strata and Epistemic Lens pods in the top control bar.
fn container_type_filter_attrs(container_type: &str) -> (&'static str, &'static str) {
    match container_type {
        // Social strata
        "social"
        | "connection-requests"
        | "reputation"
        | "conversations"
        | "channels"
        | "presence"
        | "webrtc"
        | "webview" => ("social", "intersubjective"),

        // Environmental strata
        "map" | "media" | "3d" | "vision" | "listen" | "triad" | "portal" | "anatomy"
        | "health" => ("environmental", "objective"),

        // Legal strata
        "rights" | "protection-policies" | "capabilities" => ("legal", "normative"),

        // Financial strata
        "finance" | "wallet" => ("financial", "objective"),

        // Technical strata
        "code" | "ontology" | "graph" | "doc" | "sheet" | "latex" | "slide" | "subcanvas"
        | "pulse" | "aura" | "library" => ("technical", "objective"),

        // Workflow panels — technical
        "checkpoint-tray"
        | "credential-inspector"
        | "context-markup-editor"
        | "provenance-panel"
        | "publication-workflow"
        | "constituency-manager"
        | "capability-badge"
        | "checkpoint-indicator"
        | "consent-indicator" => ("technical", "normative"),

        // Project/ERP containers — governance strata
        "kanban"
        | "project_sheet"
        | "budget"
        | "cost_base"
        | "deliverable"
        | "review"
        | "discussion"
        | "roadmap"
        | "commons"
        | "agreement_builder"
        | "compensation_model"
        | "contribution_ledger"
        | "license_builder"
        | "obligation_tracker"
        | "ip_registry"
        | "data_sources"
        | "disputes"
        | "complaints"
        | "corrections"
        | "governance_meetings"
        | "conflict_of_interest"
        | "onboarding"
        | "bulk_import"
        | "knowledge_base"
        | "agent_console"
        | "awards"
        | "token_mgr"
        | "dashboard"
        | "wiki"
        | "governance"
        | "credentials"
        | "gantt"
        | "timeline"
        | "calendar"
        | "doc_mgmt"
        | "resource_report"
        | "time_tracking"
        | "voting"
        | "risk" => ("governance", "normative"),

        // Settings — technical/normative
        "settings" => ("technical", "normative"),

        _ => ("technical", "objective"),
    }
}

fn container_type_tag(container_type: &str) -> (&'static str, &'static str) {
    match container_type {
        "social" => ("tag-social", "SOCIAL"),
        "connection-requests" => ("tag-social", "SOCIAL"),
        "reputation" => ("tag-social", "SOCIAL"),
        "protection-policies" => ("tag-ontology", "ONTOLOGY"),
        "settings" => ("tag-default", "CONFIG"),
        "capabilities" => ("tag-ontology", "CAPS"),
        "conversations" => ("tag-social", "CHAT"),
        "channels" => ("tag-webrtc", "RTC"),
        "presence" => ("tag-social", "PRESENCE"),
        "map" => ("tag-map", "GIS"),
        "media" => ("tag-media", "MEDIA"),
        "code" => ("tag-code", "VIBE"),
        "doc" => ("tag-doc", "DOC"),
        // Workflow panel containers
        "checkpoint-tray" => ("tag-workflow", "WORKFLOW"),
        "credential-inspector" => ("tag-workflow", "CREDENTIAL"),
        "context-markup-editor" => ("tag-workflow", "MARKUP"),
        "provenance-panel" => ("tag-workflow", "PROVENANCE"),
        "publication-workflow" => ("tag-workflow", "PUBLISH"),
        "constituency-manager" => ("tag-workflow", "CONSTITUENCY"),
        // Widget containers
        "capability-badge" => ("tag-widget", "BADGE"),
        "checkpoint-indicator" => ("tag-widget", "INDICATOR"),
        "consent-indicator" => ("tag-widget", "INDICATOR"),
        "kanban" => ("tag-workflow", "KANBAN"),
        "project_sheet" => ("tag-workflow", "PROJECT"),
        "budget" => ("tag-workflow", "BUDGET"),
        "cost_base" => ("tag-workflow", "COST"),
        "deliverable" => ("tag-workflow", "DELIVER"),
        "review" => ("tag-workflow", "REVIEW"),
        "discussion" => ("tag-social", "DISCUSS"),
        "roadmap" => ("tag-workflow", "ROADMAP"),
        "commons" => ("tag-governance", "COMMONS"),
        "agreement_builder" => ("tag-governance", "AGREE"),
        "compensation_model" => ("tag-governance", "COMP"),
        "contribution_ledger" => ("tag-governance", "LEDGER"),
        "license_builder" => ("tag-governance", "LICENSE"),
        "obligation_tracker" => ("tag-governance", "OBLIG"),
        "ip_registry" => ("tag-governance", "IP"),
        "data_sources" => ("tag-governance", "DATA"),
        "disputes" => ("tag-governance", "DISPUTE"),
        "complaints" => ("tag-governance", "COMPLAIN"),
        "corrections" => ("tag-governance", "CORRECT"),
        "governance_meetings" => ("tag-governance", "MEET"),
        "conflict_of_interest" => ("tag-governance", "COI"),
        "onboarding" => ("tag-governance", "ONBOARD"),
        "bulk_import" => ("tag-governance", "BULK"),
        "knowledge_base" => ("tag-governance", "KB"),
        "agent_console" => ("tag-governance", "AGENT"),
        "awards" => ("tag-governance", "AWARD"),
        "token_mgr" => ("tag-governance", "TOKEN"),
        "dashboard" => ("tag-governance", "DASH"),
        "wiki" => ("tag-governance", "WIKI"),
        "governance" => ("tag-governance", "GOV"),
        "credentials" => ("tag-governance", "CRED"),
        "gantt" => ("tag-governance", "GANTT"),
        "timeline" => ("tag-governance", "TL"),
        "calendar" => ("tag-governance", "CAL"),
        "doc_mgmt" => ("tag-governance", "DOC"),
        "resource_report" => ("tag-governance", "RES"),
        "time_tracking" => ("tag-governance", "TIME"),
        "voting" => ("tag-governance", "VOTE"),
        "risk" => ("tag-governance", "RISK"),
        _ => ("tag-default", "CONTAINER"),
    }
}
