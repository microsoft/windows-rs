#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Toggle,
    PaneOpenChanged(bool),
}

struct SplitViewSample {
    open: bool,
}

impl Component for SplitViewSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { open: true }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Toggle => self.open = !self.open,
            Message::PaneOpenChanged(open) => self.open = open,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("SplitView");
        SplitView::new()
            .display_mode(SplitViewDisplayMode::Inline)
            .is_pane_open(self.open)
            .open_pane_length(200.0)
            .on_pane_closed(context.callback(Message::PaneOpenChanged))
            .slots([
                SlotView::new(
                    SplitViewSlot::Content,
                    StackPanel::new().spacing(12.0).children((
                        TextBlock::new().text(format!(
                            "Pane is {}",
                            if self.open { "open" } else { "closed" }
                        )),
                        Button::new()
                            .on_click(context.message(Message::Toggle))
                            .content(TextBlock::new().text("Toggle Pane")),
                    )),
                ),
                SlotView::new(
                    SplitViewSlot::Pane,
                    StackPanel::new().spacing(8.0).children((
                        TextBlock::new().text("Pane Content"),
                        TextBlock::new().text("Item A"),
                        TextBlock::new().text("Item B"),
                        TextBlock::new().text("Item C"),
                    )),
                ),
            ])
    }
}

fn main() {
    App::run_component::<SplitViewSample>(()).unwrap();
}
