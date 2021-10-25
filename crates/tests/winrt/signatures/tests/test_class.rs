#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;
use Component::Simple::Class;

#[implement(Component::Signatures::ITestClass)]
struct RustTest();

impl RustTest {
    fn SignatureClass(&self, a: &Option<Class>, b: &mut Option<Class>) -> Result<Class> {
        *b = a.clone();
        // TODO: Since `a` is an Option<Class>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }
    fn ArraySignatureClass(
        &self,
        a: &[Option<Class>],
        b: &mut [Option<Class>],
        c: &mut Array<Class>,
    ) -> Result<Array<Class>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureClass(&self, handler: &Option<SignatureClass>) -> Result<()> {
        let a: Option<Class> = Some(Class::new()?.into());
        let mut b = None;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &Some(c));
        Ok(())
    }
    fn CallArraySignatureClass(&self, handler: &Option<ArraySignatureClass>) -> Result<()> {
        let a = [
            Some(Class::new()?.into()),
            Some(Class::new()?.into()),
            Some(Class::new()?.into()),
        ];
        let mut b = [None, None, None];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestClass) -> Result<()> {
    let a: Option<Class> = Some(Class::new()?.into());
    let mut b = None;
    let c = test.SignatureClass(&a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &Some(c));

    test.CallSignatureClass(SignatureClass::new(|a, b| {
        *b = a.clone();
        // TODO: Since `a` is an Option<Class>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }))?;

    let a = [
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
    ];
    let mut b = [None, None, None];
    let mut c = Array::new();
    let d = test.ArraySignatureClass(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureClass(ArraySignatureClass::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
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
