#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ComboBoxSample {
    selected: i32,
}

impl Component for ComboBoxSample {
    type Message = i32;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { selected: -1 }
    }

    fn update(&mut self, message: i32, _context: &ComponentContext<Self>) {
        self.selected = message;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let colors = ["Red", "Green", "Blue"];
        let label = if self.selected >= 0 {
            colors
                .get(self.selected as usize)
                .copied()
                .unwrap_or("(out of range)")
        } else {
            "(none)"
        };

        context.window_title("ComboBox");
        StackPanel::new().spacing(8.0).max_width(320.0).children((
            ComboBox::new()
                .items_source(colors)
                .placeholder_text("Pick a color")
                .selected_index(self.selected)
                .on_selection_changed(context.callback(|index| index))
                .slot(ComboBoxSlot::Header, "Color"),
            format!("selected_index = {} ({label})", self.selected),
            ComboBox::new()
                .items_source(["Cat", "Dog", "Fox"])
                .placeholder_text("Type or pick an animal")
                .is_editable(true)
                .slot(ComboBoxSlot::Header, "Editable"),
            ComboBox::new()
                .items_source(["A", "B", "C"])
                .selected_index(0)
                .is_enabled(false)
                .slot(ComboBoxSlot::Header, "Disabled"),
        ))
    }
}

fn main() {
    App::run_component::<ComboBoxSample>(()).unwrap();
}
