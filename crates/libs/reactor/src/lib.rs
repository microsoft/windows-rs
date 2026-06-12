mod app;
mod app_shim;
#[expect(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations
)]
mod bindings;
pub mod bootstrap;
mod diagnostics;
#[doc(hidden)]
pub mod imp;
mod winui;

use windows_core::{Error, HRESULT, Interface};

pub use windows_core::Result;
pub use windows_time::{DateTime, TimeSpan};

pub use app::*;
pub use imp::*;
pub use winui::*;
