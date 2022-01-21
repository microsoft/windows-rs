use windows::core::*;
use windows::Foundation::Collections::*;

// TODO: test whether we can implement two different IIterable's.

#[implement(
    IVectorView<i32>,
    IIterable<i32>,
)]
struct Thing();

#[allow(non_snake_case)]
impl IVectorView_Impl<i32> for Thing {
    fn GetAt(&mut self, index: u32) -> Result<i32> {
        Ok(index as _)
    }

    fn Size(&mut self) -> Result<u32> {
        Ok(123)
    }

    fn IndexOf(&mut self, value: &i32, index: &mut u32) -> Result<bool> {
        *index = *value as _;
        Ok(true)
    }

    fn GetMany(&mut self, _startindex: u32, _items: &mut [i32]) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<i32> for Thing {
    fn First(&mut self) -> Result<IIterator<i32>> {
        panic!();
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<i32> = Thing().into();
    assert_eq!(012, v.GetAt(012)?);
    assert_eq!(123, v.Size()?);
    let mut index = 0;
    assert_eq!(true, v.IndexOf(456, &mut index)?);
    assert_eq!(456, index);

    Ok(())
}
