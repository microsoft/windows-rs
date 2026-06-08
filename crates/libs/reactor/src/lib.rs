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
pub mod dsl;

#[doc(hidden)]
pub mod winui;

#[doc(hidden)]
pub mod app;
pub(crate) mod diagnostics;

mod app_shim;
pub mod bootstrap;

pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};

pub use app::*;
pub use core::animation::*;
pub use core::backend::{Backend, ControlId};
pub use core::callback::*;
pub use core::component::*;
pub use core::component_element::*;
pub use core::context::Context;
pub use core::custom::*;
pub use core::dispatcher::*;
pub use core::element::*;
pub use core::element::{HorizontalAlignment, VerticalAlignment};
pub use core::error_boundary::*;
pub use core::into_elements::IntoElements;
pub use core::render_context::*;
pub use core::render_host::*;
pub use core::resource::*;
pub use core::templated_list::{SelectionMode, flip_view, grid_view, list_view, virtual_list};
pub use core::theme::*;
pub use core::window::*;
pub use dsl::*;
pub use winui::dispatcher::WinUIDispatcher;
pub use winui::host::{Backdrop, RequestedTheme, set_backdrop, set_requested_theme};
pub use winui::{DispatcherTimer, Rendering, on_rendering};
