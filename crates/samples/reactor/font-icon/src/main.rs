#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Appearance(usize),
    Theme,
}

struct FontIconSample {
    appearance: usize,
    dark: bool,
}

impl Component for FontIconSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            appearance: 0,
            dark: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Appearance(value) => self.appearance = value,
            Message::Theme => self.dark = !self.dark,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("FontIcon size and foreground");
        context.window_visuals(WindowVisuals::new().client_size(540.0, 320.0).theme(
            if self.dark {
                WindowTheme::Dark
            } else {
                WindowTheme::Light
            },
        ));
        let icon = FontIcon::new().glyph("\u{E80F}");
        let icon = match self.appearance {
            0 => icon,
            1 => icon.font_size(16.0).foreground(Color::rgb(220, 20, 40)),
            2 => icon.font_size(48.0).foreground(Color::rgb(20, 70, 220)),
            3 => icon.font_size(48.0).foreground(ThemeBrush::PrimaryText),
            _ => unreachable!(),
        };
        Border::new()
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new().spacing(12.0).children((
                    "The icon inherits its color from the hyperlink unless overridden.",
                    Border::new().height(100.0).content(
                        HyperlinkButton::new()
                            .automation_name("Preview")
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center)
                            .content(icon),
                    ),
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children(
                            ["Native defaults", "16 / red", "48 / blue", "Theme text"]
                                .into_iter()
                                .enumerate()
                                .map(|(index, label)| {
                                    Button::new()
                                        .content(label)
                                        .on_click(context.message(Message::Appearance(index)))
                                        .into()
                                })
                                .collect::<Vec<View>>(),
                        ),
                    Button::new()
                        .content("Toggle theme")
                        .on_click(context.message(Message::Theme)),
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<FontIconSample>(()).unwrap();
}
