#![allow(non_snake_case)]

use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;
use Component::Simple::Class;
use Component::Structs::Blittable;
use Windows::Win32::Foundation::*;

#[test]
fn SignatureBoolean() -> Result<()> {
    let a = true;
    let mut b = false;
    let c = Test::SignatureBoolean(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureBoolean(SignatureBoolean::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureBoolean() -> Result<()> {
    let a = [true, false, true];
    let mut b = [false; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureBoolean(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureBoolean(ArraySignatureBoolean::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureUInt8() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureUInt8(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureUInt8(SignatureUInt8::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureUInt8() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureUInt8(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureUInt8(ArraySignatureUInt8::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureUInt16() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureUInt16(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureUInt16(SignatureUInt16::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureUInt16() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureUInt16(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureUInt16(ArraySignatureUInt16::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureUInt32() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureUInt32(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureUInt32(SignatureUInt32::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureUInt32() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureUInt32(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureUInt32(ArraySignatureUInt32::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureUInt64() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureUInt64(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureUInt64(SignatureUInt64::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureUInt64() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureUInt64(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureUInt64(ArraySignatureUInt64::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureInt16() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureInt16(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureInt16(SignatureInt16::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureInt16() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureInt16(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureInt16(ArraySignatureInt16::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureInt32() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureInt32(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureInt32(SignatureInt32::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureInt32() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureInt32(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureInt32(ArraySignatureInt32::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureInt64() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureInt64(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureInt64(SignatureInt64::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureInt64() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureInt64(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureInt64(ArraySignatureInt64::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureSingle() -> Result<()> {
    let a = 123.0;
    let mut b = 0.0;
    let c = Test::SignatureSingle(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureSingle(SignatureSingle::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureSingle() -> Result<()> {
    let a = [1.0, 2.0, 3.0];
    let mut b = [0.0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureSingle(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureSingle(ArraySignatureSingle::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureDouble() -> Result<()> {
    let a = 123.0;
    let mut b = 0.0;
    let c = Test::SignatureDouble(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureDouble(SignatureDouble::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureDouble() -> Result<()> {
    let a = [1.0, 2.0, 3.0];
    let mut b = [0.0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureDouble(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureDouble(ArraySignatureDouble::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureChar() -> Result<()> {
    let a = 123;
    let mut b = 0;
    let c = Test::SignatureChar(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureChar(SignatureChar::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureChar() -> Result<()> {
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = Test::ArraySignatureChar(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureChar(ArraySignatureChar::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.copy_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.copy_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureString() -> Result<()> {
    let a: HSTRING = "string".into();
    let mut b = HSTRING::new();
    let c = Test::SignatureString(&a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureString(SignatureString::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureString() -> Result<()> {
    let a: [HSTRING; 3] = ["first".into(), "second".into(), "third".into()];
    let mut b = [HSTRING::new(), HSTRING::new(), HSTRING::new()];
    let mut c = Array::new();
    let d = Test::ArraySignatureString(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureString(ArraySignatureString::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureGuid() -> Result<()> {
    let a = Guid::new()?;
    let mut b = Guid::zeroed();
    let c = Test::SignatureGuid(&a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureGuid(SignatureGuid::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureGuid() -> Result<()> {
    let a: [Guid; 3] = [Guid::new()?, Guid::new()?, Guid::new()?];
    let mut b = [Guid::zeroed(), Guid::zeroed(), Guid::zeroed()];
    let mut c = Array::new();
    let d = Test::ArraySignatureGuid(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureGuid(ArraySignatureGuid::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureHResult() -> Result<()> {
    let a = E_NOINTERFACE;
    let mut b = S_FALSE;
    let c = Test::SignatureHResult(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureHResult(SignatureHResult::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureHResult() -> Result<()> {
    let a = [E_NOINTERFACE, S_OK, E_POINTER];
    let mut b = [S_FALSE, S_FALSE, S_FALSE];
    let mut c = Array::new();
    let d = Test::ArraySignatureHResult(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureHResult(ArraySignatureHResult::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureObject() -> Result<()> {
    let a: Option<IInspectable> = Some(Class::new()?.into());
    let mut b = None;
    let c = Test::SignatureObject(&a, &mut b)?;

    assert!(a == b);
    assert!(a == Some(c));

    Test::CallSignatureObject(SignatureObject::new(|a, b| {
        *b = a.clone();
        // TODO: Since `a` is an Option<IInspectable>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureObject() -> Result<()> {
    let a = [
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
        Some(Class::new()?.into()),
    ];
    let mut b = [None, None, None];
    let mut c = Array::new();
    let d = Test::ArraySignatureObject(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureObject(ArraySignatureObject::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureClass() -> Result<()> {
    let a: Option<Class> = Some(Class::new()?.into());
    let mut b = None;
    let c = Test::SignatureClass(&a, &mut b)?;

    assert!(a == b);
    assert!(a == Some(c));

    Test::CallSignatureClass(SignatureClass::new(|a, b| {
        *b = a.clone();
        // TODO: Since `a` is an Option<Class>, this is a bit messy...
        Ok(a.as_ref().unwrap().clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureClass() -> Result<()> {
    let a = [
        Some(Class::new()?),
        Some(Class::new()?),
        Some(Class::new()?),
    ];
    let mut b = [None, None, None];
    let mut c = Array::new();
    let d = Test::ArraySignatureClass(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureClass(ArraySignatureClass::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}

#[test]
fn SignatureBlittable() -> Result<()> {
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
    let c = Test::SignatureBlittable(&a, &a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignatureBlittable(SignatureBlittable::new(|a, b, c| {
        assert!(a == b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

#[test]
fn ArraySignatureBlittable() -> Result<()> {
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
    let d = Test::ArraySignatureBlittable(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignatureBlittable(ArraySignatureBlittable::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        // TODO: need a more convenient/idiomatic way to create arrays?
        *c = Array::with_len(a.len());
        c.clone_from_slice(a);
        let mut d = Array::with_len(a.len());
        d.clone_from_slice(a);
        Ok(d)
    }))?;

    Ok(())
}
