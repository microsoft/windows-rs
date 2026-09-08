use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

fn main() -> Result<()> {
    let options = ControllerOptions::new()
        .profile_name("sample")
        .in_private_mode(true);

    run_with_options(
        "WebView2 profile - windows-rs",
        options,
        |_controller, webview| {
            let profile = webview.profile()?;
            println!(
                "profile {:?} (in private mode = {})",
                profile.name(),
                profile.is_in_private_mode()
            );
            profile.set_preferred_color_scheme(PreferredColorScheme::Dark)?;
            profile.clear_browsing_data_all(|result| {
                println!("clear browsing data: {result:?}");
            })?;

            webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
            Ok(Vec::new())
        },
    )
}

fn run_with_options<F>(title: &str, options: ControllerOptions, setup: F) -> Result<()>
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
    let handle = match environment.create_controller_with_options(&window, &options) {
        Ok(handle) => handle,
        Err(error) if error.code().0 == E_ABORT || error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    let (width, height) = window.client_size();
    handle.set_bounds(0, 0, width, height)?;

    let webview = handle.webview()?;
    let registrations = setup(&handle, &webview)?;

    *controller.borrow_mut() = Some(handle);
    run();
    drop(registrations);
    Ok(())
}

const E_ABORT: i32 = 0x8000_4004u32 as i32;
