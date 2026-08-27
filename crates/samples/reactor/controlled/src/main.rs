use windows_reactor::*;

struct Controlled {
    number: f64,
    text: String,
}

enum Message {
    Number(f64),
    Text(String),
}

impl Component for Controlled {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            number: 5.0,
            text: String::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Number(value) => self.number = value,
            Message::Text(value) => self.text = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            TextBox::new()
                .text(self.text.clone())
                .placeholder_text("Type here")
                .on_text_changed(context.callback(Message::Text)),
            TextBlock::new().text(self.text.clone()),
            NumberBox::new()
                .minimum(0.0)
                .maximum(10.0)
                .value(self.number)
                .on_value_changed(context.callback(Message::Number)),
            TextBlock::new().text(self.number.to_string()),
        ))
    }
}

fn main() {
    App::run_component::<Controlled>(()).unwrap();
}
