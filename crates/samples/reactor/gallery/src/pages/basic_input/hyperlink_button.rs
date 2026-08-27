use crate::controls::*;
use windows_reactor::*;

pub struct HyperlinkButtonPage {
    clicks: u32,
}

impl Component for HyperlinkButtonPage {
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
            "HyperlinkButton",
            "A button that appears as a hyperlink.",
            [
                KeyedView::new(
                    "navigate",
                    sample_card(
                        "Navigate to URI",
                        HyperlinkButton::new()
                            .navigate_uri("https://www.microsoft.com")
                            .unwrap()
                            .content("Visit Microsoft"),
                        "HyperlinkButton::new().navigate_uri(uri)?.content(label)",
                    ),
                ),
                KeyedView::new(
                    "click",
                    sample_card(
                        "Click Handler",
                        StackPanel::new().spacing(8.0).children((
                            HyperlinkButton::new()
                                .on_click(context.forward())
                                .content("Click me"),
                            TextBlock::new()
                                .text(format!("Clicked: {} times", self.clicks))
                                .opacity(0.6),
                        )),
                        "HyperlinkButton::new().on_click(handler).content(label)",
                    ),
                ),
            ],
        )
    }
}
