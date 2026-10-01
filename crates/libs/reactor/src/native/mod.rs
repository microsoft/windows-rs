#[allow(
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#[cfg_attr(not(any(test, feature = "test")), allow(dead_code))]
mod bindings;

mod app;
mod app_shim;
mod bootstrap;
mod transient_menu;
#[cfg_attr(not(any(test, feature = "test")), allow(dead_code))]
mod winui;

pub use app::*;
pub use winui::*;
