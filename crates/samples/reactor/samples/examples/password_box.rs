#![windows_subsystem = "windows"]

use windows_reactor::*;

struct PasswordBoxSample {
    password: String,
}

impl Component for PasswordBoxSample {
    type Message = String;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            password: String::new(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.password = message;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("PasswordBox");
        StackPanel::new().spacing(8.0).max_width(320.0).children((
            PasswordBox::new()
                .password(self.password.clone())
                .placeholder_text("Type a password...")
                .on_password_changed(context.callback(|password| password))
                .slot(PasswordBoxSlot::Header, "Password"),
            format!("captured length = {}", self.password.chars().count()),
            PasswordBox::new()
                .placeholder_text("Reveal hidden")
                .password_reveal_mode(PasswordRevealMode::Hidden)
                .slot(PasswordBoxSlot::Header, "No reveal button"),
            PasswordBox::new()
                .is_enabled(false)
                .slot(PasswordBoxSlot::Header, "Disabled"),
        ))
    }
}

fn main() {
    App::run_component::<PasswordBoxSample>(()).unwrap();
}
