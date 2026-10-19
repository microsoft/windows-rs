#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Preset {
    Native,
    Loose,
    Tight,
}

struct TypographySample {
    preset: Preset,
}

impl Component for TypographySample {
    type Input = ();
    type Message = Preset;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            preset: Preset::Native,
        }
    }

    fn update(&mut self, message: Preset, _: &ComponentContext<Self>) {
        self.preset = message;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TextBlock typography");
        context.window_visuals(WindowVisuals::new().client_size(560.0, 360.0));
        let text = TextBlock::new()
            .text("The quick brown fox\njumps over the lazy dog.\n0123456789")
            .font_size(20.0);
        let (description, text) = match self.preset {
            Preset::Native => ("Native font, spacing, and line height", text),
            Preset::Loose => (
                "Consolas / spacing 150 / line height 36",
                text.font_family("Consolas")
                    .character_spacing(150)
                    .line_height(36.0),
            ),
            Preset::Tight => (
                "Arial / spacing -100 / automatic line height",
                text.font_family("Arial")
                    .character_spacing(-100)
                    .line_height(0.0),
            ),
        };
        Border::new()
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new().spacing(12.0).children((
                    description,
                    Border::new().height(160.0).content(text),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            Button::new()
                                .content("Native defaults")
                                .on_click(context.message(Preset::Native)),
                            Button::new()
                                .content("Loose")
                                .on_click(context.message(Preset::Loose)),
                            Button::new()
                                .content("Tight")
                                .on_click(context.message(Preset::Tight)),
                        )),
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<TypographySample>(()).unwrap();
}
