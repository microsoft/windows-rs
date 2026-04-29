use windows::{core::*, Win32::Foundation::E_BOUNDS};
use windows_collections::*;

#[test]
fn primitive() -> Result<()> {
    let v = IVectorView::<i32>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);
    assert!(!(v.IndexOf(0, &mut 0)?));
    assert_eq!(v.GetMany(0, &mut [0; 5])?, 0);

    let v = IVectorView::<i32>::from(vec![1, 2, 3]);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);
    assert_eq!(v.Size()?, 3);
    let mut index = 0;
    assert!(!(v.IndexOf(0, &mut index)?));
    assert_eq!(index, 0);
    assert!(v.IndexOf(1, &mut index)?);
    assert_eq!(index, 0);
    assert!(v.IndexOf(2, &mut index)?);
    assert_eq!(index, 1);
    assert!(v.IndexOf(3, &mut index)?);
    assert_eq!(index, 2);
    // IndexOf resets *result to 0 when the value is not found
    index = 99;
    assert!(!(v.IndexOf(0, &mut index)?));
    assert_eq!(index, 0);

    let mut values = [0; 5];
    assert_eq!(v.GetMany(0, &mut values)?, 3);
    assert_eq!(values, [1, 2, 3, 0, 0]);

    let mut values = [0; 5];
    assert_eq!(v.GetMany(1, &mut values)?, 2);
    assert_eq!(values, [2, 3, 0, 0, 0]);

    let mut values = [0; 1];
    assert_eq!(v.GetMany(1, &mut values)?, 1);
    assert_eq!(values, [2]);

    let able: IIterable<i32> = v.cast()?;
    let v2: IVectorView<i32> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let able = IVectorView::<i32>::from(vec![]);
    let iter = able.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    assert_eq!(iter.GetMany(&mut [0; 5])?, 0);

    let able = IVectorView::<i32>::from(vec![1, 2, 3]);
    let iter = able.First()?;

    assert_eq!(iter.Current()?, 1);
    assert_eq!(iter.Current()?, 1);

    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?, 2);
    assert_eq!(iter.Current()?, 2);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?, 3);
    assert_eq!(iter.Current()?, 3);
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    let iter = able.First()?;
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert_eq!(values, [1, 2, 3, 0, 0]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = able.First()?;
    let mut values = [0; 1];
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert_eq!(values, [1]);
    let mut values = [0; 2];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values, [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    // MoveNext followed by GetMany reads from the advanced position
    let iter = able.First()?;
    assert!(iter.MoveNext()?);
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values[..2], [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let v = IVectorView::<HSTRING>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);

    let v = IVectorView::<HSTRING>::from(vec![
        HSTRING::from("one"),
        HSTRING::from("two"),
        HSTRING::from("three"),
    ]);
    assert_eq!(v.Size()?, 3);
    assert_eq!(&v.GetAt(0)?, h!("one"));
    assert_eq!(&v.GetAt(1)?, h!("two"));
    assert_eq!(&v.GetAt(2)?, h!("three"));
    assert_eq!(v.GetAt(3).unwrap_err().code(), E_BOUNDS);

    let mut index = 0;
    assert!(!(v.IndexOf(h!("missing"), &mut index)?));
    assert_eq!(index, 0);
    assert!(v.IndexOf(h!("one"), &mut index)?);
    assert_eq!(index, 0);
    assert!(v.IndexOf(h!("two"), &mut index)?);
    assert_eq!(index, 1);
    assert!(v.IndexOf(h!("three"), &mut index)?);
    assert_eq!(index, 2);

    let mut values = vec![HSTRING::default(); 5];
    assert_eq!(v.GetMany(0, &mut values)?, 3);
    assert_eq!(&values[0], h!("one"));
    assert_eq!(&values[1], h!("two"));
    assert_eq!(&values[2], h!("three"));

    let able: IIterable<HSTRING> = v.cast()?;
    let v2: IVectorView<HSTRING> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}
