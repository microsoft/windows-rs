use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use windows::Win32::*;
use windows_core::w;
use windows_reactor::*;

const ICON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");

fn own_window(title: windows_core::PCWSTR) -> HWND {
    find_own_window(title).unwrap()
}

fn find_own_window(title: windows_core::PCWSTR) -> Option<HWND> {
    next_own_window(title, None)
}

fn next_own_window(title: windows_core::PCWSTR, mut after: Option<HWND>) -> Option<HWND> {
    loop {
        unsafe {
            let hwnd = FindWindowExW(None, after, None, title);
            if hwnd.is_null() {
                return None;
            }
            let mut process = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process));
            if process == GetCurrentProcessId() {
                return Some(hwnd);
            }
            after = Some(hwnd);
        }
    }
}

struct MenuApp(bool);

impl Application for MenuApp {
    type Input = Rc<Cell<bool>>;
    type Message = u8;

    fn create(_: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        assert!(context.sender().send(0));
        Self(true)
    }

    fn update(&mut self, message: u8, _: &ApplicationContext<Self>) {
        match message {
            0 => unsafe {
                let first = own_window(w!("windows-notifyicon"));
                let second = next_own_window(w!("windows-notifyicon"), Some(first)).unwrap();
                SendMessageW(first, WM_USER as u32 + 1, 0, WM_CONTEXTMENU as isize);
                SendMessageW(first, WM_USER as u32 + 1, 0, WM_CONTEXTMENU as isize);
                SendMessageW(second, WM_USER as u32 + 1, 0, WM_CONTEXTMENU as isize);
                PostMessageW(Some(second), WM_USER as u32 + 1, 0, 0x400).unwrap();
            },
            1 => self.0 = false,
            _ => unreachable!(),
        }
    }

    fn view(
        &self,
        completed: &Self::Input,
        context: &ApplicationViewContext<Self>,
    ) -> ApplicationView {
        let mut view = ApplicationView::new();
        if self.0 {
            for key in ["first", "second"] {
                view = view.notify_icon(
                    key,
                    NotifyIcon::new(ICON)
                        .on_activate(context.callback(|_| 1))
                        .menu(Menu::new(
                            [MenuItem::item("close", key)],
                            context.callback(|_| 1),
                        )),
                );
            }
        } else {
            completed.set(true);
        }
        view
    }
}

#[derive(Default)]
struct Observations {
    mounted: Cell<u32>,
    dropped: Cell<u32>,
    inputs: Cell<u32>,
    completed: Cell<bool>,
}

#[derive(Clone)]
struct WindowInput {
    observations: Rc<Observations>,
    phase: u32,
    tick: Callback<()>,
    closed: Callback<()>,
}

impl PartialEq for WindowInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.observations, &other.observations)
            && self.phase == other.phase
            && self.tick == other.tick
            && self.closed == other.closed
    }
}

struct TestWindow {
    input: WindowInput,
    _timer: ComponentTimer,
}

impl Component for TestWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input
            .observations
            .mounted
            .set(input.observations.mounted.get() + 1);
        Self {
            input: input.clone(),
            _timer: context.set_timeout(Duration::from_millis(150), ()),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, context: &ComponentContext<Self>) {
        self.input = input.clone();
        input
            .observations
            .inputs
            .set(input.observations.inputs.get() + 1);
        if input.phase == 4 {
            assert!(context.close_window());
        }
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        self.input.tick.call(());
        self._timer = context.set_timeout(Duration::from_millis(150), ());
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame(
            "Application resource fixture",
            format!("Phase {}", input.phase),
        )
    }
}

impl Drop for TestWindow {
    fn drop(&mut self) {
        self.input
            .observations
            .dropped
            .set(self.input.observations.dropped.get() + 1);
        self.input.closed.call(());
    }
}

#[derive(Clone, Copy)]
enum Message {
    Tick,
    Closed,
    Activate,
}

