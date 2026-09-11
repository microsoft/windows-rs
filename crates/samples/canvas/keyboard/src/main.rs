#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

enum Message {
    Key(KeyEventInfo),
    Character(u16),
    ColorScheme(ColorScheme),
    Focus(bool),
    Clear,
}

struct Sample {
    text: Vec<u16>,
    color_scheme: ColorScheme,
    focused: bool,
    status: String,
    format: TextFormat,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _cx: &ComponentContext<Self>) -> Self {
        Self {
            text: "Type here".encode_utf16().collect(),
            color_scheme: ColorScheme::Light,
            focused: false,
            status: "Click the canvas or press Tab to focus".to_string(),
            format: TextFormat::new("Cascadia Mono", 28.0).unwrap(),
        }
    }

    fn update(&mut self, message: Message, _cx: &ComponentContext<Self>) {
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
            Message::ColorScheme(color_scheme) => self.color_scheme = color_scheme,
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
    }

    fn view(&self, _input: &(), cx: &mut ViewContext<Self>) -> View {
        let border = Border::new()
            .is_tab_stop(true)
            .automation_name("Keyboard input canvas")
            .focus_on_pointer_release(true)
            .corner_radius(8.0)
            .grid_row(1)
            .border_brush(if self.focused {
                Color::rgb(80, 150, 255)
            } else {
                Color::rgb(70, 76, 88)
            })
            .border_thickness(Thickness::uniform(if self.focused { 3.0 } else { 1.0 }))
            .on_preview_key_down(cx.routed_callback(|info: KeyEventInfo| {
                if matches!(info.key, VirtualKey::BACK | VirtualKey::DELETE) {
                    RoutedMessage::handled(Message::Key(info))
                } else {
                    RoutedMessage::bubble_without_message()
                }
            }))
            .on_character_received(cx.routed_callback(|info: CharacterEventInfo| {
                if info.character >= 0x20 && info.character != 0x7F {
                    RoutedMessage::handled(Message::Character(info.character))
                } else {
                    RoutedMessage::bubble_without_message()
                }
            }))
            .on_got_focus(cx.callback(|info: FocusEventInfo| Message::Focus(info.is_direct)))
            .on_lost_focus(cx.callback(|_| Message::Focus(false)));

        let text = String::from_utf16_lossy(&self.text);
        let format = self.format.clone();
        let color_scheme = self.color_scheme;
        let surface = border.content(canvas(move |cx| draw(cx, &text, &format, color_scheme)));

        let content = Grid::new()
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
                    .text("Type on the Canvas, tab to switch focus")
                    .font_size(18.0)
                    .font_weight(FontWeight::BOLD),
                surface,
                TextBlock::new()
                    .text(&self.status)
                    .font_size(14.0)
                    .grid_row(2),
                Button::new()
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .grid_row(3)
                    .on_click(cx.callback(|()| Message::Clear))
                    .content("Clear text"),
            ));

        cx.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Acrylic));
        cx.on_color_scheme(cx.callback(Message::ColorScheme));
        cx.window_frame("Canvas keyboard", content)
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

fn draw(cx: &DrawContext, text: &str, format: &TextFormat, scheme: ColorScheme) -> Result<()> {
    cx.clear(ColorF::TRANSPARENT);

    let color = match scheme {
        ColorScheme::Light => ColorF::from_rgba8(0, 0, 0, 228),
        ColorScheme::Dark => ColorF::WHITE,
    };

    cx.draw_text(
        text,
        format,
        &Rect::new(32.0, 32.0, cx.width - 32.0, 80.0),
        &cx.create_solid_brush(color)?,
    );

    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
