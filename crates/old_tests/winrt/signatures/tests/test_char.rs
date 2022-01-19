#![allow(non_snake_case)]

use core::convert::TryInto;
use test_winrt_signatures::*;
use windows::core::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestChar)]
struct RustTest();

impl RustTest {
    fn SignatureChar(&self, a: u16, b: &mut u16) -> Result<u16> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureChar(&self, a: &[u16], b: &mut [u16], c: &mut Array<u16>) -> Result<Array<u16>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureChar(&self, handler: &Option<SignatureChar>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureChar(&self, handler: &Option<ArraySignatureChar>) -> Result<()> {
        let a = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestChar) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureChar(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureChar(SignatureChar::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4, 5, 6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureChar(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureChar(ArraySignatureChar::new(|a, b, c| {
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
