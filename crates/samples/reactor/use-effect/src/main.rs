use std::rc::Rc;
use windows_reactor::*;

struct Editor {
    editing: bool,
    text: Rc<str>,
    input: ElementRef<TextBox>,
}

#[derive(Clone)]
enum Message {
    Toggle,
    Text(Rc<str>),
}

impl Component for Editor {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            editing: false,
            text: Rc::from("Edit this note"),
            input: ElementRef::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Toggle => self.editing = !self.editing,
            Message::Text(text) => self.text = text,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let content: View = if self.editing {
            let input = self.input.clone();
            // The effect runs after the new TextBox has been mounted and its reference bound.
            context.use_effect("focus-editor", (), move || {
                assert!(input.request_focus());
                None
            });
            TextBox::new()
                .element_ref(&self.input)
                .text(self.text.clone())
                .on_text_changed(context.callback(Message::Text))
                .into()
        } else {
            TextBlock::new()
                .text(self.text.clone())
                .text_wrapping(TextWrapping::Wrap)
                .into()
        };
        context.window_frame(
            "Focus after mounting",
            StackPanel::new().margin(24.0).spacing(12.0).children((
                "Start editing to mount and focus a TextBox. Typing does not rerun the effect.",
                Button::new()
                    .content(if self.editing { "Done" } else { "Edit" })
                    .on_click(context.message(Message::Toggle)),
                content,
            )),
        )
    }
}

fn main() {
    App::run_component::<Editor>(()).unwrap();
}
