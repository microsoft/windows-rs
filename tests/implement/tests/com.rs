use test_implement::*;
use windows::*;
use Windows::Foundation::IStringable;
use Windows::Win32::System::WinRT::ISwapChainInterop;

#[implement(
    Windows::Foundation::IStringable,
    Windows::Win32::System::WinRT::ISwapChainInterop
)]
struct Mix();

#[allow(non_snake_case)]
impl Mix {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Mix".into())
    }

    fn SetSwapChain(&self, _: &Option<IUnknown>) -> Result<()> {
        Ok(())
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

    Ok(())
}
