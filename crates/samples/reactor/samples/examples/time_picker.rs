use windows_reactor::*;

struct TimePickerSample {
    label: String,
}

impl Component for TimePickerSample {
    type Message = TimeSpan;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "No time picked".to_string(),
        }
    }

    fn update(&mut self, time: TimeSpan, _context: &ComponentContext<Self>) {
        let hours = time.whole_hours();
        let minutes = time.whole_minutes() % 60;
        self.label = format!("Picked: {hours:02}:{minutes:02}");
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TimePicker");
        StackPanel::new().spacing(8.0).children((
            TimePicker::new()
                .minute_increment(15)
                .on_selected_time_changed(context.forward())
                .slot(TimePickerSlot::Header, "Pick a time"),
            self.label.as_str(),
        ))
    }
}

fn main() {
    App::run_component::<TimePickerSample>(()).unwrap();
}
