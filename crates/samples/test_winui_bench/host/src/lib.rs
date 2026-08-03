use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows::Win32::{DispatchMessageW, GetMessageW, MSG, PostQuitMessage, TranslateMessage};
use windows_core::{Error, HRESULT};
use windows_reactor::{App, DispatcherTimer, on_rendering};

#[unsafe(no_mangle)]
pub extern "system" fn run(
    launch: extern "system" fn() -> i32,
    tick: extern "system" fn(),
    frame: extern "system" fn(),
    finish: extern "system" fn(),
    duration_ms: u32,
    settle_ms: u32,
) -> i32 {
    match App::new().run_custom(move |_| {
        let result = launch();
        if result < 0 {
            unsafe { PostQuitMessage(0) };
        }

        let running = Rc::new(Cell::new(result >= 0));
        let running_for_frame = running.clone();
        let _rendering = (result >= 0)
            .then(|| {
                on_rendering(move || {
                    if running_for_frame.get() {
                        frame();
                    }
                })
            })
            .transpose()?;
        let running_for_tick = running.clone();
        let _tick = (result >= 0)
            .then(|| {
                DispatcherTimer::new(Duration::from_millis(33), move || {
                    if running_for_tick.get() {
                        tick();
                    }
                })
            })
            .transpose()?;
        let running_for_finish = running;
        let _finish = (result >= 0)
            .then(|| {
                DispatcherTimer::new_one_shot(
                    Duration::from_millis(duration_ms.into()),
                    move || {
                        running_for_finish.set(false);
                        finish();
                    },
                )
            })
            .transpose()?;
        let _quit = (result >= 0)
            .then(|| {
                DispatcherTimer::new_one_shot(
                    Duration::from_millis(duration_ms.saturating_add(settle_ms.max(1)).into()),
                    || unsafe { PostQuitMessage(0) },
                )
            })
            .transpose()?;

        unsafe {
            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).0 > 0 {
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        if result < 0 {
            Err(Error::from_hresult(HRESULT(result)))
        } else {
            Ok(())
        }
    }) {
        Ok(()) => 0,
        Err(error) => error.code().0,
    }
}
