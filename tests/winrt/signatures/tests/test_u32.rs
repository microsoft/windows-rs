#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestUInt32)]
struct RustTest();

impl RustTest {
    fn SignatureUInt32(&self, a: u32, b: &mut u32) -> Result<u32> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureUInt32(
        &self,
        a: &[u32],
        b: &mut [u32],
        c: &mut Array<u32>,
    ) -> Result<Array<u32>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureUInt32(&self, handler: &Option<SignatureUInt32>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureUInt32(&self, handler: &Option<ArraySignatureUInt32>) -> Result<()> {
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

fn test_interface(test: &ITestUInt32) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureUInt32(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureUInt32(SignatureUInt32::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4, 5, 6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureUInt32(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureUInt32(ArraySignatureUInt32::new(|a, b, c| {
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
