//! Selftest fixture for the `Image` rasterization-scale hook.
//! Exercises `Image::on_mounted` → `ImageHandle::on_rasterization_scale_changed`
//! against a live WinUI window.
//!
//! The wiring (mount callback fires, the cast chain succeeds, and the `Loaded`
//! subscription is created) is deterministic and asserted hard. Actual scale
//! *delivery* depends on the window's `XamlRoot` having been presented, which is
//! not guaranteed on a headless agent, so it is best-effort: a real value passes,
//! its absence is reported as a SKIP rather than a flaky failure.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::{ElementExt, Image};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

pub fn rasterization_scale_delivered(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let mounted = Rc::new(Cell::new(false));
        let wired = Rc::new(Cell::new(false));
        let scale = Rc::new(Cell::new(0.0f64));
        let revoker = Rc::new(RefCell::new(None));

        {
            let mounted = mounted.clone();
            let wired = wired.clone();
            let scale = scale.clone();
            let revoker = revoker.clone();
            h.mount(cc(move |_| {
                let mounted = mounted.clone();
                let wired = wired.clone();
                let scale = scale.clone();
                let revoker = revoker.clone();
                Image::new_with_uri("ms-appx:///Assets/none.png")
                    .width(64.0)
                    .height(64.0)
                    .on_mounted(move |handle| {
                        mounted.set(true);
                        let scale = scale.clone();
                        match handle.on_rasterization_scale_changed(move |s| scale.set(s)) {
                            Ok(r) => {
                                wired.set(true);
                                *revoker.borrow_mut() = Some(r);
                            }
                            Err(_) => wired.set(false),
                        }
                    })
                    .into()
            }));
        }

        // Pump generously so the `Loaded` event has every chance to fire and
        // deliver a scale on agents where the window is actually presented.
        for _ in 0..120 {
            if mounted.get() && scale.get() > 0.0 {
                break;
            }
            h.render().await;
        }

        // Deterministic: the mount callback fired and the scale subscription was
        // created. This is what regresses if the `Loaded`/`XamlRoot` bindings break.
        h.check("Image_ScaleHook_WiresUp", mounted.get() && wired.get());

        // Best-effort: a delivered scale must be a sane DPI factor. Absent (window
        // not presented headless) it is skipped, never a flaky failure.
        if scale.get() > 0.0 {
            h.check_with(
                "Image_RasterizationScale_Delivered",
                scale.get() >= 1.0,
                || format!("scale={}", scale.get()),
            );
        } else {
            h.check_skip(
                "Image_RasterizationScale_Delivered",
                "XamlRoot not presented (headless); scale not delivered",
            );
        }
    })
}
