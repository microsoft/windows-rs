use crate::controls::*;
use windows_reactor::*;

pub struct PivotPage {
    selected: i32,
}

impl Component for PivotPage {
    type Message = i32;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { selected: 0 }
    }

    fn update(&mut self, selected: i32, _: &ComponentContext<Self>) {
        self.selected = selected;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let items = |prefix: &'static str| {
            ["Overview", "Details", "History"]
                .into_iter()
                .map(move |header| {
                    KeyedView::new(
                        format!("{prefix}-{header}"),
                        PivotItem::new()
                            .header(header)
                            .content(format!("{header} content")),
                    )
                })
        };
        page_content(
            "Pivot",
            "A tabbed interface for switching between content sections.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Pivot",
                        Pivot::new()
                            .selected_index(self.selected)
                            .on_selection_changed(context.forward())
                            .collection_slot(PivotSlot::Items, items("basic")),
                        "Pivot::new().selected_index(index)\n    \
                         .collection_slot(PivotSlot::Items, items)",
                    ),
                ),
                KeyedView::new(
                    "tracking",
                    sample_card(
                        "Selection Tracking",
                        StackPanel::new().spacing(8.0).children((
                            Pivot::new()
                                .selected_index(self.selected)
                                .on_selection_changed(context.forward())
                                .collection_slot(PivotSlot::Items, items("tracking")),
                            TextBlock::new()
                                .text(format!("Active tab index: {}", self.selected))
                                .opacity(0.6),
                        )),
                        "Pivot::new().on_selection_changed(handler)",
                    ),
                ),
            ],
        )
    }
}
