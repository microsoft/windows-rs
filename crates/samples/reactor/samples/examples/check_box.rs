#![windows_subsystem = "windows"]

use windows_reactor::*;

struct CheckBoxSample {
    checked: bool,
}

impl Component for CheckBoxSample {
    type Message = bool;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { checked: false }
    }

    fn update(&mut self, checked: bool, _context: &ComponentContext<Self>) {
        self.checked = checked;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("CheckBox");
        StackPanel::new().spacing(8.0).children((
            CheckBox::new()
                .is_checked(self.checked)
                .on_is_checked_changed(context.callback(|checked| checked))
                .content("I accept the terms"),
            if self.checked {
                "Accepted"
            } else {
                "Not yet accepted"
            },
            CheckBox::new()
                .is_checked(true)
                .is_enabled(false)
                .content("Disabled (always on)"),
        ))
    }
}

fn main() {
    App::run_component::<CheckBoxSample>(()).unwrap();
}
