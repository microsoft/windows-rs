#![allow(non_snake_case)]

use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;

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
