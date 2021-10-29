#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestSingle)]
struct RustTest();

impl RustTest {
    fn SignatureSingle(&self, a: f32, b: &mut f32) -> Result<f32> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureSingle(&self, a: &[f32], b: &mut [f32], c: &mut Array<f32>) -> Result<Array<f32>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureSingle(&self, handler: &Option<SignatureSingle>) -> Result<()> {
        let a = 123.0;
        let mut b = 0.0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureSingle(&self, handler: &Option<ArraySignatureSingle>) -> Result<()> {
        let a = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestSingle) -> Result<()> {
    let a = 123.0;
    let mut b = 0.0;
    let c = test.SignatureSingle(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureSingle(SignatureSingle::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4.0, 5.0, 6.0];
    let mut b = [0.0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureSingle(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureSingle(ArraySignatureSingle::new(|a, b, c| {
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
