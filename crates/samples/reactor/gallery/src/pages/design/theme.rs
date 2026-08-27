use crate::controls::*;
use windows_reactor::*;

pub struct ThemePage;

impl Component for ThemePage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        page_content(
            "Theme",
            "Theme resources adapt surfaces and text to the active window theme.",
            [
                KeyedView::new(
                    "surfaces",
                    sample_card(
                        "Adaptive Surface Tokens",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(12.0)
                            .children((
                                Border::new()
                                    .background(ThemeBrush::CardBackground)
                                    .border_brush(ThemeBrush::CardStroke)
                                    .border_thickness(1.0)
                                    .padding(16.0)
                                    .corner_radius(8.0)
                                    .content("Card background"),
                                Border::new()
                                    .background(ThemeBrush::SolidBackground)
                                    .border_brush(ThemeBrush::CardStroke)
                                    .border_thickness(1.0)
                                    .padding(16.0)
                                    .corner_radius(8.0)
                                    .content("Solid background"),
                            )),
                        "Border::new().background(ThemeBrush::CardBackground)",
                    ),
                ),
                KeyedView::new(
                    "opacity",
                    sample_card(
                        "Text Hierarchy",
                        StackPanel::new().spacing(4.0).children((
                            TextBlock::new().text("Primary text").font_weight(700),
                            TextBlock::new().text("Secondary text").opacity(0.7),
                            TextBlock::new().text("Tertiary text").opacity(0.5),
                        )),
                        "TextBlock::new().text(value).opacity(0.7)",
                    ),
                ),
            ],
        )
    }
}
