#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::WinRT::Composition::*;
use windows::Win32::System::WinRT::Display::*;

#[implement(windows::Foundation::IStringable, windows::Win32::System::WinRT::Composition::ISwapChainInterop, windows::Win32::System::WinRT::Display::IDisplayPathInterop)]
struct Mix();

impl IStringable_Impl for Mix {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Mix".into())
    }
}
impl ISwapChainInterop_Impl for Mix {
    fn SetSwapChain(&self, _: &Option<IUnknown>) -> Result<()> {
        Ok(())
    }
}

impl IDisplayPathInterop_Impl for Mix {
    fn GetSourceId(&self) -> Result<u32> {
        Ok(123)
    }

    fn CreateSourcePresentationHandle(&self) -> Result<HANDLE> {
        Ok(HANDLE::default())
    }
}

#[test]
fn mix() -> Result<()> {
    let a: ISwapChainInterop = Mix().into();
    unsafe { a.SetSwapChain(None)? };

    let b: IStringable = a.cast()?;
    assert!(b.ToString()? == "Mix");

    let c: IStringable = Mix().into();
    assert!(c.ToString()? == "Mix");

    let d: ISwapChainInterop = c.cast()?;
    unsafe { d.SetSwapChain(None)? };

    let e: IDisplayPathInterop = d.cast()?;
    unsafe { assert!(e.GetSourceId()? == 123) };
    unsafe { assert!(e.CreateSourcePresentationHandle()? == HANDLE::default()) };

    Ok(())
}
