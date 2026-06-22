use super::*;
use crate::handler::subscription;

/// Hosts a WebView2 browser inside a parent window, controlling its bounds,
/// visibility, and lifetime.
pub struct Controller(pub(crate) ICoreWebView2Controller);

impl Controller {
    /// Returns the [`WebView`] used to navigate and script the hosted browser.
    pub fn webview(&self) -> Result<WebView> {
        unsafe { Ok(WebView(self.0.CoreWebView2()?)) }
    }

    /// Sets the bounds of the browser within the parent window, in pixels.
    pub fn set_bounds(&self, left: i32, top: i32, right: i32, bottom: i32) -> Result<()> {
        unsafe {
            self.0.SetBounds(RECT {
                left,
                top,
                right,
                bottom,
            })
        }
    }

    /// Shows or hides the browser.
    pub fn set_visible(&self, visible: bool) -> Result<()> {
        unsafe { self.0.SetIsVisible(visible) }
    }

    /// Closes the browser and releases its resources.
    pub fn close(&self) -> Result<()> {
        unsafe { self.0.Close() }
    }

    /// Moves focus into the browser, as if focus arrived for the given
    /// [`reason`](MoveFocusReason). Call this from the host's `WM_SETFOCUS`
    /// handler so the browser takes keyboard focus.
    pub fn move_focus(&self, reason: MoveFocusReason) -> Result<()> {
        unsafe { self.0.MoveFocus(reason.to_raw()) }
    }

    /// Subscribes to the got-focus event, raised when the browser gains focus.
    pub fn on_got_focus<F: FnMut() + 'static>(&self, handler: F) -> Result<EventRegistration> {
        let handler = handler::FocusChanged::create(handler);
        let token = unsafe { self.0.add_GotFocus(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_GotFocus(token) };
        }))
    }

    /// Subscribes to the lost-focus event, raised when the browser loses focus.
    pub fn on_lost_focus<F: FnMut() + 'static>(&self, handler: F) -> Result<EventRegistration> {
        let handler = handler::FocusChanged::create(handler);
        let token = unsafe { self.0.add_LostFocus(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_LostFocus(token) };
        }))
    }

    subscription! {
        /// Subscribes to the move-focus-requested event, raised when focus is
        /// leaving the browser (for example the user tabbed past the last
        /// element). Move focus to the appropriate host control and call
        /// [`MoveFocusRequestedArgs::set_handled`]. Wiring this is what keeps Tab
        /// navigation and screen readers working when the browser is embedded.
        on_move_focus_requested(MoveFocusRequestedArgs) =>
            MoveFocusRequested, add_MoveFocusRequested / remove_MoveFocusRequested
    }

    subscription! {
        /// Subscribes to the accelerator-key-pressed event, raised for
        /// browser-level keys (such as function keys or a key combined with Ctrl
        /// or Alt) before the page handles them. Inspect
        /// [`AcceleratorKeyPressedArgs`] to implement application keyboard
        /// shortcuts.
        on_accelerator_key_pressed(AcceleratorKeyPressedArgs) =>
            AcceleratorKeyPressed, add_AcceleratorKeyPressed / remove_AcceleratorKeyPressed
    }
}
