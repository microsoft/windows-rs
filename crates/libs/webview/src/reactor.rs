use super::*;
use crate::reactor_bindings::{CoreWebView2, IFrameworkElement, IWebView2};
use std::cell::RefCell;
use std::rc::Rc;

/// Hosts a WebView2 in a [`windows-reactor`](windows_reactor) UI tree.
///
/// Calls `on_ready` on the UI thread after the XAML control initializes.
pub fn webview(on_ready: impl Fn(WebView) + 'static) -> windows_reactor::WebView2 {
    let on_ready: Rc<dyn Fn(WebView)> = Rc::new(on_ready);
    let state: Rc<RefCell<Mounted>> = Rc::new(RefCell::new(Mounted::default()));

    let mount_state = state.clone();
    windows_reactor::web_view2()
        .on_mounted(move |handle| {
            let inspectable = handle.as_inspectable().clone();
            let Ok(element) = inspectable.cast::<IFrameworkElement>() else {
                return;
            };

            // `CoreWebView2` cannot be created until the XAML control is loaded.
            let begin = make_begin(on_ready.clone(), mount_state.clone());
            if element.IsLoaded().unwrap_or(false) {
                begin(&inspectable);
            } else if let Ok(revoker) = element.Loaded(move |sender, _args| {
                if let Some(sender) = sender.as_ref() {
                    begin(sender);
                }
            }) {
                mount_state.borrow_mut().loaded = Some(revoker);
            }
        })
        .on_unmounted(move |_| {
            *state.borrow_mut() = Mounted::default();
        })
}

fn make_begin(
    on_ready: Rc<dyn Fn(WebView)>,
    state: Rc<RefCell<Mounted>>,
) -> Rc<dyn Fn(&IInspectable)> {
    Rc::new(move |inspectable: &IInspectable| {
        if state.borrow().action.is_some() {
            return;
        }
        let Ok(control) = inspectable.cast::<IWebView2>() else {
            return;
        };

        let on_ready = on_ready.clone();
        let registration = control.CoreWebView2Initialized(move |sender, _args| {
            if let Some(sender) = sender.as_ref()
                && let Ok(control) = sender.cast::<IWebView2>()
                && let Some(web) = bridge(&control)
            {
                on_ready(web);
            }
        });

        let mut state = state.borrow_mut();
        if let Ok(registration) = registration {
            state.revoker = Some(registration);
        }
        // Keep the async action alive until initialization completes.
        if let Ok(action) = control.EnsureCoreWebView2Async() {
            state.action = Some(action);
        }
    })
}

#[derive(Default)]
struct Mounted {
    loaded: Option<EventRevoker>,
    revoker: Option<EventRevoker>,
    action: Option<windows_future::IAsyncAction>,
}

fn bridge(control: &IWebView2) -> Option<WebView> {
    let core: CoreWebView2 = control.CoreWebView2().ok()?;
    let interop: ICoreWebView2Interop2 = core.cast().ok()?;
    let com_core: ICoreWebView2 = unsafe { interop.GetComICoreWebView2().ok()? };
    Some(WebView::from_core(com_core))
}
