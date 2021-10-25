#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestInt64)]
struct RustTest();

impl RustTest {
    fn SignatureInt64(&self, a: i64, b: &mut i64) -> Result<i64> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureInt64(
        &self,
        a: &[i64],
        b: &mut [i64],
        c: &mut Array<i64>,
    ) -> Result<Array<i64>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureInt64(&self, handler: &Option<SignatureInt64>) -> Result<()> {
        let a = 123;
        let mut b = 0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureInt64(&self, handler: &Option<ArraySignatureInt64>) -> Result<()> {
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

fn test_interface(test: &ITestInt64) -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = test.SignatureInt64(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureInt64(SignatureInt64::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4, 5, 6];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureInt64(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureInt64(ArraySignatureInt64::new(|a, b, c| {
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
