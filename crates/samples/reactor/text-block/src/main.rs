#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Text Block");
        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text("Plain text"),
            TextBlock::new().text("Larger text").font_size(20.0),
            TextBlock::new()
                .text("Bold + larger")
                .font_weight(FontWeight::BOLD)
                .font_size(28.0),
            TextBlock::new()
                .text("Selectable text - try selecting this with your mouse")
                .is_text_selection_enabled(true),
            TextBlock::new()
                .text("Selectable + wrapped text that demonstrates both features working together on a TextBlock element")
                .is_text_selection_enabled(true)
                .text_wrapping(TextWrapping::Wrap),
        ))
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