struct ResourceApp {
    observations: Rc<Observations>,
    phase: u32,
    icon: bool,
    window: bool,
}

impl Application for ResourceApp {
    type Input = Rc<Observations>;
    type Message = Message;

    fn create(input: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        context.show_window("main");
        Self {
            observations: Rc::clone(input),
            phase: 0,
            icon: true,
            window: true,
        }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Tick => match self.phase {
                0 => {
                    assert_eq!(self.observations.mounted.get(), 1);
                    self.icon = false;
                    self.phase = 1;
                }
                1 => {
                    assert_eq!(self.observations.mounted.get(), 1);
                    self.icon = true;
                    self.phase = 2;
                }
                2 => {
                    unsafe {
                        let hwnd = own_window(w!("Application resource fixture"));
                        _ = ShowWindow(hwnd, SW_MINIMIZE);
                        assert!(IsIconic(hwnd).as_bool());
                    }
                    context.show_window("main");
                    context.show_window("main");
                    self.phase = 3;
                }
                3 => {
                    unsafe {
                        assert!(
                            !IsIconic(own_window(w!("Application resource fixture"))).as_bool()
                        );
                    }
                    assert_eq!(self.observations.mounted.get(), 1);
                    context.show_window("main");
                    context.show_window("main");
                    // This publication asks the window to close while activation is pending.
                    self.phase = 4;
                }
                5 => {
                    assert_eq!(self.observations.mounted.get(), 2);
                    assert!(self.observations.inputs.get() >= 3);
                    self.window = false;
                    self.phase = 6;
                }
                _ => {}
            },
            Message::Closed => match self.phase {
                4 => {
                    assert_eq!(self.observations.dropped.get(), 1);
                    self.phase = 5;
                    context.show_window("main");
                    context.show_window("main");
                }
                6 => {
                    assert_eq!(self.observations.dropped.get(), 2);
                    // Only the icon remains. Drive its public activation callback through Shell
                    // protocol messages, then remove the final resource from that callback.
                    self.phase = 7;
                    unsafe {
                        let hwnd = own_window(w!("windows-notifyicon"));
                        PostMessageW(Some(hwnd), WM_USER as u32 + 1, 0, 0x400).unwrap();
                    }
                }
                _ => panic!("unexpected window close"),
            },
            Message::Activate => {
                if self.phase == 8 {
                    self.observations.completed.set(true);
                    self.icon = false;
                    self.phase = 9;
                    return;
                }
                assert_eq!(self.phase, 7);
                // Exercise Explorer recovery without restarting Explorer: only our own callback
                // window receives TaskbarCreated.
                unsafe {
                    let hwnd = own_window(w!("windows-notifyicon"));
                    let message = RegisterWindowMessageW(w!("TaskbarCreated"));
                    assert_ne!(message, 0);
                    SendMessageW(hwnd, message, 0, 0);
                    SendMessageW(hwnd, WM_USER as u32 + 1, 0, WM_CONTEXTMENU as isize);
                    PostMessageW(Some(hwnd), WM_USER as u32 + 1, 0, 0x400).unwrap();
                }
                self.phase = 8;
            }
        }
    }

    fn view(&self, _: &Self::Input, context: &ApplicationViewContext<Self>) -> ApplicationView {
        let mut view = ApplicationView::new();
        if self.window {
            view = view.window::<TestWindow>(
                "main",
                WindowInput {
                    observations: Rc::clone(&self.observations),
                    phase: self.phase,
                    tick: context.callback(|()| Message::Tick),
                    closed: context.callback(|()| Message::Closed),
                },
            );
        }
        if self.icon {
            view = view.notify_icon(
                "tray",
                NotifyIcon::new(ICON)
                    .tooltip(format!("Application resources {}", self.phase))
                    .on_activate(context.callback(|_| Message::Activate))
                    .menu(Menu::new(
                        [MenuItem::item("finish", "Finish")],
                        context.callback(|_| Message::Activate),
                    )),
            );
        }
        view
    }
}

