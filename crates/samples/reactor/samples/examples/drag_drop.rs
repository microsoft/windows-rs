#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Hover(DragKind),
    Leave,
    Drop(DroppedData),
}

struct DragDrop {
    hover: Option<DragKind>,
    dropped: Option<String>,
}

impl Component for DragDrop {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            hover: None,
            dropped: None,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Hover(kind) => self.hover = Some(kind),
            Message::Leave => self.hover = None,
            Message::Drop(data) => {
                self.hover = None;
                self.dropped = match data {
                    DroppedData::StorageItems(items) if items.len() == 1 => {
                        Some(items[0].path.clone())
                    }
                    DroppedData::StorageItems(items) => Some(format!(
                        "{} files dropped: {}",
                        items.len(),
                        items
                            .iter()
                            .map(|item| item.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                    DroppedData::Text(text) => Some(text),
                    DroppedData::Unsupported => None,
                };
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Drag & Drop Example");
        context.window_visuals(WindowVisuals::new().client_size(800.0, 600.0));
        let background = match self.hover {
            None => Color::rgb(255, 255, 255),
            Some(DragKind::StorageItems) => Color::rgb(92, 202, 221),
            Some(DragKind::Text) => Color::rgb(155, 219, 90),
            Some(DragKind::Unsupported) => Color::rgb(255, 180, 180),
        };

        Border::new()
            .background(background)
            .border_brush(Color::rgb(224, 224, 224))
            .border_thickness(Thickness::uniform(1.5))
            .corner_radius(CornerRadius::uniform(12.0))
            .padding(Thickness::uniform(20.0))
            .margin(Thickness::uniform(40.0))
            .drop_policy(
                DragDropPolicy::new()
                    .storage_items(
                        DragDropAction::new(DragDropOperation::Link)
                            .caption("Drop to link file(s)"),
                    )
                    .text(
                        DragDropAction::new(DragDropOperation::Copy).caption("Drop to paste text"),
                    ),
            )
            .on_drag_enter(context.callback(Message::Hover))
            .on_drag_over(context.callback(Message::Hover))
            .on_drag_leave(context.message(Message::Leave))
            .on_drop(context.callback(Message::Drop))
            .content(
                TextBlock::new()
                    .text(
                        self.dropped
                            .as_deref()
                            .unwrap_or("Drop files or some text here"),
                    )
                    .text_wrapping(TextWrapping::Wrap)
                    .font_size(24.0)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
    }
}

fn main() {
    App::run_component::<DragDrop>(()).unwrap();
}
