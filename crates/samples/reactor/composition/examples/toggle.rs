//! Minimal show/hide: a reactor button toggles the visibility of the host's
//! composition visual, showing that the composition tree and the reactor UI are
//! independent and can be wired together by ordinary reactor state.
//!
//! ```text
//! cargo run -p reactor_composition --example toggle
//! ```

#![windows_subsystem = "windows"]

use windows_composition::{Color, SpriteVisual};
use windows_reactor::*;

/// Builds a solid-colored visual and attaches it as the host's child visual.
fn build(host: &CompositionHostHandle) -> Result<SpriteVisual> {
    let compositor = host.compositor()?;
    let visual = compositor.create_sprite_visual();
    visual.set_brush(&compositor.create_color_brush(Color::rgb(0, 153, 102)));
    host.set_child_visual(&visual)?;
    Ok(visual)
}

fn app(cx: &mut RenderCx) -> Element {
    let (shown, set_shown) = cx.use_state(true);
    let visual = cx.use_ref::<Option<SpriteVisual>>(None);

    // Mirror reactor state onto the composition visual's visibility.
    {
        let visual = visual.clone();
        cx.use_effect((shown,), move || {
            if let Some(visual) = visual.borrow().as_ref() {
                visual.set_visible(shown);
            }
        });
    }

    grid((
        Element::from(
            composition_host()
                .on_mounted({
                    let visual = visual.clone();
                    move |host| match build(&host) {
                        Ok(built) => visual.set(Some(built)),
                        Err(e) => eprintln!("composition init failed: {e}"),
                    }
                })
                .on_resize(move |w, h| {
                    if let Some(visual) = visual.borrow().as_ref() {
                        visual.set_size(w as f32, h as f32);
                    }
                }),
        )
        .grid_row(0),
        Element::from(
            button(if shown { "Hide visual" } else { "Show visual" })
                .on_click(move || set_shown.call(!shown)),
        )
        .grid_row(1)
        .margin(Thickness::uniform(16.0)),
    ))
    .rows([GridLength::STAR, GridLength::Auto])
    .into()
}

fn main() -> Result<()> {
    reactor_composition::run("Composition Toggle", app)
}
