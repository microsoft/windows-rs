use crate::controls::*;
use windows_reactor::*;

pub struct SpacingPage;

impl Component for SpacingPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        let rows = [
            ("XXSmall", 2.0),
            ("XSmall", 4.0),
            ("Small", 8.0),
            ("Medium", 12.0),
            ("Large", 16.0),
            ("XLarge", 24.0),
            ("XXLarge", 32.0),
            ("XXXLarge", 48.0),
        ]
        .into_iter()
        .map(|(name, size)| {
            KeyedView::new(
                name,
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .children((
                        TextBlock::new()
                            .text(format!("{name} ({size}px)"))
                            .width(180.0),
                        Border::new()
                            .background(Color::rgb(0, 120, 212))
                            .width(size)
                            .height(24.0)
                            .corner_radius(4.0),
                    )),
            )
        });
        page_content(
            "Spacing",
            "Standard spacing values used to create consistent layouts.",
            [KeyedView::new(
                "spacing-scale",
                StackPanel::new().spacing(8.0).keyed_children(rows),
            )],
        )
    }
}
