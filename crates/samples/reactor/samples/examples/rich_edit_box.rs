use windows_reactor::*;

struct RichEditBoxSample {
    text: String,
}

impl Component for RichEditBoxSample {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            text: String::new(),
        }
    }

    fn update(&mut self, text: String, _context: &ComponentContext<Self>) {
        self.text = text;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("RichEditBox");
        StackPanel::new().spacing(8.0).children((
            RichEditBox::new()
                .text(self.text.clone())
                .placeholder_text("Type rich text here...")
                .on_text_changed(context.callback(std::convert::identity))
                .height(200.0)
                .slots([SlotView::new(RichEditBoxSlot::Header, "Rich Editor")]),
            format!("Plain text: {}", self.text),
            RichEditBox::new()
                .text("Read-only content.")
                .is_read_only(true)
                .height(100.0)
                .slots([SlotView::new(RichEditBoxSlot::Header, "Read Only")]),
        ))
    }
}

fn main() {
    App::run_component::<RichEditBoxSample>(()).unwrap();
}
