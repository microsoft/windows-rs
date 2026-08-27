#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

struct RadioButtonSample {
    size: Size,
}

enum Message {
    Checked(Size, bool),
}

impl Component for RadioButtonSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            size: Size::default(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        let Message::Checked(size, true) = message else {
            return;
        };
        self.size = size;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("RadioButton");
        let radio = |label: &'static str, size| {
            RadioButton::new()
                .group_name("size")
                .is_checked(self.size == size)
                .on_checked(context.callback(move |checked| Message::Checked(size, checked)))
                .content(TextBlock::new().text(label))
        };
        let label = match self.size {
            Size::Small => "Small",
            Size::Medium => "Medium",
            Size::Large => "Large",
        };

        StackPanel::new().spacing(4.0).children((
            radio("Small", Size::Small),
            radio("Medium", Size::Medium),
            radio("Large", Size::Large),
            TextBlock::new().text(format!("size = {label}")),
            RadioButton::new()
                .group_name("other")
                .is_checked(true)
                .is_enabled(false)
                .content(TextBlock::new().text("Disabled")),
        ))
    }
}

fn main() {
    App::run_component::<RadioButtonSample>(()).unwrap();
}
