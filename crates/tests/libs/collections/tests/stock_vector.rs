use windows::{core::*, Win32::Foundation::E_BOUNDS};
use windows_collections::*;

#[test]
fn primitive() -> Result<()> {
    let v = IVector::<i32>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);
    assert!(!(v.IndexOf(0, &mut 0)?));
    assert_eq!(v.GetMany(0, &mut [0; 5])?, 0);

    let v = IVector::<i32>::from(vec![1, 2, 3]);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);
    assert_eq!(v.GetAt(3).unwrap_err().code(), E_BOUNDS);
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
    let v2: IVector<i32> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}

#[test]
fn primitive_iterator() -> Result<()> {
    let v = IVector::<i32>::from(vec![]);
    let iter = v.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    assert_eq!(iter.GetMany(&mut [0; 5])?, 0);

    let v = IVector::<i32>::from(vec![1, 2, 3]);
    let iter = v.First()?;

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

    let iter = v.First()?;
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert_eq!(values, [1, 2, 3, 0, 0]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = v.First()?;
    let mut values = [0; 1];
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert_eq!(values, [1]);
    let mut values = [0; 2];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values, [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    // MoveNext followed by GetMany reads from the advanced position
    let iter = v.First()?;
    assert!(iter.MoveNext()?);
    let mut values = [0; 5];
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values[..2], [2, 3]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

#[test]
fn primitive_mutable() -> Result<()> {
    let v = IVector::<i32>::from(vec![]);

    // Append
    v.Append(1)?;
    v.Append(2)?;
    v.Append(3)?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);

    // SetAt
    v.SetAt(1, 20)?;
    assert_eq!(v.GetAt(1)?, 20);
    assert_eq!(v.SetAt(3, 0).unwrap_err().code(), E_BOUNDS);

    // InsertAt
    v.InsertAt(1, 10)?;
    assert_eq!(v.Size()?, 4);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 10);
    assert_eq!(v.GetAt(2)?, 20);
    assert_eq!(v.GetAt(3)?, 3);
    assert_eq!(v.InsertAt(5, 0).unwrap_err().code(), E_BOUNDS);

    // InsertAt at end (equivalent to append)
    v.InsertAt(4, 99)?;
    assert_eq!(v.Size()?, 5);
    assert_eq!(v.GetAt(4)?, 99);

    // RemoveAt
    v.RemoveAt(4)?;
    assert_eq!(v.Size()?, 4);
    assert_eq!(v.RemoveAt(4).unwrap_err().code(), E_BOUNDS);

    // RemoveAtEnd
    v.RemoveAtEnd()?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 10);
    assert_eq!(v.GetAt(2)?, 20);

    // Clear
    v.Clear()?;
    assert_eq!(v.Size()?, 0);
    assert_eq!(v.RemoveAtEnd().unwrap_err().code(), E_BOUNDS);

    // ReplaceAll
    v.ReplaceAll(&[7, 8, 9])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 7);
    assert_eq!(v.GetAt(1)?, 8);
    assert_eq!(v.GetAt(2)?, 9);

    // ReplaceAll with empty clears the vector
    v.ReplaceAll(&[])?;
    assert_eq!(v.Size()?, 0);

    Ok(())
}

#[test]
fn get_view() -> Result<()> {
    let v = IVector::<i32>::from(vec![1, 2, 3]);

    // GetView returns a snapshot
    let view = v.GetView()?;
    assert_eq!(view.Size()?, 3);
    assert_eq!(view.GetAt(0)?, 1);
    assert_eq!(view.GetAt(1)?, 2);
    assert_eq!(view.GetAt(2)?, 3);

    // Mutating the vector after GetView does not affect the snapshot
    v.Append(4)?;
    assert_eq!(v.Size()?, 4);
    assert_eq!(view.Size()?, 3);

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let v = IVector::<HSTRING>::from(vec![]);
    assert_eq!(v.GetAt(0).unwrap_err().code(), E_BOUNDS);
    assert_eq!(v.Size()?, 0);

    v.Append(h!("one"))?;
    v.Append(h!("two"))?;
    v.Append(h!("three"))?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(&v.GetAt(0)?, h!("one"));
    assert_eq!(&v.GetAt(1)?, h!("two"));
    assert_eq!(&v.GetAt(2)?, h!("three"));

    v.SetAt(1, h!("TWO"))?;
    assert_eq!(&v.GetAt(1)?, h!("TWO"));

    v.RemoveAt(0)?;
    assert_eq!(v.Size()?, 2);
    assert_eq!(&v.GetAt(0)?, h!("TWO"));
    assert_eq!(&v.GetAt(1)?, h!("three"));

    v.Clear()?;
    assert_eq!(v.Size()?, 0);

    let able: IIterable<HSTRING> = v.cast()?;
    let v2: IVector<HSTRING> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}
