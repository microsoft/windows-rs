//! The ordered list of fixtures the self-test runs.

use crate::fixtures::*;
use crate::harness::Harness;

pub type FixtureFn = fn(&Harness);

pub static FIXTURES: &[(&str, FixtureFn)] = &[
    ("Navigation_LifecycleOrder", navigation::lifecycle_order),
    ("Navigation_DocumentTitle", navigation::document_title),
    ("Navigation_DataUri", navigation::data_uri),
    (
        "Script_ExecuteReturnsValue",
        scripting::execute_returns_value,
    ),
    ("Script_OnDocumentCreated", scripting::on_document_created),
    ("Ipc_PageToHost", ipc::page_to_host),
    ("Ipc_HostToPageRoundTrip", ipc::host_to_page_round_trip),
    (
        "Protocol_WebResourceServedFromMemory",
        protocol::web_resource_served_from_memory,
    ),
    ("Cookie_AddAndGet", cookies::add_and_get),
    ("Controller_ZoomRoundTrip", controller::zoom_round_trip),
    (
        "Controller_RasterizationScale",
        controller::rasterization_scale,
    ),
    ("Controller_VisibilityToggle", controller::visibility_toggle),
    (
        "Settings_DisableScriptBlocksExecution",
        settings::disable_script_blocks_execution,
    ),
    ("Profile_PrivateMode", profile::private_mode),
];
