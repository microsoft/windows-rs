//! Environment fixture: a custom-options environment hosts a working browser.

use windows_webview::{Environment, EnvironmentOptions};
use windows_window::Window;

use crate::harness::Harness;

/// `Environment::with_options` creates a usable environment whose controller
/// hosts a browser that navigates. This guards the options object's default
/// `TargetCompatibleBrowserVersion`, which WebView2 rejects when empty.
pub fn with_options_creates_environment(harness: &Harness) {
    // A distinct user-data folder is required: WebView2 rejects a second
    // environment that shares the primary environment's default folder but
    // carries different browser arguments.
    let user_data = std::env::temp_dir().join("windows-webview-selftest");
    let options = EnvironmentOptions::new()
        .user_data_folder(user_data.to_string_lossy())
        .additional_browser_arguments("--disable-features=msSmartScreenProtection");
    let Ok(environment) = Environment::with_options(&options) else {
        harness.check("Environment_WithOptions", false);
        return;
    };

    let Ok(window) = Window::new("selftest-options").size(640, 480).create() else {
        harness.check("Environment_Window", false);
        return;
    };

    let Ok(controller) = (unsafe { environment.create_controller(window.hwnd()) }) else {
        harness.check("Environment_Controller", false);
        return;
    };

    match controller.webview() {
        Ok(webview) => harness.check(
            "Environment_WithOptions",
            webview
                .navigate_to_string("<!DOCTYPE html><html></html>")
                .is_ok(),
        ),
        Err(_) => harness.check("Environment_WithOptions", false),
    }

    let _ = controller.close();
}
