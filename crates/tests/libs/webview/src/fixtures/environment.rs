use windows_webview::{Environment, EnvironmentOptions};
use windows_window::Window;

use crate::harness::Harness;

// WebView2 rejects an empty default `TargetCompatibleBrowserVersion`.
pub fn with_options_creates_environment(harness: &Harness) {
    // WebView2 rejects different browser arguments for a shared user-data folder.
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

    let Ok(controller) = environment.create_controller(&window) else {
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
