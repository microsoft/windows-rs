#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
mod bindings;

mod app;
mod app_shim;
mod backend;
mod bootstrap;
mod diagnostics;
mod element;
mod engine;
mod generated;
mod hooks;
mod host;
mod interaction;
mod reconciler;
mod style;
mod widget;
mod widgets;

pub use app::*;
pub use backend::*;
pub use bootstrap::*;
pub use element::*;
pub use engine::*;
pub use hooks::*;
pub use host::*;
pub use interaction::*;
pub use reconciler::*;
pub use style::*;
pub use widget::*;
pub use widgets::*;
pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};
