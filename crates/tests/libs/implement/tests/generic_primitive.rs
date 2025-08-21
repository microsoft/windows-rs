use windows::core::*;
use windows_collections::*;

// TODO: test whether we can implement two different IIterable's.

#[implement(
    IVectorView<i32>,
    IIterable<i32>,
)]
struct Thing();

impl IVectorView_Impl<i32> for Thing_Impl {
    fn GetAt(&self, index: u32) -> Result<i32> {
        Ok(index as i32)
    }

    fn Size(&self) -> Result<u32> {
        Ok(123)
    }

    fn IndexOf(&self, value: Ref<i32>, index: &mut u32) -> Result<bool> {
        *index = *value as u32;
        Ok(true)
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [i32]) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<i32> for Thing_Impl {
    fn First(&self) -> Result<IIterator<i32>> {
        panic!();
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<i32> = Thing().into();
    assert_eq!(12, v.GetAt(12)?);
    assert_eq!(123, v.Size()?);
    let mut index = 0;
    assert!(v.IndexOf(456, &mut index)?);
    assert_eq!(456, index);

    Ok(())
}
