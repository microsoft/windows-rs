#![allow(non_snake_case)]

use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestBoolean)]
struct RustTest();

impl RustTest {
    fn SignatureBoolean(&self, a: bool, b: &mut bool) -> Result<bool> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureBoolean(
        &self,
        a: &[bool],
        b: &mut [bool],
        c: &mut Array<bool>,
    ) -> Result<Array<bool>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureBoolean(&self, handler: &Option<SignatureBoolean>) -> Result<()> {
        let a = true;
        let mut b = false;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureBoolean(&self, handler: &Option<ArraySignatureBoolean>) -> Result<()> {
        let a = [true, false, true];
        let mut b = [false; 3];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestBoolean) -> Result<()> {
    let a = true;
    let mut b = false;
    let c = test.SignatureBoolean(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureBoolean(SignatureBoolean::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [true, false, true];
    let mut b = [false; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureBoolean(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureBoolean(ArraySignatureBoolean::new(|a, b, c| {
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
    test_interface(&Test::new()?.into())?;
    test_interface(&RustTest().into())?;
    Ok(())
}
