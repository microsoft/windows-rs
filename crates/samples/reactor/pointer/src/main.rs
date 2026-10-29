#![windows_subsystem = "windows"]

use windows_reactor::*;

enum Message {
    Tracked(PointerEventInfo),
    Pressed(PointerEventInfo),
    Exited,
    ResizeStart(PointerEventInfo),
    ResizeMove(PointerEventInfo),
    ResizeEnd,
}

#[derive(Default)]
struct Pointer {
    position: Option<(f64, f64)>,
    modifiers: InputModifiers,
    width: f64,
    drag_start: Option<(f64, f64)>,
}

impl Component for Pointer {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            width: 260.0,
            ..Default::default()
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Tracked(info) => self.position = Some((info.x, info.y)),
            Message::Pressed(info) => self.modifiers = info.modifiers,
            Message::Exited => self.position = None,
            Message::ResizeStart(info) => {
                if info.is_left_button_pressed && info.capture_succeeded == Some(true) {
                    self.drag_start = Some((info.window_x, self.width));
                }
            }
            Message::ResizeMove(info) => {
                if !info.is_left_button_pressed {
                    self.drag_start = None;
                } else if let Some((start_x, start_width)) = self.drag_start {
                    self.width = (start_width + info.window_x - start_x).clamp(140.0, 520.0);
                }
            }
            Message::ResizeEnd => self.drag_start = None,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer");
        let white = Color::rgb(255, 255, 255);

        let tracking = match self.position {
            Some((x, y)) => format!("Pointer at ({x:.0}, {y:.0})"),
            None => "Move the pointer into the box".to_string(),
        };
        let modifiers = format!(
            "Last click - Ctrl: {}  Shift: {}  Alt: {}  Windows: {}",
            self.modifiers.contains(InputModifiers::CONTROL),
            self.modifiers.contains(InputModifiers::SHIFT),
            self.modifiers.contains(InputModifiers::ALT),
            self.modifiers.contains(InputModifiers::WINDOWS),
        );

        // Pointer capture keeps move events flowing to the handle while it is dragged, and
        // window_x stays stable even though the handle itself moves.
        let resize = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .height(160.0)
            .children((
                Border::new()
                    .width(self.width)
                    .background(Color::rgb(35, 90, 150))
                    .padding(Thickness::uniform(16.0))
                    .content(TextBlock::new().text("Resizable pane").foreground(white)),
                Border::new()
                    .width(44.0)
                    .background(Color::rgb(90, 90, 100))
                    .capture_pointer_on_press(true)
                    .on_pointer_pressed(context.callback(Message::ResizeStart))
                    .on_pointer_moved(context.callback(Message::ResizeMove))
                    .on_pointer_released(context.callback(|_| Message::ResizeEnd))
                    .on_pointer_capture_lost(context.callback(|_| Message::ResizeEnd))
                    .on_pointer_canceled(context.callback(|_| Message::ResizeEnd))
                    .content(TextBlock::new().text("Drag").foreground(white)),
            ));

        Border::new()
            .padding(Thickness::uniform(24.0))
            .content(
                StackPanel::new().spacing(12.0).children((
                    TextBlock::new().text(tracking).font_size(20.0),
                    modifiers,
                    Border::new()
                        .background(if self.position.is_some() {
                            Color::rgb(40, 160, 90)
                        } else {
                            Color::rgb(40, 120, 200)
                        })
                        .padding(Thickness::uniform(40.0))
                        .width(360.0)
                        .height(160.0)
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .on_pointer_entered(context.callback(Message::Tracked))
                        .on_pointer_moved(context.callback(Message::Tracked))
                        .on_pointer_exited(context.callback(|_| Message::Exited))
                        .on_pointer_pressed(context.callback(Message::Pressed))
                        .content(
                            TextBlock::new()
                                .text("Move here, or hold modifier keys and click")
                                .foreground(white),
                        ),
                    format!("Left pane width: {:.0} DIPs", self.width),
                    resize,
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<Pointer>(()).unwrap();
}
