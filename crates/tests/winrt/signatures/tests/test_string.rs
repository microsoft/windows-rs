#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestString)]
struct RustTest();

impl RustTest {
    fn SignatureString(&self, a: &HSTRING, b: &mut HSTRING) -> Result<HSTRING> {
        *b = a.clone();
        Ok(a.clone())
    }
    fn ArraySignatureString(
        &self,
        a: &[HSTRING],
        b: &mut [HSTRING],
        c: &mut Array<HSTRING>,
    ) -> Result<Array<HSTRING>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureString(&self, handler: &Option<SignatureString>) -> Result<()> {
        let a: HSTRING = "string".into();
        let mut b = HSTRING::new();
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &c);
        Ok(())
    }
    fn CallArraySignatureString(&self, handler: &Option<ArraySignatureString>) -> Result<()> {
        let a: [HSTRING; 3] = ["first".into(), "second".into(), "third".into()];
        let mut b = [HSTRING::new(), HSTRING::new(), HSTRING::new()];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestString) -> Result<()> {
    let a: HSTRING = "string".into();
    let mut b = HSTRING::new();
    let c = test.SignatureString(&a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &c);

    test.CallSignatureString(SignatureString::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    let a: [HSTRING; 3] = ["first".into(), "second".into(), "third".into()];
    let mut b = [HSTRING::new(), HSTRING::new(), HSTRING::new()];
    let mut c = Array::new();
    let d = test.ArraySignatureString(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureString(ArraySignatureString::new(|a, b, c| {
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
