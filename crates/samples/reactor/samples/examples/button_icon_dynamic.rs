use windows_reactor::*;

struct DynamicIconSample {
    count: u32,
}

impl Component for DynamicIconSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let content = |symbol, label| {
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(6.0)
                .children((SymbolIcon::new().symbol(symbol), label))
        };
        context.window_title("ButtonIconDynamic");
        StackPanel::new().spacing(12.0).children((
            Button::new().on_click(context.message(())).content(content(
                Symbol::Favorite,
                format!("Clicked {} times", self.count),
            )),
            Button::new()
                .style(ButtonStyle::Accent)
                .on_click(context.message(()))
                .content(content(
                    Symbol::Save,
                    if self.count == 0 { "Save" } else { "Saved!" }.to_string(),
                )),
            TextBlock::new()
                .text("Click the buttons - the icons should remain visible.")
                .opacity(0.6),
        ))
    }
}

fn main() {
    App::run_component::<DynamicIconSample>(()).unwrap();
}
