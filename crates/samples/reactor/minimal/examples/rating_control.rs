//! Minimal sample for the `RatingControl` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (rating, set_rating) = cx.use_state(3.0_f64);

    let update = move |v: f64| set_rating.call(v);

    vstack((
        RatingControl::new(rating).on_changed(update),
        text_block(format!("Rating: {rating:.1} / 5")),
        RatingControl::new(4.0).max_rating(10).caption("Out of 10"),
        RatingControl::new(2.5).read_only(),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
