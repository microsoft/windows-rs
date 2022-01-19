use test_implement::*;
use windows::core::*;
use Windows::Foundation::IStringable;
use Windows::Win32::Foundation::HANDLE;
use Windows::Win32::System::WinRT::Composition::ISwapChainInterop;
use Windows::Win32::System::WinRT::Display::IDisplayPathInterop;

#[implement(Windows::Foundation::IStringable, Windows::Win32::System::WinRT::Composition::ISwapChainInterop, Windows::Win32::System::WinRT::Display::IDisplayPathInterop)]
struct Mix();

#[allow(non_snake_case)]
impl Mix {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Mix".into())
    }

    fn SetSwapChain(&self, _: &Option<IUnknown>) -> Result<()> {
        Ok(())
    }

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
