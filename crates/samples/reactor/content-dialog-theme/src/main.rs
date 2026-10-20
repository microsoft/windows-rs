#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Theme(WindowTheme),
    Open(bool),
}

struct DialogThemeSample {
    theme: WindowTheme,
    open: bool,
}

impl Component for DialogThemeSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            theme: WindowTheme::Light,
            open: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Theme(theme) => self.theme = theme,
            Message::Open(open) => self.open = open,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(
            WindowVisuals::new()
                .client_size(520.0, 360.0)
                .theme(self.theme),
        );
        let button = |theme| {
            Button::new()
                .content(format!("{theme:?}"))
                .on_click(context.message(Message::Theme(theme)))
        };
        let themes = || {
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    button(WindowTheme::Light),
                    button(WindowTheme::Dark),
                    button(WindowTheme::System),
                ))
        };
        context.window_frame(
            "ContentDialog theme",
            StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .children((
                    "Choose a window theme, then open the dialog.",
                    themes(),
                    Button::new()
                        .content("Open dialog")
                        .on_click(context.message(Message::Open(true))),
                ))
                .content_dialog(
                    ContentDialog::new()
                        .title("Same theme as the window")
                        .close_button_text("Close")
                        .is_open(self.open)
                        .on_closed(context.callback(|_| Message::Open(false)))
                        .content(StackPanel::new().spacing(12.0).children((
                            format!("Window theme: {:?}", self.theme),
                            "These buttons change the window theme while the dialog is open.",
                            themes(),
                        ))),
                ),
        )
    }
}

fn main() {
    App::run_component::<DialogThemeSample>(()).unwrap();
}
