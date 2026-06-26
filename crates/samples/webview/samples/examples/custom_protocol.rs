//! Serving an app entirely from memory with `on_web_resource_requested`.

use webview_samples::*;

const INDEX: &str = r#"<!DOCTYPE html><html><head>
<link rel="stylesheet" href="https://app.example/style.css">
</head><body>
<h1>Served from Rust memory</h1>
<p>This page and its stylesheet were produced by a custom protocol handler,
not fetched from the network.</p>
</body></html>"#;

const STYLE: &str =
    "body { font-family: Segoe UI, sans-serif; margin: 3rem; } h1 { color: #0078d4; }";

fn main() -> Result<()> {
    run(
        "WebView2 custom protocol - windows-rs",
        |_controller, webview| {
            let protocol =
                webview.on_web_resource_requested("https://app.example/*", |request| {
                    println!("serving from memory: {}", request.uri());
                    if request.uri().ends_with("/style.css") {
                        Some(WebResourceResponse::new(STYLE).content_type("text/css"))
                    } else {
                        Some(WebResourceResponse::new(INDEX).content_type("text/html"))
                    }
                })?;

            webview.navigate("https://app.example/")?;
            Ok(vec![protocol])
        },
    )
}
