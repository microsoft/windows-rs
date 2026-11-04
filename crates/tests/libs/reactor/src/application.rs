use super::*;
use crate::bindings::*;
use std::cell::{Cell, RefCell};

pub(super) struct ApplicationFixture {
    value: u32,
    mounted: u32,
    closed: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Show,
    Increment,
    Mounted,
    Closed,
}

impl Application for ApplicationFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self {
            value: 0,
            mounted: 0,
            closed: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Show => {
                context.show_window("secondary");
                context.show_window("secondary");
            }
            Message::Increment => self.value += 1,
            Message::Mounted => self.mounted += 1,
            Message::Closed => self.closed += 1,
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        let input = |main| WindowInput {
            main,
            value: self.value,
            mounted: self.mounted,
            closed: self.closed,
            on_show: context.message(Message::Show),
            on_increment: context.message(Message::Increment),
            on_mounted: context.message(Message::Mounted),
            on_closed: context.message(Message::Closed),
        };
        ApplicationView::new()
            .window::<ApplicationWindow>("main", input(true))
            .window::<ApplicationWindow>("secondary", input(false))
    }
}

#[derive(Clone, PartialEq)]
struct WindowInput {
    main: bool,
    value: u32,
    mounted: u32,
    closed: u32,
    on_show: Callback<()>,
    on_increment: Callback<()>,
    on_mounted: Callback<()>,
    on_closed: Callback<()>,
}

struct ApplicationWindow(Option<Callback<()>>);

impl Drop for ApplicationWindow {
    fn drop(&mut self) {
        if let Some(closed) = &self.0 {
            closed.call(());
        }
    }
}

impl Component for ApplicationWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(input: &WindowInput, _: &ComponentContext<Self>) -> Self {
        if !input.main {
            input.on_mounted.call(());
        }
        Self((!input.main).then(|| input.on_closed.clone()))
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        assert!(context.close_window());
    }

    fn view(&self, input: &WindowInput, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        if !input.main {
            context.window_title("Reactor secondary fixture");
            return StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .children((
                    TextBlock::new()
                        .automation_id("secondary-value")
                        .text(format!("Value {}", input.value)),
                    Button::new()
                        .automation_id("close-secondary")
                        .content("Close")
                        .on_click(context.message(())),
                ))
                .into();
        }
        context.window_title("Reactor UI fixture: application");
        let show = input.on_show.clone();
        let increment = input.on_increment.clone();
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("show")
                    .content("Show secondary twice")
                    .on_click(move || show.call(())),
                Button::new()
                    .automation_id("increment")
                    .content("Increment")
                    .on_click(move || increment.call(())),
                TextBlock::new().automation_id("lifetime").text(format!(
                    "Mounted {}; closed {}",
                    input.mounted, input.closed
                )),
            ))
            .into()
    }
}

pub(super) struct ReplacingWindow(u32);

#[derive(Clone, Copy)]
pub(super) enum ReplacementMessage {
    Replace,
    Ping,
}

impl Component for ReplacingWindow {
    type Input = bool;
    type Message = ReplacementMessage;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, message: ReplacementMessage, context: &ComponentContext<Self>) {
        match message {
            ReplacementMessage::Replace => {
                assert!(context.open_window::<Self>(true));
                assert!(context.close_window());
            }
            ReplacementMessage::Ping => self.0 += 1,
        }
    }

    fn view(&self, replacement: &bool, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: window-replacement");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        if !replacement {
            return Button::new()
                .automation_id("replace-window")
                .content("Replace last window")
                .on_click(context.message(ReplacementMessage::Replace))
                .into();
        }
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("ping")
                    .content("Ping replacement")
                    .on_click(context.message(ReplacementMessage::Ping)),
                TextBlock::new()
                    .automation_id("replacement")
                    .text(format!("Updates {}", self.0)),
            ))
            .into()
    }
}

/// Exits from application state while its window is open. Teardown must close the window and
/// `run_application` must return.
pub(super) struct ExitFixture;

impl Application for ExitFixture {
    type Input = ();
    type Message = ();

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self
    }

    fn update(&mut self, _: (), context: &ApplicationContext<Self>) {
        context.exit();
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new().window::<ExitWindow>("main", context.message(()))
    }
}

