#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestGuid)]
struct RustTest();

impl RustTest {
    fn SignatureGuid(&self, a: &Guid, b: &mut Guid) -> Result<Guid> {
        *b = a.clone();
        Ok(a.clone())
    }
    fn ArraySignatureGuid(
        &self,
        a: &[Guid],
        b: &mut [Guid],
        c: &mut Array<Guid>,
    ) -> Result<Array<Guid>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureGuid(&self, handler: &Option<SignatureGuid>) -> Result<()> {
        let a = Guid::new()?;
        let mut b = Guid::zeroed();
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &c);
        Ok(())
    }
    fn CallArraySignatureGuid(&self, handler: &Option<ArraySignatureGuid>) -> Result<()> {
        let a: [Guid; 3] = [Guid::new()?, Guid::new()?, Guid::new()?];
        let mut b = [Guid::zeroed(), Guid::zeroed(), Guid::zeroed()];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestGuid) -> Result<()> {
    let a = Guid::new()?;
    let mut b = Guid::zeroed();
    let c = test.SignatureGuid(&a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &c);

    test.CallSignatureGuid(SignatureGuid::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    let a: [Guid; 3] = [Guid::new()?, Guid::new()?, Guid::new()?];
    let mut b = [Guid::zeroed(), Guid::zeroed(), Guid::zeroed()];
    let mut c = Array::new();
    let d = test.ArraySignatureGuid(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureGuid(ArraySignatureGuid::new(|a, b, c| {
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
