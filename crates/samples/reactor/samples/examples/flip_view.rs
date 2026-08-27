use windows_reactor::*;

struct FlipViewSample {
    page: i32,
}

#[derive(Clone)]
enum Message {
    Previous,
    Next,
    Selected(i32),
}

impl Component for FlipViewSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { page: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        self.page = match message {
            Message::Previous => (self.page - 1).max(0),
            Message::Next => (self.page + 1).min(2),
            Message::Selected(page) => page,
        };
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let item = |key, label| {
            KeyedView::new(
                key,
                Border::new()
                    .background(Color::rgb(245, 230, 220))
                    .padding(Thickness::uniform(24.0))
                    .content(
                        TextBlock::new()
                            .text(label)
                            .font_size(20.0)
                            .font_weight(700),
                    ),
            )
        };

        context.window_title("Sample");
        StackPanel::new().spacing(8.0).max_width(360.0).children((
            FlipView::new()
                .selected_index(self.page)
                .on_selection_changed(context.callback(Message::Selected))
                .height(180.0)
                .collection_slot(
                    FlipViewSlot::Items,
                    [
                        item("red", "Red"),
                        item("green", "Green"),
                        item("blue", "Blue"),
                    ],
                ),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Message::Previous))
                        .content("Prev"),
                    Button::new()
                        .on_click(context.message(Message::Next))
                        .content("Next"),
                    TextBlock::new()
                        .text(format!("page = {}", self.page))
                        .opacity(0.7),
                )),
        ))
    }
}

fn main() {
    App::run_component::<FlipViewSample>(()).unwrap();
}
