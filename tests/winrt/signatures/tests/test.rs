use test_winrt_signatures::*;
use windows::*;
use Component::Signatures::*;

#[test]
fn signature1() -> Result<()> {
    let a = true;
    let mut b = false;
    let c = Test::Signature1(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    Test::CallSignature1(Signature1::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    Ok(())
}

#[test]
fn array_signature1() -> Result<()> {
    let a = [true, false, true];
    let mut b = [false; 3];
    let mut c = Array::new();
    let d = Test::ArraySignature1(&a, &mut b, &mut c)?;

    assert!(a == b);
    // TODO: should `a == c` be sufficient? Does that work for Vec?
    assert!(a == c[..]);
    assert!(a == d[..]);

    Test::CallArraySignature1(ArraySignature1::new(|a, b, c| {
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
