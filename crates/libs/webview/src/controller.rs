use super::*;

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
}
