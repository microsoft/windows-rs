use windows_collections::*;
use windows_core::*;

const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as _);

// `E_FAIL` is a generic Win32 HRESULT used here as a stand-in error code in
// the failing-iterator tests below. Define it locally so this test file does
// not need to depend on `windows::Win32::Foundation`.
const E_FAIL: HRESULT = HRESULT(0x80004005_u32 as _);

// `Calendar` is a runtime-activated WinRT class that requires the WinRT
// activation factory linker symbols, so this test only runs on Windows.
#[cfg(windows)]
#[test]
fn calendar() -> Result<()> {
    use windows::Globalization::*;

    let languages = IIterable::from(vec![HSTRING::from("he-IL"), HSTRING::from("ja-JP")]);
    let calendar = Calendar::CreateCalendar(
        &languages,
        &CalendarIdentifiers::Hebrew()?,
        &ClockIdentifiers::TwentyFourHour()?,
    )?;

    let languages2 = calendar.Languages()?;
    assert_eq!(languages2.Size()?, 2);
    assert_eq!(&languages2.GetAt(0)?, h!("he-IL"));
    assert_eq!(&languages2.GetAt(1)?, h!("ja-JP"));

    Ok(())
}

#[test]
fn primitive() -> Result<()> {
    let able = IIterable::<i32>::from(vec![]);
    let iter = able.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    assert_eq!(iter.GetMany(&mut [0; 5])?, 0);

    let able = IIterable::<i32>::from(vec![1, 2, 3]);
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

    Ok(())
}

#[test]
fn hstring() -> Result<()> {
    let able = IIterable::<HSTRING>::from(vec![]);
    let iter = able.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let able = IIterable::<HSTRING>::from(vec![
        HSTRING::from("one"),
        HSTRING::from("two"),
        HSTRING::from("three"),
    ]);
    let iter = able.First()?;

    assert_eq!(&iter.Current()?, h!("one"));
    assert_eq!(&iter.Current()?, h!("one"));

    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(&iter.Current()?, h!("two"));
    assert_eq!(&iter.Current()?, h!("two"));
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(&iter.Current()?, h!("three"));
    assert_eq!(&iter.Current()?, h!("three"));
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize_with(5, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert_eq!(
        values,
        [
            HSTRING::from("one"),
            HSTRING::from("two"),
            HSTRING::from("three"),
            HSTRING::default(),
            HSTRING::default()
        ]
    );
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize_with(1, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert_eq!(values, [HSTRING::from("one")]);
    let mut values = vec![];
    values.resize_with(2, Default::default);
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values, [HSTRING::from("two"), HSTRING::from("three")]);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

// `IStringable` lives in `windows::Foundation`, which only compiles on
// Windows. Gate the test that exercises it accordingly. The plain Stock
// adapter behaviour for non-WinRT element types is covered by the
// `primitive` and `hstring` tests above, which run on every platform.
#[cfg(windows)]
#[implement(windows::Foundation::IStringable)]
struct Stringable(HSTRING);

#[cfg(windows)]
impl windows::Foundation::IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.clone())
    }
}

#[cfg(windows)]
fn stringable(value: &str) -> windows::Foundation::IStringable {
    Stringable(value.into()).into()
}

#[cfg(windows)]
#[test]
fn defaulted() -> Result<()> {
    use windows::Foundation::IStringable;
    let able = IIterable::<IStringable>::from(vec![]);
    let iter = able.First()?;

    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);

    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);

    let mut values = vec![];
    values.resize(5, None);
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let able = IIterable::<IStringable>::from(vec![
        Some(stringable("one")),
        Some(stringable("two")),
        Some(stringable("three")),
    ]);
    let iter = able.First()?;

    assert_eq!(iter.Current()?.ToString()?, "one");
    assert_eq!(iter.Current()?.ToString()?, "one");

    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?.ToString()?, "two");
    assert_eq!(iter.Current()?.ToString()?, "two");
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(iter.MoveNext()?);
    assert_eq!(iter.Current()?.ToString()?, "three");
    assert_eq!(iter.Current()?.ToString()?, "three");
    assert!(iter.HasCurrent()?);
    assert!(iter.HasCurrent()?);

    assert!(!iter.MoveNext()?);
    assert!(!iter.MoveNext()?);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert_eq!(iter.Current().unwrap_err().code(), E_BOUNDS);
    assert!(!iter.HasCurrent()?);
    assert!(!iter.HasCurrent()?);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize(5, None);
    assert_eq!(iter.GetMany(&mut values)?, 3);
    assert_eq!(values[0].as_ref().unwrap().ToString()?, "one");
    assert_eq!(values[1].as_ref().unwrap().ToString()?, "two");
    assert_eq!(values[2].as_ref().unwrap().ToString()?, "three");
    assert!(values[3].is_none());
    assert!(values[4].is_none());
    assert_eq!(iter.GetMany(&mut values)?, 0);

    let iter = able.First()?;
    let mut values = vec![];
    values.resize(1, None);
    assert_eq!(iter.GetMany(&mut values)?, 1);
    assert_eq!(values[0].as_ref().unwrap().ToString()?, "one");
    let mut values = vec![];
    values.resize(2, None);
    assert_eq!(iter.GetMany(&mut values)?, 2);
    assert_eq!(values[0].as_ref().unwrap().ToString()?, "two");
    assert_eq!(values[1].as_ref().unwrap().ToString()?, "three");
    assert_eq!(iter.GetMany(&mut values)?, 0);

    Ok(())
}

