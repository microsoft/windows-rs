#![allow(unused_imports)]

pub(crate) mod animation;
pub(crate) mod backend;
pub(crate) mod callback;
pub(crate) mod component;
pub(crate) mod component_element;
pub(crate) mod context;
pub(crate) mod custom;
pub(crate) mod dispatcher;
pub(crate) mod element;
pub(crate) mod element_ext;
pub(crate) mod error_boundary;
pub(crate) mod generated_bindings;
pub(crate) mod geometry;
pub(crate) mod into_elements;
pub(crate) mod keyboard;
pub(crate) mod modifiers;
pub(crate) mod prop_binding;
pub(crate) mod rc_fn;
pub(crate) mod reconciler;
pub(crate) mod render_context;
pub(crate) mod render_host;
pub(crate) mod resource;
pub(crate) mod rich_text;
pub(crate) mod templated_list;
pub(crate) mod theme;
pub(crate) mod widget;
pub(crate) mod widgets;

// Public API re-exports — flow through `pub use core::*` in lib.rs
pub use animation::*;
pub use backend::{Backend, ControlId, ControlKind, Event, EventHandler, Prop, PropValue};
pub use callback::*;
pub use component::*;
pub use component_element::*;
pub use context::*;
pub use custom::*;
pub use dispatcher::{Dispatcher, DispatcherQueuePriority, SendDispatcher};
pub use element::{Element, can_skip_update, group};
pub use element_ext::*;
pub use error_boundary::*;
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
pub(crate) use dispatcher::{
    ChannelDispatcher, RunOnDemandDispatcher, UiMarshaller, UiRerenderGuard,
    request_ui_rerender_on_ui_thread, set_ui_rerender,
};
pub(crate) use element::GroupElement;
pub(crate) use prop_binding::*;
pub(crate) use reconciler::*;
pub(crate) use render_host::*;
pub(crate) use widget::*;
