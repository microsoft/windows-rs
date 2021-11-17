use windows::core::*;
use windows::Foundation::Collections::*;
use windows as Windows;

#[implement(
    Windows::Foundation::Collections::IVectorView<i32>,
    Windows::Foundation::Collections::IIterable<i32>,
)]
struct Thing();

#[allow(non_snake_case)]
impl Thing {
    pub fn GetAt(&self, index: u32) -> Result<i32> {
        Ok(index as _)
    }

    pub fn Size(&self) -> Result<u32> {
        Ok(123)
    }

    pub fn IndexOf(&self, value: i32, index: &mut u32) -> Result<bool> {
        *index = value as _;
        Ok(true)
    }

    pub fn GetMany(&self, _startindex: u32, _items: &mut [i32]) -> Result<u32> {
        panic!();
    }

    fn First(&self) -> Result<IIterator<i32>> {
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
