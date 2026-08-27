use windows_reactor::*;

struct TabViewItemKeySample {
    keyed: bool,
    last_close_key: String,
}

#[derive(Clone)]
enum Message {
    ToggleKey,
    Close(String),
}

impl Component for TabViewItemKeySample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            keyed: true,
            last_close_key: String::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::ToggleKey => self.keyed = !self.keyed,
            Message::Close(key) => {
                self.last_close_key = if key.is_empty() {
                    "<none>".to_string()
                } else {
                    key
                };
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TabView Item Key");
        let mut item = TabViewItem::new().header("Document").is_closable(true);
        if self.keyed {
            item = item.tag("document");
        }

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(8.0).children((
                Button::new()
                    .on_click(context.message(Message::ToggleKey))
                    .content(if self.keyed {
                        "Remove item key"
                    } else {
                        "Restore item key"
                    }),
                TabView::new()
                    .on_close_requested(context.callback(Message::Close))
                    .slots([SlotView::collection(
                        TabViewSlot::TabItems,
                        [KeyedView::new(
                            "doc",
                            item.content("Close the tab to inspect its key."),
                        )],
                    )]),
                format!(
                    "configured key: {}; last close request: {}",
                    if self.keyed { "document" } else { "<none>" },
                    if self.last_close_key.is_empty() {
                        "<not requested>"
                    } else {
                        &self.last_close_key
                    }
                ),
            )),
        )
    }
}

fn main() {
    App::run_component::<TabViewItemKeySample>(()).unwrap();
}
