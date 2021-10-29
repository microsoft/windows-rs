#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestInt16)]
struct RustTest();

impl RustTest {
    fn SignatureInt16(&self, a: i16, b: &mut i16) -> Result<i16> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureInt16(&self, a: &[i16], b: &mut [i16], c: &mut Array<i16>) -> Result<Array<i16>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureInt16(&self, handler: &Option<SignatureInt16>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureInt16(&self, handler: &Option<ArraySignatureInt16>) -> Result<()> {
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

fn test_interface(test: &ITestInt16) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureInt16(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureInt16(SignatureInt16::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4, 5, 6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureInt16(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureInt16(ArraySignatureInt16::new(|a, b, c| {
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
