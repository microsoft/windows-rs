use crate::controls::*;
use windows_reactor::*;

pub struct InfoBadgePage {
    count: i32,
}

#[derive(Clone)]
pub enum Message {
    Add,
    Clear,
}

impl Component for InfoBadgePage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { count: 3 }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count += 1,
            Message::Clear => self.count = 0,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "InfoBadge",
            "A small indicator conveying status on another element.",
            [
                KeyedView::new(
                    "dynamic",
                    sample_card(
                        "Dynamic Counter",
                        StackPanel::new().spacing(12.0).children((
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    Button::new()
                                        .on_click(context.message(Message::Add))
                                        .content("Add notification"),
                                    Button::new()
                                        .on_click(context.message(Message::Clear))
                                        .content("Clear"),
                                )),
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    InfoBadge::new().value(self.count),
                                    TextBlock::new()
                                        .text(format!("{} unread", self.count))
                                        .opacity(0.6),
                                )),
                        )),
                        "InfoBadge::new().value(count)",
                    ),
                ),
                KeyedView::new(
                    "numeric",
                    sample_card(
                        "Numeric InfoBadges",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(16.0)
                            .children((
                                InfoBadge::new().value(1),
                                InfoBadge::new().value(12),
                                InfoBadge::new().value(99),
                            )),
                        "InfoBadge::new().value(12)",
                    ),
                ),
            ],
        )
    }
}
