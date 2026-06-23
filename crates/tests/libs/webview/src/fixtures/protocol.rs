//! Custom-protocol and virtual-host fixtures: serve pages from memory and from
//! a mapped local folder.

use std::cell::RefCell;
use std::rc::Rc;

use windows_webview::{HostResourceAccessKind, WebResourceResponse};

use crate::harness::Harness;

/// `on_web_resource_requested` serves an in-memory document that then loads, and
/// the handler sees the intercepted request's URI and method.
pub fn web_resource_served_from_memory(harness: &Harness) {
    let webview = harness.webview();
    let request: Rc<RefCell<Option<(String, String)>>> = Rc::new(RefCell::new(None));
    let sink = request.clone();

    let Ok(registration) =
        webview.on_web_resource_requested("https://selftest.example/*", move |request| {
            *sink.borrow_mut() = Some((request.uri(), request.method()));
            Some(
                WebResourceResponse::new(
                    "<!DOCTYPE html><html><head><title>FromMemory</title></head>\
                     <body>served from rust</body></html>",
                )
                .content_type("text/html"),
            )
        })
    else {
        harness.check("Protocol_Subscribe", false);
        return;
    };

    let ok = harness.navigate_uri("https://selftest.example/index.html");
    harness.check("Protocol_NavSuccess", ok);

    let titled = harness.wait(|| harness.webview().document_title() == "FromMemory");
    harness.check("Protocol_Title", titled);

    let request = request.borrow();
    harness.check(
        "Protocol_RequestUri",
        matches!(request.as_ref(), Some((uri, _)) if uri == "https://selftest.example/index.html"),
    );
    harness.check(
        "Protocol_RequestMethod",
        matches!(request.as_ref(), Some((_, method)) if method == "GET"),
    );

    drop(registration);
}

/// `set_virtual_host_name_to_folder_mapping` serves files from a local folder
/// over a normal `https` URL.
pub fn virtual_host_serves_folder(harness: &Harness) {
    let folder = std::env::temp_dir().join("windows-webview-selftest-vhost");
    if std::fs::create_dir_all(&folder).is_err()
        || std::fs::write(
            folder.join("index.html"),
            "<!DOCTYPE html><html><head><title>VirtualHost</title></head></html>",
        )
        .is_err()
    {
        harness.check("VirtualHost_Setup", false);
        return;
    }

    let webview = harness.webview();
    if webview
        .set_virtual_host_name_to_folder_mapping(
            "vhost.example",
            &folder.to_string_lossy(),
            HostResourceAccessKind::Allow,
        )
        .is_err()
    {
        harness.check("VirtualHost_Map", false);
        return;
    }

    harness.check(
        "VirtualHost_Nav",
        harness.navigate_uri("https://vhost.example/index.html"),
    );
    harness.check(
        "VirtualHost_Title",
        harness.wait(|| harness.webview().document_title() == "VirtualHost"),
    );

    let _ = webview.clear_virtual_host_name_to_folder_mapping("vhost.example");
}
