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
mod bootstrap;
mod core;
mod diagnostics;
mod winui;

pub use app::*;
pub use bootstrap::*;
pub use core::*;
pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};
pub use winui::backend::WinUIBackend;
pub use winui::dispatcher::WinUIDispatcher;
pub use winui::hooks::*;
pub use winui::host::*;
