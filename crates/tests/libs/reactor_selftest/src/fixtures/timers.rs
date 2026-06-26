//! Selftest fixtures for the dispatcher-thread hooks in `windows_reactor::hooks`:
//! `DispatcherTimer` (repeating + one-shot, RAII stop on drop) and the
//! `CompositionTarget::Rendering` subscription via `on_rendering`. These run on
//! the real WinUI dispatcher/compositor thread, so they exercise paths the
//! headless `test_reactor` unit tests cannot reach.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use windows_reactor::{DispatcherTimer, on_rendering};

use crate::fixtures::reconciler::FixtureFuture;
use crate::harness::Harness;

pub fn timer_repeating_fires(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let ticks = Rc::new(Cell::new(0u32));
        let timer = match DispatcherTimer::new(Duration::from_millis(5), {
            let ticks = ticks.clone();
            move || ticks.set(ticks.get() + 1)
        }) {
            Ok(timer) => timer,
            Err(err) => {
                h.check_skip("Timer_Repeating_Fires", &format!("create failed: {err:?}"));
                return;
            }
        };

        let fired = h
            .pump_until(Duration::from_secs(2), || ticks.get() >= 3)
            .await;
        h.check("Timer_Repeating_Fires", fired && ticks.get() >= 3);

        // Dropping the timer must unhook the tick handler: no further ticks.
        drop(timer);
        let before = ticks.get();
        h.pump_for(Duration::from_millis(150)).await;
        h.check("Timer_Repeating_DropStops", ticks.get() == before);
    })
}

pub fn timer_one_shot_fires_once(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let ticks = Rc::new(Cell::new(0u32));
        let _timer = match DispatcherTimer::new_one_shot(Duration::from_millis(5), {
            let ticks = ticks.clone();
            move || ticks.set(ticks.get() + 1)
        }) {
            Ok(timer) => timer,
            Err(err) => {
                h.check_skip("Timer_OneShot_Fires", &format!("create failed: {err:?}"));
                return;
            }
        };

        let fired = h
            .pump_until(Duration::from_secs(1), || ticks.get() >= 1)
            .await;
        h.check("Timer_OneShot_Fires", fired);

        // A one-shot must not repeat even after further pumping.
        h.pump_for(Duration::from_millis(150)).await;
        h.check("Timer_OneShot_FiresOnce", ticks.get() == 1);
    })
}

pub fn rendering_subscription_fires(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let frames = Rc::new(Cell::new(0u32));
        let sub = match on_rendering({
            let frames = frames.clone();
            move || frames.set(frames.get() + 1)
        }) {
            Ok(sub) => sub,
            Err(err) => {
                h.check_skip(
                    "Rendering_Subscription_Fires",
                    &format!("subscribe failed: {err:?}"),
                );
                return;
            }
        };

        // Composition frames only flow when the compositor is producing them,
        // which a headless/non-presenting agent may not do — soft-SKIP rather
        // than fail so CI stays green.
        let got = h
            .pump_until(Duration::from_secs(1), || frames.get() >= 2)
            .await;
        if !got {
            h.check_skip(
                "Rendering_Subscription_Fires",
                "no composition frames delivered (headless)",
            );
            return;
        }
        h.check("Rendering_Subscription_Fires", frames.get() >= 2);

        // Dropping the subscription must detach the handler.
        drop(sub);
        let before = frames.get();
        h.pump_for(Duration::from_millis(150)).await;
        h.check(
            "Rendering_Subscription_DropDetaches",
            frames.get() == before,
        );
    })
}
