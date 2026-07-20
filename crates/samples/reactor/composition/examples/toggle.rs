//! Minimal attach/detach: a reactor button attaches or clears the host's child
//! visual with `set_child_visual` / `clear_child_visual`, showing that the
//! composition tree and the reactor UI are independent and can be wired together
//! by ordinary reactor state.
//!
//! ```text
//! cargo run -p reactor_composition --example toggle
//! ```

#![windows_subsystem = "windows"]

use windows_composition::{Color, CompositionHostExt, SpriteVisual};
use windows_reactor::*;

struct State {
    host: CompositionHostHandle,
    visual: SpriteVisual,
    width: f32,
    height: f32,
}

fn make_visual(host: &CompositionHostHandle) -> Result<SpriteVisual> {
    let compositor = host.compositor()?;
    let visual = compositor.create_sprite_visual()?;
    visual.set_brush(&compositor.create_color_brush(Color::rgb(0, 153, 102))?)?;
    Ok(visual)
}

fn app(cx: &mut RenderCx) -> Element {
    let (shown, set_shown) = cx.use_state(true);
    let state = cx.use_ref::<Option<State>>(None);

    {
        let state = state.clone();
        cx.use_effect((shown,), move || {
            if let Some(state) = state.borrow().as_ref() {
                if shown {
                    _ = state.visual.set_size(state.width, state.height);
                    _ = state.host.set_child_visual(&state.visual);
                } else {
                    _ = state.host.clear_child_visual();
                }
            }
        });
    }

    grid((
        Element::from(
            composition_host()
                .on_mounted({
                    let state = state.clone();
                    move |host| match make_visual(&host) {
                        Ok(visual) => {
                            _ = host.set_child_visual(&visual);
                            state.set(Some(State {
                                host,
                                visual,
                                width: 400.0,
                                height: 300.0,
                            }));
                        }
                        Err(e) => eprintln!("composition init failed: {e}"),
                    }
                })
                .on_resize(move |w, h| {
                    if let Some(state) = state.borrow_mut().as_mut() {
                        state.width = w as f32;
                        state.height = h as f32;
                        _ = state.visual.set_size(state.width, state.height);
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
