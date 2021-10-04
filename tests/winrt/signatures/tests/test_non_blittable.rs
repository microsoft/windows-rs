#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;
use Component::Structs::*;
use Windows::Foundation::PropertyValue;

#[implement(Component::Signatures::ITestNonBlittable)]
struct RustTest();

impl RustTest {
    fn SignatureNonBlittable(&self, a: &NonBlittable, b: &NonBlittable, c: &mut NonBlittable) -> Result<NonBlittable> {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }
    fn ArraySignatureNonBlittable(
        &self,
        a: &[NonBlittable],
        b: &mut [NonBlittable],
        c: &mut Array<NonBlittable>,
    ) -> Result<Array<NonBlittable>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureNonBlittable(&self, handler: &Option<SignatureNonBlittable>) -> Result<()> {
        let a = NonBlittable {
            String: "string".into(),
            // TODO: better boxing support is needed
            RefInt64: Some(PropertyValue::CreateInt64(123)?.cast()?),
        };
    
        let mut b = NonBlittable::default();

        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &c);
        Ok(())
    }
    fn CallArraySignatureNonBlittable(&self, handler: &Option<ArraySignatureNonBlittable>) -> Result<()> {
        let a = [
            NonBlittable {
                String: "first".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
            },
            NonBlittable {
                String: "second".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(2)?.cast()?),
            },
            NonBlittable {
                String: "third".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(3)?.cast()?),
            },
        ];
    
        let mut b = [
            NonBlittable::default(),
            NonBlittable::default(),
            NonBlittable::default(),
        ];

        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestNonBlittable) -> Result<()> {
    let a = NonBlittable {
        String: "string".into(),
        // TODO: better boxing support is needed
        RefInt64: Some(PropertyValue::CreateInt64(123)?.cast()?),
    };

    let mut b = NonBlittable::default();

    let c = test.SignatureNonBlittable(&a, &a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &c);

    test.CallSignatureNonBlittable(SignatureNonBlittable::new(|a, b, c| {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    let a = [
        NonBlittable {
            String: "first".into(),
            // TODO: better boxing support is needed
            RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
        },
        NonBlittable {
            String: "second".into(),
            // TODO: better boxing support is needed
            RefInt64: Some(PropertyValue::CreateInt64(2)?.cast()?),
        },
        NonBlittable {
            String: "third".into(),
            // TODO: better boxing support is needed
            RefInt64: Some(PropertyValue::CreateInt64(3)?.cast()?),
        },
    ];

    let mut b = [
        NonBlittable::default(),
        NonBlittable::default(),
        NonBlittable::default(),
    ];

    let mut c = Array::new();
    let d = test.ArraySignatureNonBlittable(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureNonBlittable(ArraySignatureNonBlittable::new(|a, b, c| {
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
