#![windows_subsystem = "windows"]

use windows_reactor::*;

struct PointerPosition {
    pointer: Option<PointerEventInfo>,
}

impl Component for PointerPosition {
    type Message = PointerEventInfo;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { pointer: None }
    }

    fn update(&mut self, info: PointerEventInfo, _context: &ComponentContext<Self>) {
        self.pointer = Some(info);
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer Modifiers");
        let label = match self.pointer {
            Some(info) => format!(
                "Position: ({:.0}, {:.0})\nCtrl: {}  Shift: {}  Alt: {}  Windows: {}",
                info.x,
                info.y,
                info.modifiers.contains(InputModifiers::CONTROL),
                info.modifiers.contains(InputModifiers::SHIFT),
                info.modifiers.contains(InputModifiers::ALT),
                info.modifiers.contains(InputModifiers::WINDOWS),
            ),
            None => "Hold modifier keys and click inside the box".to_string(),
        };
        StackPanel::new()
            .spacing(12.0)
            .children((
                TextBlock::new().text(label).font_size(20.0),
                Border::new()
                    .background(Color::rgb(40, 120, 200))
                    .padding(Thickness::uniform(40.0))
                    .width(360.0)
                    .height(240.0)
                    .on_pointer_pressed(context.forward())
                    .content(
                        TextBlock::new()
                            .text("Hold Ctrl, Shift, Alt, or Windows and click")
                            .foreground(Color::rgb(255, 255, 255)),
                    ),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<PointerPosition>(()).unwrap();
}
