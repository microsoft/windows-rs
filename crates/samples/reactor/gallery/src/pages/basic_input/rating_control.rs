use crate::controls::*;
use windows_reactor::*;

pub struct RatingControlPage {
    rating: Option<f64>,
}

impl Component for RatingControlPage {
    type Message = Option<f64>;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { rating: Some(3.0) }
    }

    fn update(&mut self, rating: Option<f64>, _: &ComponentContext<Self>) {
        self.rating = rating;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "RatingControl",
            "A control that lets users provide a star rating.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic RatingControl",
                        StackPanel::new().spacing(8.0).children((
                            RatingControl::new()
                                .value(self.rating)
                                .on_value_changed(context.forward()),
                            TextBlock::new()
                                .text(format!(
                                    "Rating: {} / 5",
                                    self.rating.map_or_else(
                                        || "(none)".to_string(),
                                        |value| format!("{value:.1}")
                                    )
                                ))
                                .opacity(0.6),
                        )),
                        "RatingControl::new().value(rating).on_value_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "custom",
                    sample_card(
                        "Custom Max Rating",
                        RatingControl::new()
                            .value(7.0)
                            .max_rating(10)
                            .caption("Score out of 10"),
                        "RatingControl::new().value(7.0).max_rating(10)",
                    ),
                ),
                KeyedView::new(
                    "read-only",
                    sample_card(
                        "Read-only Rating",
                        RatingControl::new()
                            .value(4.5)
                            .is_read_only(true)
                            .caption("Average user rating"),
                        "RatingControl::new().value(4.5).is_read_only(true)",
                    ),
                ),
            ],
        )
    }
}
