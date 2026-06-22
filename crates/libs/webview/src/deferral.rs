use super::*;

/// Keeps an event request pending so it can be resolved after the handler
/// returns. Obtain one from an event's `defer` method (for example
/// [`PermissionRequestedArgs::defer`]) and call [`complete`](Self::complete)
/// once the decision has been made. Dropping the deferral completes it
/// automatically.
#[must_use]
pub struct Deferral(Option<ICoreWebView2Deferral>);

impl Deferral {
    pub(crate) fn new(deferral: ICoreWebView2Deferral) -> Self {
        Self(Some(deferral))
    }

    /// Completes the deferred request, allowing WebView2 to proceed with the
    /// state set on the originating event args.
    pub fn complete(mut self) -> Result<()> {
        match self.0.take() {
            Some(deferral) => unsafe { deferral.Complete() },
            None => Ok(()),
        }
    }
}

impl Drop for Deferral {
    fn drop(&mut self) {
        if let Some(deferral) = self.0.take() {
            let _ = unsafe { deferral.Complete() };
        }
    }
}
