use super::*;
use crate::handler::subscription;

/// A 32-bit RGBA colour, used for the browser's
/// [default background](Controller::set_default_background_color).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// A fully transparent colour, letting the host window show through where the
    /// page has not painted.
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    fn from_raw(value: COREWEBVIEW2_COLOR) -> Self {
        Self {
            r: value.R,
            g: value.G,
            b: value.B,
            a: value.A,
        }
    }

    fn to_raw(self) -> COREWEBVIEW2_COLOR {
        COREWEBVIEW2_COLOR {
            A: self.a,
            R: self.r,
            G: self.g,
            B: self.b,
        }
    }
}

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

    /// Returns the zoom factor applied to the page, where `1.0` is 100%.
    pub fn zoom_factor(&self) -> f64 {
        unsafe { self.0.ZoomFactor() }.unwrap_or(1.0)
    }

    /// Sets the zoom factor applied to the page, where `1.0` is 100%.
    pub fn set_zoom_factor(&self, zoom_factor: f64) -> Result<()> {
        unsafe { self.0.SetZoomFactor(zoom_factor) }
    }

    /// Returns the colour painted behind the page before content loads and
    /// wherever the page is transparent.
    pub fn default_background_color(&self) -> Result<Color> {
        let source: ICoreWebView2Controller2 = self.0.cast()?;
        Ok(Color::from_raw(unsafe { source.DefaultBackgroundColor()? }))
    }

    /// Sets the colour painted behind the page before content loads and wherever
    /// the page is transparent. Use [`Color::TRANSPARENT`] to let the host window
    /// show through; only fully opaque (`a = 255`) and fully transparent
    /// (`a = 0`) colours are supported.
    pub fn set_default_background_color(&self, color: Color) -> Result<()> {
        let source: ICoreWebView2Controller2 = self.0.cast()?;
        unsafe { source.SetDefaultBackgroundColor(color.to_raw()) }
    }

    /// Returns the scale used to rasterize page content, which the browser
    /// derives from the monitor DPI.
    pub fn rasterization_scale(&self) -> Result<f64> {
        let source: ICoreWebView2Controller3 = self.0.cast()?;
        unsafe { source.RasterizationScale() }
    }

    /// Sets the scale used to rasterize page content. Set
    /// [`set_should_detect_monitor_scale_changes(false)`](Self::set_should_detect_monitor_scale_changes)
    /// first to stop the browser overriding this when the monitor DPI changes.
    pub fn set_rasterization_scale(&self, scale: f64) -> Result<()> {
        let source: ICoreWebView2Controller3 = self.0.cast()?;
        unsafe { source.SetRasterizationScale(scale) }
    }

    /// Returns `true` if the browser updates the
    /// [rasterization scale](Self::rasterization_scale) automatically as the
    /// monitor DPI changes.
    pub fn should_detect_monitor_scale_changes(&self) -> Result<bool> {
        let source: ICoreWebView2Controller3 = self.0.cast()?;
        Ok(unsafe { source.ShouldDetectMonitorScaleChanges()? }.as_bool())
    }

    /// Sets whether the browser updates the
    /// [rasterization scale](Self::rasterization_scale) automatically as the
    /// monitor DPI changes. Disable it to manage the scale yourself.
    pub fn set_should_detect_monitor_scale_changes(&self, detect: bool) -> Result<()> {
        let source: ICoreWebView2Controller3 = self.0.cast()?;
        unsafe { source.SetShouldDetectMonitorScaleChanges(detect) }
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
