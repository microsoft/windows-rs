//! Shared scaffold for the `windows-webview` examples.
//!
//! Each example in `examples/` demonstrates one capability. They all share the
//! same hosting boilerplate — a [`windows_window`] window whose resize forwards
//! to [`Controller::set_bounds`], an [`Environment`], and a [`Controller`] — so
//! that boilerplate lives here and each example stays focused on its feature.
//!
//! Requires the Microsoft Edge WebView2 runtime to be installed.

use std::cell::RefCell;
use std::rc::Rc;
use windows_window::Window;

pub use windows_webview::*;
pub use windows_window::Result;

/// Hosts a WebView in a window titled `title` and runs the message loop.
///
/// `setup` receives the [`Controller`] and [`WebView`] once they are ready; it
/// wires the example's feature, navigates, and returns any
/// [`EventRegistration`]s to keep alive for the lifetime of the window.
pub fn run<F>(title: &str, setup: F) -> Result<()>
where
    F: FnOnce(&Controller, &WebView) -> Result<Vec<EventRegistration>>,
{
    run_core(title, setup, |environment, hwnd| unsafe {
        environment.create_controller(hwnd)
    })
}

/// Like [`run`], but creates the controller with the given [`ControllerOptions`]
/// (profile name, private mode, initial background colour).
pub fn run_with_options<F>(title: &str, options: ControllerOptions, setup: F) -> Result<()>
where
    F: FnOnce(&Controller, &WebView) -> Result<Vec<EventRegistration>>,
{
    run_core(title, setup, move |environment, hwnd| unsafe {
        environment.create_controller_with_options(hwnd, &options)
    })
}

fn run_core<F, C>(title: &str, setup: F, create: C) -> Result<()>
where
    F: FnOnce(&Controller, &WebView) -> Result<Vec<EventRegistration>>,
    C: FnOnce(&Environment, HWND) -> Result<Controller>,
{
    let controller: Rc<RefCell<Option<Controller>>> = Rc::new(RefCell::new(None));

    let resize = controller.clone();
    let window = Window::new(title)
        .size(1024, 768)
        .on_resize(move |width, height| {
            if let Some(controller) = resize.borrow().as_ref() {
                _ = controller.set_bounds(0, 0, width, height);
            }
        })
        .create()?;

    let environment = Environment::new()?;
    let handle = create(&environment, window.hwnd())?;
    let (width, height) = window.client_size();
    handle.set_bounds(0, 0, width, height)?;

    let webview = handle.webview()?;
    let registrations = setup(&handle, &webview)?;

    *controller.borrow_mut() = Some(handle);
    windows_window::run();
    drop(registrations);
    Ok(())
}
