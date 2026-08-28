use crate::controls::*;
use windows_reactor::*;

pub struct ScrollViewPage {
    count: u32,
}

#[derive(Clone)]
pub enum Message {
    More,
    Fewer,
}

impl Component for ScrollViewPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { count: 30 }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::More => self.count += 10,
            Message::Fewer => self.count = self.count.saturating_sub(10).max(5),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let items = (1..=self.count).map(|index| KeyedView::new(index, format!("Item {index}")));
        page_content(
            "ScrollView",
            "A scrollable container for overflowing content.",
            [KeyedView::new(
                "dynamic",
                sample_card(
                    "Dynamic ScrollView",
                    StackPanel::new().spacing(12.0).children((
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                Button::new()
                                    .on_click(context.message(Message::More))
                                    .content("More items"),
                                Button::new()
                                    .on_click(context.message(Message::Fewer))
                                    .content("Fewer items"),
                                TextBlock::new()
                                    .text(format!("{} items", self.count))
                                    .opacity(0.6),
                            )),
                        ScrollView::new()
                            .height(200.0)
                            .content(StackPanel::new().spacing(4.0).keyed_children(items)),
                    )),
                    "ScrollView::new().height(200.0).content(items)",
                ),
            )],
        )
    }
}
