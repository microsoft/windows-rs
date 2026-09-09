use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;

const PAGE: &str = r#"<!DOCTYPE html><html><body>
<h1>windows-webview script injection</h1>
<p>This page ships no script of its own; the host injected one.</p>
</body></html>"#;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 script injection - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            let id = Rc::new(RefCell::new(None));
            let remove_id = Rc::clone(&id);
            let reader = webview.clone();
            let remover = webview.clone();
            let registration = webview.on_navigation_completed(move |args| {
                if !args.is_success() {
                    return;
                }

                reader
                    .execute_script("String(window.injectedAt)", |result| {
                        println!("injected timestamp read from page: {result:?}");
                    })
                    .unwrap();

                if let Some(id) = remove_id.borrow_mut().take() {
                    remover
                        .remove_script_to_execute_on_document_created(&id)
                        .unwrap();
                }
            })?;

            let navigate = webview.clone();
            webview.add_script_to_execute_on_document_created(
                r#"window.injectedAt = Date.now();
                   document.addEventListener('DOMContentLoaded', () => {
                       document.documentElement.style.background = '#1e1e2e';
                       document.documentElement.style.color = '#cdd6f4';
                   });"#,
                move |result| {
                    let script = result.unwrap();
                    println!("registered document-created script: {}", script.as_str());
                    *id.borrow_mut() = Some(script);
                    navigate.navigate_to_string(PAGE).unwrap();
                },
            )?;
            host.retain(registration);
            Ok(())
        })
}
