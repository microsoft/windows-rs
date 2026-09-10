#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::*;

const TEXT_X: f32 = 28.0;
const TEXT_Y: f32 = 36.0;

struct Editor {
    text: Vec<u16>,
    caret: usize,
    focused: bool,
    status: String,
}

impl Editor {
    fn new() -> Self {
        let text: Vec<u16> = "Keyboard input on Canvas".encode_utf16().collect();
        Self {
            caret: text.len(),
            text,
            focused: false,
            status: "Click the canvas or press Tab to focus".to_string(),
        }
    }

    fn previous(&self) -> usize {
        previous_boundary(&self.text, self.caret)
    }

    fn next(&self) -> usize {
        next_boundary(&self.text, self.caret)
    }
}

enum Message {
    Key(KeyEventInfo),
    Character(u16),
    Focus(bool),
    Pressed(PointerEventInfo),
}

struct Sample {
    editor: Rc<RefCell<Editor>>,
    format: Rc<RefCell<Option<TextFormat>>>,
    layout: Rc<RefCell<Option<TextLayout>>>,
    invalidator: Invalidator,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            editor: Rc::new(RefCell::new(Editor::new())),
            format: Rc::new(RefCell::new(None)),
            layout: Rc::new(RefCell::new(None)),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        let mut editor = self.editor.borrow_mut();
        match message {
            Message::Key(info) => {
                match info.key {
                    VirtualKey::LEFT => editor.caret = editor.previous(),
                    VirtualKey::RIGHT => editor.caret = editor.next(),
                    VirtualKey::HOME => editor.caret = 0,
                    VirtualKey::END => editor.caret = editor.text.len(),
                    VirtualKey::BACK => {
                        let start = editor.previous();
                        let end = editor.caret;
                        editor.text.drain(start..end);
                        editor.caret = start;
                    }
                    VirtualKey::DELETE => {
                        let end = editor.next();
                        let start = editor.caret;
                        editor.text.drain(start..end);
                    }
                    _ => {}
                }
                editor.status = format!(
                    "{:?} | modifiers: {:?} | repeat: {}",
                    info.key, info.modifiers, info.status.repeat_count
                );
            }
            Message::Character(character) => {
                let caret = editor.caret;
                editor.text.insert(caret, character);
                editor.caret += 1;
                editor.status = format!("CharacterReceived: U+{character:04X}");
            }
            Message::Focus(focused) => {
                editor.focused = focused;
                editor.status = if focused {
                    "Focused - type or use Left/Right/Home/End/Backspace/Delete".to_string()
                } else {
                    "Not focused - click the canvas or press Tab".to_string()
                };
            }
            Message::Pressed(info) => {
                if let Some(layout) = self.layout.borrow().as_ref() {
                    let hit = layout.hit_test_point(Vector2::new(
                        info.x as f32 - TEXT_X,
                        info.y as f32 - TEXT_Y,
                    ));
                    let position = normalize_boundary(
                        &editor.text,
                        (hit.text_position as usize).min(editor.text.len()),
                    );
                    editor.caret = if hit.is_trailing_hit {
                        next_boundary(&editor.text, position)
                    } else {
                        position
                    };
                }
            }
        }
        drop(editor);
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas keyboard input");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let focused = self.editor.borrow().focused;
        let editor = Rc::clone(&self.editor);
        let format = Rc::clone(&self.format);
        let layout = Rc::clone(&self.layout);
        let surface = Border::new()
            .is_tab_stop(true)
            .allow_focus_on_interaction(true)
            .background(Color::rgb(16, 20, 28))
            .border_brush(if focused {
                Color::rgb(80, 150, 255)
            } else {
                Color::rgb(70, 76, 88)
            })
            .border_thickness(Thickness::uniform(if focused { 3.0 } else { 1.0 }))
            .corner_radius(8.0)
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
            .on_pointer_pressed(context.callback(Message::Pressed))
            .grid_row(1)
            .content(canvas_invalidated(&self.invalidator, move |ctx| {
                draw(ctx, &editor.borrow(), &format, &layout)
            }));

        let status = self.editor.borrow().status.clone();
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .row_spacing(12.0)
            .margin(Thickness::uniform(24.0))
            .children((
                TextBlock::new()
                    .text("A custom Canvas control using WinUI routed keyboard input")
                    .font_size(18.0)
                    .font_weight(FontWeight::BOLD),
                surface,
                TextBlock::new().text(status).font_size(14.0).grid_row(2),
            ))
    }
}

fn route_key(info: KeyEventInfo) -> RoutedMessage<Message> {
    if matches!(
        info.key,
        VirtualKey::LEFT
            | VirtualKey::RIGHT
            | VirtualKey::HOME
            | VirtualKey::END
            | VirtualKey::BACK
            | VirtualKey::DELETE
    ) {
        RoutedMessage::handled(Message::Key(info))
    } else {
        RoutedMessage::bubble_without_message()
    }
}

fn previous_boundary(text: &[u16], caret: usize) -> usize {
    if caret >= 2
        && (0xDC00..=0xDFFF).contains(&text[caret - 1])
        && (0xD800..=0xDBFF).contains(&text[caret - 2])
    {
        caret - 2
    } else {
        caret.saturating_sub(1)
    }
}

fn next_boundary(text: &[u16], caret: usize) -> usize {
    if caret + 1 < text.len()
        && (0xD800..=0xDBFF).contains(&text[caret])
        && (0xDC00..=0xDFFF).contains(&text[caret + 1])
    {
        caret + 2
    } else {
        (caret + 1).min(text.len())
    }
}

fn normalize_boundary(text: &[u16], caret: usize) -> usize {
    if caret > 0
        && caret < text.len()
        && (0xDC00..=0xDFFF).contains(&text[caret])
        && (0xD800..=0xDBFF).contains(&text[caret - 1])
    {
        caret - 1
    } else {
        caret
    }
}

fn draw(
    ctx: &DrawContext,
    editor: &Editor,
    current_format: &RefCell<Option<TextFormat>>,
    current_layout: &RefCell<Option<TextLayout>>,
) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));

    let text = String::from_utf16_lossy(&editor.text);
    if current_format.borrow().is_none() {
        *current_format.borrow_mut() = Some(TextFormat::new("Cascadia Mono", 24.0)?);
    }
    let format = current_format.borrow();
    let layout = TextLayout::new(
        &text,
        format.as_ref().unwrap(),
        (ctx.width - 2.0 * TEXT_X).max(1.0),
        48.0,
    )?;
    let text_brush = ctx.create_solid_brush(ColorF::WHITE)?;
    ctx.draw_text_layout(Vector2::new(TEXT_X, TEXT_Y), &layout, &text_brush);

    if editor.focused {
        let caret = if editor.caret == editor.text.len() && editor.caret > 0 {
            layout.caret_bounds(previous_boundary(&editor.text, editor.caret) as u32, true)
        } else {
            layout.caret_bounds(editor.caret as u32, false)
        };
        let caret_brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
        ctx.draw_line(
            Vector2::new(TEXT_X + caret.left, TEXT_Y + caret.top),
            Vector2::new(TEXT_X + caret.left, TEXT_Y + caret.bottom),
            &caret_brush,
            2.0,
        );
    }

    *current_layout.borrow_mut() = Some(layout);
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_boundaries_keep_surrogate_pairs_together() {
        let text: Vec<u16> = "a\u{1F642}b".encode_utf16().collect();
        assert_eq!(previous_boundary(&text, 3), 1);
        assert_eq!(next_boundary(&text, 1), 3);
        assert_eq!(normalize_boundary(&text, 2), 1);
    }
}
