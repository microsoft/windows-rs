//! Host <-> page messaging fixtures.

use std::cell::RefCell;
use std::rc::Rc;

use crate::harness::Harness;

/// A page that calls `chrome.webview.postMessage` is delivered to the host.
pub fn page_to_host(harness: &Harness) {
    let received: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = received.clone();
    let webview = harness.webview();

    let Ok(registration) = webview.on_web_message_received(move |args| {
        if let Ok(message) = args.try_web_message_as_string() {
            *sink.borrow_mut() = Some(message);
        }
    }) else {
        harness.check("Ipc_PageToHost_Subscribe", false);
        return;
    };

    let Ok(script_id) = webview
        .add_script_to_execute_on_document_created("window.chrome.webview.postMessage('ping');")
    else {
        harness.check("Ipc_PageToHost_Inject", false);
        drop(registration);
        return;
    };

    harness.navigate_html("<!DOCTYPE html><html></html>");
    let got = harness.wait(|| received.borrow().is_some());
    harness.check(
        "Ipc_PageToHost_Received",
        got && received.borrow().as_deref() == Some("ping"),
    );

    let _ = webview.remove_script_to_execute_on_document_created(&script_id);
    drop(registration);
}

/// A message posted from the host reaches the page, which echoes it back.
pub fn host_to_page_round_trip(harness: &Harness) {
    let received: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = received.clone();
    let webview = harness.webview();

    let Ok(registration) = webview.on_web_message_received(move |args| {
        if let Ok(message) = args.try_web_message_as_string() {
            *sink.borrow_mut() = Some(message);
        }
    }) else {
        harness.check("Ipc_RoundTrip_Subscribe", false);
        return;
    };

    let page = "<!DOCTYPE html><html><body><script>\
        window.chrome.webview.addEventListener('message', e => {\
            window.chrome.webview.postMessage('echo:' + e.data);\
        });\
        </script></body></html>";
    harness.check("Ipc_RoundTrip_Nav", harness.navigate_html(page));

    if webview.post_web_message_as_string("hello").is_err() {
        harness.check("Ipc_RoundTrip_Post", false);
        drop(registration);
        return;
    }

    let got = harness.wait(|| received.borrow().is_some());
    harness.check(
        "Ipc_RoundTrip_Received",
        got && received.borrow().as_deref() == Some("echo:hello"),
    );

    drop(registration);
}
