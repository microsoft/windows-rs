#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct RowInput {
    name: String,
}

struct Row {
    clicks: u32,
}

impl Component for Row {
    type Message = ();
    type Input = RowInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        Border::new().padding(6.0).content(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    TextBlock::new().text(format!("{}: {}", input.name, self.clicks)),
                    Button::new()
                        .on_click(context.message(()))
                        .content(TextBlock::new().text(format!("Increment {}", input.name))),
                )),
        )
    }
}

struct KeyedListReorderSample {
    items: Vec<String>,
}

impl Component for KeyedListReorderSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            items: ["Alpha", "Beta", "Gamma", "Delta"]
                .map(ToOwned::to_owned)
                .into(),
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.items.rotate_left(1);
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("KeyedListReorder");
        let rows = self.items.iter().map(|name| {
            KeyedView::new(
                name.clone(),
                View::component::<Row>(RowInput { name: name.clone() }),
            )
        });
        Border::new().padding(16.0).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new()
                    .text("Increment a row, then rotate the list. The count stays with its name."),
                Button::new()
                    .on_click(context.message(()))
                    .content(TextBlock::new().text("Rotate")),
                StackPanel::new().spacing(8.0).keyed_children(rows),
            )),
        )
    }
}

fn main() {
    App::run_component::<KeyedListReorderSample>(()).unwrap();
}
