use crate::controls::*;
use windows_reactor::*;

pub struct ProgressBarPage {
    value: f64,
    loading: bool,
}

#[derive(Clone)]
pub enum Message {
    Value(f64),
    Loading(bool),
}

impl Component for ProgressBarPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            value: 60.0,
            loading: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Value(value) => self.value = value,
            Message::Loading(value) => self.loading = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "ProgressBar",
            "A horizontal bar that shows progress of an operation.",
            [
                KeyedView::new(
                    "determinate",
                    sample_card(
                        "Determinate ProgressBar",
                        StackPanel::new().spacing(8.0).children((
                            ProgressBar::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(self.value)
                                .width(300.0),
                            Slider::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(self.value)
                                .width(300.0)
                                .on_value_changed(context.callback(Message::Value)),
                            TextBlock::new()
                                .text(format!("Progress: {:.0}%", self.value))
                                .opacity(0.6),
                        )),
                        "ProgressBar::new().value(value)",
                    ),
                ),
                KeyedView::new(
                    "indeterminate",
                    sample_card(
                        "Indeterminate ProgressBar",
                        StackPanel::new().spacing(8.0).children((
                            ProgressBar::new()
                                .is_indeterminate(self.loading)
                                .value(100.0)
                                .width(300.0),
                            ToggleSwitch::new()
                                .is_on(self.loading)
                                .on_toggled(context.callback(Message::Loading))
                                .slots([
                                    SlotView::new(
                                        ToggleSwitchSlot::OnContent,
                                        TextBlock::new().text("Loading"),
                                    ),
                                    SlotView::new(
                                        ToggleSwitchSlot::OffContent,
                                        TextBlock::new().text("Complete"),
                                    ),
                                ]),
                        )),
                        "ProgressBar::new().is_indeterminate(loading)",
                    ),
                ),
                KeyedView::new(
                    "states",
                    sample_card(
                        "Paused and Error States",
                        StackPanel::new().spacing(8.0).children((
                            ProgressBar::new().value(45.0).show_paused(true),
                            ProgressBar::new().value(70.0).show_error(true),
                        )),
                        "ProgressBar::new().show_paused(true)",
                    ),
                ),
            ],
        )
    }
}
