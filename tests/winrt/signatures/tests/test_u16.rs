#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestUInt16)]
struct RustTest();

impl RustTest {
    fn SignatureUInt16(&self, a: u16, b: &mut u16) -> Result<u16> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureUInt16(
        &self,
        a: &[u16],
        b: &mut [u16],
        c: &mut Array<u16>,
    ) -> Result<Array<u16>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureUInt16(&self, handler: &Option<SignatureUInt16>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureUInt16(&self, handler: &Option<ArraySignatureUInt16>) -> Result<()> {
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

fn test_interface(test: &ITestUInt16) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureUInt16(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureUInt16(SignatureUInt16::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4, 5, 6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureUInt16(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureUInt16(ArraySignatureUInt16::new(|a, b, c| {
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
