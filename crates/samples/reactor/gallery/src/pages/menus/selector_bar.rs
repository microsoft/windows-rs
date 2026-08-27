use crate::controls::*;
use windows_reactor::*;

pub struct SelectorBarPage {
    selected: String,
}

#[derive(Clone)]
pub enum Message {
    SelectionChanged(Option<String>),
}

impl Component for SelectorBarPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            selected: "Recent".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::SelectionChanged(Some(selected)) => self.selected = selected,
            Message::SelectionChanged(None) => {}
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let item = |text: &'static str| {
            KeyedView::new(
                text,
                SelectorBarItem::new()
                    .text(text)
                    .is_selected(self.selected == text),
            )
        };

        page_content(
            "SelectorBar",
            "Switch between different views or modes.",
            [KeyedView::new(
                "basic-selector-bar",
                sample_card(
                    "Basic SelectorBar",
                    StackPanel::new().spacing(8.0).children((
                        SelectorBar::new()
                            .on_selected_text_changed(context.callback(Message::SelectionChanged))
                            .slots([SlotView::collection(
                                SelectorBarSlot::Items,
                                [item("Recent"), item("Shared"), item("Favorites")],
                            )]),
                        TextBlock::new()
                            .text(format!("Selected: {}", self.selected))
                            .opacity(0.6),
                    )),
                    r#"SelectorBar::new().on_selected_text_changed(h).slots([...])"#,
                ),
            )],
        )
    }
}
