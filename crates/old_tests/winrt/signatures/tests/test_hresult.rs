#![allow(non_snake_case)]

use core::convert::TryInto;
use test_winrt_signatures::*;
use windows::core::*;
use Component::Signatures::*;
use Windows::Win32::Foundation::*;

#[implement(Component::Signatures::ITestHResult)]
struct RustTest();

impl RustTest {
    fn SignatureHResult(&self, a: HRESULT, b: &mut HRESULT) -> Result<HRESULT> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureHResult(&self, a: &[HRESULT], b: &mut [HRESULT], c: &mut Array<HRESULT>) -> Result<Array<HRESULT>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureHResult(&self, handler: &Option<SignatureHResult>) -> Result<()> {
        let a = E_NOINTERFACE;
        let mut b = S_FALSE;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureHResult(&self, handler: &Option<ArraySignatureHResult>) -> Result<()> {
        let a = [E_NOINTERFACE, S_OK, E_POINTER];
        let mut b = [S_FALSE, S_FALSE, S_FALSE];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestHResult) -> Result<()> {
    let a = E_NOINTERFACE;
    let mut b = S_FALSE;
    let c = test.SignatureHResult(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureHResult(SignatureHResult::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [E_NOINTERFACE, S_OK, E_POINTER];
    let mut b = [S_FALSE, S_FALSE, S_FALSE];
    let mut c = Array::new();
    let d = test.ArraySignatureHResult(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureHResult(ArraySignatureHResult::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }))?;

    Ok(())
}

#[test]
fn test() -> Result<()> {
    test_interface(&Test::new()?.try_into()?)?;
    test_interface(&RustTest().into())?;
    Ok(())
}
