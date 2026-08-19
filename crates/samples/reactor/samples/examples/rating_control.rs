#![windows_subsystem = "windows"]

use windows_reactor::{Element, RatingControl, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let rating = cx.use_state(|| 3.0_f64);
    let extended = cx.use_state(|| 4.0_f64);
    let current = rating.value();
    let current_extended = extended.value();

    vstack(
        8.0,
        [
            RatingControl::new(current, move |value| {
                if let Some(value) = value {
                    rating.set(value);
                }
            })
            .build(),
            TextBlock::new(format!("Rating: {current:.1} / 5")).build(),
            RatingControl::new(current_extended, move |value| {
                if let Some(value) = value {
                    extended.set(value);
                }
            })
            .max_rating(10)
            .caption("Out of 10")
            .build(),
            RatingControl::display(2.5).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RatingControl", app)
}
