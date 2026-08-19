#![windows_subsystem = "windows"]

use windows_composition::{Color, SpriteVisual};
use windows_reactor::{
    Button, CompositionContent, CompositionHost, Element, Grid, GridChild, GridLength, RenderCx,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let shown = cx.use_state(|| true);
    let host = cx.use_composition_host_ref::<SpriteVisual>();
    let toggle = host.clone();

    Grid::new([
        GridChild::new(
            CompositionHost::new(
                &host,
                |compositor| {
                    let visual = compositor.create_sprite_visual();
                    visual.set_brush(&compositor.create_color_brush(Color::rgb(0, 153, 102)));
                    Ok(CompositionContent::new(visual.clone(), visual))
                },
                |visual, layout| {
                    visual.set_size(layout.width, layout.height);
                    Ok(())
                },
            )
            .build(),
        )
        .row(0),
        GridChild::new(
            Button::new(if shown.value() {
                "Hide visual"
            } else {
                "Show visual"
            })
            .on_click(move || {
                let visible = !shown.value();
                shown.set(visible);
                toggle.update(move |visual| {
                    visual.set_visible(visible);
                    Ok(())
                });
            })
            .build(),
        )
        .row(1),
    ])
    .rows([GridLength::STAR, GridLength::Auto])
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Typed Composition Host", app)
}
