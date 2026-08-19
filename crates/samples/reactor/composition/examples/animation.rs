#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_composition::{Color, SpriteVisual};
use windows_numerics::Vector3;
use windows_reactor::{CompositionContent, CompositionHost, Element, RenderCx};

const SIZE: f32 = 160.0;

fn app(cx: &mut RenderCx<'_>) -> Element {
    let host = cx.use_composition_host_ref::<SpriteVisual>();
    CompositionHost::new(
        &host,
        |compositor| {
            let visual = compositor.create_sprite_visual();
            visual.set_size(SIZE, SIZE);
            visual.set_center_point(Vector3::new(SIZE / 2.0, SIZE / 2.0, 0.0));
            visual.set_brush(&compositor.create_color_brush(Color::rgb(0, 153, 188)));

            let pulse = compositor.create_vector3_key_frame_animation();
            pulse.insert_key_frame(0.0, Vector3::new(1.0, 1.0, 1.0));
            pulse.insert_key_frame(0.5, Vector3::new(1.5, 1.5, 1.0));
            pulse.insert_key_frame(1.0, Vector3::new(1.0, 1.0, 1.0));
            pulse.set_duration(Duration::from_millis(1500));
            pulse.set_iterate_forever();
            visual.start_animation("Scale", &pulse);

            Ok(CompositionContent::new(visual.clone(), visual))
        },
        |visual, layout| {
            visual.set_offset(
                (layout.width - SIZE) / 2.0,
                (layout.height - SIZE) / 2.0,
                0.0,
            );
            Ok(())
        },
    )
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_composition::run("Composition Animation", app)
}
