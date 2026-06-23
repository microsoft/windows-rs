//! Profile fixture: a controller created in private mode reports it.

use windows_webview::ControllerOptions;
use windows_window::Window;

use crate::harness::Harness;

/// A controller created with `in_private_mode` exposes a private profile.
pub fn private_mode(harness: &Harness) {
    let Ok(window) = Window::new("selftest-private").size(640, 480).create() else {
        harness.check("Profile_Window", false);
        return;
    };

    let options = ControllerOptions::new().in_private_mode(true);
    let Ok(controller) = (unsafe {
        harness
            .environment()
            .create_controller_with_options(window.hwnd(), &options)
    }) else {
        harness.check("Profile_Controller", false);
        return;
    };

    let Ok(webview) = controller.webview() else {
        harness.check("Profile_WebView", false);
        return;
    };

    match webview.profile() {
        Ok(profile) => harness.check("Profile_PrivateMode", profile.is_in_private_mode()),
        Err(_) => harness.check("Profile_PrivateMode", false),
    }

    let _ = controller.close();
}
