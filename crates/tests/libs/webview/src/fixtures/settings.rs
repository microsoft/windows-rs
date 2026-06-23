//! Settings fixture: disabling script prevents the page from running it.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::harness::Harness;

const SETS_TITLE: &str = "<!DOCTYPE html><html><head><title>original</title></head>\
     <body><script>document.title = 'changed';</script></body></html>";

/// With script disabled the page's inline script does not run, so the title
/// stays as authored; re-enabling lets it run.
pub fn disable_script_blocks_execution(harness: &Harness) {
    let Ok(settings) = harness.webview().settings() else {
        harness.check("Settings_Get", false);
        return;
    };

    if settings.set_script_enabled(false).is_err() {
        harness.check("Settings_Disable", false);
        return;
    }
    harness.navigate_html(SETS_TITLE);
    harness.check(
        "Settings_ScriptDisabled_TitleUnchanged",
        harness.webview().document_title() == "original",
    );

    let _ = settings.set_script_enabled(true);
    harness.navigate_html(SETS_TITLE);
    let changed = harness.wait(|| harness.webview().document_title() == "changed");
    harness.check("Settings_ScriptEnabled_TitleChanged", changed);
}

/// The user-agent string round-trips through the setter and getter, and the
/// page sees the overridden value.
pub fn user_agent_round_trip(harness: &Harness) {
    let Ok(settings) = harness.webview().settings() else {
        harness.check("Settings_UserAgent_Get", false);
        return;
    };

    let original = settings.user_agent().unwrap_or_default();
    if settings.set_user_agent("Selftest/1.0").is_err() {
        harness.check("Settings_UserAgent_Set", false);
        return;
    }
    harness.check(
        "Settings_UserAgent_Getter",
        settings.user_agent().ok().as_deref() == Some("Selftest/1.0"),
    );

    harness.navigate_html("<!DOCTYPE html><html></html>");
    let seen = matches!(
        harness.execute_script("navigator.userAgent"),
        Some(Ok(value)) if value == "\"Selftest/1.0\""
    );
    harness.check("Settings_UserAgent_PageSees", seen);

    let _ = settings.set_user_agent(&original);
}

/// `chrome.webview.postMessage` is only delivered to the host when
/// `is_web_message_enabled` is set: with it off the host receives nothing, and
/// turning it back on delivers the message.
pub fn web_message_enabled_gates_ipc(harness: &Harness) {
    let Ok(settings) = harness.webview().settings() else {
        harness.check("Settings_WebMessage_Get", false);
        return;
    };

    let received: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let sink = received.clone();
    let Ok(registration) = harness.webview().on_web_message_received(move |args| {
        if let Ok(message) = args.try_web_message_as_string() {
            *sink.borrow_mut() = Some(message);
        }
    }) else {
        harness.check("Settings_WebMessage_Subscribe", false);
        return;
    };

    // The page always posts (ignoring any error when the API is gated off) and
    // signals via its title that it finished running.
    const PAGE: &str = "<!DOCTYPE html><html><body><script>\
        try { window.chrome.webview.postMessage('hi'); } catch (e) {}\
        document.title = 'ran';\
        </script></body></html>";

    // Disabled: the page runs and posts, but nothing reaches the host.
    if settings.set_web_message_enabled(false).is_err() {
        harness.check("Settings_WebMessage_Disable", false);
        drop(registration);
        return;
    }
    *received.borrow_mut() = None;
    harness.navigate_html(PAGE);
    let ran = harness.wait(|| harness.webview().document_title() == "ran");
    // Give any (erroneously delivered) message time to arrive before asserting.
    harness.pump_until(|| false, Duration::from_millis(500));
    harness.check(
        "Settings_WebMessage_DisabledNotReceived",
        ran && received.borrow().is_none(),
    );

    // Enabled: the message is delivered.
    if settings.set_web_message_enabled(true).is_err() {
        harness.check("Settings_WebMessage_Enable", false);
        drop(registration);
        return;
    }
    *received.borrow_mut() = None;
    harness.navigate_html(PAGE);
    let got = harness.wait(|| received.borrow().is_some());
    harness.check(
        "Settings_WebMessage_EnabledReceived",
        got && received.borrow().as_deref() == Some("hi"),
    );

    drop(registration);
}
