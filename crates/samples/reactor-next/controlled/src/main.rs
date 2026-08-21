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

    fn update(&mut self, message: Message, _context: &mut ComponentContext<Self>) {
        match message {
            Message::Number(value) => self.number = value,
            Message::Text(value) => self.text = value,
        }
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
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
    bootstrap().unwrap();
    App::run_component::<Controlled>(()).unwrap();
}
