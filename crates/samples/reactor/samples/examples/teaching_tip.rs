use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Open,
    Action,
    Closed,
}

struct TeachingTipSample {
    is_open: bool,
    status: String,
}

impl Component for TeachingTipSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            is_open: false,
            status: "(tip closed)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Open => self.is_open = true,
            Message::Action => self.status = "Action button clicked!".to_string(),
            Message::Closed => {
                self.is_open = false;
                self.status = "Tip was closed/dismissed".to_string();
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TeachingTip");
        StackPanel::new().spacing(12.0).children((
            Button::new()
                .on_click(context.message(Message::Open))
                .content("Show Teaching Tip"),
            format!("Status: {}", self.status),
            TeachingTip::new()
                .title("Welcome!")
                .subtitle("This is a teaching tip with action and close buttons.")
                .is_open(self.is_open)
                .is_light_dismiss_enabled(true)
                .preferred_placement(TeachingTipPlacementMode::Bottom)
                .action_button_content("Got it")
                .close_button_content("Dismiss")
                .on_action_button_click(context.message(Message::Action))
                .on_closed(context.message(Message::Closed)),
        ))
    }
}

fn main() {
    App::run_component::<TeachingTipSample>(()).unwrap();
}
