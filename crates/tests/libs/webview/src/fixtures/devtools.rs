use std::cell::RefCell;
use std::rc::Rc;

use windows_core::Result;

use crate::harness::Harness;

fn call(harness: &Harness, method: &str, params_json: &str) -> Option<Result<String>> {
    let slot: Rc<RefCell<Option<Result<String>>>> = Rc::new(RefCell::new(None));
    let sink = slot.clone();
    if let Err(error) =
        harness
            .webview()
            .call_dev_tools_protocol_method(method, params_json, move |result| {
                *sink.borrow_mut() = Some(result);
            })
    {
        return Some(Err(error));
    }
    if !harness.wait(|| slot.borrow().is_some()) {
        return None;
    }
    slot.borrow_mut().take()
}

pub fn call_returns_json(harness: &Harness) {
    let version = call(harness, "Browser.getVersion", "{}");
    harness.check(
        "DevTools_Call_Result",
        matches!(version, Some(Ok(ref value)) if value.contains("product")),
    );

    let evaluated = call(
        harness,
        "Runtime.evaluate",
        r#"{"expression":"6 * 7","returnByValue":true}"#,
    );
    harness.check(
        "DevTools_Call_ParamsForwarded",
        matches!(evaluated, Some(Ok(ref value)) if value.contains("\"value\":42")),
    );
}

pub fn event_received(harness: &Harness) {
    let received: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = received.clone();
    let webview = harness.webview();

    let Ok(registration) =
        webview.on_dev_tools_protocol_event("Runtime.consoleAPICalled", move |args| {
            *sink.borrow_mut() = Some(args.parameter_object_as_json());
        })
    else {
        harness.check("DevTools_Event_Subscribe", false);
        return;
    };

    // CDP reports console messages only after enabling the Runtime domain.
    harness.check(
        "DevTools_Event_Enable",
        matches!(call(harness, "Runtime.enable", "{}"), Some(Ok(_))),
    );

    harness.check(
        "DevTools_Event_Nav",
        harness.navigate_html(
            "<!DOCTYPE html><html><body><script>console.log('selftest-cdp');</script></body></html>",
        ),
    );

    let got = harness.wait(|| received.borrow().is_some());
    harness.check(
        "DevTools_Event_Received",
        got && received
            .borrow()
            .as_deref()
            .is_some_and(|json| json.contains("selftest-cdp")),
    );

    let _ = call(harness, "Runtime.disable", "{}");
    drop(registration);
}
