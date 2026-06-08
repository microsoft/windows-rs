//! Sample for the `Slider` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (volume, set_volume) = cx.use_state(35.0_f64);

    let update_volume = move |v: f64| set_volume.call(v);

    vstack((
        Slider::new(volume)
            .range(0.0, 100.0)
            .step(1.0)
            .header("Volume")
            .on_value_changed(update_volume),
        text_block(format!("Volume = {volume:.0}")),
        Slider::new(50.0)
            .range(0.0, 100.0)
            .header("Vertical")
            .vertical()
            .height(120.0),
        Slider::new(50.0)
            .range(0.0, 100.0)
            .header("Disabled")
            .enabled(false),
    ))
    .spacing(8.0)
    .max_width(320.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("Slider", app)
}
