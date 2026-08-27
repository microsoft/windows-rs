#![windows_subsystem = "windows"]

use windows_reactor::*;

struct RatingControlSample {
    rating: f64,
}

enum Message {
    RatingChanged(f64),
}

impl Component for RatingControlSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { rating: 3.0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::RatingChanged(value) => self.rating = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("RatingControl");
        StackPanel::new().spacing(8.0).children((
            RatingControl::new()
                .value(self.rating)
                .on_value_changed(context.callback(Message::RatingChanged)),
            format!("Rating: {:.1} / 5", self.rating),
            RatingControl::new()
                .value(4.0)
                .max_rating(10)
                .caption("Out of 10"),
            RatingControl::new().value(2.5).is_read_only(true),
        ))
    }
}

fn main() {
    App::run_component::<RatingControlSample>(()).unwrap();
}
