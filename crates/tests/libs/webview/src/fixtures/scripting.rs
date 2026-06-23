//! Script execution fixtures: host-initiated `execute_script` and
//! document-created bootstrap scripts.

use crate::harness::Harness;

/// `execute_script` returns the evaluated expression's JSON-encoded result.
pub fn execute_returns_value(harness: &Harness) {
    harness.check(
        "Script_Execute_NavReady",
        harness.navigate_html("<!DOCTYPE html><html></html>"),
    );

    let result = harness.execute_script("1 + 2");
    harness.check(
        "Script_Execute_Result",
        matches!(result, Some(Ok(ref value)) if value == "3"),
    );
}

/// A document-created script runs before page scripts and is observable through
/// a later `execute_script`.
pub fn on_document_created(harness: &Harness) {
    let webview = harness.webview();
    let Ok(id) = webview.add_script_to_execute_on_document_created("window.__selftest = 42;")
    else {
        harness.check("Script_DocCreated_Add", false);
        return;
    };

    harness.check(
        "Script_DocCreated_Nav",
        harness.navigate_html("<!DOCTYPE html><html></html>"),
    );

    let result = harness.execute_script("window.__selftest");
    harness.check(
        "Script_DocCreated_Value",
        matches!(result, Some(Ok(ref value)) if value == "42"),
    );

    let _ = webview.remove_script_to_execute_on_document_created(&id);
}
