use std::rc::Rc;

use windows_reactor::*;

struct TextBoxBorderSample {
    text: Rc<str>,
}

impl Component for TextBoxBorderSample {
    type Message = Rc<str>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            text: Rc::default(),
        }
    }

    fn update(&mut self, text: Rc<str>, _context: &ComponentContext<Self>) {
        self.text = text;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TextBox border");
        let text_box = |placeholder| {
            TextBox::new(self.text.clone())
                .placeholder_text(placeholder)
                .on_text_changed(context.forward())
        };
        StackPanel::new()
            .spacing(8.0)
            .children((
                "1. Default TextBox",
                text_box("Default style"),
                "2. Custom border (brush + thickness)",
                text_box("Thick blue border")
                    .border_brush(Color::rgb(60, 120, 220))
                    .border_thickness(Thickness::uniform(2.0)),
                "3. Borderless + transparent (chat/search bar)",
                text_box("Type a message...")
                    .background(Color::transparent())
                    .border_thickness(Thickness::uniform(0.0)),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<TextBoxBorderSample>(()).unwrap();
}
