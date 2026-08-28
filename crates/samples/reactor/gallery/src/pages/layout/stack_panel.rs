use crate::controls::*;
use windows_reactor::*;

pub struct StackPanelPage {
    count: u32,
}

#[derive(Clone)]
pub enum Message {
    Add,
    Remove,
}

impl Component for StackPanelPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { count: 3 }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count += 1,
            Message::Remove => self.count = self.count.saturating_sub(1),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let items = (1..=self.count).map(|index| KeyedView::new(index, format!("Item {index}")));
        page_content(
            "StackPanel",
            "Arranges children in a single horizontal or vertical line.",
            [
                KeyedView::new(
                    "dynamic",
                    sample_card(
                        "Dynamic Items",
                        StackPanel::new().spacing(12.0).children((
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    Button::new()
                                        .on_click(context.message(Message::Add))
                                        .content("Add"),
                                    Button::new()
                                        .is_enabled(self.count > 0)
                                        .on_click(context.message(Message::Remove))
                                        .content("Remove"),
                                )),
                            StackPanel::new().spacing(4.0).keyed_children(items),
                        )),
                        "StackPanel::new().spacing(4.0).keyed_children(items)",
                    ),
                ),
                KeyedView::new(
                    "horizontal",
                    sample_card(
                        "Horizontal Stack",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                Button::new().content("A"),
                                Button::new().content("B"),
                                Button::new().content("C"),
                            )),
                        "StackPanel::new().orientation(Orientation::Horizontal)",
                    ),
                ),
            ],
        )
    }
}
