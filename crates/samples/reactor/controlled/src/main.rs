use std::rc::Rc;

use windows_reactor::*;

struct Controlled {
    number: Option<f64>,
    text: Rc<str>,
}

enum Message {
    Number(Option<f64>),
    Text(Rc<str>),
}

impl Component for Controlled {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            number: Some(5.0),
            text: Rc::from(""),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Number(value) => self.number = value,
            Message::Text(value) => self.text = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(8.0)
            .children((
                TextBox::new(self.text.clone())
                    .placeholder_text("Type here")
                    .on_text_changed(context.callback(Message::Text)),
                self.text.clone(),
                NumberBox::new()
                    .minimum(0.0)
                    .maximum(10.0)
                    .value(self.number)
                    .on_value_changed(context.callback(Message::Number)),
                self.number
                    .map_or_else(|| "(empty)".to_string(), |number| number.to_string()),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Controlled>(()).unwrap();
}
