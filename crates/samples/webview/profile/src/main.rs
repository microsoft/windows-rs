use windows_webview::*;

fn main() -> Result<()> {
    let options = ControllerOptions::new()
        .profile_name("sample")
        .in_private_mode(true);

    WebViewWindow::new("WebView2 profile - windows-rs")
        .size(1024, 768)
        .controller_options(options)
        .run(|host| {
            let webview = host.webview();
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
            Ok(())
        })
}
