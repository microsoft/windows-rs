//! Minimal off-thread animation: after a single `start_animation` call the
//! visual pulses forever on the compositor thread — reactor never re-renders
//! and no per-frame callback runs. This is composition's distinctive value over
//! immediate-mode drawing.
//!
//! ```text
//! cargo run -p reactor_composition --example animation
//! ```

#![windows_subsystem = "windows"]

use std::time::Duration;
use windows_composition::{Color, SpriteVisual};
use windows_numerics::Vector3;
use windows_reactor::*;

const SIZE: f32 = 160.0;

fn build(host: &CompositionHostHandle) -> Result<SpriteVisual> {
    let compositor = host.compositor()?;

    let visual = compositor.create_sprite_visual();
    visual.set_size(SIZE, SIZE);
    visual.set_center_point(Vector3 {
        x: SIZE / 2.0,
        y: SIZE / 2.0,
        z: 0.0,
    });
    visual.set_brush(&compositor.create_color_brush(Color::rgb(0, 153, 188)));

    let pulse = compositor.create_vector3_key_frame_animation();
    pulse.insert_key_frame(
        0.0,
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    );
    pulse.insert_key_frame(
        0.5,
        Vector3 {
            x: 1.5,
            y: 1.5,
            z: 1.0,
        },
    );
    pulse.insert_key_frame(
        1.0,
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    );
    pulse.set_duration(Duration::from_millis(1500));
    pulse.set_iterate_forever();
    visual.start_animation("Scale", &pulse);

    host.set_child_visual(&visual)?;
    Ok(visual)
}

fn app(cx: &mut RenderCx) -> Element {
    let visual = cx.use_ref::<Option<SpriteVisual>>(None);
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
                visual.set_offset((w as f32 - SIZE) / 2.0, (h as f32 - SIZE) / 2.0, 0.0);
            }
        })
        .into()
}

fn main() -> Result<()> {
    reactor_composition::run("Composition Animation", app)
}
