#![windows_subsystem = "windows"]

use windows_composition::{Color, SpriteVisual};
use windows_reactor::{
    CompositionContent, CompositionHost, Element, Grid, GridChild, GridLength, RenderCx, TextBlock,
    Thickness,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let scale = cx.use_state(|| 1.0_f32);
    let host = cx.use_composition_host_ref::<SpriteVisual>();
    let update_scale = scale.clone();

    Grid::new([
        GridChild::new(
            TextBlock::new(format!("rasterization scale: {:.2}x", scale.value()))
                .font_size(20.0)
                .padding(Thickness::uniform(16.0))
                .build(),
        )
        .row(0),
        GridChild::new(
            CompositionHost::new(
                &host,
                |compositor| {
                    let visual = compositor.create_sprite_visual();
                    visual.set_brush(&compositor.create_color_brush(Color::rgb(96, 64, 160)));
                    Ok(CompositionContent::new(visual.clone(), visual))
                },
                move |visual, layout| {
                    update_scale.set(layout.rasterization_scale);
                    visual.set_size(layout.width, layout.height);
                    Ok(())
                },
            )
            .build(),
        )
        .row(1),
    ])
    .rows([GridLength::Auto, GridLength::STAR])
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_composition::run("Composition DPI", app)
}
