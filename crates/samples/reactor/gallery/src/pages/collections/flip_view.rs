use crate::controls::*;
use windows_reactor::*;

pub struct FlipViewPage {
    selected: i32,
}

#[derive(Clone)]
pub enum Message {
    Selected(i32),
}

impl Component for FlipViewPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { selected: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(index) => self.selected = index,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let slide = |key: &'static str, label: &'static str, index: i32| {
            KeyedView::new(
                key,
                Border::new()
                    .padding(Thickness::uniform(24.0))
                    .corner_radius(8.0)
                    .content(
                        StackPanel::new().spacing(8.0).children((
                            TextBlock::new()
                                .text(label)
                                .font_size(24.0)
                                .font_weight(700),
                            TextBlock::new()
                                .text(format!("Slide {} of 4", index + 1))
                                .opacity(0.6),
                        )),
                    ),
            )
        };

        page_content(
            "FlipView",
            "Presents one item at a time with flipping navigation.",
            [KeyedView::new(
                "interactive-flip-view",
                sample_card(
                    "Interactive FlipView",
                    StackPanel::new().spacing(8.0).children((
                        FlipView::new()
                            .selected_index(self.selected)
                            .on_selection_changed(context.callback(Message::Selected))
                            .height(200.0)
                            .collection_slot(
                                FlipViewSlot::Items,
                                [
                                    slide("welcome", "Welcome", 0),
                                    slide("features", "Features", 1),
                                    slide("getting-started", "Getting Started", 2),
                                    slide("resources", "Resources", 3),
                                ],
                            ),
                        TextBlock::new()
                            .text(format!("Current slide: {}", self.selected + 1))
                            .opacity(0.6),
                    )),
                    r#"FlipView::new().selected_index(selected).on_selection_changed(...)
    .collection_slot(FlipViewSlot::Items, [...])"#,
                ),
            )],
        )
    }
}
