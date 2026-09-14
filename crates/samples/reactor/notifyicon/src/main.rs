#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_notifyicon::{NotifyIcon, NotifyIconEvent};
use windows_reactor::*;

struct AppState {
    activate_window: RefCell<Option<Callback<()>>>,
    app: AppContext,
    icon: RefCell<Option<NotifyIcon>>,
    window_open: Cell<bool>,
}

impl AppState {
    fn add_icon(self: &Rc<Self>) -> windows_notifyicon::Result<()> {
        if self.icon.borrow().is_some() {
            return Ok(());
        }

        let state = Rc::downgrade(self);
        let icon = NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\icon\\icon.ico"))
            .tooltip("Left-click to open; right-click to exit")
            .on_event(move |event| {
                let Some(state) = state.upgrade() else {
                    return;
                };
                match event {
                    NotifyIconEvent::Activate { .. } => state.open_window(),
                    NotifyIconEvent::ContextMenu { .. } => state.exit(),
                    _ => {}
                }
            })
            .build()?;
        *self.icon.borrow_mut() = Some(icon);
        Ok(())
    }

    fn toggle_icon(self: &Rc<Self>) {
        let removed = self.icon.borrow_mut().take().is_some();
        if !removed && let Err(error) = self.add_icon() {
            eprintln!("could not add notification icon: {error}");
        }
    }

    fn open_window(self: &Rc<Self>) {
        if self.window_open.replace(true) {
            if let Some(activate) = self.activate_window.borrow().as_ref() {
                _ = activate.call(());
            }
            return;
        }
        if let Err(error) =
            self.app
                .open_window(View::component::<NotifyWindow>(NotifyWindowInput(
                    Rc::clone(self),
                )))
        {
            self.window_open.set(false);
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
        *input.0.activate_window.borrow_mut() =
            Some(context.sender().callback(|()| Message::Activate));
        Self {
            state: Rc::clone(&input.0),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.state = Rc::clone(&input.0);
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
        self.state.activate_window.borrow_mut().take();
        self.state.window_open.set(false);
        if self.state.icon.borrow().is_none() {
            self.state.exit();
        }
    }
}

fn main() {
    App::run_with(|app| {
        let state = Rc::new(AppState {
            activate_window: RefCell::new(None),
            app: app.clone(),
            icon: RefCell::new(None),
            window_open: Cell::new(false),
        });
        state.add_icon()?;
        Ok(state)
    })
    .unwrap();
}
