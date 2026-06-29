use windows_collections::*;
use windows_core::*;

const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as _);

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

// BufferedIterator fills a fixed-size block via GetMany and yields from it. The
// internal block size is private, so these tests cover every size around the
// boundaries it could plausibly use: empty, single, just under/over, and exact
// multiples spanning several blocks. If the constant ever changes these stay valid.
#[test]
fn buffered_iterator_sizes() {
    for size in [
        0usize, 1, 2, 63, 64, 65, 127, 128, 129, 255, 256, 257, 384, 1000,
    ] {
        let source: Vec<i32> = (0..size as i32).collect();
        let v = IVector::<i32>::from(source.clone());

        // Borrowed IntoIterator yields exactly the source, in order.
        let collected: Vec<i32> = (&v).into_iter().collect();
        assert_eq!(collected, source, "borrowed iterate size {size}");

        // Owned IntoIterator yields the same.
        let collected: Vec<i32> = v.clone().into_iter().collect();
        assert_eq!(collected, source, "owned iterate size {size}");

        // count() drains every element including the final empty refill.
        assert_eq!((&v).into_iter().count(), size, "count size {size}");

        // sum verifies no element is dropped, duplicated, or shifted.
        let sum: i64 = (&v).into_iter().map(i64::from).sum();
        let expect: i64 = (0..size as i64).sum();
        assert_eq!(sum, expect, "sum size {size}");
    }
}

// Early termination must release a partially consumed block at any offset without
// double-counting or leaking; tested at every block-boundary-relative position.
#[test]
fn buffered_iterator_early_break() {
    let v = IVector::<i32>::from((0..400).collect::<Vec<_>>());

    for stop in [0usize, 1, 127, 128, 129, 255, 256, 257, 399, 400] {
        let mut count = 0;
        let mut iter = (&v).into_iter();
        while count < stop {
            assert_eq!(iter.next(), Some(count as i32), "elem at stop {stop}");
            count += 1;
        }
        drop(iter);
        assert_eq!(count, stop);
    }
}

// Reference-counted elements exercise per-block release: a wrong block-reset would
// drop live strings (UB) or leak them. Cross several blocks and break mid-block.
#[test]
fn buffered_iterator_hstring() {
    let source: Vec<HSTRING> = (0..300).map(|i| HSTRING::from(i.to_string())).collect();
    let v = IVector::<HSTRING>::from(source.clone());

    let collected: Vec<HSTRING> = (&v).into_iter().collect();
    assert_eq!(collected, source);

    for stop in [64usize, 128, 200, 300] {
        let mut count = 0;
        for value in &v {
            assert_eq!(value, source[count]);
            count += 1;
            if count == stop {
                break;
            }
        }
        assert_eq!(count, stop);
    }
}

// Interface (ref-counted COM) elements: same release path, different ABI default.
#[test]
fn buffered_iterator_interface() {
    let source: Vec<IInspectable> = (0..200)
        .map(|i| {
            IVector::<i32>::from(vec![i])
                .cast::<IInspectable>()
                .unwrap()
        })
        .collect();
    let v = IVector::<IInspectable>::from(vec![]);
    for item in &source {
        v.Append(item).unwrap();
    }

    let collected: Vec<IInspectable> = (&v).into_iter().collect();
    assert_eq!(collected, source);
    assert_eq!((&v).into_iter().count(), 200);
}

// Iteration also reaches collections through IIterable and IVectorView, which use
// the same BufferedIterator; confirm both honor the boundary spanning.
#[test]
fn buffered_iterator_view_and_iterable() -> Result<()> {
    let source: Vec<i32> = (0..300).collect();
    let v = IVector::<i32>::from(source.clone());

    let view = v.GetView()?;
    assert_eq!((&view).into_iter().collect::<Vec<_>>(), source);

    let iterable: IIterable<i32> = v.cast()?;
    assert_eq!((&iterable).into_iter().collect::<Vec<_>>(), source);

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

    // GetMany on IVector<HSTRING> exercises *mut AbiType<HSTRING> in the vtable.
    let mut values = vec![HSTRING::new(); 5];
    assert_eq!(v.GetMany(0, &mut values)?, 2);
    assert_eq!(values[0], *h!("TWO"));
    assert_eq!(values[1], *h!("three"));
    assert_eq!(values[2], HSTRING::new());

    // ReplaceAll on IVector<HSTRING> exercises *const AbiType<HSTRING> in the vtable.
    v.ReplaceAll(&[h!("a").clone(), h!("b").clone(), h!("c").clone()])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(&v.GetAt(0)?, h!("a"));
    assert_eq!(&v.GetAt(1)?, h!("b"));
    assert_eq!(&v.GetAt(2)?, h!("c"));

    v.ReplaceAll(&[])?;
    assert_eq!(v.Size()?, 0);

    let able: IIterable<HSTRING> = v.cast()?;
    let v2: IVector<HSTRING> = able.cast()?;
    assert_eq!(v, v2);

    Ok(())
}
