// TODO: remove when done

use super::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Hosts a WebView2 in a [`windows-reactor`](windows_reactor) UI tree.
///
/// Calls `on_ready` on the UI thread after the XAML control initializes.
pub fn webview(on_ready: impl Fn(WebView) + 'static) -> windows_reactor::WebView2 {
    let on_ready: Rc<dyn Fn(WebView)> = Rc::new(on_ready);
    let state = Rc::new(RefCell::new(None));

    let mount_state = Rc::clone(&state);
    windows_reactor::web_view2()
        .on_mounted(move |handle| {
            let on_ready = Rc::clone(&on_ready);
            *mount_state.borrow_mut() =
                XamlWebViewHost::new(handle.as_inspectable(), move |result| {
                    if let Ok(webview) = result {
                        on_ready(webview);
                    }
                })
                .ok();
        })
        .on_unmounted(move |_| {
            *state.borrow_mut() = None;
        })
}
