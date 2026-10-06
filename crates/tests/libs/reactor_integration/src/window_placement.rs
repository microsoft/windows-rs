use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;
use windows::Win32::{HHOOK, RECT, processthreadsapi::GetCurrentThreadId, winuser::*};
use windows_core::PWSTR;
use windows_reactor::*;

struct FirstShow {
    rect: RECT,
    maximized: bool,
    title: String,
}

thread_local! {
    static FIRST_SHOW: RefCell<HashMap<usize, FirstShow>> = RefCell::default();
}

struct ShowObserver(HHOOK);

impl ShowObserver {
    fn new() -> Self {
        let hook = unsafe {
            SetWindowsHookExW(
                WH_CALLWNDPROCRET,
                Some(observe_show),
                None,
                GetCurrentThreadId(),
            )
        };
        assert!(!hook.is_null());
        Self(hook)
    }
}

impl Drop for ShowObserver {
    fn drop(&mut self) {
        unsafe { UnhookWindowsHookEx(self.0).ok().unwrap() };
    }
}

unsafe extern "system" fn observe_show(code: i32, wparam: usize, lparam: isize) -> isize {
    if code >= 0 {
        let message = unsafe { &*(lparam as *const CWPRETSTRUCT) };
        if message.message == WM_NCDESTROY as u32 {
            FIRST_SHOW.with(|shows| shows.borrow_mut().remove(&(message.hwnd as usize)));
        } else if message.message == WM_WINDOWPOSCHANGED as u32
            && unsafe { GetWindowLongW(message.hwnd, GWL_STYLE) } & WS_CHILD == 0
            && unsafe { IsWindowVisible(message.hwnd) }.as_bool()
        {
            FIRST_SHOW.with(|shows| {
                shows
                    .borrow_mut()
                    .entry(message.hwnd as usize)
                    .or_insert_with(|| {
                        let mut rect = RECT::default();
                        let mut title = [0u16; 128];
                        unsafe { GetWindowRect(message.hwnd, &mut rect).ok().unwrap() };
                        let len =
                            unsafe { GetWindowTextW(message.hwnd, PWSTR(title.as_mut_ptr()), 128) };
                        FirstShow {
                            rect,
                            maximized: unsafe { IsZoomed(message.hwnd) }.as_bool(),
                            title: String::from_utf16_lossy(&title[..len as usize]),
                        }
                    });
            });
        }
    }
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

#[derive(Clone, Copy, PartialEq)]
enum Case {
    Normal,
    Maximized,
    Position,
    Offscreen,
    SameBounds,
    Constrained,
}

fn initial(case: Case) -> WindowPlacement {
    WindowPlacement {
        x: if case == Case::Offscreen { 32000 } else { 160 },
        y: if case == Case::Offscreen { 32000 } else { 140 },
        width: 640,
        height: 480,
        maximized: case == Case::Maximized,
    }
}

struct Fixture {
    case: Case,
    phase: u8,
    expected: Option<WindowPlacement>,
    _timeout: ComponentTimer,
    delay: Option<ComponentTimer>,
}

enum Message {
    Placement(WindowPlacement),
    NativeOperation,
    Minimized,
    Restored,
    Replaced(WindowPlacement),
    MoveUnobserved,
    Resubscribe,
    PreparedBounds(WindowPlacement),
    MaximizeSameBounds,
    Timeout,
}

impl Component for Fixture {
    type Input = Case;
    type Message = Message;

    fn create(case: &Case, context: &ComponentContext<Self>) -> Self {
        Self {
            case: *case,
            phase: 0,
            expected: None,
            _timeout: context.set_timeout(Duration::from_secs(10), Message::Timeout),
            delay: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Placement(placement) => match self.phase {
                0 => {
                    let expected = initial(self.case);
                    match self.case {
                        Case::Normal | Case::Maximized | Case::SameBounds => {
                            assert_eq!(placement, expected);
                        }
                        Case::Position => {
                            assert_eq!((placement.x, placement.y), (expected.x, expected.y));
                        }
                        Case::Offscreen => assert!(placement.x < 32000 && placement.y < 32000),
                        Case::Constrained => assert!(placement.width >= 800),
                    }
                    let position_only = self.case == Case::Position;
                    let constrained = self.case == Case::Constrained;
                    let mut moved = placement;
                    moved.x += 40;
                    moved.y += 30;
                    moved.maximized = false;
                    self.expected = Some(moved);
                    self.phase = 1;
                    assert!(context.run_window(move |window| {
                        let hwnd = window.as_raw().cast();
                        if position_only {
                            assert_client_size(hwnd, 400, 240);
                        }
                        if constrained {
                            let mut client = RECT::default();
                            unsafe { GetClientRect(hwnd, &mut client).ok().unwrap() };
                            let dpi = unsafe { GetDpiForWindow(hwnd) } as i32;
                            assert!(client.right >= 800 * dpi / 96);
                        }
                        FIRST_SHOW.with(|shows| {
                            let shows = shows.borrow();
                            let first = shows.get(&(hwnd as usize)).unwrap();
                            assert_eq!(first.title, "Placement fixture");
                            assert_eq!(first.maximized, placement.maximized);
                            if !placement.maximized {
                                assert_eq!(first.rect.left, placement.x);
                                assert_eq!(first.rect.top, placement.y);
                                assert_eq!(first.rect.right - first.rect.left, placement.width);
                                assert_eq!(first.rect.bottom - first.rect.top, placement.height);
                            }
                        });
                        unsafe {
                            _ = ShowWindow(hwnd, SW_RESTORE);
                            SetWindowPos(
                                hwnd,
                                None,
                                moved.x,
                                moved.y,
                                moved.width,
                                moved.height,
                                (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
                            )
                            .ok()
                            .unwrap();
                        }
                        Message::NativeOperation
                    }));
                }
                1 => {
                    assert_eq!(Some(placement), self.expected);
                    self.phase = 2;
                    let same_bounds = self.case == Case::SameBounds;
                    assert!(context.run_window(move |window| {
                        let hwnd = window.as_raw().cast();
                        unsafe { _ = ShowWindow(hwnd, SW_SHOWMAXIMIZED) };
                        if same_bounds {
                            let mut rect = RECT::default();
                            unsafe {
                                GetWindowRect(hwnd, &mut rect).ok().unwrap();
                                _ = ShowWindow(hwnd, SW_RESTORE);
                                SetWindowPos(
                                    hwnd,
                                    None,
                                    rect.left,
                                    rect.top,
                                    rect.right - rect.left,
                                    rect.bottom - rect.top,
                                    (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
                                )
                                .ok()
                                .unwrap();
                            }
                            Message::PreparedBounds(WindowPlacement {
                                x: rect.left,
                                y: rect.top,
                                width: rect.right - rect.left,
                                height: rect.bottom - rect.top,
                                maximized: false,
                            })
                        } else {
                            Message::NativeOperation
                        }
                    }));
                }
                2 => {
                    assert_eq!(
                        placement,
                        WindowPlacement {
                            maximized: true,
                            ..self.expected.unwrap()
                        }
                    );
                    self.expected = Some(placement);
                    self.phase = 3;
                    assert!(context.run_window(|window| {
                        unsafe { _ = ShowWindow(window.as_raw().cast(), SW_MINIMIZE) };
                        Message::NativeOperation
                    }));
                    self.delay =
                        Some(context.set_timeout(Duration::from_millis(150), Message::Minimized));
                }
                7 => {
                    assert_eq!(Some(placement), self.expected);
                    self.phase = 8;
                }
                8 => {
                    assert!(!placement.maximized);
                    self.phase = 9;
                    assert!(context.run_window(|window| {
                        assert_client_size(window.as_raw().cast(), 500, 300);
                        Message::NativeOperation
                    }));
                    assert!(context.close_window());
                }
                10 => assert_eq!(Some(placement), self.expected),
                _ => panic!(
                    "unexpected placement notification in phase {}: {placement:?}",
                    self.phase
                ),
            },
            Message::Minimized => {
                assert_eq!(self.phase, 3);
                self.phase = 4;
                assert!(context.run_window(|window| {
                    unsafe { _ = ShowWindow(window.as_raw().cast(), SW_RESTORE) };
                    Message::NativeOperation
                }));
                self.delay =
                    Some(context.set_timeout(Duration::from_millis(150), Message::Restored));
            }
            Message::Restored => {
                assert_eq!(self.phase, 4);
                self.phase = 5;
                assert!(context.run_window(|window| {
                    assert!(unsafe { IsZoomed(window.as_raw().cast()) }.as_bool());
                    Message::NativeOperation
                }));
            }
            Message::Replaced(placement) => {
                assert_eq!(self.phase, 5);
                assert_eq!(Some(placement), self.expected);
                self.phase = 6;
                self.delay =
                    Some(context.set_timeout(Duration::from_millis(150), Message::MoveUnobserved));
            }
            Message::MoveUnobserved => {
                assert_eq!(self.phase, 6);
                let expected = self.expected.as_mut().unwrap();
                expected.x += 20;
                expected.y += 20;
                expected.maximized = false;
                let expected = *expected;
                assert!(context.run_window(move |window| {
                    let hwnd = window.as_raw().cast();
                    unsafe {
                        _ = ShowWindow(hwnd, SW_RESTORE);
                        SetWindowPos(
                            hwnd,
                            None,
                            expected.x,
                            expected.y,
                            expected.width,
                            expected.height,
                            (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
                        )
                        .ok()
                        .unwrap();
                    }
                    Message::NativeOperation
                }));
                self.delay =
                    Some(context.set_timeout(Duration::from_millis(150), Message::Resubscribe));
            }
            Message::Resubscribe => {
                assert_eq!(self.phase, 6);
                self.phase = 7;
            }
            Message::PreparedBounds(placement) => {
                self.expected = Some(placement);
                self.phase = 10;
                self.delay = Some(
                    context.set_timeout(Duration::from_millis(150), Message::MaximizeSameBounds),
                );
            }
            Message::MaximizeSameBounds => {
                assert_eq!(self.phase, 10);
                self.phase = 2;
                assert!(context.run_window(|window| {
                    let hwnd = window.as_raw().cast();
                    let mut before = RECT::default();
                    let mut after = RECT::default();
                    unsafe {
                        assert!(!IsZoomed(hwnd).as_bool());
                        GetWindowRect(hwnd, &mut before).ok().unwrap();
                        _ = ShowWindow(hwnd, SW_SHOWMAXIMIZED);
                        GetWindowRect(hwnd, &mut after).ok().unwrap();
                        assert!(IsZoomed(hwnd).as_bool());
                    }
                    assert_eq!(before, after);
                    Message::NativeOperation
                }));
            }
            Message::NativeOperation => {}
            Message::Timeout => panic!("placement fixture stalled in phase {}", self.phase),
        }
    }

    fn view(&self, case: &Case, context: &mut ViewContext<Self>) -> View {
        context.window_title("Placement fixture");
        match self.phase {
            5 => context.on_window_placement(context.callback(Message::Replaced)),
            6 | 9 => {}
            _ => context.on_window_placement(context.callback(Message::Placement)),
        }
        let mut visuals = if matches!(self.phase, 8 | 9) {
            WindowVisuals::new().client_size(500.0, 300.0)
        } else {
            WindowVisuals::new().client_size(400.0, 240.0)
        };
        if self.phase == 0 {
            let expected = initial(*case);
            if *case == Case::Constrained {
                visuals = visuals.constraints(WindowConstraints {
                    min_width: Some(800.0),
                    ..Default::default()
                });
            }
            visuals = if *case == Case::Position {
                visuals.initial_position(ScreenPoint::new(expected.x, expected.y))
            } else {
                visuals
                    .initial_position(ScreenPoint::new(40, 40))
                    .initial_placement(expected)
            };
        } else {
            visuals = visuals.initial_placement(WindowPlacement {
                x: 800,
                y: 600,
                width: 900,
                height: 700,
                maximized: false,
            });
        }
        context.window_visuals(visuals);
        TextBlock::new().text("Placement fixture").into()
    }
}

fn assert_client_size(hwnd: windows::Win32::HWND, width: i32, height: i32) {
    let mut client = RECT::default();
    unsafe { GetClientRect(hwnd, &mut client).ok().unwrap() };
    let dpi = unsafe { GetDpiForWindow(hwnd) } as i32;
    assert_eq!(client.right, width * dpi / 96);
    assert_eq!(client.bottom, height * dpi / 96);
}

struct Launcher;

impl Component for Launcher {
    type Input = ();
    type Message = ();

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        assert!(
            context.open_window_with_policy::<Fixture>(
                Case::Maximized,
                WindowPolicy::new()
                    .client_size(250.0, 180.0)
                    .minimum_client_size(900.0, 600.0),
            )
        );
        assert!(context.close_window());
        Self
    }

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        "Launching secondary window".into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        eprintln!("window placement fixture timed out");
        std::process::exit(1);
    });
    let _observer = ShowObserver::new();
    match std::env::args().nth(1).as_deref() {
        Some("normal") => App::run_component::<Fixture>(Case::Normal),
        Some("maximized") => App::run_component::<Fixture>(Case::Maximized),
        Some("position") => App::run_component::<Fixture>(Case::Position),
        Some("offscreen") => App::run_component::<Fixture>(Case::Offscreen),
        Some("secondary") => App::run_component::<Launcher>(()),
        Some("same-bounds") => App::run_component::<Fixture>(Case::SameBounds),
        Some("constrained") => App::run_component::<Fixture>(Case::Constrained),
        _ => panic!("expected a window placement case"),
    }
}
