#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;
use Component::Simple::Class;

#[implement(Component::Signatures::ITestObject)]
struct RustTest();

impl RustTest {
    fn SignatureObject(
        &self,
        a: &Option<IInspectable>,
        b: &mut Option<IInspectable>,
    ) -> Result<IInspectable> {
        *b = a.clone();
        // TODO: Since `a` is an Option<IInspectable>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }
    fn ArraySignatureObject(
        &self,
        a: &[Option<IInspectable>],
        b: &mut [Option<IInspectable>],
        c: &mut Array<IInspectable>,
    ) -> Result<Array<IInspectable>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureObject(&self, handler: &Option<SignatureObject>) -> Result<()> {
        let a: Option<IInspectable> = Some(Class::new()?.into());
        let mut b = None;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &Some(c));
        Ok(())
    }
    fn CallArraySignatureObject(&self, handler: &Option<ArraySignatureObject>) -> Result<()> {
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

fn test_interface(test: &ITestObject) -> Result<()> {
    let a: Option<IInspectable> = Some(Class::new()?.into());
    let mut b = None;
    let c = test.SignatureObject(&a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &Some(c));

    test.CallSignatureObject(SignatureObject::new(|a, b| {
        *b = a.clone();
        // TODO: Since `a` is an Option<IInspectable>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }))?;

    let a = [
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
    ];
    let mut b = [None, None, None];
    let mut c = Array::new();
    let d = test.ArraySignatureObject(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureObject(ArraySignatureObject::new(|a, b, c| {
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
