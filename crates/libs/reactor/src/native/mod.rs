use super::*;
use crate::core::*;

pub(crate) enum FeedbackExpectation {
    Exact(EventPayload),
    Normalized { observation: Option<QueuedEvent> },
    Suppressed,
}

mod winui;

pub use winui::*;
