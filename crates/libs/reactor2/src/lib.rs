#![doc = include_str!("../readme.md")]

#[cfg(test)]
extern crate self as windows_reactor;

#[macro_use]
mod control_capabilities;

#[macro_use]
mod framework_properties;

mod app;
mod arena;
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
#[expect(
    dead_code,
    reason = "generated WinUI bindings include ABI-required members outside the selected surface"
)]
mod bindings;
#[cfg(feature = "canvas")]
mod canvas;
mod composition;
mod element;
mod engine;
mod framework_state;
mod hooks;
mod id;
mod interaction;
mod mounted;
#[cfg(test)]
#[doc(hidden)]
#[path = "../testing/private/performance.rs"]
pub mod performance;
mod references;
mod resources;
mod runtime;
#[cfg(feature = "webview")]
mod webview;
mod winui;

#[cfg(feature = "canvas")]
pub use canvas::{
    CanvasDrawContext, CanvasImage, CanvasInvalidator, SwapChainCanvas, SwapChainHost,
    SwapChainHostContent, SwapChainHostFrame, SwapChainHostLayout, SwapChainHostRef,
    animated_canvas, canvas_image, canvas_image_invalidated, swap_chain_canvas,
    swap_chain_canvas_invalidated,
};
pub use composition::{
    CompositionContent, CompositionHost, CompositionHostLayout, CompositionHostRef, CompositionRoot,
};
pub use element::*;
#[cfg(feature = "webview")]
pub use webview::{WebViewHost, WebViewNavigationCompleted, WebViewRef};
pub use windows_time::{DateTime, TimeSpan};
pub use winui::bootstrap;
pub use winui::{run_reactor_winui, run_reactor_winui_app};

#[cfg(test)]
#[doc(hidden)]
pub mod testing {
    pub(crate) use crate::{
        app::Reactor,
        arena::*,
        engine::{Engine, EngineError},
        id::NodeId,
        runtime::*,
        tests::support::*,
    };
}

#[cfg(test)]
#[path = "../testing/unit/mod.rs"]
mod tests;
