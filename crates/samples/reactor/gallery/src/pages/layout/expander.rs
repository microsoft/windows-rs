use crate::controls::*;
use windows_reactor::*;

pub struct ExpanderPage {
    expanded: bool,
}

impl Component for ExpanderPage {
    type Message = bool;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { expanded: true }
    }

    fn update(&mut self, expanded: bool, _: &ComponentContext<Self>) {
        self.expanded = expanded;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Expander",
            "Expands and collapses to show or hide content.",
            [
                KeyedView::new(
                    "controlled",
                    sample_card(
                        "Controlled Expander",
                        StackPanel::new().spacing(8.0).children((
                            Expander::new()
                                .is_expanded(self.expanded)
                                .on_is_expanded_changed(context.forward())
                                .slots([
                                    SlotView::new(
                                        ExpanderSlot::Header,
                                        "Click to expand or collapse",
                                    ),
                                    SlotView::new(
                                        ExpanderSlot::Content,
                                        "This content can be shown or hidden.",
                                    ),
                                ]),
                            TextBlock::new()
                                .text(if self.expanded {
                                    "Expanded"
                                } else {
                                    "Collapsed"
                                })
                                .opacity(0.6),
                        )),
                        "Expander::new().is_expanded(state).on_is_expanded_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "collapsed",
                    sample_card(
                        "Collapsed by Default",
                        Expander::new().is_expanded(false).slots([
                            SlotView::new(ExpanderSlot::Header, "More info"),
                            SlotView::new(ExpanderSlot::Content, "Hidden by default."),
                        ]),
                        "Expander::new().is_expanded(false)",
                    ),
                ),
            ],
        )
    }
}