struct ExitWindow;

impl Component for ExitWindow {
    type Input = Callback<()>;
    type Message = ();

    fn create(exit: &Callback<()>, _: &ComponentContext<Self>) -> Self {
        exit.call(());
        Self
    }

    fn view(&self, _: &Callback<()>, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: exit-with-window");
        TextBlock::new().text("Exiting").into()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum HostingMode {
    Normal,
    Maximized,
    CloseBeforeShow,
    Hidden,
    Queued,
}

#[derive(Default)]
struct WindowProbe {
    hwnd: Cell<HWND>,
    first_show: Cell<Option<(RECT, bool)>>,
    destroyed: Cell<bool>,
    dropped: Cell<bool>,
    delivered_mount_message: Cell<bool>,
}

struct HostingLog {
    mode: HostingMode,
    windows: RefCell<Vec<Rc<WindowProbe>>>,
    ready: RefCell<Vec<usize>>,
    shown: RefCell<Vec<usize>>,
    progress: Cell<bool>,
    progress_sender: RefCell<Option<LocalSender<HostingMessage>>>,
}

pub(super) fn run_hosting(mode: HostingMode) -> windows_core::Result<()> {
    let log = Rc::new(HostingLog {
        mode,
        windows: RefCell::new(Vec::new()),
        ready: RefCell::new(Vec::new()),
        shown: RefCell::new(Vec::new()),
        progress: Cell::new(false),
        progress_sender: RefCell::new(None),
    });
    if mode == HostingMode::Hidden {
        App::run_application::<HostingApplication>(Rc::clone(&log))?;
        assert_eq!(*log.ready.borrow(), [1, 2]);
        assert_eq!(*log.shown.borrow(), [1]);
    } else {
        App::run_component::<HostingWindow>(HostingInput {
            log: Rc::clone(&log),
            events: None,
        })?;
    }
    let windows = log.windows.borrow();
    assert_eq!(
        windows.len(),
        match mode {
            HostingMode::Hidden => 2,
            HostingMode::Queued => 3,
            _ => 1,
        }
    );
    for (index, probe) in windows.iter().enumerate() {
        assert!(probe.destroyed.get());
        assert!(probe.dropped.get());
        assert!(!unsafe { IsWindow(probe.hwnd.get()) }.as_bool());
        let shown =
            mode != HostingMode::CloseBeforeShow && (index == 0 || mode == HostingMode::Queued);
        assert_eq!(probe.first_show.get().is_some(), shown);
    }
    Ok(())
}

fn assert_restored_bounds(hwnd: HWND) {
    let mut bounds = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut bounds) }.unwrap();
    assert_eq!(
        (bounds.left, bounds.top, bounds.right, bounds.bottom),
        (80, 100, 720, 580)
    );
}

unsafe extern "system" fn observe_presentation(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    id: usize,
    data: usize,
) -> LRESULT {
    // The fixture log retains every probe until the application has returned.
    let probe = unsafe { &*(data as *const WindowProbe) };
    if message == WM_NCDESTROY as u32 {
        probe.destroyed.set(true);
        assert!(unsafe { RemoveWindowSubclass(hwnd, Some(observe_presentation), id) }.as_bool());
    }
    let result = unsafe { DefSubclassProc(hwnd, message, wparam, lparam) };
    if message == WM_WINDOWPOSCHANGED as u32
        && probe.first_show.get().is_none()
        && unsafe { IsWindowVisible(hwnd) }.as_bool()
    {
        let mut bounds = RECT::default();
        unsafe { GetWindowRect(hwnd, &mut bounds) }.unwrap();
        probe
            .first_show
            .set(Some((bounds, unsafe { IsZoomed(hwnd) }.as_bool())));
    }
    result
}

#[derive(Clone)]
struct HostingInput {
    log: Rc<HostingLog>,
    events: Option<Callback<HostingEvent>>,
}

impl PartialEq for HostingInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.log, &other.log) && self.events == other.events
    }
}

struct HostingWindow {
    input: HostingInput,
    probe: Rc<WindowProbe>,
    ordinal: usize,
    presented: bool,
}

enum HostingMessage {
    Progress,
    Stop,
    DuringMount,
    Configured,
    Presented,
}

