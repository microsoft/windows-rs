//! The smallest host: create a webview and navigate to a page.

use webview_minimal::*;

fn main() -> Result<()> {
    run("WebView2 minimal - windows-rs", |_controller, webview| {
        webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
        Ok(Vec::new())
    })
}
