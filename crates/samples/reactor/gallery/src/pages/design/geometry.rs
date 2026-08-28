use crate::controls::*;
use windows_reactor::*;

pub struct GeometryPage;

impl Component for GeometryPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        let radius_sample = |radius, label| {
            StackPanel::new().spacing(6.0).children((
                Border::new()
                    .background(Color::rgb(0, 120, 212))
                    .width(80.0)
                    .height(56.0)
                    .corner_radius(radius),
                TextBlock::new().text(label).font_size(12.0),
            ))
        };
        page_content(
            "Geometry",
            "Shared corner radii give controls and surfaces a consistent shape language.",
            [
                KeyedView::new(
                    "radii",
                    sample_card(
                        "Corner Radius Resources",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(24.0)
                            .children((
                                radius_sample(4.0, "Control - 4px"),
                                radius_sample(8.0, "Overlay - 8px"),
                                radius_sample(16.0, "Large surface - 16px"),
                            )),
                        "Border::new().corner_radius(4.0)",
                    ),
                ),
                KeyedView::new(
                    "surface",
                    sample_card(
                        "Overlay Surface",
                        Border::new()
                            .padding(16.0)
                            .corner_radius(8.0)
                            .border_brush(ThemeBrush::CardStroke)
                            .border_thickness(1.0)
                            .content(
                                StackPanel::new().spacing(4.0).children((
                                    TextBlock::new()
                                        .text("Dialog title")
                                        .font_weight(FontWeight::BOLD),
                                    TextBlock::new()
                                        .text("Overlay surfaces use a larger radius.")
                                        .opacity(0.6),
                                )),
                            ),
                        "Border::new().corner_radius(8.0).content(dialog)",
                    ),
                ),
            ],
        )
    }
}
