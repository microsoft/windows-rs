use windows_reactor_next::*;

struct Controlled {
    number: f64,
    text: String,
}

enum Message {
    Number(f64),
    Text(String),
}

impl Component for Controlled {
    type Props = ();
    type Message = Message;

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self {
            number: 5.0,
            text: String::new(),
        }
    }

    fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, message: Message, _context: &mut ComponentContext<Self>) {
        match message {
            Message::Number(value) => self.number = value,
            Message::Text(value) => self.text = value,
        }
    }

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        let changed = context.sender();
        let number_changed = changed.clone();
        View::children(
            StackPanel::new().spacing(8.0),
            [
                KeyedView::new(
                    "input",
                    View::native(
                        TextBox::new()
                            .text(self.text.clone())
                            .placeholder_text("Type here")
                            .on_text_changed(move |value| {
                                changed.send(Message::Text(value));
                            }),
                    ),
                ),
                KeyedView::new(
                    "value",
                    View::native(TextBlock::new().text(self.text.clone())),
                ),
                KeyedView::new(
                    "number-input",
                    View::native(
                        NumberBox::new()
                            .minimum(0.0)
                            .maximum(10.0)
                            .value(self.number)
                            .on_value_changed(move |value| {
                                number_changed.send(Message::Number(value));
                            }),
                    ),
                ),
                KeyedView::new(
                    "number-value",
                    View::native(TextBlock::new().text(self.number.to_string())),
                ),
            ],
        )
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<Controlled>(()).unwrap();
}
