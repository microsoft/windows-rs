use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 minimal - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
            Ok(())
        })
}
