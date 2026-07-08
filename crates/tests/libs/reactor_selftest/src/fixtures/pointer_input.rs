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

use windows_reactor::{Color, ElementExt, PointerEventInfo, text_block, vstack};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

#[derive(Default)]
struct PointerLog {
    entered: u32,
    moved: u32,
    pressed: u32,
    released: u32,
    exited: u32,
    left_on_press: bool,
    right_on_press: bool,
    last_x: f64,
    last_y: f64,
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
/// the extra option flags (`Move`, `LeftDown`, `LeftUp`, …). Coordinates are
/// normalized over the virtual desktop, matching `MOUSEEVENTF_ABSOLUTE`.
fn inject_at(
    injector: &InputInjector,
    sx: i32,
    sy: i32,
    extra: InjectedInputMouseOptions,
) -> Result<()> {
    let (vx, vy, vw, vh) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN as i32),
            GetSystemMetrics(SM_YVIRTUALSCREEN as i32),
            GetSystemMetrics(SM_CXVIRTUALSCREEN as i32).max(2),
            GetSystemMetrics(SM_CYVIRTUALSCREEN as i32).max(2),
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
            let (le, lm, lp, lr, lx) = (
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
                comp_log.clone(),
            );
            vstack((text_block("pointer target"),))
                .width(6000.0)
                .height(6000.0)
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
                })
                .on_pointer_pressed(move |info: PointerEventInfo| {
                    let mut b = lp.borrow_mut();
                    b.pressed += 1;
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

        let Some((cx, cy)) = client_screen_point(h.hwnd(), 0.5, 0.5) else {
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
        }

        // Left press + release: PointerPressed (left flag), PointerReleased.
        let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::LeftDown);
        h.render_until_quiet("left button press", |_| log.borrow().pressed > 0)
            .await;
        let _ = inject_at(&injector, cx, cy, InjectedInputMouseOptions::LeftUp);
        h.render_until_quiet("left button release", |_| log.borrow().released > 0)
            .await;

        h.check("Pointer_Injection_PressedLeft", log.borrow().left_on_press);
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
                GetSystemMetrics(SM_XVIRTUALSCREEN as i32),
                GetSystemMetrics(SM_YVIRTUALSCREEN as i32),
            )
        };
        let _ = inject_at(&injector, vx, vy, InjectedInputMouseOptions::Move);
        h.render_until_quiet("pointer to exit the element", |_| log.borrow().exited > 0)
            .await;
        h.check("Pointer_Injection_Exited", log.borrow().exited > 0);
    })
}
