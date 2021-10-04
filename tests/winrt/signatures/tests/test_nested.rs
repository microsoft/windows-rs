#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;
use Component::Structs::*;
use Windows::Foundation::PropertyValue;

#[implement(Component::Signatures::ITestNested)]
struct RustTest();

impl RustTest {
    fn SignatureNested(&self, a: &Nested, b: &Nested, c: &mut Nested) -> Result<Nested> {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }
    fn ArraySignatureNested(
        &self,
        a: &[Nested],
        b: &mut [Nested],
        c: &mut Array<Nested>,
    ) -> Result<Array<Nested>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureNested(&self, handler: &Option<SignatureNested>) -> Result<()> {
        let a = Nested {
            Blittable: Blittable {
                Bool: true,
                Char: 0x41,
                UInt8: 1,
                UInt16: 2,
                UInt32: 3,
                UInt64: 4,
                Int16: -1,
                Int32: -2,
                Int64: -3,
                Single: 1.0,
                Double: 0.1,
                Guid: "B0180C8C-8FEB-448A-A915-AC92E05135FE".into(),
            },
            NonBlittable: NonBlittable {
                String: "string".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(123)?.cast()?),
            },
        };

        let mut b = Nested::default();

        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &c);
        Ok(())
    }
    fn CallArraySignatureNested(&self, handler: &Option<ArraySignatureNested>) -> Result<()> {
        let a = [
            Nested {
                Blittable: Blittable {
                    Bool: true,
                    Char: 0x41,
                    UInt8: 1,
                    UInt16: 2,
                    UInt32: 3,
                    UInt64: 4,
                    Int16: -1,
                    Int32: -2,
                    Int64: -3,
                    Single: 1.0,
                    Double: 0.1,
                    Guid: "B0180C8C-8FEB-448A-A915-AC92E05135FE".into(),
                },
                NonBlittable: NonBlittable {
                    String: "first".into(),
                    // TODO: better boxing support is needed
                    RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
                },
            },
            Nested {
                Blittable: Blittable {
                    Bool: false,
                    Char: 0x42,
                    UInt8: 1,
                    UInt16: 2,
                    UInt32: 3,
                    UInt64: 4,
                    Int16: -1,
                    Int32: -2,
                    Int64: -3,
                    Single: 1.0,
                    Double: 0.1,
                    Guid: "9E234A6E-DF89-4891-AAD5-632692BBB1DC".into(),
                },
                NonBlittable: NonBlittable {
                    String: "second".into(),
                    // TODO: better boxing support is needed
                    RefInt64: Some(PropertyValue::CreateInt64(2)?.cast()?),
                },
            },
            Nested {
                Blittable: Blittable {
                    Bool: true,
                    Char: 0x43,
                    UInt8: 1,
                    UInt16: 2,
                    UInt32: 3,
                    UInt64: 4,
                    Int16: -1,
                    Int32: -2,
                    Int64: -3,
                    Single: 1.0,
                    Double: 0.1,
                    Guid: "286F8B75-2DF4-49CF-841C-52438E2D5326".into(),
                },
                NonBlittable: NonBlittable {
                    String: "first".into(),
                    // TODO: better boxing support is needed
                    RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
                },
            },
        ];

        let mut b = [Nested::default(), Nested::default(), Nested::default()];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestNested) -> Result<()> {
    let a = Nested {
        Blittable: Blittable {
            Bool: true,
            Char: 0x41,
            UInt8: 1,
            UInt16: 2,
            UInt32: 3,
            UInt64: 4,
            Int16: -1,
            Int32: -2,
            Int64: -3,
            Single: 1.0,
            Double: 0.1,
            Guid: "B0180C8C-8FEB-448A-A915-AC92E05135FE".into(),
        },
        NonBlittable: NonBlittable {
            String: "string".into(),
            // TODO: better boxing support is needed
            RefInt64: Some(PropertyValue::CreateInt64(123)?.cast()?),
        },
    };

    let mut b = Nested::default();

    let c = test.SignatureNested(&a, &a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &c);

    test.CallSignatureNested(SignatureNested::new(|a, b, c| {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    let a = [
        Nested {
            Blittable: Blittable {
                Bool: true,
                Char: 0x41,
                UInt8: 1,
                UInt16: 2,
                UInt32: 3,
                UInt64: 4,
                Int16: -1,
                Int32: -2,
                Int64: -3,
                Single: 1.0,
                Double: 0.1,
                Guid: "B0180C8C-8FEB-448A-A915-AC92E05135FE".into(),
            },
            NonBlittable: NonBlittable {
                String: "first".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
            },
        },
        Nested {
            Blittable: Blittable {
                Bool: false,
                Char: 0x42,
                UInt8: 1,
                UInt16: 2,
                UInt32: 3,
                UInt64: 4,
                Int16: -1,
                Int32: -2,
                Int64: -3,
                Single: 1.0,
                Double: 0.1,
                Guid: "9E234A6E-DF89-4891-AAD5-632692BBB1DC".into(),
            },
            NonBlittable: NonBlittable {
                String: "second".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(2)?.cast()?),
            },
        },
        Nested {
            Blittable: Blittable {
                Bool: true,
                Char: 0x43,
                UInt8: 1,
                UInt16: 2,
                UInt32: 3,
                UInt64: 4,
                Int16: -1,
                Int32: -2,
                Int64: -3,
                Single: 1.0,
                Double: 0.1,
                Guid: "286F8B75-2DF4-49CF-841C-52438E2D5326".into(),
            },
            NonBlittable: NonBlittable {
                String: "first".into(),
                // TODO: better boxing support is needed
                RefInt64: Some(PropertyValue::CreateInt64(1)?.cast()?),
            },
        },
    ];

    let mut b = [Nested::default(), Nested::default(), Nested::default()];

    let mut c = Array::new();
    let d = test.ArraySignatureNested(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureNested(ArraySignatureNested::new(|a, b, c| {
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
