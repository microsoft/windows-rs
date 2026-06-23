//! Custom-protocol fixture: serve a page entirely from memory.

use windows_webview::WebResourceResponse;

use crate::harness::Harness;

/// `on_web_resource_requested` serves an in-memory document that then loads.
pub fn web_resource_served_from_memory(harness: &Harness) {
    let webview = harness.webview();

    let Ok(registration) = webview.on_web_resource_requested("https://selftest.example/*", |_| {
        Some(
            WebResourceResponse::new(
                "<!DOCTYPE html><html><head><title>FromMemory</title></head>\
                 <body>served from rust</body></html>",
            )
            .content_type("text/html"),
        )
    }) else {
        harness.check("Protocol_Subscribe", false);
        return;
    };

    let ok = harness.navigate_uri("https://selftest.example/index.html");
    harness.check("Protocol_NavSuccess", ok);

    let titled = harness.wait(|| harness.webview().document_title() == "FromMemory");
    harness.check("Protocol_Title", titled);

    drop(registration);
}
