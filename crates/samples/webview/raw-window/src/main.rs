use std::cell::RefCell;
use std::rc::Rc;
use windows_core::{Error, Result};
use windows_webview::WebViewHost;
use windows_window::Window;

const WM_CLOSE: u32 = 0x0010;

#[derive(Default)]
struct App {
    host: Option<Rc<WebViewHost>>,
    failure: Option<Error>,
    pending: bool,
    close_requested: bool,
}

fn main() -> Result<()> {
    let _apartment = windows_core::init_sta()?;
    let app = Rc::new(RefCell::new(App {
        pending: true,
        ..Default::default()
    }));

    let message_app = Rc::clone(&app);
    let resize_app = Rc::clone(&app);
    let move_app = Rc::clone(&app);
    let close_app = Rc::clone(&app);
    let window = Rc::new(
        Window::new("WebView2 raw window - windows-rs")
            .client_size(1024, 768)
            .on_message(move |_hwnd, message, _wparam, _lparam| {
                let mut app = message_app.borrow_mut();
                if message == WM_CLOSE && app.pending {
                    app.close_requested = true;
                    Some(0)
                } else {
                    None
                }
            })
            .on_resize(move |width, height| {
                let host = resize_app.borrow().host.clone();
                if let Some(host) = host {
                    _ = host.controller().set_bounds(0, 0, width, height);
                }
            })
            .on_move(move || {
                let host = move_app.borrow().host.clone();
                if let Some(host) = host {
                    _ = host.controller().notify_parent_window_position_changed();
                }
            })
            .on_close(move || {
                let host = close_app.borrow_mut().host.take();
                if let Some(host) = host {
                    _ = host.close();
                }
            })
            .create()?,
    );

    let completed_app = Rc::clone(&app);
    let completed_window = Rc::clone(&window);
    WebViewHost::builder().create_for_hwnd(window.hwnd(), move |result| {
        let close_requested = {
            let mut app = completed_app.borrow_mut();
            app.pending = false;
            app.close_requested
        };
        if close_requested {
            if let Ok(host) = result {
                _ = host.close();
            }
            completed_window.close();
            return;
        }

        let result = result.and_then(|host| {
            let (width, height) = completed_window.client_size();
            host.controller().set_bounds(0, 0, width, height)?;
            host.webview()
                .navigate("https://learn.microsoft.com/windows/dev-environment/")?;
            completed_app.borrow_mut().host = Some(Rc::new(host));
            Ok(())
        });
        if let Err(error) = result {
            completed_app.borrow_mut().failure = Some(error);
            completed_window.close();
        }
    })?;

    windows_window::run();
    let host = app.borrow_mut().host.take();
    if let Some(host) = &host {
        host.close()?;
    }
    drop(host);
    drop(window);

    match app.borrow_mut().failure.take() {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
