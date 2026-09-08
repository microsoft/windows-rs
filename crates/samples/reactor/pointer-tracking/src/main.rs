#![windows_subsystem = "windows"]

use windows_reactor::*;

enum PointerMessage {
    Entered(PointerEventInfo),
    Moved(PointerEventInfo),
    Exited,
}

struct PointerTracking {
    position: Option<(f64, f64)>,
    inside: bool,
}

impl Component for PointerTracking {
    type Message = PointerMessage;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            position: None,
            inside: false,
        }
    }

    fn update(&mut self, message: PointerMessage, _context: &ComponentContext<Self>) {
        match message {
            PointerMessage::Entered(info) | PointerMessage::Moved(info) => {
                self.inside = true;
                self.position = Some((info.x, info.y));
            }
            PointerMessage::Exited => {
                self.inside = false;
                self.position = None;
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer Tracking");
        let label = match (self.inside, self.position) {
            (true, Some((x, y))) => format!("Tracking at ({x:.0}, {y:.0})"),
            (true, None) => "Pointer entered".to_string(),
            (false, _) => "Move the pointer into the box".to_string(),
        };
        let fill = if self.inside {
            Color::rgb(40, 160, 90)
        } else {
            Color::rgb(40, 120, 200)
        };

        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text(label).font_size(20.0),
            Border::new()
                .background(fill)
                .padding(40.0)
                .width(360.0)
                .height(240.0)
                .on_pointer_entered(context.callback(PointerMessage::Entered))
                .on_pointer_moved(context.callback(PointerMessage::Moved))
                .on_pointer_exited(context.callback(|_| PointerMessage::Exited))
                .content(
                    TextBlock::new()
                        .text("Move the pointer over me")
                        .foreground(Color::rgb(255, 255, 255)),
                ),
        ))
    }
}

fn main() {
    App::run_component::<PointerTracking>(()).unwrap();
}
