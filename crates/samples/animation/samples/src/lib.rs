//! Samples for the `windows-animation` crate.
//!
//! Each file in `examples/` is a standalone console program that drives the
//! Windows Animation Manager on an explicit timeline and prints the animated
//! value, so they run headless with no window. Run one with
//! `cargo run -p animation_samples --example <name>`.

pub use windows_animation::*;

/// Joins the process to a COM multithreaded apartment so the animation
/// manager's in-process COM server can be created.
pub fn init_com() {
    unsafe {
        windows_core::link!("combase.dll" "system" fn CoIncrementMTAUsage(cookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
        let mut cookie = core::ptr::null_mut();
        let _ = CoIncrementMTAUsage(&mut cookie);
    }
}

/// Renders a value in `0..=max` as a simple text bar for console output.
pub fn bar(value: f64, max: f64) -> String {
    let width = 40.0;
    let filled = ((value / max) * width).round().clamp(0.0, width) as usize;
    "#".repeat(filled)
}
