//! Host ↔ JavaScript messaging: an injected bootstrap script, received
//! messages, replies from Rust, and `execute_script`.

use webview_samples::*;

const PAGE: &str = r#"<!DOCTYPE html><html><body>
<h1>windows-webview IPC</h1>
<button onclick="chrome.webview.postMessage('ping from page')">Send to host</button>
<pre id="log"></pre>
<script>
  chrome.webview.addEventListener('message', e => {
    document.getElementById('log').textContent += 'host says: ' + e.data + '\n';
  });
</script>
</body></html>"#;

fn main() -> Result<()> {
    run("WebView2 IPC - windows-rs", |_controller, webview| {
        webview.add_script_to_execute_on_document_created(
            "chrome.webview.postMessage('document created: ' + location.href);",
        )?;

        let reply = webview.clone();
        let script = webview.clone();

        let registrations = vec![
            webview.on_web_message_received(move |args| {
                let message = args.web_message_as_json();
                println!("page sent: {message}");
                reply
                    .post_web_message_as_string(&format!("echo {message}"))
                    .unwrap();
            })?,
            webview.on_navigation_completed(move |args| {
                if args.is_success() {
                    script
                        .execute_script("document.title", |result| {
                            println!("execute_script returned: {result:?}");
                        })
                        .unwrap();
                }
            })?,
        ];

        webview.navigate_to_string(PAGE)?;
        Ok(registrations)
    })
}
