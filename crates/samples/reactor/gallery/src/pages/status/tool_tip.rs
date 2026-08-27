use crate::controls::*;
use windows_reactor::*;

pub struct ToolTipPage {
    clicks: u32,
}

impl Component for ToolTipPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "ToolTip",
            "A popup with helpful text on hover.",
            [
                KeyedView::new(
                    "interactive",
                    sample_card(
                        "Interactive ToolTip",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.forward())
                                .content("Hover target")
                                .tooltip_with(Tooltip::rich(
                                    StackPanel::new().spacing(4.0).children((
                                        TextBlock::new()
                                            .text("Interactive tooltip")
                                            .font_weight(FontWeight::BOLD),
                                        "Click the target to update the count.",
                                        format!("Click count: {}", self.clicks),
                                    )),
                                )),
                            TextBlock::new()
                                .text(format!("Click count: {}", self.clicks))
                                .opacity(0.6),
                        )),
                        "control.tooltip_with(Tooltip::rich(content))",
                    ),
                ),
                KeyedView::new(
                    "simple",
                    sample_card(
                        "Button with ToolTip",
                        Button::new()
                            .content("Hover me")
                            .tooltip("This is a helpful tooltip"),
                        "control.tooltip(\"Helpful text\")",
                    ),
                ),
            ],
        )
    }
}
