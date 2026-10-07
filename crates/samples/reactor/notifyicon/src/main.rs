#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Open,
    ToggleIcon,
    Exit,
    Menu(Key),
}

struct TrayApp {
    icon: bool,
}

impl Application for TrayApp {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ApplicationContext<Self>) -> Self {
        Self { icon: true }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Open => context.show_window("main"),
            Message::ToggleIcon => self.icon = !self.icon,
            Message::Exit => context.exit(),
            Message::Menu(key) => match key.as_str() {
                Some("open") => context.show_window("main"),
                Some("exit") => context.exit(),
                _ => {}
            },
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        let view = ApplicationView::new().window::<NotifyWindow>(
            "main",
            WindowInput {
                icon: self.icon,
                on_toggle: context.callback(|()| Message::ToggleIcon),
                on_exit: context.callback(|()| Message::Exit),
            },
        );
        if self.icon {
            view.notify_icon(
                "tray",
                NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\icon\\icon.ico"))
                    .tooltip("Left-click to open; right-click for menu")
                    .on_activate(context.callback(|_| Message::Open))
                    .menu(Menu::new(
                        [
                            MenuItem::item("open", "Open"),
                            MenuItem::separator("separator"),
                            MenuItem::item("exit", "Exit"),
                        ],
                        context.callback(Message::Menu),
                    )),
            )
        } else {
            view
        }
    }
}

#[derive(Clone, PartialEq)]
struct WindowInput {
    icon: bool,
    on_toggle: Callback<()>,
    on_exit: Callback<()>,
}

struct NotifyWindow;

impl Component for NotifyWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame(
            "Reactor notification icon",
            StackPanel::new().spacing(8.0).children((
                "This window is independent of the notification icon.",
                "Close it and use the notification icon to open another.",
                Button::new()
                    .on_click(input.on_toggle.clone())
                    .content(if input.icon {
                        "Remove notification icon"
                    } else {
                        "Add notification icon"
                    }),
                Button::new()
                    .on_click(input.on_exit.clone())
                    .content("Exit application"),
            )),
        )
    }
}

fn main() {
    App::run_application::<TrayApp>(()).unwrap();
}
