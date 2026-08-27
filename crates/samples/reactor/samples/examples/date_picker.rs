use windows_reactor::*;

struct DatePickerSample {
    label: String,
}

impl Component for DatePickerSample {
    type Message = DateTime;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            label: "No date picked".to_string(),
        }
    }

    fn update(&mut self, date: DateTime, _context: &ComponentContext<Self>) {
        self.label = format!("Picked: {date}");
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("DatePicker");
        StackPanel::new().spacing(8.0).children((
            DatePicker::new()
                .on_selected_date_changed(context.callback(std::convert::identity))
                .slots([SlotView::new(
                    DatePickerSlot::Header,
                    TextBlock::new().text("Pick a date"),
                )]),
            TextBlock::new().text(&self.label),
        ))
    }
}

fn main() {
    App::run_component::<DatePickerSample>(()).unwrap();
}
