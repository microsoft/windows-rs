use super::*;
use crate::core::*;

#[derive(Clone)]
pub(crate) enum FeedbackExpectation {
    Any,
    Exact(EventPayload),
}

#[cfg(any(test, feature = "test"))]
mod recording;
mod winui;

#[cfg(any(test, feature = "test"))]
pub use recording::*;
pub use winui::*;
