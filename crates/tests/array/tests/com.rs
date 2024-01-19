use windows::{core::*, Win32::Media::MediaFoundation::*, Win32::System::Com::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let mut data = std::ptr::null_mut();
        let mut len = 0;

        MFTEnumEx(
            MFT_CATEGORY_VIDEO_ENCODER,
            MFT_ENUM_FLAG_HARDWARE,
            None,
            None,
            &mut data,
            &mut len,
        )?;

        let array = Array::<IMFActivate>::from_raw_parts(data as _, len);

        for i in array.as_slice() {
            println!("{}", i.as_ref().unwrap().GetCount()?);
        }

        Ok(())
    }
}
