//! Minimal DPI interop: read the host element's rasterization (DPI) scale and
//! surface it as reactor state. The `on_rasterization_scale_changed` hook fires
//! after the element loads and again whenever the scale changes (e.g. the window
//! moves to a monitor with a different DPI). The returned revoker is kept alive
//! in a `use_ref` so the subscription outlives the callback.
//!
//! ```text
//! cargo run -p reactor_composition --example dpi
//! ```

#![windows_subsystem = "windows"]

use windows_composition::{Color, CompositionHostExt, SpriteVisual};
use windows_core::EventRevoker;
use windows_reactor::*;

fn build(host: &CompositionHostHandle) -> Result<SpriteVisual> {
    let compositor = host.compositor()?;
    let visual = compositor.create_sprite_visual()?;
    visual.set_brush(&compositor.create_color_brush(Color::rgb(96, 64, 160))?)?;
    host.set_child_visual(&visual)?;
    Ok(visual)
}

fn app(cx: &mut RenderCx) -> Element {
    let (scale, set_scale) = cx.use_state(1.0_f64);
    let visual = cx.use_ref::<Option<SpriteVisual>>(None);
    let revoker = cx.use_ref::<Option<EventRevoker>>(None);

    grid((
        Element::from(
            text_block(format!("rasterization scale: {scale:.2}×"))
                .font_size(20.0)
                .bold()
                .margin(Thickness::uniform(16.0)),
        )
        .grid_row(0),
        Element::from(
            composition_host()
                .on_mounted({
                    let visual = visual.clone();
                    move |host| {
                        match build(&host) {
                            Ok(built) => visual.set(Some(built)),
                            Err(e) => eprintln!("composition init failed: {e}"),
                        }
                        let set_scale = set_scale.clone();
                        if let Ok(rev) =
                            host.on_rasterization_scale_changed(move |s| set_scale.call(s))
                        {
                            revoker.set(Some(rev));
                        }
                    }
                })
                .on_resize(move |w, h| {
                    if let Some(visual) = visual.borrow().as_ref() {
                        _ = visual.set_size(w as f32, h as f32);
                    }
                }),
        )
        .grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::STAR])
    .into()
}

fn main() -> Result<()> {
    reactor_composition::run("Composition DPI", app)
}
