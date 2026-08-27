use crate::controls::*;
use windows_reactor::*;

pub struct DatePickerPage {
    label: String,
}

impl Component for DatePickerPage {
    type Message = DateTime;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "No date selected".to_string(),
        }
    }

    fn update(&mut self, date: DateTime, _context: &ComponentContext<Self>) {
        self.label = format!("Selected: {date}");
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "DatePicker",
            "Pick a date using spinners.",
            [
                KeyedView::new(
                    "full-date-picker",
                    sample_card(
                        "Full DatePicker",
                        StackPanel::new().spacing(8.0).children((
                            DatePicker::new()
                                .on_selected_date_changed(context.callback(std::convert::identity))
                                .slots([SlotView::new(DatePickerSlot::Header, "Select date")]),
                            TextBlock::new().text(&self.label).opacity(0.6),
                        )),
                        "DatePicker::new()\n    .on_selected_date_changed(|date| ...)",
                    ),
                ),
                KeyedView::new(
                    "month-year-date-picker",
                    sample_card(
                        "Month and Year Only",
                        DatePicker::new()
                            .day_visible(false)
                            .slots([SlotView::new(DatePickerSlot::Header, "Month/Year")]),
                        "DatePicker::new().day_visible(false)",
                    ),
                ),
            ],
        )
    }
}
