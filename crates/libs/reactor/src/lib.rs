#![doc = include_str!("../readme.md")]

mod app;
mod core;
mod element;
mod generated;
mod native;
mod reference;
#[cfg(feature = "test")]
mod test_support;

use generated::*;

pub use app::*;
pub use core::public::*;
pub use element::*;
pub use generated::public::*;
#[doc(hidden)]
pub use reference::ReferenceType;
pub use reference::{
    CompositionHostError, CompositionHostEvent, ElementObservation, ElementRef, FocusControl,
    FocusError, ImageSourceError, IntegrationError, SwapChainPanelError, SwapChainPanelEvent,
    WebView2Error, WindowRef,
};
#[cfg(feature = "test")]
pub use test_support::*;
pub use windows_time::{DateTime, TimeSpan};
