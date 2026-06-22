use super::*;

/// Details about a completed navigation, delivered to a
/// [`WebView::on_navigation_completed`] handler.
pub struct NavigationCompletedArgs(pub(crate) ICoreWebView2NavigationCompletedEventArgs);

impl NavigationCompletedArgs {
    /// Returns `true` if the navigation succeeded.
    pub fn is_success(&self) -> bool {
        unsafe { self.0.IsSuccess() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the unique identifier for the navigation.
    pub fn navigation_id(&self) -> u64 {
        unsafe { self.0.NavigationId() }.unwrap_or(0)
    }
}

/// Details about a navigation that is about to start, delivered to a
/// [`WebView::on_navigation_starting`] handler. The navigation can be cancelled
/// with [`set_cancel`](Self::set_cancel).
pub struct NavigationStartingArgs(pub(crate) ICoreWebView2NavigationStartingEventArgs);

impl NavigationStartingArgs {
    /// Returns the URI the navigation is targeting.
    pub fn uri(&self) -> String {
        unsafe { self.0.Uri() }
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default()
    }

    /// Returns `true` if the navigation was initiated by the user (for example a
    /// link click) rather than by script.
    pub fn is_user_initiated(&self) -> bool {
        unsafe { self.0.IsUserInitiated() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns `true` if the navigation is a redirect.
    pub fn is_redirected(&self) -> bool {
        unsafe { self.0.IsRedirected() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the unique identifier for the navigation.
    pub fn navigation_id(&self) -> u64 {
        unsafe { self.0.NavigationId() }.unwrap_or(0)
    }

    /// Returns `true` if the navigation is currently marked to be cancelled.
    pub fn is_cancelled(&self) -> bool {
        unsafe { self.0.Cancel() }.is_ok_and(|value| value.as_bool())
    }

    /// Cancels (or un-cancels) the navigation.
    pub fn set_cancel(&self, cancel: bool) -> Result<()> {
        unsafe { self.0.SetCancel(cancel) }
    }
}

/// A message posted from the hosted page's JavaScript, delivered to a
/// [`WebView::on_web_message_received`] handler.
pub struct WebMessageReceivedArgs(pub(crate) ICoreWebView2WebMessageReceivedEventArgs);

impl WebMessageReceivedArgs {
    /// Returns the URI of the document that sent the message.
    pub fn source(&self) -> String {
        unsafe { self.0.Source() }
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default()
    }

    /// Returns the message serialized as a JSON string. Messages sent with
    /// `window.chrome.webview.postMessage` arrive here regardless of type.
    pub fn web_message_as_json(&self) -> String {
        unsafe { self.0.WebMessageAsJson() }
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default()
    }

    /// Returns the message as a string. Fails if the page posted a value that is
    /// not a JavaScript string.
    pub fn try_web_message_as_string(&self) -> Result<String> {
        let value = unsafe { self.0.TryGetWebMessageAsString()? };
        Ok(unsafe { string::take(value) })
    }
}

/// An RAII guard for an event subscription. The handler stays registered until
/// this value is dropped or [`EventRegistration::remove`] is called.
#[must_use]
pub struct EventRegistration(Option<Box<dyn FnOnce()>>);

impl EventRegistration {
    pub(crate) fn new<F: FnOnce() + 'static>(remove: F) -> Self {
        Self(Some(Box::new(remove)))
    }

    /// Unsubscribes the handler.
    pub fn remove(mut self) {
        if let Some(remove) = self.0.take() {
            remove();
        }
    }
}

impl Drop for EventRegistration {
    fn drop(&mut self) {
        if let Some(remove) = self.0.take() {
            remove();
        }
    }
}
