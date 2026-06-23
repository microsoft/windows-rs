//! Navigation lifecycle, document title, and `data:` URI fixtures.

use std::cell::RefCell;
use std::rc::Rc;

use crate::harness::Harness;

/// The navigation events fire in order and the completed event reports success.
pub fn lifecycle_order(harness: &Harness) {
    let events: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let webview = harness.webview();

    let starting_sink = events.clone();
    let loading_sink = events.clone();
    let completed_sink = events.clone();

    let (Ok(starting), Ok(loading), Ok(completed)) = (
        webview.on_navigation_starting(move |_| starting_sink.borrow_mut().push("starting")),
        webview.on_content_loading(move |_| loading_sink.borrow_mut().push("content_loading")),
        webview.on_navigation_completed(move |_| completed_sink.borrow_mut().push("completed")),
    ) else {
        harness.check("Navigation_LifecycleOrder_Subscribe", false);
        return;
    };

    let ok = harness.navigate_html(
        "<!DOCTYPE html><html><head><title>Hi</title></head><body><h1>hi</h1></body></html>",
    );
    harness.check("Navigation_LifecycleOrder_Success", ok);

    let order = events.borrow().clone();
    harness.check(
        "Navigation_LifecycleOrder_Starting",
        order.first() == Some(&"starting"),
    );
    harness.check(
        "Navigation_LifecycleOrder_ContentLoading",
        order.contains(&"content_loading"),
    );
    harness.check(
        "Navigation_LifecycleOrder_Completed",
        order.last() == Some(&"completed"),
    );
    harness.check(
        "Navigation_LifecycleOrder_Source",
        !harness.webview().source().is_empty(),
    );

    drop((starting, loading, completed));
}

/// The document-title-changed event delivers the new title and `document_title`
/// reflects it.
pub fn document_title(harness: &Harness) {
    let title: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = title.clone();
    let Ok(registration) = harness
        .webview()
        .on_document_title_changed(move |value| *sink.borrow_mut() = Some(value))
    else {
        harness.check("Navigation_DocumentTitle_Subscribe", false);
        return;
    };

    harness.navigate_html("<!DOCTYPE html><html><head><title>Test123</title></head></html>");
    let changed = harness.wait(|| title.borrow().is_some());
    harness.check(
        "Navigation_DocumentTitle_Event",
        changed && title.borrow().as_deref() == Some("Test123"),
    );
    harness.check(
        "Navigation_DocumentTitle_Getter",
        harness.webview().document_title() == "Test123",
    );

    drop(registration);
}

/// A `data:` URI navigates and completes successfully.
pub fn data_uri(harness: &Harness) {
    let ok = harness.navigate_uri("data:text/html,<h1>ok</h1>");
    harness.check("Navigation_DataUri_Success", ok);
}
