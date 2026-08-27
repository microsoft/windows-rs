use crate::controls::*;
use windows_reactor::*;

pub struct TimePickerPage {
    label: String,
}

impl Component for TimePickerPage {
    type Message = TimeSpan;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "No time selected".to_string(),
        }
    }

    fn update(&mut self, time: TimeSpan, _context: &ComponentContext<Self>) {
        let hours = time.whole_hours();
        let minutes = time.whole_minutes() % 60;
        self.label = format!("Selected: {hours:02}:{minutes:02}");
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "TimePicker",
            "Pick a time using spinners.",
            [
                KeyedView::new(
                    "basic-time-picker",
                    sample_card(
                        "Basic TimePicker",
                        StackPanel::new().spacing(8.0).children((
                            TimePicker::new()
                                .on_selected_time_changed(context.callback(std::convert::identity))
                                .slots([SlotView::new(TimePickerSlot::Header, "Select time")]),
                            TextBlock::new().text(&self.label).opacity(0.6),
                        )),
                        "TimePicker::new()\n    .on_selected_time_changed(|time| ...)",
                    ),
                ),
                KeyedView::new(
                    "15-minute-time-picker",
                    sample_card(
                        "15-Minute Increments",
                        TimePicker::new()
                            .minute_increment(15)
                            .slots([SlotView::new(TimePickerSlot::Header, "Meeting time")]),
                        "TimePicker::new().minute_increment(15)",
                    ),
                ),
            ],
        )
    }
}
