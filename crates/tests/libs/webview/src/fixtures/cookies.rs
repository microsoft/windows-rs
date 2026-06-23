//! Cookie manager fixture: add a cookie and read it back.

use std::cell::RefCell;
use std::rc::Rc;

use windows_core::Result;
use windows_webview::Cookie;

use crate::harness::Harness;

/// A cookie written with `add_or_update_cookie` is returned by `get_cookies`.
pub fn add_and_get(harness: &Harness) {
    let Ok(manager) = harness.webview().cookie_manager() else {
        harness.check("Cookie_Manager", false);
        return;
    };

    let cookie = Cookie::new("selftest", "value1", "example.com", "/");
    if manager.add_or_update_cookie(&cookie).is_err() {
        harness.check("Cookie_Add", false);
        return;
    }

    let result: Rc<RefCell<Option<Result<Vec<Cookie>>>>> = Rc::new(RefCell::new(None));
    let sink = result.clone();
    if manager
        .get_cookies("http://example.com/", move |cookies| {
            *sink.borrow_mut() = Some(cookies);
        })
        .is_err()
    {
        harness.check("Cookie_Get", false);
        return;
    }

    let got = harness.wait(|| result.borrow().is_some());
    let found = matches!(
        result.borrow().as_ref(),
        Some(Ok(cookies)) if cookies.iter().any(|c| c.name == "selftest" && c.value == "value1")
    );
    harness.check("Cookie_Found", got && found);
}
