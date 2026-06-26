//! Shared window harness for canvas minimal examples.
//!
//! Each example calls `canvas_samples::run(draw)` to open a window with
//! `animated_canvas` filling the full client area. The example only provides
//! the draw callback.

use windows_canvas::*;
use windows_reactor::*;

/// Run a canvas example in a full-client-area window.
///
/// The `draw` closure is called every frame with a `DrawContext` that provides
/// the drawing session, device, and dimensions.
pub fn run(title: &'static str, draw: fn(&DrawContext)) -> Result<()> {
    bootstrap()?;
    let app_fn = move |_cx: &mut RenderCx| -> Element { animated_canvas(draw).into() };

    App::new()
        .title(title)
        .backdrop(Backdrop::Mica)
        .render(app_fn)
}
