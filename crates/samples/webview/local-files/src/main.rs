use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 local files - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            let folder = concat!(env!("CARGO_MANIFEST_DIR"), "\\web");
            webview.set_virtual_host_name_to_folder_mapping(
                "app.example",
                folder,
                HostResourceAccessKind::Deny,
            )?;

            webview.navigate("https://app.example/index.html")?;
            Ok(())
        })
}
