//! Custom-protocol and virtual-host fixtures: serve pages from memory and from
//! a mapped local folder.

use std::cell::RefCell;
use std::rc::Rc;

use windows_webview::{HostResourceAccessKind, NavigationRequest, WebResourceResponse};

use crate::harness::Harness;

/// `on_web_resource_requested` serves an in-memory document that then loads, and
/// the handler sees the intercepted request's URI and method.
pub fn web_resource_served_from_memory(harness: &Harness) {
    let webview = harness.webview();
    let request: Rc<RefCell<Option<(String, String)>>> = Rc::new(RefCell::new(None));
    let sink = request.clone();

    let Ok(registration) =
        webview.on_web_resource_requested("https://selftest.example/*", move |request| {
            // Capture only the first intercepted request: the browser fires
            // follow-up requests (e.g. a favicon) that also match the `/*`
            // filter, and a last-wins sink would race them against the assertion.
            sink.borrow_mut()
                .get_or_insert_with(|| (request.uri(), request.method()));
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

/// `navigate_with_request` sends the chosen HTTP method and body, which the
/// `on_web_resource_requested` handler reads back from the intercepted request.
pub fn navigate_with_request_post(harness: &Harness) {
    let webview = harness.webview();
    let request: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = request.clone();

    let Ok(registration) =
        webview.on_web_resource_requested("https://selftest.post/*", move |request| {
            *sink.borrow_mut() = Some(request.method());
            Some(
                WebResourceResponse::new(
                    "<!DOCTYPE html><html><head><title>Posted</title></head></html>",
                )
                .content_type("text/html"),
            )
        })
    else {
        harness.check("Post_Subscribe", false);
        return;
    };

    // Wait for the navigation to fully complete so it does not bleed into the
    // next fixture's navigation.
    let completed: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(None));
    let sink = completed.clone();
    let Ok(nav_registration) =
        webview.on_navigation_completed(move |args| *sink.borrow_mut() = Some(args.is_success()))
    else {
        harness.check("Post_NavSubscribe", false);
        return;
    };

    let nav = NavigationRequest::new("https://selftest.post/submit")
        .method("POST")
        .header("Content-Type", "text/plain")
        .body(b"payload".to_vec());
    if webview.navigate_with_request(&nav).is_err() {
        harness.check("Post_Navigate", false);
        return;
    }

    let done = harness.wait(|| completed.borrow().is_some());
    harness.check(
        "Post_NavCompleted",
        done && completed.borrow().unwrap_or(false),
    );
    harness.check("Post_Method", request.borrow().as_deref() == Some("POST"));

    drop(nav_registration);
    drop(registration);
}
