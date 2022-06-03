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
    fn GetAt(&self, index: u32) -> Result<IStringable> {
        self.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }

    fn Size(&self) -> Result<u32> {
        panic!();
    }

    fn IndexOf(&self, _value: &Option<IStringable>, _index: &mut u32) -> Result<bool> {
        panic!();
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [Option<IStringable>]) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<IStringable> for Thing {
    fn First(&self) -> Result<IIterator<IStringable>> {
        todo!()
    }
}

#[test]
fn test_implement() -> Result<()> {
    let url1: HSTRING = "http://one/".into();
    let url2: HSTRING = "http://two/".into();
    let url3: HSTRING = "http://three/".into();
    let v: IVectorView<IStringable> = Thing(vec![Uri::CreateUri(&url1)?.try_into().unwrap(), Uri::CreateUri(&url2)?.try_into().unwrap(), Uri::CreateUri(&url3)?.try_into().unwrap()]).into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
