//! An in-private controller, the dark color scheme, and clearing browsing data.

use webview_samples::*;

fn main() -> Result<()> {
    let options = ControllerOptions::new()
        .profile_name("sample")
        .in_private_mode(true);

    run_with_options(
        "WebView2 profile - windows-rs",
        options,
        |_controller, webview| {
            let profile = webview.profile()?;
            println!(
                "profile {:?} (in private mode = {})",
                profile.name(),
                profile.is_in_private_mode()
            );
            profile.set_preferred_color_scheme(PreferredColorScheme::Dark)?;
            profile.clear_browsing_data_all(|result| {
                println!("clear browsing data: {result:?}");
            })?;

            webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
            Ok(Vec::new())
        },
    )
}
