#![cfg(windows)]
use windows::{
    Win32::{combaseapi::*, objbase::*},
    Win32::{mfapi::*, mfobjects::*},
    core::*,
};

#[test]
fn test() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        let mut data = std::ptr::null_mut();
        let mut len = 0;

        MFTEnumEx(
            MFT_CATEGORY_VIDEO_ENCODER,
            MFT_ENUM_FLAG_HARDWARE as u32,
            None,
            None,
            &mut data,
            &mut len,
        )
        .ok()?;

        let array = Array::<IMFActivate>::from_raw_parts(data as _, len);

        for i in array.as_slice() {
            println!("{}", i.as_ref().unwrap().GetCount()?);
        }

        Ok(())
    }
}
