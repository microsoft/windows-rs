use windows_reactor::*;

struct DatePickerSample {
    label: String,
}

impl Component for DatePickerSample {
    type Message = Option<DateTime>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "No date picked".to_string(),
        }
    }

    fn update(&mut self, date: Option<DateTime>, _context: &ComponentContext<Self>) {
        self.label = date.map_or_else(
            || "No date picked".to_string(),
            |date| format!("Picked: {date}"),
        );
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("DatePicker");
        StackPanel::new().spacing(8.0).children((
            DatePicker::new()
                .on_selected_date_changed(context.forward())
                .slot(DatePickerSlot::Header, "Pick a date"),
            self.label.as_str(),
        ))
    }
}

fn main() {
    App::run_component::<DatePickerSample>(()).unwrap();
}
