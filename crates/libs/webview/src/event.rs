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
