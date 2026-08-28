use std::sync::LazyLock;

use windows_reactor::*;

static THEME: LazyLock<Context<String>> = LazyLock::new(|| Context::new("light".to_string()));

struct Leaf;

impl Component for Leaf {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let theme = context.use_context(&THEME);
        let (background, foreground) = match theme.as_str() {
            "dark" => (Color::rgb(30, 30, 30), Color::rgb(255, 255, 255)),
            "neon" => (Color::rgb(50, 200, 150), Color::rgb(0, 0, 0)),
            _ => (Color::rgb(240, 240, 240), Color::rgb(0, 0, 0)),
        };
        Border::new().background(background).content(
            Border::new().padding(Thickness::uniform(16.0)).content(
                TextBlock::new()
                    .text(format!("Leaf sees theme = {theme}"))
                    .font_size(16.0)
                    .foreground(foreground),
            ),
        )
    }
}

struct ContextSample {
    theme: String,
}

impl Component for ContextSample {
    type Message = &'static str;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            theme: "light".to_string(),
        }
    }

    fn update(&mut self, theme: Self::Message, _context: &ComponentContext<Self>) {
        self.theme = theme.to_string();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Context");
        let pick = |name| Button::new().on_click(context.message(name)).content(name);
        View::provide(
            &THEME,
            self.theme.clone(),
            Border::new().padding(Thickness::uniform(16.0)).content(
                StackPanel::new().spacing(12.0).children((
                    TextBlock::new()
                        .text("Pick a theme; the leaf below reads it via context.")
                        .font_size(12.0),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((pick("light"), pick("dark"), pick("neon"))),
                    Border::new()
                        .padding(Thickness::uniform(8.0))
                        .content(View::component::<Leaf>(())),
                )),
            ),
        )
    }
}

fn main() {
    App::run_component::<ContextSample>(()).unwrap();
}
