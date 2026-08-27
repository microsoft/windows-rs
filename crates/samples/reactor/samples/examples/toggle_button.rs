#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Bold(bool),
    Italic(bool),
}

struct ToggleButtonSample {
    bold: bool,
    italic: bool,
}

impl Component for ToggleButtonSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            bold: false,
            italic: false,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Bold(value) => self.bold = value,
            Message::Italic(value) => self.italic = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ToggleButton");
        let style = match (self.bold, self.italic) {
            (true, true) => "Bold + Italic",
            (true, false) => "Bold",
            (false, true) => "Italic",
            (false, false) => "Normal",
        };
        StackPanel::new().spacing(8.0).children((
            ToggleButton::new()
                .is_checked(self.bold)
                .on_is_checked_changed(context.callback(Message::Bold))
                .content(TextBlock::new().text("Bold")),
            ToggleButton::new()
                .is_checked(self.italic)
                .on_is_checked_changed(context.callback(Message::Italic))
                .content(TextBlock::new().text("Italic")),
            TextBlock::new().text(format!("Style: {style}")),
        ))
    }
}

fn main() {
    App::run_component::<ToggleButtonSample>(()).unwrap();
}
