use crate::controls::*;
use windows_reactor::*;

pub fn rating_control_page(_: &(), cx: &mut RenderCx) -> Element {
    let (rating, set_rating) = cx.use_state(3.0_f64);

    page_content(
        "RatingControl",
        "A control that lets users provide a star rating.",
        vec![
            sample_card(
                "Basic RatingControl",
                vstack((
                    RatingControl::new(rating).on_changed({
                        let set_rating = set_rating;
                        move |v| set_rating.call(v)
                    }),
                    text_block(format!("Rating: {rating:.1} / 5")).opacity(0.6),
                ))
                .spacing(8.0),
                "RatingControl::new(rating).on_changed(handler)",
            ),
            sample_card(
                "Custom Max Rating",
                RatingControl::new(7.0)
                    .max_rating(10)
                    .caption("Score out of 10"),
                r#"RatingControl::new(7.0).max_rating(10).caption("Score out of 10")"#,
            ),
            sample_card(
                "Read-only Rating",
                RatingControl::new(4.5)
                    .read_only()
                    .caption("Average user rating"),
                r#"RatingControl::new(4.5).read_only().caption("Average")"#,
            ),
        ],
    )
    .into()
}
