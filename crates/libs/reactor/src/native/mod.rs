use super::*;
use crate::core::*;

pub(crate) enum FeedbackExpectation {
    Exact(EventPayload),
    Normalized { observation: Option<QueuedEvent> },
    Suppressed,
}

#[cfg(any(test, feature = "test"))]
mod recording;
mod winui;

#[cfg(any(test, feature = "test"))]
pub use recording::*;
pub use winui::*;
