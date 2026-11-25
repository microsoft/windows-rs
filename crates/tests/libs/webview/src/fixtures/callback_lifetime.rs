use crate::harness::Harness;
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, ThreadId},
};
use windows_webview::WebResourceResponse;
use windows_window::Window;

#[derive(Clone, Default)]
struct Trace(Arc<Mutex<Vec<(String, ThreadId)>>>);

impl Trace {
    fn contains(&self, event: &str) -> bool {
        self.0.lock().unwrap().iter().any(|entry| entry.0 == event)
    }
}

struct Capture {
    name: &'static str,
    trace: Trace,
    _local: Rc<()>,
}

impl Capture {
    fn new(name: &'static str, trace: &Trace) -> Self {
        Self {
            name,
            trace: trace.clone(),
            _local: Rc::new(()),
        }
    }

    fn record(&self, phase: &str) {
        self.trace
            .0
            .lock()
            .unwrap()
            .push((format!("{}/{phase}", self.name), thread::current().id()));
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        self.record("drop");
    }
}

pub fn native_capture_lifetime(harness: &Harness) {
    let owner = thread::current().id();
    let trace = Trace::default();
    let capture = Capture::new("script", &trace);
    let result = Harness::complete(|completed| {
        harness.webview().execute_script("1 + 2", move |result| {
            capture.record("invoke");
            completed(result);
        })
    });
    harness.check(
        "Lifetime_ScriptResult",
        matches!(result, Some(Ok(value)) if value == "3"),
    );
    harness.check(
        "Lifetime_ScriptDrop",
        harness.wait(|| trace.contains("script/drop")),
    );

    let capture = Capture::new("navigation", &trace);
    let registration = harness
        .webview()
        .on_navigation_completed(move |_| {
            capture.record("invoke");
        })
        .unwrap();
    harness.check(
        "Lifetime_Navigation",
        harness.navigate_html("<!DOCTYPE html><title>lifetime</title>"),
    );
    harness.check(
        "Lifetime_NavigationDelivered",
        trace.contains("navigation/invoke"),
    );
    drop(registration);
    harness.check(
        "Lifetime_UnsubscribeDrop",
        harness.wait(|| trace.contains("navigation/drop")),
    );

    let capture = Capture::new("protocol", &trace);
    let registration = harness
        .webview()
        .on_web_resource_requested("https://lifetime.example/*", move |_| {
            capture.record("invoke");
            Some(
                WebResourceResponse::new("<!DOCTYPE html><title>lifetime</title>")
                    .content_type("text/html"),
            )
        })
        .unwrap();
    harness.check(
        "Lifetime_ProtocolNavigation",
        harness.navigate_uri("https://lifetime.example/"),
    );
    harness.check(
        "Lifetime_ProtocolDelivered",
        trace.contains("protocol/invoke"),
    );
    drop(registration);
    harness.check(
        "Lifetime_ProtocolDrop",
        harness.wait(|| trace.contains("protocol/drop")),
    );

    let window = Window::new("WebView2 pending completion lifetime")
        .size(320, 240)
        .quit_on_close(false)
        .create()
        .unwrap();
    let Some(Ok(controller)) = Harness::complete(|completed| {
        harness
            .environment()
            .create_controller_for_hwnd(window.hwnd(), completed)
    }) else {
        harness.check("Lifetime_ControllerCreated", false);
        return;
    };
    let webview = controller.webview().unwrap();
    let controller = Rc::new(controller);
    let retained_controller = controller.clone();
    let capture = Capture::new("pending-close", &trace);
    webview
        .execute_script("1 + 2", move |_| {
            let _retained_controller = retained_controller;
            capture.record("invoke");
        })
        .unwrap();
    controller.close().unwrap();
    drop(webview);
    harness.check(
        "Lifetime_PendingCloseDrop",
        harness.wait(|| trace.contains("pending-close/drop")),
    );
    harness.check(
        "Lifetime_RetainedControllerDrop",
        Rc::strong_count(&controller) == 1,
    );
    drop(controller);
    drop(window);

    let events = trace.0.lock().unwrap();
    for (event, thread) in events.iter() {
        println!("# {event}: {thread:?}, owner {owner:?}");
    }
    harness.check(
        "Lifetime_OwnerThread",
        events.iter().all(|entry| entry.1 == owner),
    );
}
