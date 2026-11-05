use crate::controls::*;
use std::rc::Rc;
use windows_reactor::*;

pub struct SelectorBarPage {
    selected: Rc<str>,
}

impl Component for SelectorBarPage {
    type Message = Option<Rc<str>>;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            selected: "Recent".into(),
        }
    }

    fn update(&mut self, selected: Option<Rc<str>>, _context: &ComponentContext<Self>) {
        if let Some(selected) = selected {
            self.selected = selected;
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let item = |text: &'static str| {
            KeyedView::new(
                text,
                SelectorBarItem::new()
                    .text(text)
                    .is_selected(self.selected.as_ref() == text),
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
                            .on_selected_text_changed(context.forward())
                            .keyed_items([item("Recent"), item("Shared"), item("Favorites")]),
                        TextBlock::new()
                            .text(format!("Selected: {}", self.selected))
                            .opacity(0.6),
                    )),
                ),
            )],
        )
    }
}
