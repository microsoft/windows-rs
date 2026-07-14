//! Chrome DevTools Protocol: call a CDP method for browser-level information and
//! subscribe to a CDP event, the programmatic automation channel that reaches
//! past page JavaScript.

use webview_samples::*;

const PAGE: &str = r#"<!DOCTYPE html><html><body>
<h1>windows-webview DevTools protocol</h1>
<p>The host is driving this page over the Chrome DevTools Protocol.</p>
<script>console.log('hello from the page');</script>
</body></html>"#;

fn main() -> Result<()> {
    run(
        "WebView2 DevTools protocol - windows-rs",
        |_controller, webview| {
            // Browser-level info that page script cannot reach.
            webview.call_dev_tools_protocol_method("Browser.getVersion", "{}", |result| {
                println!("Browser.getVersion -> {result:?}");
            })?;

            // Console messages are only reported over CDP once the Runtime domain
            // is enabled.
            webview.call_dev_tools_protocol_method("Runtime.enable", "{}", |_| {})?;

            let registration =
                webview.on_dev_tools_protocol_event("Runtime.consoleAPICalled", |args| {
                    println!("console event -> {}", args.parameter_object_as_json());
                })?;

            webview.navigate_to_string(PAGE)?;
            Ok(vec![registration])
        },
    )
}
