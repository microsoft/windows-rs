// Remove when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64"))]

use test_win32_static_simple::*;
use windows::*;
use Component::Win32::Static::Simple::*;

#[test]
fn test() -> Result<()> {
    unsafe {
        SimpleFunction();
        Ok(())
    }
}
