#![doc = include_str!("../readme.md")]

mod app;
mod core;
mod element;
mod generated;
mod native;
mod reference;
#[cfg(any(test, feature = "test"))]
pub mod test;

use generated::*;
#[cfg(test)]
pub(crate) use test::*;

pub use app::*;
pub use core::public::*;
pub use element::*;
pub use generated::public::*;
pub use reference::{
    CompositionHostError, CompositionHostEvent, ElementObservation, ElementRef, FocusControl,
    FocusError, ImageSourceError, IntegrationError, ReferenceControl, SwapChainPanelError,
    SwapChainPanelEvent, WebView2Error, WindowRef,
};
pub use windows_time::{DateTime, TimeSpan};
