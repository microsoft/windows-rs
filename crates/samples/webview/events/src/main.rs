use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

fn main() -> Result<()> {
    run("WebView2 events - windows-rs", |_controller, webview| {
        let completed = webview.clone();

        let registrations = vec![
            webview.on_navigation_starting(|args| {
                println!(
                    "navigation {} starting: {} (user initiated = {})",
                    args.navigation_id(),
                    args.uri(),
                    args.is_user_initiated()
                );
            })?,
            webview.on_content_loading(|args| {
                println!(
                    "content loading: navigation {} (error page = {})",
                    args.navigation_id(),
                    args.is_error_page()
                );
            })?,
            webview.on_navigation_completed(move |args| {
                println!(
                    "navigation {} completed: success = {}, title = {:?}",
                    args.navigation_id(),
                    args.is_success(),
                    completed.document_title()
                );
            })?,
            webview.on_document_title_changed(|title| {
                println!("document title changed: {title}");
            })?,
            webview.on_new_window_requested(|args| {
                println!("blocking new window for: {}", args.uri());
                args.set_handled(true).unwrap();
            })?,
            webview.on_permission_requested(|args| {
                println!("denying {:?} permission for {}", args.kind(), args.uri());
                args.set_state(PermissionState::Deny).unwrap();
            })?,
            webview.on_window_close_requested(|| {
                println!("page requested window close");
            })?,
            webview.on_process_failed(|args| {
                println!("process failed: {:?}", args.kind());
            })?,
        ];

        webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
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
