#![windows_subsystem = "windows"]

use windows_reactor::*;

struct TextBoxSample {
    name: String,
    notes: String,
}

enum Message {
    NameChanged(String),
    NotesChanged(String),
}

impl Component for TextBoxSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            name: String::new(),
            notes: String::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::NameChanged(value) => self.name = value,
            Message::NotesChanged(value) => self.notes = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TextBox");
        StackPanel::new().spacing(8.0).children((
            TextBox::new()
                .text(self.name.clone())
                .placeholder_text("Type your name...")
                .on_text_changed(context.callback(Message::NameChanged))
                .slots([SlotView::new(
                    TextBoxSlot::Header,
                    TextBlock::new().text("Display name"),
                )]),
            TextBlock::new().text(format!(
                "Hello, {}!",
                if self.name.is_empty() {
                    "stranger"
                } else {
                    self.name.as_str()
                }
            )),
            TextBox::new()
                .text(self.notes.clone())
                .placeholder_text("Write something longer...")
                .accepts_return(true)
                .text_wrapping(TextWrapping::Wrap)
                .height(100.0)
                .on_text_changed(context.callback(Message::NotesChanged))
                .slots([SlotView::new(
                    TextBoxSlot::Header,
                    TextBlock::new().text("Notes"),
                )]),
            TextBox::new()
                .text("read-only")
                .is_enabled(false)
                .slots([SlotView::new(
                    TextBoxSlot::Header,
                    TextBlock::new().text("Disabled"),
                )]),
        ))
    }
}

fn main() {
    App::run_component::<TextBoxSample>(()).unwrap();
}
