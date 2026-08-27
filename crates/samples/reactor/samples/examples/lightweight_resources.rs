#![windows_subsystem = "windows"]

use windows_reactor::*;

struct LightweightResources {
    styled: bool,
}

impl Component for LightweightResources {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { styled: true }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.styled = !self.styled;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Lightweight Resources");
        let target = if self.styled {
            Button::new()
                .resource_overrides(
                    ResourceOverrides::new()
                        .set("ButtonBackground", Color::rgb(178, 34, 34))
                        .set("ButtonForeground", Color::rgb(255, 255, 255))
                        .set("ButtonBorderThemeThickness", Thickness::uniform(0.0))
                        .set("ControlCornerRadius", CornerRadius::uniform(8.0)),
                )
                .content(TextBlock::new().text("Delete"))
        } else {
            Button::new().content(TextBlock::new().text("Delete"))
        };

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new()
                    .text("Element resources override WinUI lightweight styling values."),
                target,
                Button::new()
                    .on_click(context.callback(|_| ()))
                    .content(TextBlock::new().text(if self.styled {
                        "Clear resources"
                    } else {
                        "Apply resources"
                    })),
            )),
        )
    }
}

fn main() {
    App::run_component::<LightweightResources>(()).unwrap();
}