// Validate the `Iterator` adapter generated by `windows-bindgen` for `IIterator<T>`
// against a normal iterable produced by `IIterable::from(vec)`.
#[test]
fn rust_iterator_adapter() -> Result<()> {
    let able = IIterable::<i32>::from(vec![1, 2, 3]);
    let mut iter = able.First()?;
    let collected: Vec<i32> = (&mut iter).collect();
    assert_eq!(collected, [1, 2, 3]);

    // After the iterator is exhausted, further calls return None and the
    // underlying COM iterator reports !HasCurrent.
    assert!(Iterator::next(&mut iter).is_none());
    assert!(!iter.HasCurrent()?);

    // An empty iterable yields nothing without panicking.
    let empty = IIterable::<i32>::from(vec![]);
    let mut iter = empty.First()?;
    assert!(Iterator::next(&mut iter).is_none());
    assert_eq!((&mut iter).count(), 0);

    Ok(())
}

// Custom `IIterator<i32>` whose `MoveNext` fails on a chosen step. After the
// failure the iterator is considered broken: `HasCurrent` reports an error
// (which the generated adapter treats as `false` via `.unwrap_or(false)`).
// This is used to verify that `Iterator::next` does NOT silently drop the
// item already fetched by `Current` when `MoveNext` returns Err — see
// `windows-bindgen`'s `IIterator` adapter.
#[implement(IIterator<i32>)]
struct FailingIterator {
    values: Vec<i32>,
    current: std::sync::atomic::AtomicUsize,
    failed: std::sync::atomic::AtomicBool,
    fail_move_next_at: usize,
}

impl IIterator_Impl<i32> for FailingIterator_Impl {
    fn Current(&self) -> Result<i32> {
        if self.failed.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(Error::from(E_FAIL));
        }
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        self.values
            .get(current)
            .copied()
            .ok_or_else(|| Error::from(E_BOUNDS))
    }

    fn HasCurrent(&self) -> Result<bool> {
        if self.failed.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(Error::from(E_FAIL));
        }
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        Ok(current < self.values.len())
    }

    fn MoveNext(&self) -> Result<bool> {
        if self.failed.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(Error::from(E_FAIL));
        }
        let prev = self
            .current
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if prev + 1 == self.fail_move_next_at {
            self.failed
                .store(true, std::sync::atomic::Ordering::Relaxed);
            return Err(Error::from(E_FAIL));
        }
        Ok(prev + 1 < self.values.len())
    }

    fn GetMany(&self, _values: &mut [i32]) -> Result<u32> {
        Err(Error::from(E_BOUNDS))
    }
}

// Regression test for the `windows-bindgen` IIterator adapter: when
// `MoveNext` returns Err mid-iteration, the value already fetched by
// `Current` for the current step must still be yielded; the error is
// effectively treated as end-of-stream on the next call (because the
// failed iterator subsequently reports !HasCurrent).
#[test]
fn rust_iterator_adapter_yields_value_when_move_next_fails() {
    // MoveNext fails on the very first call (after reading element 0).
    // Before the fix this would return None and drop the first item.
    let failing: IIterator<i32> = FailingIterator {
        values: vec![10, 20, 30],
        current: 0.into(),
        failed: false.into(),
        fail_move_next_at: 1,
    }
    .into();

    let mut iter = failing;
    assert_eq!(Iterator::next(&mut iter), Some(10));
    // After MoveNext failed the iterator reports an error from HasCurrent,
    // which the adapter treats as end-of-stream.
    assert!(Iterator::next(&mut iter).is_none());

    // MoveNext fails on the second call (after reading element 1), so the
    // first two items must be yielded before the iterator ends.
    let failing: IIterator<i32> = FailingIterator {
        values: vec![10, 20, 30],
        current: 0.into(),
        failed: false.into(),
        fail_move_next_at: 2,
    }
    .into();
    let collected: Vec<i32> = failing.collect();
    assert_eq!(collected, [10, 20]);
}
