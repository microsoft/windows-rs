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

#[doc(hidden)]
pub mod core;
#[doc(hidden)]
pub mod winui;

mod app;
pub(crate) mod diagnostics;

/// Test infrastructure — not part of the public API.
#[doc(hidden)]
pub mod imp;

mod app_shim;
pub mod bootstrap;

pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};

pub use app::*;
pub use core::*;
pub use winui::*;
