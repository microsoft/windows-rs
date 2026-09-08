use std::cell::RefCell;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

fn main() -> Result<()> {
    run("WebView2 cookies - windows-rs", |_controller, webview| {
        let cookies = webview.cookie_manager()?;

        let mut cookie = Cookie::new("session", "abc123", "example.com", "/");
        cookie.is_http_only = true;
        cookie.same_site = SameSite::Strict;
        cookies.add_or_update_cookie(&cookie)?;

        cookies.get_cookies("https://example.com/", |result| match result {
            Ok(cookies) => {
                println!("{} cookie(s) for example.com:", cookies.len());
                for cookie in cookies {
                    println!("  {} = {}", cookie.name, cookie.value);
                }
            }
            Err(error) => println!("failed to read cookies: {error}"),
        })?;

        webview.navigate("https://example.com/")?;
        Ok(Vec::new())
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
