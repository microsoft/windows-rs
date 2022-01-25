use core::convert::TryInto;
use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::*;

#[implement(
    windows::Foundation::Collections::IVectorView<windows::Foundation::IStringable>,
)]
struct Thing(Vec<IStringable>);

#[allow(non_snake_case)]
impl IVectorView_Impl<IStringable> for Thing {
    fn GetAt(&mut self, index: u32) -> Result<IStringable> {
        self.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }

    fn Size(&mut self) -> Result<u32> {
        panic!();
    }

    fn IndexOf(&mut self, _value: &Option<IStringable>, _index: &mut u32) -> Result<bool> {
        panic!();
    }

    fn GetMany(&mut self, _startindex: u32, _items: &mut [Option<IStringable>]) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<IStringable> for Thing {
    fn First(&mut self) -> Result<IIterator<IStringable>> {
        todo!()
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<IStringable> = Thing(vec![Uri::CreateUri("http://one/")?.try_into().unwrap(), Uri::CreateUri("http://two/")?.try_into().unwrap(), Uri::CreateUri("http://three/")?.try_into().unwrap()]).into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
