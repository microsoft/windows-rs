//! Test infrastructure — not part of the public API.
//!
//! Test crates import from here: `use windows_reactor::imp::*;`

// Headless test harness
pub use crate::core::reconciler::Reconciler;
pub use crate::core::render_host::{RenderCompleteInfo, RenderHost, RenderStats};

// Recording backend for assertions
pub use crate::core::backend::{Op, RecordingBackend};

// Test dispatchers
pub use crate::core::dispatcher::{
    ChannelDispatcher, RunOnDemandDispatcher, UiMarshaller, UiRerenderGuard,
};

// Internal element variants
pub use crate::core::element::GroupElement;

// WinUI platform types (for integration test harness)
pub use crate::winui::backend::WinUIBackend;
pub use crate::winui::dispatcher::WinUIDispatcher;
