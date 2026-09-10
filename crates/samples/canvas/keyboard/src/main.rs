#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::*;

enum Message {
    Key(KeyEventInfo),
    Character(u16),
    Focus(bool),
    Clear,
}

struct Sample {
    text: Vec<u16>,
    focused: bool,
    status: String,
    format: Rc<RefCell<Option<TextFormat>>>,
    invalidator: Invalidator,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            text: "Type here".encode_utf16().collect(),
            focused: false,
            status: "Click the canvas or press Tab to focus".to_string(),
            format: Rc::new(RefCell::new(None)),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Key(info) => {
                match info.key {
                    VirtualKey::BACK => pop_utf16_character(&mut self.text),
                    VirtualKey::DELETE => self.text.clear(),
                    _ => {}
                }
                self.status = format!(
                    "{:?} | modifiers: {:?} | repeat: {}",
                    info.key, info.modifiers, info.status.repeat_count
                );
            }
            Message::Character(character) => {
                self.text.push(character);
                self.status = format!("CharacterReceived: U+{character:04X}");
            }
            Message::Focus(focused) => {
                self.focused = focused;
                self.status = if focused {
                    "Canvas focused - type text, Backspace, or Delete".to_string()
                } else {
                    "Not focused - click the canvas or press Tab".to_string()
                };
            }
            Message::Clear => {
                self.text.clear();
                self.status = "Text cleared - click the canvas or press Shift+Tab".to_string();
            }
        }
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas keyboard input");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let text = String::from_utf16_lossy(&self.text);
        let format = Rc::clone(&self.format);
        let canvas = canvas_invalidated(&self.invalidator, move |ctx| draw(ctx, &text, &format));
        let surface = Border::new()
            .is_tab_stop(true)
            .focus_on_pointer_release(true)
            .background(Color::rgb(16, 20, 28))
            .border_brush(if self.focused {
                Color::rgb(80, 150, 255)
            } else {
                Color::rgb(70, 76, 88)
            })
            .border_thickness(Thickness::uniform(if self.focused { 3.0 } else { 1.0 }))
            .corner_radius(8.0)
            .grid_row(1)
            .on_preview_key_down(context.routed_callback(route_key))
            .on_character_received(context.routed_callback(|info: CharacterEventInfo| {
                if info.character >= 0x20 && info.character != 0x7F {
                    RoutedMessage::handled(Message::Character(info.character))
                } else {
                    RoutedMessage::bubble_without_message()
                }
            }))
            .on_got_focus(context.callback(|info: FocusEventInfo| Message::Focus(info.is_direct)))
            .on_lost_focus(context.callback(|_| Message::Focus(false)))
            .content(canvas);

        Grid::new()
            .rows([
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
                GridLength::Auto,
            ])
            .row_spacing(12.0)
            .margin(Thickness::uniform(24.0))
            .children((
                TextBlock::new()
                    .text("Type on the Canvas, then use the button to transfer focus")
                    .font_size(18.0)
                    .font_weight(FontWeight::BOLD),
                surface,
                TextBlock::new()
                    .text(self.status.clone())
                    .font_size(14.0)
                    .grid_row(2),
                Button::new()
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .grid_row(3)
                    .on_click(context.callback(|()| Message::Clear))
                    .content("Clear text"),
            ))
    }
}

fn route_key(info: KeyEventInfo) -> RoutedMessage<Message> {
    if matches!(info.key, VirtualKey::BACK | VirtualKey::DELETE) {
        RoutedMessage::handled(Message::Key(info))
    } else {
        RoutedMessage::bubble_without_message()
    }
}

fn pop_utf16_character(text: &mut Vec<u16>) {
    let Some(last) = text.pop() else {
        return;
    };
    if (0xDC00..=0xDFFF).contains(&last)
        && text
            .last()
            .is_some_and(|unit| (0xD800..=0xDBFF).contains(unit))
    {
        text.pop();
    }
}

fn draw(ctx: &DrawContext, text: &str, current_format: &RefCell<Option<TextFormat>>) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));

    if current_format.borrow().is_none() {
        *current_format.borrow_mut() = Some(TextFormat::new("Cascadia Mono", 28.0)?);
    }
    let format = current_format.borrow();
    let text_brush = ctx.create_solid_brush(ColorF::WHITE)?;
    ctx.draw_text(
        text,
        format.as_ref().unwrap(),
        &Rect::new(32.0, 32.0, ctx.width - 32.0, 80.0),
        &text_brush,
    );

    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backspace_keeps_surrogate_pairs_together() {
        let mut text: Vec<u16> = "a\u{1F642}".encode_utf16().collect();
        pop_utf16_character(&mut text);
        assert_eq!(text, ['a' as u16]);
    }
}
