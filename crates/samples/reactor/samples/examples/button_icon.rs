use windows_reactor::*;

struct ButtonIconSample {
    count: u32,
}

#[derive(Clone)]
enum Message {
    Add,
    Delete,
}

fn icon_content(symbol: Symbol, label: impl Into<String>) -> View {
    let label: String = label.into();
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(6.0)
        .children((SymbolIcon::new().symbol(symbol), label))
}

impl Component for ButtonIconSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count += 1,
            Message::Delete => self.count = self.count.saturating_sub(1),
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ButtonIcon");
        StackPanel::new().spacing(8.0).children((
            Button::new()
                .on_click(context.message(Message::Add))
                .content("Plain Button"),
            Button::new()
                .on_click(context.message(Message::Add))
                .content(icon_content(Symbol::Add, "Add Item")),
            Button::new()
                .on_click(context.message(Message::Delete))
                .content(icon_content(Symbol::Delete, "Delete")),
            Button::new()
                .style(ButtonStyle::Accent)
                .content(icon_content(Symbol::Save, "Save")),
            format!("Count: {}", self.count),
        ))
    }
}

fn main() {
    App::run_component::<ButtonIconSample>(()).unwrap();
}
