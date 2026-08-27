use crate::controls::*;
use windows_reactor::*;

#[derive(Clone)]
pub enum Message {
    OpenBasic,
    ClosedBasic,
    OpenAction,
    Action,
    ClosedAction,
}

pub struct TeachingTipPage {
    show_basic: bool,
    show_action: bool,
    action_count: i32,
}

impl Component for TeachingTipPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            show_basic: false,
            show_action: false,
            action_count: 0,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::OpenBasic => self.show_basic = true,
            Message::ClosedBasic => self.show_basic = false,
            Message::OpenAction => self.show_action = true,
            Message::Action => self.action_count += 1,
            Message::ClosedAction => self.show_action = false,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "TeachingTip",
            "A notification flyout for guiding users.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic TeachingTip",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.message(Message::OpenBasic))
                                .content("Show Tip"),
                            TeachingTip::new()
                                .title("Did you know?")
                                .subtitle("You can customize this teaching tip with a subtitle.")
                                .is_open(self.show_basic)
                                .on_closed(context.message(Message::ClosedBasic)),
                        )),
                        r#"TeachingTip::new()
    .title("Did you know?")
    .subtitle("You can customize this teaching tip with a subtitle.")
    .is_open(show_basic)
    .on_closed(context.message(Message::ClosedBasic))"#,
                    ),
                ),
                KeyedView::new(
                    "action",
                    sample_card(
                        "TeachingTip with Action",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.message(Message::OpenAction))
                                .content("Show Action Tip"),
                            TeachingTip::new()
                                .title("Take Action")
                                .subtitle("Click the action button to increment the counter.")
                                .is_open(self.show_action)
                                .action_button_content("Got it!")
                                .close_button_content("Dismiss")
                                .on_action_button_click(context.message(Message::Action))
                                .on_closed(context.message(Message::ClosedAction)),
                            TextBlock::new()
                                .text(format!("Action clicked: {} times", self.action_count))
                                .opacity(0.6),
                        )),
                        r#"TeachingTip::new()
    .title("Take Action")
    .action_button_content("Got it!")
    .close_button_content("Dismiss")
    .on_action_button_click(context.message(Message::Action))
    .on_closed(context.message(Message::ClosedAction))"#,
                    ),
                ),
            ],
        )
    }
}
