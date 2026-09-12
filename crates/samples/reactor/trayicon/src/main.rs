#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::*;
use windows_trayicon::{TrayIcon, TrayIconEvent};

struct AppState {
    activate_window: RefCell<Option<Callback<()>>>,
    app: AppContext,
    tray: RefCell<Option<TrayIcon>>,
    window_open: Cell<bool>,
}

impl AppState {
    fn add_tray(self: &Rc<Self>) -> windows_trayicon::Result<()> {
        if self.tray.borrow().is_some() {
            return Ok(());
        }

        let state = Rc::downgrade(self);
        let tray = TrayIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\icon\\icon.ico"))
            .tooltip("Left-click to open; right-click to exit")
            .on_event(move |event| {
                let Some(state) = state.upgrade() else {
                    return;
                };
                match event {
                    TrayIconEvent::Activate { .. } => state.open_window(),
                    TrayIconEvent::ContextMenu { .. } => state.exit(),
                    _ => {}
                }
            })
            .build()?;
        *self.tray.borrow_mut() = Some(tray);
        Ok(())
    }

    fn toggle_tray(self: &Rc<Self>) {
        let removed = self.tray.borrow_mut().take().is_some();
        if !removed && let Err(error) = self.add_tray() {
            eprintln!("could not add tray icon: {error}");
        }
    }

    fn open_window(self: &Rc<Self>) {
        if self.window_open.replace(true) {
            if let Some(activate) = self.activate_window.borrow().as_ref() {
                _ = activate.call(());
            }
            return;
        }
        if let Err(error) = self
            .app
            .open_window(View::component::<TrayWindow>(TrayWindowInput(Rc::clone(
                self,
            ))))
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
struct TrayWindowInput(Rc<AppState>);

impl PartialEq for TrayWindowInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Copy)]
enum Message {
    Activate,
    Exit,
    ToggleTray,
}

struct TrayWindow {
    state: Rc<AppState>,
}

impl Component for TrayWindow {
    type Input = TrayWindowInput;
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
            Message::ToggleTray => self.state.toggle_tray(),
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let tray_button = if self.state.tray.borrow().is_some() {
            "Remove tray icon"
        } else {
            "Add tray icon"
        };
        context.window_frame(
            "Reactor tray icon",
            StackPanel::new().spacing(8.0).children((
                "This window is independent of the tray icon.",
                "Close it and use the tray icon to open another.",
                Button::new()
                    .on_click(context.message(Message::ToggleTray))
                    .content(tray_button),
                Button::new()
                    .on_click(context.message(Message::Exit))
                    .content("Exit application"),
            )),
        )
    }
}

impl Drop for TrayWindow {
    fn drop(&mut self) {
        self.state.activate_window.borrow_mut().take();
        self.state.window_open.set(false);
        if self.state.tray.borrow().is_none() {
            self.state.exit();
        }
    }
}

fn main() {
    App::run_with(|app| {
        let state = Rc::new(AppState {
            activate_window: RefCell::new(None),
            app: app.clone(),
            tray: RefCell::new(None),
            window_open: Cell::new(false),
        });
        state.add_tray()?;
        Ok(state)
    })
    .unwrap();
}
