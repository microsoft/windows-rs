#![windows_subsystem = "windows"]

use windows_composition::{Color, SpriteVisual};
use windows_reactor::*;

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
