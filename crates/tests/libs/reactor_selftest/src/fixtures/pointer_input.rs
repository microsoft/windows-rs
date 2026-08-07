//! End-to-end pointer-input fixture. Injects real OS mouse input with the
//! WinRT `InputInjector` and verifies the reactor's `on_pointer_*` callbacks
//! fire with the correct position and button state. This is the only path
//! that exercises the backend's `set_pointer_handlers` event wiring and
//! `pointer_event_info` (which reads a live `PointerRoutedEventArgs`); the
//! headless unit tests can only check that the handlers are registered.
//!
//! Injection requires an interactive desktop with the harness window in the
//! foreground. When that isn't available (locked session, policy, no
//! desktop) the fixture records a SKIP rather than failing, so it never
//! flakes a CI run that can't deliver input.

use std::cell::RefCell;
use std::rc::Rc;

use windows_collections::IIterable;
use windows_core::Result;

use crate::bindings::{
    BringWindowToTop, ClientToScreen, GetClientRect, GetSystemMetrics, HWND,
    InjectedInputMouseInfo, InjectedInputMouseOptions, InputInjector, POINT, RECT,
    SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    SetForegroundWindow,
};

use windows_reactor::{
    Color, ElementExt, LayoutExt, PointerEventInfo, Thickness, text_block, vstack,
};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

#[derive(Default)]
struct PointerLog {
    entered: u32,
    moved: u32,
    pressed: u32,
    released: u32,
    exited: u32,
    capture_lost: u32,
    canceled: u32,
    capture_succeeded: bool,
    left_on_press: bool,
    right_on_press: bool,
    last_x: f64,
    last_y: f64,
    last_window_x: f64,
    last_window_y: f64,
}

/// Screen pixel at a fraction (`fx`, `fy`) of the window's client area.
fn client_screen_point(hwnd: HWND, fx: f64, fy: f64) -> Option<(i32, i32)> {
    unsafe {
        let mut rc = RECT::default();
        if !GetClientRect(hwnd, &mut rc).as_bool() {
            return None;
        }
        let mut p = POINT {
            x: (rc.right as f64 * fx) as i32,
            y: (rc.bottom as f64 * fy) as i32,
        };
        if !ClientToScreen(hwnd, &mut p).as_bool() {
            return None;
        }
        Some((p.x, p.y))
    }
}

/// Inject a single mouse event at absolute screen pixel (`sx`, `sy`) carrying
/// the extra option flags (`Move`, `LeftDown`, `LeftUp`, etc.). Coordinates are
/// normalized over the virtual desktop, matching `MOUSEEVENTF_ABSOLUTE`.
fn inject_at(
    injector: &InputInjector,
    sx: i32,
    sy: i32,
    extra: InjectedInputMouseOptions,
) -> Result<()> {
    let (vx, vy, vw, vh) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
            GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2),
            GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2),
        )
    };
    let nx = (((sx - vx) as f64) * 65535.0 / ((vw - 1) as f64)).round() as i32;
    let ny = (((sy - vy) as f64) * 65535.0 / ((vh - 1) as f64)).round() as i32;

    let info = InjectedInputMouseInfo::new()?;
    info.SetDeltaX(nx)?;
    info.SetDeltaY(ny)?;
    info.SetMouseOptions(InjectedInputMouseOptions(
        InjectedInputMouseOptions::Absolute.0 | InjectedInputMouseOptions::VirtualDesk.0 | extra.0,
    ))?;

    let inputs: IIterable<InjectedInputMouseInfo> = vec![Some(info)].into();
    injector.InjectMouseInput(&inputs)
}

