//! Shared window harness for canvas minimal examples.
//!
//! Each example calls `canvas_samples::run(draw)` to open a window with a
//! demand-driven `canvas` filling the full client area. The example only provides
//! the draw callback, which runs on the first layout and on resize - not every
//! frame.

use windows_canvas::*;
use windows_reactor::*;

/// Run a canvas example in a full-client-area window.
///
/// `draw` runs on the first layout and on resize or scale change, idle otherwise.
/// For content that animates every frame, use [`run_animated`].
pub fn run(title: &'static str, draw: fn(&DrawContext)) -> Result<()> {
    bootstrap()?;
    App::new()
        .title(title)
        .backdrop(Backdrop::Mica)
        .render(move |_cx: &mut RenderCx| -> Element { canvas(draw).into() })
}

/// Run a canvas example that repaints every frame, for animated content.
///
/// Prefer [`run`] for static drawings; use this only when `draw` changes each
/// frame.
pub fn run_animated(title: &'static str, draw: fn(&DrawContext)) -> Result<()> {
    bootstrap()?;
    App::new()
        .title(title)
        .backdrop(Backdrop::Mica)
        .render(move |_cx: &mut RenderCx| -> Element { animated_canvas(draw).into() })
}
