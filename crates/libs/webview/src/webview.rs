use super::*;

/// A WebView2 browser. Navigate to URLs and run JavaScript against the hosted
/// page.
pub struct WebView(pub(crate) ICoreWebView2);

impl WebView {
    /// Navigates the browser to the given URI.
    pub fn navigate(&self, uri: &str) -> Result<()> {
        let uri = string::encode(uri);
        unsafe { self.0.Navigate(uri.as_ptr()) }
    }

    /// Asynchronously runs JavaScript in the context of the current page. The
    /// `handler` closure receives the JSON-encoded result on the UI thread.
    pub fn execute_script<F: FnOnce(Result<String>) + 'static>(
        &self,
        javascript: &str,
        handler: F,
    ) -> Result<()> {
        let javascript = string::encode(javascript);
        let handler = handler::ExecuteScriptCompleted::create(handler);
        unsafe { self.0.ExecuteScript(javascript.as_ptr(), &handler) }
    }
}
