#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;

use windows_notifyicon::{NotifyIcon, NotifyIconEvent};
use windows_reactor::*;

const EXIT: &str = "Exit";
const OPEN: &str = "Open";

struct AppState {
    app: AppContext,
    icon: RefCell<Option<NotifyIcon>>,
    window: RefCell<OpenWindow>,
}

enum OpenWindow {
    Closed,
    Opening,
    Open(Callback<()>),
}

impl AppState {
    fn add_icon(self: &Rc<Self>) -> windows_notifyicon::Result<()> {
        let events = Rc::downgrade(self);
        let icon = NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\icon\\icon.ico"))
            .tooltip("Left-click to open; right-click for menu")
            .on_event(move |event| {
                let Some(state) = events.upgrade() else {
                    return;
                };
                match event {
                    NotifyIconEvent::Activate { .. } => state.open_window(),
                    NotifyIconEvent::ContextMenu { position } => state.show_menu(position),
                    NotifyIconEvent::Unavailable => {
                        eprintln!("the Windows Shell could not restore the notification icon");
                        state.exit();
                    }
                    _ => {}
                }
            })
            .build()?;
        *self.icon.borrow_mut() = Some(icon);
        Ok(())
    }

    fn toggle_icon(self: &Rc<Self>) {
        if self.icon.borrow_mut().take().is_some() {
            return;
        }
        if let Err(error) = self.add_icon() {
            eprintln!("could not add notification icon: {error}");
        }
    }

    fn show_menu(self: &Rc<Self>, position: windows_notifyicon::Point) {
        let state = Rc::clone(self);
        let menu = Menu::new(
            [
                MenuItem::item("open", OPEN),
                MenuItem::separator("separator"),
                MenuItem::item("exit", EXIT),
            ],
            move |label: String| match label.as_str() {
                OPEN => state.open_window(),
                EXIT => state.exit(),
                _ => {}
            },
        );
        if let Err(error) = self
            .app
            .show_menu_at(ScreenPoint::new(position.x, position.y), menu)
        {
            eprintln!("could not show notification icon menu: {error}");
        }
    }

    fn open_window(self: &Rc<Self>) {
        let activate = {
            let mut window = self.window.borrow_mut();
            match &*window {
                OpenWindow::Closed => {
                    *window = OpenWindow::Opening;
                    None
                }
                OpenWindow::Opening => return,
                OpenWindow::Open(activate) => Some(activate.clone()),
            }
        };
        if let Some(activate) = activate {
            _ = activate.call(());
            return;
        }
        if let Err(error) =
            self.app
                .open_window(View::component::<NotifyWindow>(NotifyWindowInput(
                    Rc::clone(self),
                )))
        {
            *self.window.borrow_mut() = OpenWindow::Closed;
            eprintln!("could not open Reactor window: {error}");
        }
    }

    fn exit(&self) {
        if let Err(error) = self.app.exit() {
            eprintln!("could not exit Reactor application: {error}");
        }
    }
}

#[derive(Clone)]
struct NotifyWindowInput(Rc<AppState>);

impl PartialEq for NotifyWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Copy)]
enum Message {
    Activate,
    Exit,
    ToggleIcon,
}

struct NotifyWindow {
    state: Rc<AppState>,
}

impl Component for NotifyWindow {
    type Input = NotifyWindowInput;
    type Message = Message;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.0.window.borrow_mut() =
            OpenWindow::Open(context.sender().callback(|()| Message::Activate));
        Self {
            state: Rc::clone(&input.0),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Activate => {
                if !context.window().request_activate() {
                    eprintln!("could not activate Reactor window");
                }
            }
            Message::Exit => self.state.exit(),
            Message::ToggleIcon => self.state.toggle_icon(),
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let icon_button = if self.state.icon.borrow().is_some() {
            "Remove notification icon"
        } else {
            "Add notification icon"
        };
        context.window_frame(
            "Reactor notification icon",
            StackPanel::new().spacing(8.0).children((
                "This window is independent of the notification icon.",
                "Close it and use the notification icon to open another.",
                Button::new()
                    .on_click(context.message(Message::ToggleIcon))
                    .content(icon_button),
                Button::new()
                    .on_click(context.message(Message::Exit))
                    .content("Exit application"),
            )),
        )
    }
}

impl Drop for NotifyWindow {
    fn drop(&mut self) {
        *self.state.window.borrow_mut() = OpenWindow::Closed;
        if self.state.icon.borrow().is_none() {
            self.state.exit();
        }
    }
}

fn main() {
    App::run_with(|app| {
        let state = Rc::new(AppState {
            app: app.clone(),
            icon: RefCell::new(None),
            window: RefCell::new(OpenWindow::Closed),
        });
        state.add_icon()?;
        Ok(state)
    })
    .unwrap();
}
