#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/details_bindings.rs"));
}

pub(super) fn wheel_amount(horizontal: bool, viewport: f32) -> f64 {
    use bindings::*;
    let mut units = 0u32;
    let action = if horizontal {
        SPI_GETWHEELSCROLLCHARS
    } else {
        SPI_GETWHEELSCROLLLINES
    };
    let succeeded =
        unsafe { SystemParametersInfoW(action as u32, 0, (&mut units as *mut u32).cast(), 0) };
    assert!(
        succeeded.as_bool(),
        "{}",
        windows_core::Error::from_thread()
    );
    if units == u32::MAX {
        f64::from(viewport)
    } else {
        f64::from(units)
            * if horizontal {
                8.0
            } else {
                f64::from(super::state::ROW)
            }
    }
}
