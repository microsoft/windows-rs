#![doc = include_str!("../readme.md")]

mod app;
mod core;
mod element;
mod generated;
mod native;
mod reference;
#[cfg(any(test, feature = "test"))]
mod test_support;

use generated::*;
#[cfg(test)]
pub(crate) use test_support::*;

pub use app::*;
pub use core::public::*;
pub use element::*;
pub use generated::public::*;
pub use reference::{
    CompositionHostError, CompositionHostEvent, ElementObservation, ElementRef, FocusControl,
    FocusError, ImageSourceError, IntegrationError, ReferenceControl, SwapChainPanelError,
    SwapChainPanelEvent, WebView2Error, WindowRef,
};
#[cfg(feature = "test")]
pub use test_support::*;
pub use windows_time::{DateTime, TimeSpan};
