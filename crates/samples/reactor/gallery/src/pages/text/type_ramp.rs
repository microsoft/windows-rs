use crate::controls::*;
use windows_reactor::*;

pub struct TypeRampPage;

impl Component for TypeRampPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        page_content(
            "Type Ramp",
            "Font sizes and weights for a clear WinUI text hierarchy.",
            [
                KeyedView::new(
                    "ramp",
                    sample_card(
                        "Type Ramp",
                        StackPanel::new().spacing(8.0).children((
                            TextBlock::new()
                                .text("Title - 28px semibold")
                                .font_size(28.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new()
                                .text("Subtitle - 20px semibold")
                                .font_size(20.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new().text("Body large - 18px").font_size(18.0),
                            TextBlock::new()
                                .text("Body strong - 14px semibold")
                                .font_size(14.0)
                                .font_weight(FontWeight::SEMI_BOLD),
                            TextBlock::new().text("Body - 14px").font_size(14.0),
                            TextBlock::new().text("Caption - 12px").font_size(12.0),
                        )),
                        "TextBlock::new().font_size(size).font_weight(weight)",
                    ),
                ),
                KeyedView::new(
                    "article",
                    sample_card(
                        "Composed Article Card",
                        Border::new().padding(16.0).corner_radius(8.0).content(
                            StackPanel::new().spacing(8.0).children((
                                TextBlock::new()
                                    .text("Release Notes")
                                    .font_size(28.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                                TextBlock::new()
                                    .text("Version 2.5")
                                    .font_size(20.0)
                                    .font_weight(FontWeight::SEMI_BOLD),
                                "This release includes performance improvements and fixes.",
                            )),
                        ),
                        "Compose title, subtitle, and body styles in a card.",
                    ),
                ),
            ],
        )
    }
}