static QUICK_DROPS: AtomicUsize = AtomicUsize::new(0);

struct QuickWindow {
    callback: Option<Callback<()>>,
    _timer: ComponentTimer,
}

impl Component for QuickWindow {
    type Input = Option<Callback<()>>;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            callback: input.clone(),
            _timer: context.set_timeout(Duration::from_millis(150), ()),
        }
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        if let Some(callback) = &self.callback {
            callback.call(());
        } else {
            assert!(context.close_window());
        }
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame("Quick application fixture", "Closing shortly")
    }
}

impl Drop for QuickWindow {
    fn drop(&mut self) {
        QUICK_DROPS.fetch_add(1, Ordering::Relaxed);
    }
}

struct EmptyApp {
    updated: bool,
    exit: bool,
}

impl Application for EmptyApp {
    type Input = String;
    type Message = ();

    fn create(input: &String, context: &ApplicationContext<Self>) -> Self {
        if input == "unknown" {
            context.show_window("missing");
        }
        if input == "exit" {
            context.exit();
        }
        if matches!(
            input.as_str(),
            "window-only" | "runtime-error" | "explicit-exit"
        ) {
            context.show_window("main");
        }
        Self {
            updated: false,
            exit: input == "explicit-exit",
        }
    }

    fn update(&mut self, _: (), context: &ApplicationContext<Self>) {
        self.updated = true;
        if self.exit {
            context.exit();
        }
    }

    fn view(&self, input: &String, context: &ApplicationViewContext<Self>) -> ApplicationView {
        let view = ApplicationView::new();
        match input.as_str() {
            "invalid-icon" | "exit" => {
                view.notify_icon("tray", NotifyIcon::new("missing-icon.ico"))
            }
            "duplicate" => view
                .notify_icon("tray", NotifyIcon::new(ICON))
                .notify_icon("tray", NotifyIcon::new(ICON)),
            "window-only" => view.window::<QuickWindow>("main", None),
            "runtime-error" | "explicit-exit" => view
                .window::<QuickWindow>("main", Some(context.callback(|()| ())))
                .notify_icon(
                    "tray",
                    NotifyIcon::new(if self.updated {
                        "missing-icon.ico"
                    } else {
                        ICON
                    }),
                ),
            _ => view,
        }
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Application resources fixture timed out");
        std::process::exit(1);
    });
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "lifecycle".into());
    if mode == "menus" {
        let completed = Rc::new(Cell::new(false));
        App::run_application::<MenuApp>(Rc::clone(&completed))?;
        assert!(completed.get());
        assert!(find_own_window(w!("windows-notifyicon")).is_none());
        return Ok(());
    }
    if mode != "lifecycle" {
        let result = App::run_application::<EmptyApp>(mode.clone());
        if matches!(mode.as_str(), "unknown" | "duplicate") {
            assert_eq!(
                result.unwrap_err().code(),
                windows_core::HRESULT(0x80070057u32 as i32)
            );
        } else if matches!(mode.as_str(), "invalid-icon" | "runtime-error") {
            assert_eq!(
                result.unwrap_err().code(),
                windows_core::HRESULT(0x80070002u32 as i32)
            );
        } else {
            result?;
        }
        if matches!(
            mode.as_str(),
            "window-only" | "runtime-error" | "explicit-exit"
        ) {
            assert_eq!(QUICK_DROPS.load(Ordering::Relaxed), 1);
            assert!(find_own_window(w!("Quick application fixture")).is_none());
        }
        assert!(find_own_window(w!("windows-notifyicon")).is_none());
        return Ok(());
    }

    let observations = Rc::new(Observations::default());
    App::run_application::<ResourceApp>(Rc::clone(&observations))?;
    assert!(observations.completed.get());
    assert_eq!(observations.mounted.get(), 2);
    assert_eq!(observations.dropped.get(), 2);
    Ok(())
}
