#![windows_subsystem = "windows"]

use windows_reactor::*;

enum PointerMessage {
    Pressed(PointerEventInfo),
    Moved(PointerEventInfo),
    Released,
}

struct PointerResize {
    width: f64,
    drag_start: Option<(f64, f64)>,
}

impl Component for PointerResize {
    type Message = PointerMessage;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            width: 260.0,
            drag_start: None,
        }
    }

    fn update(&mut self, message: PointerMessage, _context: &ComponentContext<Self>) {
        match message {
            PointerMessage::Pressed(info) => {
                if info.is_left_button_pressed && info.capture_succeeded {
                    self.drag_start = Some((info.window_x, self.width));
                }
            }
            PointerMessage::Moved(info) => {
                if !info.is_left_button_pressed {
                    self.drag_start = None;
                } else if let Some((start_x, start_width)) = self.drag_start {
                    self.width = (start_width + info.window_x - start_x).clamp(140.0, 520.0);
                }
            }
            PointerMessage::Released => {
                self.drag_start = None;
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer Resize");
        StackPanel::new().spacing(12.0).children((
            format!("Left pane width: {:.0} DIPs", self.width),
            Border::new().height(240.0).content(
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        Border::new()
                            .width(self.width)
                            .background(Color::rgb(35, 90, 150))
                            .padding(16.0)
                            .content("Resizable pane"),
                        Border::new()
                            .width(44.0)
                            .background(Color::rgb(90, 90, 100))
                            .capture_pointer_on_press(true)
                            .on_pointer_pressed(context.callback(PointerMessage::Pressed))
                            .on_pointer_moved(context.callback(PointerMessage::Moved))
                            .on_pointer_released(context.callback(|_| PointerMessage::Released))
                            .on_pointer_capture_lost(context.callback(|_| PointerMessage::Released))
                            .on_pointer_canceled(context.callback(|_| PointerMessage::Released))
                            .content(
                                TextBlock::new()
                                    .text("Drag")
                                    .foreground(Color::rgb(255, 255, 255)),
                            ),
                        Border::new()
                            .padding(16.0)
                            .content("The handle moves, but window_x remains stable."),
                    )),
            ),
        ))
    }
}

fn main() {
    App::run_component::<PointerResize>(()).unwrap();
}
