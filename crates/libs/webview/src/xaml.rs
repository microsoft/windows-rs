use super::*;
use crate::reactor_bindings::{CoreWebView2, IFrameworkElement, IWebView2};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[doc(hidden)]
pub struct XamlWebViewHost {
    state: Rc<RefCell<Mounted>>,
    live: Rc<Cell<bool>>,
}

impl XamlWebViewHost {
    pub fn new(
        control: &IInspectable,
        on_ready: impl FnOnce(Result<WebView>) + 'static,
    ) -> Result<Self> {
        let element = control.cast::<IFrameworkElement>()?;
        let state = Rc::new(RefCell::new(Mounted::default()));
        let live = Rc::new(Cell::new(true));
        let callback = Rc::new(RefCell::new(Some(on_ready)));

        if element.IsLoaded()? {
            begin(control, &state, &live, &callback)?;
        } else {
            let begin_state = Rc::clone(&state);
            let begin_live = Rc::clone(&live);
            let begin_callback = Rc::clone(&callback);
            let loaded = element.Loaded(move |sender, _args| {
                if begin_live.get()
                    && let Some(sender) = sender.as_ref()
                    && let Err(error) = begin(sender, &begin_state, &begin_live, &begin_callback)
                    && let Some(callback) = begin_callback.borrow_mut().take()
                {
                    callback(Err(error));
                }
            })?;
            state.borrow_mut().loaded = Some(loaded);
        }

        Ok(Self { state, live })
    }
}

impl Drop for XamlWebViewHost {
    fn drop(&mut self) {
        self.live.set(false);
        *self.state.borrow_mut() = Mounted::default();
    }
}

fn begin<F>(
    inspectable: &IInspectable,
    state: &Rc<RefCell<Mounted>>,
    live: &Rc<Cell<bool>>,
    callback: &Rc<RefCell<Option<F>>>,
) -> Result<()>
where
    F: FnOnce(Result<WebView>) + 'static,
{
    if state.borrow().action.is_some() {
        return Ok(());
    }
    let control = inspectable.cast::<IWebView2>()?;
    let ready_live = Rc::clone(live);
    let ready_callback = Rc::clone(callback);
    let registration = control.CoreWebView2Initialized(move |sender, _args| {
        if !ready_live.get() {
            return;
        }
        let result = sender
            .as_ref()
            .ok_or_else(|| Error::from_hresult(HRESULT(0x80004003_u32 as i32)))
            .and_then(|sender| sender.cast::<IWebView2>())
            .and_then(|control| bridge(&control));
        if let Some(callback) = ready_callback.borrow_mut().take() {
            callback(result);
        }
    })?;
    let action = control.EnsureCoreWebView2Async()?;
    let mut state = state.borrow_mut();
    state.revoker = Some(registration);
    state.action = Some(action);
    Ok(())
}

#[derive(Default)]
struct Mounted {
    loaded: Option<EventRevoker>,
    revoker: Option<EventRevoker>,
    action: Option<windows_future::IAsyncAction>,
}

fn bridge(control: &IWebView2) -> Result<WebView> {
    let core: CoreWebView2 = control.CoreWebView2()?;
    let interop: ICoreWebView2Interop2 = core.cast()?;
    let com_core: ICoreWebView2 = unsafe { interop.GetComICoreWebView2()? };
    Ok(WebView::from_core(com_core))
}
