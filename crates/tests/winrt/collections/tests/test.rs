use test_winrt_collections::*;
use windows as Windows;
use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::IStringable;
use Component::Collections::*;

#[implement(Windows::Foundation::IStringable)]
struct TestStringable(HSTRING);

#[allow(non_snake_case)]
impl TestStringable {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.clone())
    }
}

#[test]
fn int32() -> Result<()> {
    let v = Test::CreateInt32Vector()?;
    assert_eq!(v.Size()?, 0);
    v.Append(1)?;
    assert_eq!(v.Size()?, 1);
    assert_eq!(v.GetAt(0)?, 1);
    v.ReplaceAll(&[1, 2, 3])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 3);

    v.SetAt(0, 10)?;
    assert_eq!(v.GetAt(0)?, 10);

    Ok(())
}

#[test]
fn string() -> Result<()> {
    let v = Test::CreateStringVector()?;
    assert_eq!(v.Size()?, 0);
    v.Append("one")?;
    assert_eq!(v.Size()?, 1);
    assert_eq!(v.GetAt(0)?, "one");
    v.ReplaceAll(&["one".into(), "two".into(), "three".into()])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?, "one");
    assert_eq!(v.GetAt(1)?, "two");
    assert_eq!(v.GetAt(2)?, "three");

    v.SetAt(0, "ONE")?;
    assert_eq!(v.GetAt(0)?, "ONE");

    Ok(())
}

#[test]
fn stringable() -> Result<()> {
    let one: IStringable = TestStringable("one".into()).into();
    let two: IStringable = TestStringable("two".into()).into();
    let three: IStringable = TestStringable("three".into()).into();

    let v = Test::CreateStringableVector()?;
    assert_eq!(v.Size()?, 0);
    v.Append(&one)?;
    assert_eq!(v.Size()?, 1);
    assert_eq!(v.GetAt(0)?.ToString()?, "one");
    v.ReplaceAll(&[Some(one), Some(two), Some(three)])?;
    assert_eq!(v.Size()?, 3);
    assert_eq!(v.GetAt(0)?.ToString()?, "one");
    assert_eq!(v.GetAt(1)?.ToString()?, "two");
    assert_eq!(v.GetAt(2)?.ToString()?, "three");

    let one: IStringable = TestStringable("ONE".into()).into();
    v.SetAt(0, one)?;
    assert_eq!(v.GetAt(0)?.ToString()?, "ONE");

    Ok(())
}

#[test]
fn vector_iter() -> Result<()> {
    let vector = Test::CreateInt32Vector()?;
    vector.ReplaceAll(&[1, 2, 3])?;

    let values: Vec<i32> = vector.into_iter().collect();
    assert!(values == [1, 2, 3]);

    Ok(())
}

#[test]
fn vector_view_iter() -> Result<()> {
    let vector = Test::CreateInt32Vector()?;
    vector.ReplaceAll(&[1, 2, 3])?;

    let view = vector.GetView()?;

    let values: Vec<i32> = view.into_iter().collect();
    assert!(values == [1, 2, 3]);

    Ok(())
}

#[test]
fn iterable_iter() -> Result<()> {
    let vector = Test::CreateInt32Vector()?;
    vector.ReplaceAll(&[1, 2, 3])?;

    let view: IIterable<i32> = vector.cast()?;

    let values: Vec<i32> = view.into_iter().collect();
    assert!(values == [1, 2, 3]);

    Ok(())
}
