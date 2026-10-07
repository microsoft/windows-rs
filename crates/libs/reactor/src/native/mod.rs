use super::*;
use app_shim::{create_application, install_xaml_controls_resources};
use bindings::IElementFactory;
use transient_menu::TransientMenuHost;
use windows_collections::*;
use windows_core::{
    ComObject, Event as WinEvent, HRESULT, HSTRING, IInspectable, IUnknownImpl, Interface, Ref,
    implement_decl,
};

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
mod winui;

pub use app::*;
pub use winui::*;