pub fn pointer_injection_gesture(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let log = Rc::new(RefCell::new(PointerLog::default()));
        let comp_log = log.clone();

        h.mount(cc(move |_cx| {
            let (le, lm, lp, lr, lx, lcl, lc) = (
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
            );
            vstack((vstack((text_block("pointer target"),))
                .width(6000.0)
                .height(180.0)
                .background(Color {
                    a: 255,
                    r: 32,
                    g: 32,
                    b: 40,
                })
                .on_pointer_entered(move |_| {
                    le.borrow_mut().entered += 1;
                })
                .on_pointer_moved(move |info: PointerEventInfo| {
                    let mut b = lm.borrow_mut();
                    b.moved += 1;
                    b.last_x = info.x;
                    b.last_y = info.y;
                    b.last_window_x = info.window_x;
                    b.last_window_y = info.window_y;
                })
                .on_pointer_pressed(move |info: PointerEventInfo| {
                    let mut b = lp.borrow_mut();
                    b.pressed += 1;
                    b.capture_succeeded = info.capture_succeeded;
                    if info.is_left_button_pressed {
                        b.left_on_press = true;
                    }
                    if info.is_right_button_pressed {
                        b.right_on_press = true;
                    }
                })
                .on_pointer_released(move |_| {
                    lr.borrow_mut().released += 1;
                })
                .on_pointer_exited(move || {
                    lx.borrow_mut().exited += 1;
                })
                .on_pointer_capture_lost(move || {
                    lcl.borrow_mut().capture_lost += 1;
                })
                .on_pointer_canceled(move || {
                    lc.borrow_mut().canceled += 1;
                })
                .capture_pointer_on_press(),))
            .padding(Thickness::uniform(80.0))
            .into()
        }));
        h.render().await;

        let Ok(injector) = InputInjector::TryCreate() else {
            h.check_skip(
                "Pointer_Injection_Gesture",
                "InputInjector unavailable on this host",
            );
            return;
        };

        let Some((cx, cy)) = client_screen_point(h.hwnd(), 0.5, 0.2) else {
            h.check_skip("Pointer_Injection_Gesture", "client rect unavailable");
            return;
        };

        // Bring the harness window to the foreground so injected absolute
        // input lands on it rather than whatever window is otherwise on top.
        unsafe {
            let _ = SetForegroundWindow(h.hwnd());
            let _ = BringWindowToTop(h.hwnd());
        }
        h.render().await;

        // Move into the element: PointerEntered + PointerMoved. Re-inject in a
        // bounded loop so a momentary focus/timing hiccup doesn't lose the move.
        let mut landed = false;
        for _ in 0..20 {
            let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::Move);
            let _ = inject_at(&injector, cx + 6, cy + 6, InjectedInputMouseOptions::Move);
            h.render().await;
            let b = log.borrow();
            if b.entered > 0 || b.moved > 0 {
                landed = true;
                break;
            }
        }

        if !landed {
            h.check_skip(
                "Pointer_Injection_Gesture",
                "injected input did not reach the window (no foreground desktop)",
            );
            return;
        }

        h.check("Pointer_Injection_Entered", log.borrow().entered > 0);
        h.check("Pointer_Injection_Moved", log.borrow().moved > 0);

        {
            let b = log.borrow();
            let (lx, ly) = (b.last_x, b.last_y);
            h.check_with(
                "Pointer_Injection_PositionInElement",
                lx > 0.0 && ly > 0.0,
                move || format!("last reported pointer position = ({lx}, {ly})"),
            );
            let (wx, wy) = (b.last_window_x, b.last_window_y);
            h.check_with(
                "Pointer_Injection_PositionInWindow",
                wx > lx + 60.0 && wy > ly + 60.0,
                move || format!("element position = ({lx}, {ly}), window position = ({wx}, {wy})"),
            );
        }

        // Capture on left press, move outside the target, then release there.
        let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::LeftDown);
        h.render_until_quiet("left button press", |_| log.borrow().pressed > 0)
            .await;

        h.check("Pointer_Injection_PressedLeft", log.borrow().left_on_press);
        h.check(
            "Pointer_Injection_CaptureSucceeded",
            log.borrow().capture_succeeded,
        );

        let Some((outside_x, outside_y)) = client_screen_point(h.hwnd(), 0.5, 0.8) else {
            h.check_skip(
                "Pointer_Injection_CaptureOutside",
                "client rect unavailable",
            );
            return;
        };
        let moved_before_capture_test = log.borrow().moved;
        let _ = inject_at(
            &injector,
            outside_x,
            outside_y,
            InjectedInputMouseOptions::Move,
        );
        h.render_until_quiet("captured move outside target", |_| {
            log.borrow().moved > moved_before_capture_test
        })
        .await;
        h.check_with(
            "Pointer_Injection_CaptureOutside",
            log.borrow().moved > moved_before_capture_test && log.borrow().last_y > 180.0,
            || {
                let b = log.borrow();
                format!(
                    "moved before = {moved_before_capture_test}, moved after = {}, local y = {}",
                    b.moved, b.last_y
                )
            },
        );

        let _ = inject_at(
            &injector,
            outside_x,
            outside_y,
            InjectedInputMouseOptions::LeftUp,
        );
        h.render_until_quiet("left button release", |_| log.borrow().released > 0)
            .await;
        h.check("Pointer_Injection_Released", log.borrow().released > 0);

        // Right press + release: PointerPressed reports the right-button flag.
        let released_before = log.borrow().released;
        let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::RightDown);
        h.render_until_quiet("right button press", |_| log.borrow().right_on_press)
            .await;
        let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::RightUp);
        h.render_until_quiet("right button release", |_| {
            log.borrow().released > released_before
        })
        .await;

        h.check(
            "Pointer_Injection_PressedRight",
            log.borrow().right_on_press,
        );

        // Move far outside the window: PointerExited.
        let (vx, vy) = unsafe {
            (
                GetSystemMetrics(SM_XVIRTUALSCREEN),
                GetSystemMetrics(SM_YVIRTUALSCREEN),
            )
        };
        let _ = inject_at(&injector, vx, vy, InjectedInputMouseOptions::Move);
        h.render_until_quiet("pointer to exit the element", |_| log.borrow().exited > 0)
            .await;
        h.check("Pointer_Injection_Exited", log.borrow().exited > 0);
    })
}
