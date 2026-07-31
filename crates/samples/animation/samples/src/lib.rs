pub use windows_animation::*;

pub fn init_com() {
    unsafe {
        windows_core::link!("ole32.dll" "system" fn CoIncrementMTAUsage(cookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
        let mut cookie = core::ptr::null_mut();
        let _ = CoIncrementMTAUsage(&mut cookie);
    }
}

pub fn bar(value: f64, max: f64) -> String {
    let width = 40.0;
    let filled = ((value / max) * width).round().clamp(0.0, width) as usize;
    "#".repeat(filled)
}
