use crate::controls::*;
use windows_reactor::*;

pub struct CalendarDatePickerPage {
    today: DateTime,
    date: Option<DateTime>,
}

impl Component for CalendarDatePickerPage {
    type Message = Option<DateTime>;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        let today = DateTime::now();
        Self {
            today,
            date: Some(today),
        }
    }

    fn update(&mut self, date: Option<DateTime>, _context: &ComponentContext<Self>) {
        self.date = date;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "CalendarDatePicker",
            "Pick a date from a calendar dropdown.",
            [
                KeyedView::new(
                    "basic-calendar-date-picker",
                    sample_card(
                        "Controlled CalendarDatePicker",
                        StackPanel::new().spacing(8.0).children((
                            CalendarDatePicker::new()
                                .placeholder_text("Select a date")
                                .min_date(self.today)
                                .max_date(self.today + TimeSpan::from_days(30))
                                .date(self.date)
                                .on_date_changed(context.forward())
                                .header("Appointment Date"),
                            TextBlock::new().text("Choose a date within the next 30 days."),
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    Button::new()
                                        .content("Reset")
                                        .on_click(context.message(Some(self.today))),
                                    Button::new()
                                        .content("Clear")
                                        .on_click(context.message(None)),
                                )),
                            TextBlock::new()
                                .text(self.date.map_or_else(
                                    || "No date selected".to_string(),
                                    |date| format!("Selected: {date}"),
                                ))
                                .opacity(0.6),
                        )),
                    ),
                ),
                KeyedView::new(
                    "disabled-calendar-date-picker",
                    sample_card(
                        "Disabled CalendarDatePicker",
                        CalendarDatePicker::new()
                            .placeholder_text("Cannot change")
                            .is_enabled(false)
                            .header("Locked Date"),
                    ),
                ),
            ],
        )
    }
}
