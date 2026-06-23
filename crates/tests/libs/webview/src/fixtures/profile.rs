//! Profile fixtures: private mode, colour scheme, and download folder.

use windows_webview::{ControllerOptions, PreferredColorScheme};
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

/// The preferred colour scheme round-trips through the setter and getter.
pub fn color_scheme_round_trip(harness: &Harness) {
    let Ok(profile) = harness.webview().profile() else {
        harness.check("Profile_ColorScheme_Get", false);
        return;
    };

    let original = profile.preferred_color_scheme();
    harness.check(
        "Profile_ColorScheme_Set",
        profile
            .set_preferred_color_scheme(PreferredColorScheme::Dark)
            .is_ok(),
    );
    harness.check(
        "Profile_ColorScheme_Getter",
        profile.preferred_color_scheme() == PreferredColorScheme::Dark,
    );
    let _ = profile.set_preferred_color_scheme(original);
}

/// The profile exposes a non-empty default download folder path.
pub fn default_download_folder_path(harness: &Harness) {
    let Ok(profile) = harness.webview().profile() else {
        harness.check("Profile_DownloadFolder_Get", false);
        return;
    };
    harness.check(
        "Profile_DownloadFolder_NonEmpty",
        !profile.default_download_folder_path().is_empty(),
    );
}
