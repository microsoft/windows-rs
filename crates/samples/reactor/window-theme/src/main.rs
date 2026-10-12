#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy, Debug)]
enum Surface {
    Transparent,
    Theme,
    Green,
}

struct ThemeSample {
    theme: WindowTheme,
    backdrop: WindowBackdrop,
    surface: Surface,
    scheme: Option<ColorScheme>,
}

#[derive(Clone)]
enum Message {
    Theme(WindowTheme),
    Backdrop(WindowBackdrop),
    Surface(Surface),
    Scheme(ColorScheme),
}

impl Component for ThemeSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            theme: WindowTheme::System,
            backdrop: WindowBackdrop::None,
            surface: Surface::Transparent,
            scheme: None,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Theme(theme) => self.theme = theme,
            Message::Backdrop(backdrop) => self.backdrop = backdrop,
            Message::Surface(surface) => self.surface = surface,
            Message::Scheme(scheme) => self.scheme = Some(scheme),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.on_color_scheme(context.callback(Message::Scheme));
        context.window_visuals(
            WindowVisuals::new()
                .client_size(760.0, 500.0)
                .theme(self.theme)
                .backdrop(self.backdrop),
        );
        let content = StackPanel::new().spacing(16.0).margin(24.0).children((
            TextBlock::new().text("Contrast probe").font_size(28.0),
            TextBlock::new().text(format!(
                "State: {:?} / {:?} / {:?} / {:?}",
                self.theme, self.backdrop, self.surface, self.scheme,
            )),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children(
                    [WindowTheme::System, WindowTheme::Light, WindowTheme::Dark]
                        .into_iter()
                        .map(|theme| {
                            Button::new()
                                .content(format!("Theme {theme:?}"))
                                .on_click(context.message(Message::Theme(theme)))
                                .into()
                        })
                        .collect::<Vec<View>>(),
                ),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children(
                    [
                        WindowBackdrop::None,
                        WindowBackdrop::Mica,
                        WindowBackdrop::MicaAlt,
                        WindowBackdrop::Acrylic,
                    ]
                    .into_iter()
                    .map(|backdrop| {
                        Button::new()
                            .content(format!("Backdrop {backdrop:?}"))
                            .on_click(context.message(Message::Backdrop(backdrop)))
                            .into()
                    })
                    .collect::<Vec<View>>(),
                ),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children(
                    [Surface::Transparent, Surface::Theme, Surface::Green]
                        .into_iter()
                        .map(|surface| {
                            Button::new()
                                .content(format!("Surface {surface:?}"))
                                .on_click(context.message(Message::Surface(surface)))
                                .into()
                        })
                        .collect::<Vec<View>>(),
                ),
            TextBlock::new()
                .text("Theme paints an opaque root; Green shows an application-owned background.")
                .text_wrapping(TextWrapping::Wrap),
        ));
        let root = Border::new().content(context.window_frame("Window theme", content));
        match self.surface {
            Surface::Transparent => root,
            Surface::Theme => root.background(ThemeBrush::SolidBackground),
            Surface::Green => root.background(Color::rgb(0, 128, 0)),
        }
        .into()
    }
}

fn main() {
    App::run_component::<ThemeSample>(()).unwrap();
}
