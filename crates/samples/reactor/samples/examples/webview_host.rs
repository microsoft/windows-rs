#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, Grid, GridChild, GridLength, RenderCx, TextBlock, WebViewHost, hstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let status = cx.use_state(|| "Creating WebView2...".to_string());
    let webview = cx.use_webview_ref();
    let back = webview.clone();
    let reload = webview.clone();
    let created_status = status.clone();
    let navigation_status = status.clone();

    Grid::new([
        GridChild::new(
            WebViewHost::new(&webview)
                .source("https://example.com")
                .on_created(move |result| {
                    created_status.set(match result {
                        Ok(()) => "WebView2 ready".to_string(),
                        Err(error) => format!("WebView2 creation failed: {error}"),
                    });
                })
                .on_navigation_completed(move |navigation| {
                    navigation_status.set(navigation.source);
                })
                .build(),
        )
        .row(0),
        GridChild::new(hstack(
            8.0,
            [
                Button::new("Back")
                    .on_click(move || {
                        back.go_back();
                    })
                    .build(),
                Button::new("Reload")
                    .on_click(move || {
                        reload.reload();
                    })
                    .build(),
            ],
        ))
        .row(1),
        GridChild::new(TextBlock::new(status.value()).build()).row(2),
    ])
    .rows([GridLength::STAR, GridLength::Auto, GridLength::Auto])
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Typed WebView Host", app)
}
