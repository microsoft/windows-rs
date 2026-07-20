//! Script injection lifecycle: register a script that runs before every
//! document, read a value it computed back with `execute_script`, then remove
//! the registration.

use webview_samples::*;

const PAGE: &str = r#"<!DOCTYPE html><html><body>
<h1>windows-webview script injection</h1>
<p>This page ships no script of its own; the host injected one.</p>
</body></html>"#;

fn main() -> Result<()> {
    run(
        "WebView2 script injection - windows-rs",
        |_controller, webview| {
            // Runs before any page script at document creation, before the HTML
            // is parsed. Stamp a global immediately (so the host can read it
            // back), and defer DOM styling until the document exists.
            let id = webview.add_script_to_execute_on_document_created(
                r#"window.injectedAt = Date.now();
                   document.addEventListener('DOMContentLoaded', () => {
                       document.documentElement.style.background = '#1e1e2e';
                       document.documentElement.style.color = '#cdd6f4';
                   });"#,
            )?;
            println!("registered document-created script: {}", id.as_str());

            let reader = webview.clone();
            let remover = webview.clone();

            let registration = webview.on_navigation_completed(move |args| {
                if !args.is_success() {
                    return;
                }

                // The injected global exists because the script ran first.
                reader
                    .execute_script("String(window.injectedAt)", |result| {
                        println!("injected timestamp read from page: {result:?}");
                    })
                    .unwrap();

                // Later navigations should no longer inject the script.
                remover
                    .remove_script_to_execute_on_document_created(&id)
                    .unwrap();
            })?;

            webview.navigate_to_string(PAGE)?;
            Ok(vec![registration])
        },
    )
}
