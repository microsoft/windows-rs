use windows_core::Interface;

use crate::bindings::IWindow2;
use windows_reactor::Backdrop;

use crate::fixtures::reconciler::FixtureFuture;
use crate::harness::Harness;

pub fn all_materials(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let supports_iwindow2 = h.window().cast::<IWindow2>().is_ok();
        if !supports_iwindow2 {
            h.check_skip(
                "Window_Backdrop_All",
                "IWindow2 not available on this Window instance",
            );
            return;
        }

        for (name, backdrop) in [
            ("Window_Backdrop_Mica", Backdrop::Mica),
            ("Window_Backdrop_MicaAlt", Backdrop::MicaAlt),
            ("Window_Backdrop_Acrylic", Backdrop::Acrylic),
        ] {
            let result = backdrop.apply_to(h.window());
            h.check_with(name, result.is_ok(), || match result {
                Ok(()) => String::new(),
                Err(err) => format!("{err:?}"),
            });
        }
    })
}
