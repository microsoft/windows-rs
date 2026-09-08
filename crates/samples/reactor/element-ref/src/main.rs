use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Focus,
    Focused(Result<bool, FocusError>),
}

struct TypedElementReference {
    input: ElementRef<TextBox>,
    status: &'static str,
}

impl Component for TypedElementReference {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            input: ElementRef::new(),
            status: "Not focused",
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Focus => {
                let sender = context.sender();
                if !self.input.request_focus_result(move |result| {
                    sender.send(Message::Focused(result));
                }) {
                    self.status = "Focus failed";
                }
            }
            Message::Focused(Ok(true)) => self.status = "Focused",
            Message::Focused(Ok(false)) => self.status = "Focus rejected",
            Message::Focused(Err(_)) => self.status = "Focus failed",
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Typed Element Reference");
        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(8.0).children((
                "The typed reference exists across renders, points at the TextBox only while \
                 mounted, and cannot be attached to a different widget type.",
                TextBox::new().text("Focus target").element_ref(&self.input),
                Button::new()
                    .on_click(context.message(Message::Focus))
                    .content("Focus TextBox"),
                self.status,
            )),
        )
    }
}

fn main() {
    App::run_component::<TypedElementReference>(()).unwrap();
}
