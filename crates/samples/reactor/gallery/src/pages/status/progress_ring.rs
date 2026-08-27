use crate::controls::*;
use windows_reactor::*;

pub struct ProgressRingPage {
    value: f64,
}

impl Component for ProgressRingPage {
    type Message = f64;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { value: 75.0 }
    }

    fn update(&mut self, value: f64, _: &ComponentContext<Self>) {
        self.value = value;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "ProgressRing",
            "A circular indicator of ongoing progress.",
            [
                KeyedView::new(
                    "indeterminate",
                    sample_card(
                        "Indeterminate ProgressRing",
                        ProgressRing::new().is_active(true).is_indeterminate(true),
                        "ProgressRing::new().is_active(true).is_indeterminate(true)",
                    ),
                ),
                KeyedView::new(
                    "determinate",
                    sample_card(
                        "Determinate ProgressRing",
                        StackPanel::new().spacing(8.0).children((
                            ProgressRing::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(self.value)
                                .is_active(true),
                            Slider::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(self.value)
                                .on_value_changed(context.callback(std::convert::identity)),
                            TextBlock::new()
                                .text(format!("{:.0}%", self.value))
                                .opacity(0.6),
                        )),
                        "ProgressRing::new().value(value).is_active(true)",
                    ),
                ),
            ],
        )
    }
}
