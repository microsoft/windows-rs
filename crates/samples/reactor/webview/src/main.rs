#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx};

fn app(cx: &mut RenderCx<'_>) -> Element {
    reactor_samples::webview_host::app(cx)
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("WebView Host", app)
}
