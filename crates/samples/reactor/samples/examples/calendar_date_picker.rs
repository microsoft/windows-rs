use windows_reactor::*;

struct CalendarDatePickerSample {
    label: String,
}

impl Component for CalendarDatePickerSample {
    type Message = DateTime;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "Pick a date to see days from today".to_string(),
        }
    }

    fn update(&mut self, selected: DateTime, _context: &ComponentContext<Self>) {
        let now = DateTime::now();
        self.label = match selected.checked_duration_since(now) {
            Some(span) => {
                let days = span.whole_days();
                match days.cmp(&0) {
                    std::cmp::Ordering::Greater => format!("{days} day(s) from now"),
                    std::cmp::Ordering::Less => format!("{} day(s) ago", days.abs()),
                    std::cmp::Ordering::Equal => "That's today!".to_string(),
                }
            }
            None => "Date too far away to compute".to_string(),
        };
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("CalendarDatePicker");
        StackPanel::new().spacing(8.0).children((
            CalendarDatePicker::new()
                .placeholder_text("Choose...")
                .on_date_changed(context.callback(std::convert::identity))
                .slots([SlotView::new(
                    CalendarDatePickerSlot::Header,
                    TextBlock::new().text("Select a date"),
                )]),
            TextBlock::new().text(&self.label),
        ))
    }
}

fn main() {
    App::run_component::<CalendarDatePickerSample>(()).unwrap();
}
