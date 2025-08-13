use windows::{core::*, Foundation::*, Win32::Foundation::E_BOUNDS};
use windows_collections::*;

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

#[implement(IStringable)]
struct Stringable(HSTRING);

impl IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.clone())
    }
}

fn stringable(value: &str) -> IStringable {
    Stringable(value.into()).into()
}

#[test]
fn defaulted() -> Result<()> {
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
