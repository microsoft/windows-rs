#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::*;
use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

const OPEN: u32 = 1;
const EXIT: u32 = 2;

#[derive(Clone, PartialEq)]
struct TrayWindowInput {
    closed: Callback<()>,
    exit: Callback<()>,
    remove_tray: Callback<()>,
}

struct TrayWindow {
    closed: Callback<()>,
    exit: Callback<()>,
    remove_tray: Callback<()>,
}

impl Component for TrayWindow {
    type Input = TrayWindowInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            closed: input.closed.clone(),
            exit: input.exit.clone(),
            remove_tray: input.remove_tray.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.closed = input.closed.clone();
        self.exit = input.exit.clone();
        self.remove_tray = input.remove_tray.clone();
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame(
            "Reactor tray icon",
            StackPanel::new().spacing(8.0).children((
                "This window is independent of the tray icon.",
                "Close it and use the tray icon to open another.",
                Button::new()
                    .on_click(self.remove_tray.clone())
                    .content("Remove tray icon"),
                Button::new()
                    .on_click(self.exit.clone())
                    .content("Exit application"),
            )),
        )
    }
}

impl Drop for TrayWindow {
    fn drop(&mut self) {
        _ = self.closed.call(());
    }
}

fn main() {
    App::run_with(|app| {
        let tray = Rc::new(RefCell::new(None));
        let window_open = Rc::new(Cell::new(false));
        let remove_tray = {
            let tray = Rc::downgrade(&tray);
            Callback::new(move |_| {
                if let Some(tray) = tray.upgrade() {
                    tray.borrow_mut().take();
                }
            })
        };
        let exit = {
            let app = app.clone();
            Callback::new(move |_| {
                if let Err(error) = app.exit() {
                    eprintln!("could not exit Reactor application: {error}");
                }
            })
        };
        let closed = {
            let app = app.clone();
            let tray = Rc::downgrade(&tray);
            let window_open = Rc::clone(&window_open);
            Callback::new(move |_| {
                window_open.set(false);
                if tray.upgrade().is_some_and(|tray| tray.borrow().is_none())
                    && let Err(error) = app.exit()
                {
                    eprintln!("could not exit Reactor application: {error}");
                }
            })
        };
        let window_input = TrayWindowInput {
            closed,
            exit,
            remove_tray,
        };
        let open_input = window_input.clone();
        let open_app = app.clone();
        let open_window = Rc::clone(&window_open);
        let tray_icon = TrayIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\icon\\icon.ico"))
            .tooltip("Reactor tray icon sample")
            .menu(
                Menu::new()
                    .item(OPEN, "Open window")
                    .separator()
                    .item(EXIT, "Exit"),
            )
            .on_event(move |event| match event {
                TrayIconEvent::Activate { .. } | TrayIconEvent::MenuItem { id: OPEN } => {
                    if open_window.replace(true) {
                        return;
                    }
                    let input = open_input.clone();
                    if let Err(error) = open_app.open_window(View::component::<TrayWindow>(input)) {
                        open_window.set(false);
                        eprintln!("could not open Reactor window: {error}");
                    }
                }
                TrayIconEvent::MenuItem { id: EXIT } => {
                    _ = window_input.exit.call(());
                }
                _ => {}
            })
            .build()?;

        *tray.borrow_mut() = Some(tray_icon);
        Ok(tray)
    })
    .unwrap();
}
