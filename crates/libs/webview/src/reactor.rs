use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use windows_reactor::{
    Callback, Component, ComponentContext, ElementRef, IntegrationError, IntoPayloadCallback, View,
    ViewContext, WebView2,
};

/// Hosts a WebView2 in a [`windows-reactor`](windows_reactor) UI tree.
///
/// Calls `on_ready` on the UI thread after the XAML control initializes and fails fast if
/// initialization fails.
pub fn webview(on_ready: impl IntoPayloadCallback<WebView>) -> View {
    let on_ready = on_ready.into_payload_callback();
    webview_result(move |result| match result {
        Ok(webview) => {
            _ = on_ready.call(webview);
        }
        Err(IntegrationError::Native(code)) => {
            panic!("windows-webview Reactor integration failed: HRESULT({code:#010X})");
        }
        Err(IntegrationError::Unavailable) => {}
    })
}

/// Hosts a WebView2 and reports initialization success or failure on the UI thread.
pub fn webview_result(
    on_ready: impl IntoPayloadCallback<std::result::Result<WebView, IntegrationError>>,
) -> View {
    View::component::<WebViewHost>(on_ready.into_payload_callback())
}

struct WebViewHost {
    control: ElementRef<WebView2>,
    on_ready: Rc<RefCell<Callback<std::result::Result<WebView, IntegrationError>>>>,
}

impl Component for WebViewHost {
    type Input = Callback<std::result::Result<WebView, IntegrationError>>;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            control: ElementRef::new(),
            on_ready: Rc::new(RefCell::new(input.clone())),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        *self.on_ready.borrow_mut() = input.clone();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let control = self.control.clone();
        let on_ready = Rc::clone(&self.on_ready);
        context.use_effect("initialize", (), move || {
            let callback = Rc::clone(&on_ready);
            let accepted = control.request_core_web_view2(move |result| {
                let result = result.and_then(|core| bridge(&core));
                let callback = callback.borrow().clone();
                _ = callback.call(result);
            });
            if !accepted {
                let callback = on_ready.borrow().clone();
                _ = callback.call(Err(IntegrationError::Unavailable));
            }
            None
        });
        WebView2::new().element_ref(&self.control).into()
    }
}

fn bridge(core: &IUnknown) -> std::result::Result<WebView, IntegrationError> {
    let interop: ICoreWebView2Interop2 = core.cast().map_err(integration_error)?;
    let com_core: ICoreWebView2 =
        unsafe { interop.GetComICoreWebView2() }.map_err(integration_error)?;
    Ok(WebView::from_core(com_core))
}

fn integration_error(error: Error) -> IntegrationError {
    IntegrationError::Native(error.code().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_errors_preserve_the_hresult() {
        let code = HRESULT(0x8000_4005_u32 as i32);
        assert_eq!(
            integration_error(Error::from_hresult(code)),
            IntegrationError::Native(code.0)
        );
    }
}
