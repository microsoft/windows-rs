use super::*;
use crate::core::*;

pub(crate) enum FeedbackExpectation {
    Exact(EventPayload),
    Normalized { observation: Option<QueuedEvent> },
    Suppressed,
}

mod winui;

#[cfg(feature = "test")]
pub use winui::test::{
    LiveInputProbe, LiveInputProbeStage, schedule_live_test_exit, subscribe_live_rendering,
};
#[cfg(feature = "test")]
pub(crate) use winui::test::{finish_live_text_input_dispatch, subscribe_live_input_probe};
pub use winui::*;
