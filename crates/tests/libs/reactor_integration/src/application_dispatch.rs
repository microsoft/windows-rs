use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use windows::Win32::*;
use windows_core::w;
use windows_reactor::*;

#[derive(Default, PartialEq)]
struct Observations {
    active: Cell<bool>,
    updates: Cell<u32>,
    inputs: Cell<u32>,
    mounted: Cell<u32>,
    dropped: Cell<u32>,
    component_updates: Cell<u32>,
}

#[derive(Clone, PartialEq)]
struct Input {
    observations: Rc<Observations>,
    mode: String,
    phase: u32,
    callback: Callback<()>,
}

struct Model {
    phase: u32,
    mode: String,
    observations: Rc<Observations>,
}

impl Application for Model {
    type Input = (String, Rc<Observations>);
    type Message = ();

    fn create((mode, observations): &Self::Input, context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self {
            phase: 0,
            mode: mode.clone(),
            observations: Rc::clone(observations),
        }
    }

    fn update(&mut self, _: (), _: &ApplicationContext<Self>) {
        assert!(!self.observations.active.get());
        self.phase += 1;
        if self.mode == "application" && self.phase == 1 {
            self.observations.active.set(true);
            pump();
            assert_eq!(self.observations.component_updates.get(), 1);
            self.observations.active.set(false);
        }
        if self.mode == "notification" && self.phase == 1 {
            self.observations.active.set(true);
            notification_events();
            pump();
            assert_eq!(self.observations.updates.get(), 0);
            self.observations.active.set(false);
        }
    }

    fn view(
        &self,
        (mode, observations): &Self::Input,
        context: &ApplicationViewContext<Self>,
    ) -> ApplicationView {
        assert!(!observations.active.get());
        observations.updates.set(self.phase);
        let view = ApplicationView::new().window::<NestedWindow>(
            "main",
            Input {
                observations: Rc::clone(observations),
                mode: mode.clone(),
                phase: self.phase,
                callback: context.callback(|()| ()),
            },
        );
        if mode == "notification" && self.phase < 2 {
            view.notify_icon(
                "tray",
                NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"))
                    .on_activate(context.callback(|_| ())),
            )
        } else {
            view
        }
    }
}

struct NestedWindow {
    input: Input,
    _timer: ComponentTimer,
}

impl Component for NestedWindow {
    type Input = Input;
    type Message = ();

    fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
        input
            .observations
            .mounted
            .set(input.observations.mounted.get() + 1);
        Self {
            input: input.clone(),
            _timer: context.set_timeout(Duration::from_millis(200), ()),
        }
    }

    fn input_changed(&mut self, input: &Input, context: &ComponentContext<Self>) {
        assert!(!input.observations.active.get());
        assert_eq!(input.phase, 2);
        input
            .observations
            .inputs
            .set(input.observations.inputs.get() + 1);
        self.input = input.clone();
        assert!(context.close_window());
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        let observations = &self.input.observations;
        assert!(!observations.active.get());
        observations
            .component_updates
            .set(observations.component_updates.get() + 1);
        if observations.component_updates.get() == 2 {
            return;
        }
        if self.input.mode != "notification" {
            self.input.callback.call(());
            self.input.callback.call(());
        }
        if self.input.mode == "application" {
            self._timer = context.set_timeout(Duration::from_millis(20), ());
            return;
        }
        observations.active.set(true);
        if self.input.mode == "notification" {
            notification_events();
            notification_events();
        }
        // Both window-service work and application work must wait for this owner to return.
        assert!(context.activate_window());
        if self.input.mode == "close" {
            unsafe {
                let hwnd = FindWindowW(None, w!("Nested application dispatch fixture"));
                let mut process = 0;
                GetWindowThreadProcessId(hwnd, Some(&mut process));
                assert_eq!(process, GetCurrentProcessId());
                PostMessageW(Some(hwnd), WM_CLOSE as u32, 0, 0).unwrap();
            }
        }
        pump();
        assert_eq!(observations.updates.get(), 0);
        observations.active.set(false);
    }

    fn view(&self, input: &Input, context: &mut ViewContext<Self>) -> View {
        context.window_frame(
            "Nested application dispatch fixture",
            format!("Phase {}", input.phase),
        )
    }
}

fn notification_events() {
    unsafe {
        let mut after = None;
        let hwnd = loop {
            let hwnd = FindWindowExW(None, after, None, w!("windows-notifyicon"));
            assert!(!hwnd.is_null());
            let mut process = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process));
            if process == GetCurrentProcessId() {
                break hwnd;
            }
            after = Some(hwnd);
        };
        let recovery = RegisterWindowMessageW(w!("TaskbarCreated"));
        assert_ne!(recovery, 0);
        SendMessageW(hwnd, recovery, 0, 0);
        SendMessageW(hwnd, WM_USER as u32 + 1, 0, 0x400);
    }
}

fn pump() {
    let deadline = Instant::now() + Duration::from_millis(200);
    while Instant::now() < deadline {
        unsafe {
            let mut message = MSG::default();
            if PeekMessageW(&mut message, None, 0, 0, PM_REMOVE as u32).as_bool() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            } else {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

impl Drop for NestedWindow {
    fn drop(&mut self) {
        assert!(!self.input.observations.active.get());
        self.input
            .observations
            .dropped
            .set(self.input.observations.dropped.get() + 1);
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Application dispatch fixture timed out");
        std::process::exit(1);
    });
    let mode = std::env::args().nth(1).unwrap_or_else(|| "input".into());
    assert!(matches!(
        mode.as_str(),
        "input" | "close" | "application" | "notification"
    ));
    let observations = Rc::new(Observations::default());
    App::run_application::<Model>((mode.clone(), Rc::clone(&observations)))?;
    assert_eq!(observations.updates.get(), 2);
    assert_eq!(observations.inputs.get(), u32::from(mode != "close"));
    assert_eq!(
        observations.component_updates.get(),
        if mode == "application" { 2 } else { 1 }
    );
    assert_eq!(observations.mounted.get(), 1);
    assert_eq!(observations.dropped.get(), 1);
    Ok(())
}