#[derive(Clone, Copy)]
enum HostingEvent {
    Ready(usize),
    Shown(usize),
}

impl Component for HostingWindow {
    type Input = HostingInput;
    type Message = HostingMessage;

    fn create(input: &HostingInput, context: &ComponentContext<Self>) -> Self {
        let probe = Rc::new(WindowProbe::default());
        input.log.windows.borrow_mut().push(Rc::clone(&probe));
        let ordinal = input.log.windows.borrow().len();
        if input.log.mode == HostingMode::Queued && ordinal == 2 {
            *input.log.progress_sender.borrow_mut() = Some(context.sender());
        }
        let observe = Rc::clone(&probe);
        let log = Rc::clone(&input.log);
        assert!(context.sender().send(HostingMessage::DuringMount));
        if input.log.mode == HostingMode::Normal {
            assert!(context.activate_window());
        }
        assert!(context.run_window(move |window| {
            let hwnd = window.as_raw().cast();
            assert!(unsafe { IsWindow(hwnd) }.as_bool());
            assert!(!unsafe { IsWindowVisible(hwnd) }.as_bool());
            if log.mode == HostingMode::Queued && ordinal == 3 {
                assert!(log.windows.borrow()[0].destroyed.get());
            }
            assert_restored_bounds(hwnd);
            if log.mode == HostingMode::Normal {
                unsafe {
                    SetWindowPos(
                        hwnd,
                        std::ptr::null_mut(),
                        120,
                        140,
                        640,
                        480,
                        (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
                    )
                }
                .unwrap();
            }
            observe.hwnd.set(hwnd);
            assert!(
                unsafe {
                    SetWindowSubclass(
                        hwnd,
                        Some(observe_presentation),
                        1,
                        Rc::as_ptr(&observe) as usize,
                    )
                }
                .as_bool()
            );
            let mut message = MSG::default();
            for _ in 0..32 {
                if !unsafe {
                    PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE as u32)
                }
                .as_bool()
                {
                    break;
                }
                assert_ne!(message.message, WM_QUIT as u32);
                unsafe { DispatchMessageW(&message) };
            }
            assert!(!observe.delivered_mount_message.get());
            if log.mode == HostingMode::Queued && ordinal == 3 {
                assert!(
                    log.progress_sender
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .send(HostingMessage::Progress)
                );
            }
            HostingMessage::Configured
        }));
        if input.log.mode == HostingMode::Queued && ordinal == 3 {
            for index in 0..64 {
                let probe = Rc::clone(&probe);
                let log = Rc::clone(&input.log);
                assert!(context.run_window(move |window| {
                    assert!(!unsafe { IsWindowVisible(window.as_raw().cast()) }.as_bool());
                    assert!(!probe.delivered_mount_message.get());
                    if index == 63 {
                        assert!(
                            log.progress.get(),
                            "initial work monopolized Reactor's drivers"
                        );
                    }
                    HostingMessage::DuringMount
                }));
            }
        }
        if input.log.mode == HostingMode::CloseBeforeShow {
            assert!(context.close_window());
            assert!(context.run_window(|_| panic!("initial work ran after closure")));
            assert!(context.activate_window());
        }
        Self {
            input: input.clone(),
            probe,
            ordinal,
            presented: false,
        }
    }

