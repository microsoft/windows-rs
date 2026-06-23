//! Hosts a WebView2 browser inside a `windows-reactor` window.
//!
//! The reactor owns the WinUI XAML `WebView2` control; `windows-webview`'s
//! `reactor` feature bridges it to the same COM `ICoreWebView2` surface used by
//! the standalone crate, so the [`WebView`](windows_webview::webview) handed to
//! `on_ready` drives navigation, scripting, and every other wrapper identically.

#![windows_subsystem = "windows"]

use windows_reactor::*;
use windows_webview::webview;

fn app(_cx: &mut RenderCx) -> Element {
    webview(|web| {
        web.navigate("https://learn.microsoft.com/windows/apps/")
            .unwrap();
    })
    .into()
}

fn main() -> Result<()> {
    App::new().title("WebView2").render(app)
}
