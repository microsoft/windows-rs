use std::rc::Rc;
use std::time::Duration;
use windows::Win32::winuser::{ICON_BIG, ICON_SMALL, SendMessageW, WM_GETICON};
use windows_reactor::{
    App, Component, ComponentContext, ComponentTimer, TextBlock, ViewContext, WindowBackdrop,
    WindowConstraints, WindowSize, WindowTheme, WindowVisuals,
};

fn window_has_explicit_icon(window: &windows_reactor::WindowHandle<'_>) -> bool {
    let hwnd = window.as_raw().cast();
    unsafe {
        SendMessageW(hwnd, WM_GETICON as u32, ICON_BIG as usize, 0) != 0
            || SendMessageW(hwnd, WM_GETICON as u32, ICON_SMALL as usize, 0) != 0
    }
}

struct Fixture {
    icon: Rc<str>,
    phase: u8,
    timeout: Option<ComponentTimer>,
}

enum Message {
    IconCleared(bool),
    IconSet(bool),
    Size(WindowSize),
    Timeout,
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            icon: std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .into_owned()
                .into(),
            phase: 0,
            timeout: Some(context.set_timeout(Duration::from_secs(10), Message::Timeout)),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Size(size) if self.phase == 0 && size.width > 0.0 && size.height > 0.0 => {
                self.phase = 1;
                self.timeout = Some(context.set_timeout(Duration::from_secs(10), Message::Timeout));
            }
            Message::Size(size) if self.phase == 1 && size.width >= 500.0 => {
                self.phase = 2;
                assert!(
                    context
                        .run_window(|window| Message::IconSet(window_has_explicit_icon(&window)))
                );
            }
            Message::IconSet(true) if self.phase == 2 => {
                self.phase = 3;
                self.timeout = Some(context.set_timeout(Duration::from_secs(10), Message::Timeout));
            }
            Message::Size(size) if self.phase == 3 && size.width >= 540.0 => {
                self.phase = 4;
                assert!(
                    context.run_window(|window| Message::IconCleared(window_has_explicit_icon(
                        &window
                    )))
                );
            }
            Message::IconCleared(false) if self.phase == 4 => {
                self.phase = 5;
                assert!(context.close_window());
            }
            Message::IconCleared(_) | Message::IconSet(_) | Message::Size(_) => {
                eprintln!("real WinUI window icon transition was not applied");
                std::process::exit(1);
            }
            Message::Timeout => {
                eprintln!("real WinUI reactive window state observation was not delivered");
                std::process::exit(1);
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> windows_reactor::View {
        let sender = context.sender();
        context.on_window_size(move |size| {
            _ = sender.send(Message::Size(size));
        });
        match self.phase {
            0 => {
                context.window_title("Reactor reactive window");
                context.window_visuals(
                    WindowVisuals::new()
                        .theme(WindowTheme::Dark)
                        .backdrop(WindowBackdrop::Mica)
                        .client_size(480.0, 320.0)
                        .constraints(WindowConstraints {
                            min_width: Some(320.0),
                            min_height: Some(240.0),
                            max_width: Some(960.0),
                            max_height: Some(720.0),
                        }),
                );
            }
            1 | 2 => {
                context.window_title("Reactor reactive window updated");
                context.window_visuals(
                    WindowVisuals::new()
                        .theme(WindowTheme::Light)
                        .backdrop(WindowBackdrop::Acrylic)
                        .client_size(520.0, 360.0)
                        .icon(self.icon.clone()),
                );
            }
            _ => {
                context.window_title("Reactor reactive window cleared");
                context.window_visuals(
                    WindowVisuals::new()
                        .theme(WindowTheme::System)
                        .client_size(540.0, 380.0),
                );
            }
        }
        TextBlock::new().text("Reactive window state").into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(45));
        eprintln!("Reactor window state fixture timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
