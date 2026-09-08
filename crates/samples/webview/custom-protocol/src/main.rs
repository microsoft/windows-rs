use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

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
