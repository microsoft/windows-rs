#![windows_subsystem = "windows"]

use windows_reactor::*;

struct RadioButtonsSample {
    selected: i32,
}

impl Component for RadioButtonsSample {
    type Message = i32;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { selected: 0 }
    }

    fn update(&mut self, message: i32, _context: &ComponentContext<Self>) {
        self.selected = message;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        const OPTIONS: [&str; 3] = ["Email", "SMS", "None"];
        let label = usize::try_from(self.selected)
            .ok()
            .and_then(|index| OPTIONS.get(index))
            .copied()
            .unwrap_or("(none)");

        context.window_title("RadioButtons");
        StackPanel::new().spacing(8.0).children((
            RadioButtons::new()
                .items_source(OPTIONS)
                .selected_index(self.selected)
                .max_columns(3)
                .on_selection_changed(context.callback(|index| index))
                .slot(RadioButtonsSlot::Header, "Notifications"),
            format!("selected_index = {} ({label})", self.selected),
        ))
    }
}

fn main() {
    App::run_component::<RadioButtonsSample>(()).unwrap();
}
