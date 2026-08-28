use crate::controls::*;
use windows_reactor::*;

pub struct SplitViewPage {
    open: bool,
}

impl Component for SplitViewPage {
    type Message = bool;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { open: true }
    }

    fn update(&mut self, open: bool, _: &ComponentContext<Self>) {
        self.open = open;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "SplitView",
            "A collapsible pane and content area.",
            [KeyedView::new(
                "basic",
                sample_card(
                    "Basic SplitView",
                    StackPanel::new().spacing(8.0).children((
                        ToggleSwitch::new()
                            .is_on(self.open)
                            .on_toggled(context.forward())
                            .slot(ToggleSwitchSlot::Header, "Pane open"),
                        SplitView::new()
                            .is_pane_open(self.open)
                            .open_pane_length(180.0)
                            .slots([
                                SlotView::new(
                                    SplitViewSlot::Pane,
                                    Border::new().padding(16.0).content("Pane content"),
                                ),
                                SlotView::new(
                                    SplitViewSlot::Content,
                                    Border::new().padding(16.0).content("Main content area"),
                                ),
                            ]),
                    )),
                    "SplitView::new().is_pane_open(open).slots([pane, content])",
                ),
            )],
        )
    }
}
