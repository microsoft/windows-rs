//! The ordered list of fixtures the self-test runs.

use crate::fixtures::*;
use crate::harness::Harness;

pub type FixtureFn = fn(&Harness);

pub static FIXTURES: &[(&str, FixtureFn)] = &[
    ("Navigation_LifecycleOrder", navigation::lifecycle_order),
    ("Navigation_DocumentTitle", navigation::document_title),
    ("Navigation_DataUri", navigation::data_uri),
    (
        "Navigation_HistoryBackForward",
        navigation::history_back_forward,
    ),
    ("Navigation_Reload", navigation::reload),
    (
        "Script_ExecuteReturnsValue",
        scripting::execute_returns_value,
    ),
    ("Script_OnDocumentCreated", scripting::on_document_created),
    ("Ipc_PageToHost", ipc::page_to_host),
    ("Ipc_HostToPageRoundTrip", ipc::host_to_page_round_trip),
    ("Ipc_JsonRoundTrip", ipc::json_round_trip),
    (
        "Protocol_WebResourceServedFromMemory",
        protocol::web_resource_served_from_memory,
    ),
    (
        "Protocol_VirtualHostServesFolder",
        protocol::virtual_host_serves_folder,
    ),
    (
        "Protocol_NavigateWithRequestPost",
        protocol::navigate_with_request_post,
    ),
    ("Cookie_AddAndGet", cookies::add_and_get),
    ("Cookie_AddThenDeleteAll", cookies::add_then_delete_all),
    ("Controller_ZoomRoundTrip", controller::zoom_round_trip),
    ("Controller_VisibilityToggle", controller::visibility_toggle),
    (
        "Controller_BackgroundColorRoundTrip",
        controller::background_color_round_trip,
    ),
    (
        "Controller_RasterizationScaleRoundTrip",
        controller::rasterization_scale_round_trip,
    ),
    (
        "Controller_DetectMonitorScaleChanges",
        controller::detect_monitor_scale_changes_round_trip,
    ),
    (
        "Settings_DisableScriptBlocksExecution",
        settings::disable_script_blocks_execution,
    ),
    (
        "Settings_UserAgentRoundTrip",
        settings::user_agent_round_trip,
    ),
    (
        "Settings_WebMessageEnabledGatesIpc",
        settings::web_message_enabled_gates_ipc,
    ),
    ("Profile_PrivateMode", profile::private_mode),
    (
        "Profile_ColorSchemeRoundTrip",
        profile::color_scheme_round_trip,
    ),
    (
        "Profile_DefaultDownloadFolderPath",
        profile::default_download_folder_path,
    ),
    (
        "Environment_WithOptions",
        environment::with_options_creates_environment,
    ),
];
