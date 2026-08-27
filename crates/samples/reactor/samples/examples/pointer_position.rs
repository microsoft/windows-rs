#![windows_subsystem = "windows"]

use windows_reactor::*;

struct PointerPosition {
    position: Option<(f64, f64)>,
}

impl Component for PointerPosition {
    type Message = PointerEventInfo;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { position: None }
    }

    fn update(&mut self, info: PointerEventInfo, _context: &ComponentContext<Self>) {
        self.position = Some((info.x, info.y));
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer Position");
        let label = match self.position {
            Some((x, y)) => format!("Pressed at ({x:.0}, {y:.0})"),
            None => "Click anywhere in the box".to_string(),
        };
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text(label).font_size(20.0),
            Border::new()
                .background(Color::rgb(40, 120, 200))
                .padding(40.0)
                .width(360.0)
                .height(240.0)
                .on_pointer_pressed(context.forward())
                .content(
                    TextBlock::new()
                        .text("Click to read the pointer position")
                        .foreground(Color::rgb(255, 255, 255)),
                ),
        ))
    }
}

fn main() {
    App::run_component::<PointerPosition>(()).unwrap();
}
