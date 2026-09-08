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
        context.window_title("ImageIconSize");
        let source = format!(
            "file:///{}/image.svg",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let icon = |label: &'static str| {
            let image = ImageIcon::new()
                .width(20.0)
                .height(20.0)
                .source(source.clone())
                .unwrap();
            if label.is_empty() {
                Button::new().content(image)
            } else {
                Button::new().content(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(6.0)
                        .children((image, label)),
                )
            }
        };

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                "The source SVG has a 120x60 view box.",
                "As an ImageIcon it stays inside the standard 20-DIP icon box.",
                icon("SVG image icon"),
                icon(""),
                "The same source in an Image control:",
                Image::new()
                    .width(120.0)
                    .height(60.0)
                    .source(source)
                    .unwrap(),
            )),
        )
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
