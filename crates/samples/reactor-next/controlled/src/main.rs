use windows_reactor_next::*;

struct Controlled {
    text: String,
}

impl Component for Controlled {
    type Props = ();
    type Message = String;

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self {
            text: String::new(),
        }
    }

    fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, message: String, _context: &mut ComponentContext<Self>) {
        self.text = message;
    }

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        let changed = context.sender();
        View::children(
            StackPanel::new().spacing(8.0),
            [
                KeyedView::new(
                    "input",
                    View::native(
                        TextBox::new()
                            .text(self.text.clone())
                            .placeholder_text("Type here")
                            .on_text_changed(move |value| {
                                changed.send(value);
                            }),
                    ),
                ),
                KeyedView::new(
                    "value",
                    View::native(TextBlock::new().text(self.text.clone())),
                ),
            ],
        )
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<Controlled>(()).unwrap();
}
