use crate::controls::*;
use windows_reactor::*;

pub struct CalendarDatePickerPage {
    label: String,
}

impl Component for CalendarDatePickerPage {
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
            "CalendarDatePicker",
            "Pick a date from a calendar dropdown.",
            [
                KeyedView::new(
                    "basic-calendar-date-picker",
                    sample_card(
                        "Basic CalendarDatePicker",
                        StackPanel::new().spacing(8.0).children((
                            CalendarDatePicker::new()
                                .placeholder_text("Select a date")
                                .on_date_changed(context.forward())
                                .slot(CalendarDatePickerSlot::Header, "Appointment Date"),
                            TextBlock::new().text(&self.label).opacity(0.6),
                        )),
                        "CalendarDatePicker::new()\n    .placeholder_text(\"Select a date\")\n    .on_date_changed(|date| ...)",
                    ),
                ),
                KeyedView::new(
                    "disabled-calendar-date-picker",
                    sample_card(
                        "Disabled CalendarDatePicker",
                        CalendarDatePicker::new()
                            .placeholder_text("Cannot change")
                            .is_enabled(false)
                            .slot(CalendarDatePickerSlot::Header, "Locked Date"),
                        "CalendarDatePicker::new().is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
