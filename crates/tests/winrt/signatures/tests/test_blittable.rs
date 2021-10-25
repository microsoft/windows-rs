#![allow(non_snake_case)]

use std::convert::TryInto;
use test_winrt_signatures::*;
use windows::runtime::*;
use Component::Signatures::*;
use Component::Structs::*;

#[implement(Component::Signatures::ITestBlittable)]
struct RustTest();

impl RustTest {
    fn SignatureBlittable(
        &self,
        a: &Blittable,
        b: &Blittable,
        c: &mut Blittable,
    ) -> Result<Blittable> {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }
    fn ArraySignatureBlittable(
        &self,
        a: &[Blittable],
        b: &mut [Blittable],
        c: &mut Array<Blittable>,
    ) -> Result<Array<Blittable>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureBlittable(&self, handler: &Option<SignatureBlittable>) -> Result<()> {
        let a = Blittable {
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
        };

        let mut b = Blittable::default();

        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(&a, &a, &mut b)?;
        assert!(&a == &b);
        assert!(&a == &c);
        Ok(())
    }
    fn CallArraySignatureBlittable(&self, handler: &Option<ArraySignatureBlittable>) -> Result<()> {
        let a = [
            Blittable {
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
            Blittable {
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
            Blittable {
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
        ];

        let mut b = [
            Blittable::default(),
            Blittable::default(),
            Blittable::default(),
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

fn test_interface(test: &ITestBlittable) -> Result<()> {
    let a = Blittable {
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
    };

    let mut b = Blittable::default();

    let c = test.SignatureBlittable(&a, &a, &mut b)?;

    assert!(&a == &b);
    assert!(&a == &c);

    test.CallSignatureBlittable(SignatureBlittable::new(|a, b, c| {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    let a = [
        Blittable {
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
        Blittable {
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
        Blittable {
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
    ];

    let mut b = [
        Blittable::default(),
        Blittable::default(),
        Blittable::default(),
    ];

    let mut c = Array::new();
    let d = test.ArraySignatureBlittable(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureBlittable(ArraySignatureBlittable::new(|a, b, c| {
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
