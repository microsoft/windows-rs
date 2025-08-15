use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::WinRT::Display::*;

#[implement(
    windows::Foundation::IStringable,
    windows::Win32::System::WinRT::Display::IDisplayPathInterop
)]
struct Mix;

impl IStringable_Impl for Mix_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Mix".into())
    }
}

impl IDisplayPathInterop_Impl for Mix_Impl {
    fn GetSourceId(&self) -> Result<u32> {
        Ok(123)
    }

    fn CreateSourcePresentationHandle(&self) -> Result<HANDLE> {
        Ok(HANDLE::default())
    }
}

#[test]
fn mix() -> Result<()> {
    let a: IUnknown = Mix.into();

    let b: IStringable = a.cast()?;
    assert!(b.ToString()? == "Mix");

    let c: IStringable = Mix.into();
    assert!(c.ToString()? == "Mix");

    let d: IUnknown = c.cast()?;

    let e: IDisplayPathInterop = d.cast()?;
    unsafe { assert!(e.GetSourceId()? == 123) };
    unsafe { assert!(e.CreateSourcePresentationHandle()? == HANDLE::default()) };

    Ok(())
}
