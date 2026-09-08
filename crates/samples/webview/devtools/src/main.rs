use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

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
    run(
        "WebView2 DevTools protocol - windows-rs",
        |_controller, webview| {
            webview.call_dev_tools_protocol_method("Browser.getVersion", "{}", |result| {
                println!("Browser.getVersion -> {result:?}");
            })?;

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
