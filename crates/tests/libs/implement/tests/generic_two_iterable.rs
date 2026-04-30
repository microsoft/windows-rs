#![cfg(windows)]
use windows::core::*;
use windows_collections::*;

#[implement(
    IIterator<i32>,
    IIterator<HSTRING>,
)]
struct Thing();

impl IIterator_Impl<i32> for Thing_Impl {
    fn Current(&self) -> Result<i32> {
        Ok(123)
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(true)
    }

    fn MoveNext(&self) -> Result<bool> {
        Ok(false)
    }

    fn GetMany(&self, _items: &mut [i32]) -> Result<u32> {
        Ok(0)
    }
}

impl IIterator_Impl<HSTRING> for Thing_Impl {
    fn Current(&self) -> Result<HSTRING> {
        Ok("hello".into())
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(true)
    }

    fn MoveNext(&self) -> Result<bool> {
        Ok(false)
    }

    fn GetMany(&self, _items: &mut [HSTRING]) -> Result<u32> {
        Ok(0)
    }
}

#[test]
fn test_implement() -> Result<()> {
    let a: IIterator<i32> = Thing().into();
    assert_eq!(a.Current()?, 123);

    let b: IIterator<HSTRING> = a.cast()?;
    assert_eq!(b.Current()?, "hello");

    Ok(())
}
