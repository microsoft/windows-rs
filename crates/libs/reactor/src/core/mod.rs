#![allow(unused_imports)]

mod animation;
pub(crate) mod backend;
mod callback;
mod component;
mod component_element;
mod context;
mod custom;
pub(crate) mod dispatcher;
pub(crate) mod element;
mod element_ext;
mod error_boundary;
mod generated_bindings;
mod geometry;
mod into_elements;
mod keyboard;
mod modifiers;
mod prop_binding;
mod rc_fn;
pub(crate) mod reconciler;
mod render_context;
pub(crate) mod render_host;
mod resource;
mod rich_text;
mod templated_list;
mod theme;
mod widget;
mod widgets;

// Public API re-exports — flow through `pub use core::*` in lib.rs
pub use animation::*;
pub use backend::{Backend, ControlId, ControlKind, Event, EventHandler, Prop, PropValue};
pub use callback::*;
pub use component::*;
pub use component_element::{component, memo};
pub use context::*;
pub use custom::*;
pub use dispatcher::{Dispatcher, DispatcherQueuePriority, SendDispatcher};
pub use element::{Element, can_skip_update, group};
pub use element_ext::*;
pub use error_boundary::error_boundary;
pub use geometry::*;
pub use into_elements::IntoElements;
pub use keyboard::*;
pub use modifiers::*;
pub use render_context::*;
pub use resource::*;
pub use rich_text::*;
pub use templated_list::*;
pub use theme::*;
pub use widgets::*;

// Internal re-exports — available within the crate via `use super::*`
// but NOT re-exported through lib.rs's `pub use core::*`.
pub(crate) use backend::{Op, RecordingBackend};
pub(crate) use component_element::{ComponentElement, ComponentObject};
pub(crate) use dispatcher::{
    ChannelDispatcher, RunOnDemandDispatcher, UiMarshaller, UiRerenderGuard,
    request_ui_rerender_on_ui_thread, set_ui_rerender,
};
pub(crate) use element::GroupElement;
pub(crate) use error_boundary::{ErrorBoundaryElement, panic_message};
pub(crate) use prop_binding::*;
pub(crate) use reconciler::*;
pub(crate) use render_host::*;
pub(crate) use widget::*;
