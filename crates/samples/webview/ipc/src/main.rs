use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

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
