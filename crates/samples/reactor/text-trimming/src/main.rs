#![windows_subsystem = "windows"]

use windows_reactor::*;

const LONG_TEXT: &str =
    "A long line of text that should stop at the declared width instead of extending or wrapping.";

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TextTrimming");
        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(8.0).children((
                "No trimming:",
                TextBlock::new().text(LONG_TEXT).width(240.0),
                "One line with character ellipsis:",
                TextBlock::new()
                    .text(LONG_TEXT)
                    .width(240.0)
                    .max_lines(1)
                    .text_trimming(TextTrimming::CharacterEllipsis),
                "Two lines with word ellipsis:",
                TextBlock::new()
                    .text(LONG_TEXT)
                    .width(240.0)
                    .text_wrapping(TextWrapping::Wrap)
                    .max_lines(2)
                    .text_trimming(TextTrimming::WordEllipsis),
            )),
        )
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
