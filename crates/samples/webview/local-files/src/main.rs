use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

fn main() -> Result<()> {
    run(
        "WebView2 local files - windows-rs",
        |_controller, webview| {
            let folder = concat!(env!("CARGO_MANIFEST_DIR"), "\\web");
            webview.set_virtual_host_name_to_folder_mapping(
                "app.example",
                folder,
                HostResourceAccessKind::Deny,
            )?;

            webview.navigate("https://app.example/index.html")?;
            Ok(Vec::new())
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
