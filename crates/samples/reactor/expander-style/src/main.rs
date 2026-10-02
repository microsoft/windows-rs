#![windows_subsystem = "windows"]

use windows_reactor::*;

fn content(label: &str, color: Color) -> View {
    Border::new()
        .background(color)
        .padding(Thickness::uniform(12.0))
        .content(label)
        .into()
}

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Expander Styling");
        StackPanel::new()
            .spacing(24.0)
            .margin(24.0)
            .children((
                TextBlock::new()
                    .text("Compare the colored content areas")
                    .font_size(20.0),
                Expander::new()
                    .width(480.0)
                    .is_expanded(true)
                    .header("Default template alignment and padding")
                    .content(content("Default content", Color::rgb(180, 80, 80))),
                Expander::new()
                    .width(480.0)
                    .is_expanded(true)
                    .horizontal_content_alignment(HorizontalAlignment::Stretch)
                    .resource_overrides(
                        ResourceOverrides::new()
                            .set("ExpanderContentPadding", Thickness::uniform(0.0)),
                    )
                    .header("Stretched content with zero template padding")
                    .content(content("Customized content", Color::rgb(40, 120, 200))),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
