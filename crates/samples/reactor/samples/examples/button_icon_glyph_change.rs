use windows_reactor::*;

struct GlyphChangeSample {
    toggled: bool,
}

impl Component for GlyphChangeSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { toggled: false }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.toggled = !self.toggled;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let (icon, status) = if self.toggled {
            (Symbol::Save, "Save")
        } else {
            (Symbol::Favorite, "Favorite")
        };
        context.window_title("ButtonIconGlyphChange");
        StackPanel::new().spacing(12.0).children((
            Button::new().on_click(context.message(())).content(
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(6.0)
                    .children((SymbolIcon::new().symbol(icon), "Toggle Icon")),
            ),
            TextBlock::new()
                .text(format!("Current icon: {status}"))
                .opacity(0.6),
            TextBlock::new()
                .text("Click the button - the icon should change but the label stays.")
                .opacity(0.4),
        ))
    }
}

fn main() {
    App::run_component::<GlyphChangeSample>(()).unwrap();
}
