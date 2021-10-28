// Remove target_pointer_width when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64", target_env = "msvc"))]

use test_win32_static_simple::*;
use windows::runtime::*;
use StaticComponent::Win32::Simple::*;

#[test]
fn test() -> Result<()> {
    unsafe {
        SimpleFunction();
        Ok(())
    }
}