    fn update(&mut self, message: HostingMessage, context: &ComponentContext<Self>) {
        match message {
            HostingMessage::Progress => {
                assert_eq!(self.ordinal, 2);
                self.input.log.progress.set(true);
            }
            HostingMessage::Stop => {
                assert_eq!(self.ordinal, 2);
                self.close(context);
            }
            HostingMessage::DuringMount => {
                assert!(!self.probe.hwnd.get().is_null());
                self.probe.delivered_mount_message.set(true);
            }
            HostingMessage::Configured => {
                if self.input.log.mode == HostingMode::Hidden {
                    assert!(!unsafe { IsWindowVisible(self.probe.hwnd.get()) }.as_bool());
                    self.input
                        .events
                        .as_ref()
                        .unwrap()
                        .call(HostingEvent::Ready(self.ordinal));
                    if self.ordinal == 2 {
                        self.close(context);
                    }
                } else if self.input.log.mode != HostingMode::CloseBeforeShow {
                    self.check_presentation();
                    if self.input.log.mode == HostingMode::Queued && self.ordinal == 1 {
                        assert!(context.open_window::<Self>(self.input.clone()));
                        assert!(context.open_window::<Self>(self.input.clone()));
                    }
                    if self.input.log.mode == HostingMode::Queued {
                        if self.ordinal == 2 {
                            return;
                        }
                        if self.ordinal == 3 {
                            assert!(
                                self.input
                                    .log
                                    .progress_sender
                                    .borrow()
                                    .as_ref()
                                    .unwrap()
                                    .send(HostingMessage::Stop)
                            );
                        }
                    }
                    if self.input.log.mode == HostingMode::Maximized {
                        assert!(unsafe { ShowWindow(self.probe.hwnd.get(), SW_RESTORE) }.as_bool());
                        assert_restored_bounds(self.probe.hwnd.get());
                    }
                    self.close(context);
                }
            }
            HostingMessage::Presented => {
                if self.input.log.mode == HostingMode::Hidden && !self.presented {
                    self.presented = true;
                    assert_eq!(self.ordinal, 1);
                    self.check_presentation();
                    self.input
                        .events
                        .as_ref()
                        .unwrap()
                        .call(HostingEvent::Shown(self.ordinal));
                    self.close(context);
                }
            }
        }
    }

    fn view(&self, _: &HostingInput, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().initial_placement(WindowPlacement {
            x: 80,
            y: 100,
            width: 640,
            height: 480,
            maximized: self.input.log.mode == HostingMode::Maximized,
        }));
        context.on_window_placement(context.callback(|_| HostingMessage::Presented));
        context.window_frame("Reactor hosting fixture", TextBlock::new().text("Hosting"))
    }
}

impl HostingWindow {
    fn check_presentation(&self) {
        assert!(unsafe { IsWindowVisible(self.probe.hwnd.get()) }.as_bool());
        let (bounds, maximized) = self.probe.first_show.get().unwrap();
        assert_eq!(maximized, self.input.log.mode == HostingMode::Maximized);
        if !maximized {
            assert_eq!(
                (bounds.left, bounds.top, bounds.right, bounds.bottom),
                if self.input.log.mode == HostingMode::Normal {
                    (120, 140, 760, 620)
                } else {
                    (80, 100, 720, 580)
                }
            );
        }
    }

    fn close(&self, context: &ComponentContext<Self>) {
        assert!(context.close_window());
        assert!(
            context.run_window(|_| panic!("native work reached a closed or replacement window"))
        );
        assert!(context.activate_window());
    }
}

impl Drop for HostingWindow {
    fn drop(&mut self) {
        assert!(self.probe.destroyed.get());
        assert!(!unsafe { IsWindow(self.probe.hwnd.get()) }.as_bool());
        self.probe.dropped.set(true);
    }
}

struct HostingApplication(Rc<HostingLog>);

impl Application for HostingApplication {
    type Input = Rc<HostingLog>;
    type Message = HostingEvent;

    fn create(log: &Rc<HostingLog>, context: &ApplicationContext<Self>) -> Self {
        context.create_window("main");
        context.create_window("main");
        Self(Rc::clone(log))
    }

    fn update(&mut self, message: HostingEvent, context: &ApplicationContext<Self>) {
        match message {
            HostingEvent::Ready(ordinal) => {
                self.0.ready.borrow_mut().push(ordinal);
                match ordinal {
                    1 => {
                        context.create_window("main");
                        context.show_window("main");
                        context.show_window("main");
                    }
                    2 => assert!(self.0.windows.borrow()[0].dropped.get()),
                    _ => panic!("unexpected window instance"),
                }
            }
            HostingEvent::Shown(ordinal) => {
                assert_eq!(ordinal, 1);
                self.0.shown.borrow_mut().push(ordinal);
                context.create_window("main");
                context.create_window("main");
            }
        }
    }

    fn view(&self, _: &Rc<HostingLog>, context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new().window::<HostingWindow>(
            "main",
            HostingInput {
                log: Rc::clone(&self.0),
                events: Some(context.forward()),
            },
        )
    }
}
