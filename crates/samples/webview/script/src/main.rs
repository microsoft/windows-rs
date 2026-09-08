use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

const PAGE: &str = r#"<!DOCTYPE html><html><body>
<h1>windows-webview script injection</h1>
<p>This page ships no script of its own; the host injected one.</p>
</body></html>"#;

fn main() -> Result<()> {
    run(
        "WebView2 script injection - windows-rs",
        |_controller, webview| {
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

                reader
                    .execute_script("String(window.injectedAt)", |result| {
                        println!("injected timestamp read from page: {result:?}");
                    })
                    .unwrap();

                remover
                    .remove_script_to_execute_on_document_created(&id)
                    .unwrap();
            })?;

            webview.navigate_to_string(PAGE)?;
            Ok(vec![registration])
        },
    )
}

fn run<F>(title: &str, setup: F) -> Result<()>
where
    F: FnOnce(&Controller, &WebView) -> Result<Vec<EventRegistration>>,
{
    let controller: Rc<RefCell<Option<Controller>>> = Rc::new(RefCell::new(None));

    let resize = controller.clone();
    let window = Window::new(title)
        .size(1024, 768)
        .on_resize(move |width, height| {
            if let Some(controller) = resize.borrow().as_ref() {
                controller.set_bounds(0, 0, width, height).unwrap();
            }
        })
        .create()?;

    // Host destruction reports E_ABORT or the message loop's success-coded empty error.
    let environment = match Environment::new() {
        Ok(environment) => environment,
        Err(error) if error.code().0 == E_ABORT || error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    let handle = match environment.create_controller(&window) {
        Ok(handle) => handle,
        Err(error) if error.code().0 == E_ABORT || error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    let (width, height) = window.client_size();
    handle.set_bounds(0, 0, width, height)?;

    let webview = handle.webview()?;
    let registrations = setup(&handle, &webview)?;

    *controller.borrow_mut() = Some(handle);
    windows_window::run();
    drop(registrations);
    Ok(())
}

const E_ABORT: i32 = 0x8000_4004u32 as i32;
