use windows::runtime::*;
use windows::Win32::System::Com::*;
use windows::Win32::Web::MsHtml::*;

// This test exists primarily to validate that IDispatch-based interfaces are generated correctly.

#[test]
fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED)?;

        let doc: IHTMLDocument3 = CoCreateInstance(&HTMLDocument, None, CLSCTX_ALL)?;

        let text = doc.createTextNode("windows-rs")?;

        let text: IHTMLDOMTextNode = text.cast()?;

        assert_eq!("windows-rs", text.toString()?);
        assert_eq!(10, text.length()?);

        Ok(())
    }
}
