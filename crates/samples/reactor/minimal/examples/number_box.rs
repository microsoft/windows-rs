//! Minimal sample for the `NumberBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (quantity, set_quantity) = cx.use_state(3.0_f64);

    let update_quantity = move |v: f64| set_quantity.call(v);

    vstack((
        NumberBox::new(quantity)
            .range(0.0, 10.0)
            .header("Quantity")
            .on_value_changed(update_quantity),
        text_block(format!("Quantity = {quantity:.0}")),
        NumberBox::new(42.0).header("Disabled").enabled(false),
    ))
    .spacing(8.0)
    .max_width(320.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
