use windows_webview::*;

const PAGE: &str = r#"<!DOCTYPE html><html><head>
<meta name="color-scheme" content="light dark">
<style>
:root { color-scheme: light dark; }
body {
    margin: 2rem;
    font-family: "Segoe UI", sans-serif;
    background: Canvas;
    color: CanvasText;
}
</style>
</head><body>
<h1>windows-webview DevTools protocol</h1>
<p>The host is driving this page over the Chrome DevTools Protocol.</p>
<script>console.log('hello from the page');</script>
</body></html>"#;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 DevTools protocol - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            webview.call_dev_tools_protocol_method("Browser.getVersion", "{}", |result| {
                println!("Browser.getVersion -> {result:?}");
            })?;

            webview.call_dev_tools_protocol_method("Runtime.enable", "{}", |_| {})?;

            let registration =
                webview.on_dev_tools_protocol_event("Runtime.consoleAPICalled", |args| {
                    println!("console event -> {}", args.parameter_object_as_json());
                })?;

            webview.navigate_to_string(PAGE)?;
            host.retain(registration);
            Ok(())
        })
}
