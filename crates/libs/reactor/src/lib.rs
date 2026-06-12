// Top-level modules must stay `pub` due to a nightly rustc MIR generation
// bug with private modules (items lose MIR even when `pub use`-re-exported).
// The `pub use` re-exports in core/mod.rs must also stay `pub` for the same
// reason. Sub-modules inside core/ are `pub(crate)` to prevent deep access.
#[doc(hidden)]
pub mod app;
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
#[doc(hidden)]
pub mod core;
mod diagnostics;
#[doc(hidden)]
pub mod winui;

use windows_core::{Error, HRESULT, Interface};

pub use windows_core::Result;
pub use windows_time::{DateTime, TimeSpan};

pub use app::*;
pub use core::animation::*;
pub use core::backend::{
    Backend, ControlId, ControlKind, Event, EventHandler, Op, Prop, PropValue, RecordingBackend,
};
pub use core::callback::*;
pub use core::component::*;
pub use core::component_element::*;
pub use core::context::Context;
pub use core::custom::*;
pub use core::dispatcher::*;
pub use core::element::{Element, GroupElement, can_skip_update, group};
pub use core::element_ext::*;
pub use core::error_boundary::*;
pub use core::geometry::*;
pub use core::into_elements::IntoElements;
pub use core::keyboard::*;
pub use core::modifiers::*;
pub use core::reconciler::Reconciler;
pub use core::render_context::*;
pub use core::render_host::*;
pub use core::resource::*;
pub use core::rich_text::*;
pub use core::templated_list::{
    SelectionMode, TemplatedKind, flip_view, grid_view, list_view, virtual_list,
};
pub use core::theme::*;
pub use core::widgets::*;
pub use winui::dispatcher::WinUIDispatcher;
pub use winui::host::{Backdrop, RequestedTheme, set_backdrop, set_requested_theme};
pub use winui::{DispatcherTimer, Rendering, WinUIBackend, on_rendering};
