#![allow(non_snake_case)]

use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;
use std::convert::TryInto;

#[implement(Component::Signatures::ITestUInt8)]
struct RustTest();

impl RustTest {
    fn SignatureUInt8(&self, a: u8, b: &mut u8) -> Result<u8> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureUInt8(
        &self,
        a: &[u8],
        b: &mut [u8],
        c: &mut Array<u8>,
    ) -> Result<Array<u8>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureUInt8(&self, handler: &Option<SignatureUInt8>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureUInt8(&self, handler: &Option<ArraySignatureUInt8>) -> Result<()> {
        let a = [1,2,3];
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

fn test_interface(test: &ITestUInt8) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureUInt8(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureUInt8(SignatureUInt8::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4,5,6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureUInt8(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureUInt8(ArraySignatureUInt8::new(|a, b, c| {
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
