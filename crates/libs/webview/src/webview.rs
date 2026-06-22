use super::*;

/// A WebView2 browser. Navigate to URLs and run JavaScript against the hosted
/// page.
#[derive(Clone)]
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

    /// Subscribes to the navigation-completed event. The returned
    /// [`EventRegistration`] keeps the subscription alive; dropping it (or
    /// calling [`EventRegistration::remove`]) unsubscribes the handler.
    pub fn on_navigation_completed<F: FnMut(NavigationCompletedArgs) + 'static>(
        &self,
        handler: F,
    ) -> Result<EventRegistration> {
        let handler = handler::NavigationCompleted::create(handler);
        let token = unsafe { self.0.add_NavigationCompleted(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_NavigationCompleted(token) };
        }))
    }

    /// Posts a message to the hosted page as a JSON value. The page receives it
    /// via the `window.chrome.webview.addEventListener("message", …)` event,
    /// with `event.data` set to the parsed JSON.
    pub fn post_web_message_as_json(&self, json: &str) -> Result<()> {
        let json = string::encode(json);
        unsafe { self.0.PostWebMessageAsJson(json.as_ptr()) }
    }

    /// Posts a message to the hosted page as a string. The page receives it via
    /// the `window.chrome.webview.addEventListener("message", …)` event, with
    /// `event.data` set to the string.
    pub fn post_web_message_as_string(&self, message: &str) -> Result<()> {
        let message = string::encode(message);
        unsafe { self.0.PostWebMessageAsString(message.as_ptr()) }
    }

    /// Subscribes to the web-message-received event, raised when the hosted page
    /// calls `window.chrome.webview.postMessage`. The returned
    /// [`EventRegistration`] keeps the subscription alive; dropping it (or
    /// calling [`EventRegistration::remove`]) unsubscribes the handler.
    pub fn on_web_message_received<F: FnMut(WebMessageReceivedArgs) + 'static>(
        &self,
        handler: F,
    ) -> Result<EventRegistration> {
        let handler = handler::WebMessageReceived::create(handler);
        let token = unsafe { self.0.add_WebMessageReceived(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_WebMessageReceived(token) };
        }))
    }
}
