use crate::controls::*;
use windows_reactor::*;

pub struct SplitButtonPage {
    clicks: u32,
}

impl Component for SplitButtonPage {
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
            "SplitButton",
            "A button with a primary action and a flyout menu.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic SplitButton",
                        StackPanel::new().spacing(8.0).children((
                            SplitButton::new()
                                .on_click(context.message(()))
                                .content(TextBlock::new().text("Paste"))
                                .flyout_with(Flyout::rich(
                                    StackPanel::new().spacing(8.0).children((
                                        TextBlock::new().text("Paste options"),
                                        TextBlock::new().text("Keep source formatting"),
                                        TextBlock::new().text("Keep text only"),
                                    )),
                                )),
                            TextBlock::new()
                                .text(format!("Primary clicked: {} times", self.clicks))
                                .opacity(0.6),
                        )),
                        r#"SplitButton::new()
    .on_click(handler)
    .content(label)
    .flyout_with(Flyout::rich(options))"#,
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled SplitButton",
                        SplitButton::new()
                            .is_enabled(false)
                            .content(TextBlock::new().text("Disabled Action")),
                        "SplitButton::new().is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
