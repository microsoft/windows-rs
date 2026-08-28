use crate::controls::*;
use windows_reactor::*;

pub struct RepeatButtonPage {
    count: u32,
    fast_count: u32,
}

#[derive(Clone)]
pub enum Message {
    Basic,
    Fast,
}

impl Component for RepeatButtonPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            count: 0,
            fast_count: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Basic => self.count += 1,
            Message::Fast => self.fast_count += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "RepeatButton",
            "A button that raises click events repeatedly while pressed.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic RepeatButton",
                        StackPanel::new().spacing(8.0).children((
                            RepeatButton::new()
                                .on_click(context.message(Message::Basic))
                                .content("Hold me"),
                            TextBlock::new()
                                .text(format!("Count: {}", self.count))
                                .opacity(0.6),
                        )),
                        "RepeatButton::new().on_click(handler).content(label)",
                    ),
                ),
                KeyedView::new(
                    "fast",
                    sample_card(
                        "Fast Repeat",
                        StackPanel::new().spacing(8.0).children((
                            RepeatButton::new()
                                .delay(200)
                                .interval(50)
                                .on_click(context.message(Message::Fast))
                                .content("Fast +1"),
                            TextBlock::new()
                                .text(format!("Fast count: {}", self.fast_count))
                                .opacity(0.6),
                        )),
                        "RepeatButton::new().delay(200).interval(50).on_click(handler)",
                    ),
                ),
            ],
        )
    }
}
