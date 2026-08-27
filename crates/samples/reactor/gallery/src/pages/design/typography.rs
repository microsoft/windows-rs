use crate::controls::*;
use windows_reactor::*;

/// One row of the type ramp: a named style rendered at its font size/weight next to a label.
fn type_sample(name: &str, size: f64, weight: FontWeight) -> View {
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(12.0)
        .children((
            TextBlock::new()
                .text(name.to_string())
                .font_size(size)
                .font_weight(weight),
            TextBlock::new()
                .text(format!("{size}px / weight {}", weight.get()))
                .font_size(12.0)
                .opacity(0.6),
        ))
}

pub struct TypographyPage;

impl Component for TypographyPage {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
        page_content(
            "Typography",
            "The WinUI 3 type ramp provides a set of named text styles for consistent hierarchy.",
            [KeyedView::new(
                "type-ramp",
                sample_card(
                    "Type ramp",
                    StackPanel::new().spacing(12.0).children((
                        type_sample("Caption", 12.0, FontWeight::NORMAL),
                        type_sample("Body", 14.0, FontWeight::NORMAL),
                        type_sample("Body Strong", 14.0, FontWeight::BOLD),
                        type_sample("Subtitle", 20.0, FontWeight::BOLD),
                        type_sample("Title", 28.0, FontWeight::BOLD),
                        type_sample("Title Large", 40.0, FontWeight::BOLD),
                        type_sample("Display", 68.0, FontWeight::BOLD),
                    )),
                    "TextBlock::new().text(name).font_size(size).font_weight(weight)",
                ),
            )],
        )
    }
}
