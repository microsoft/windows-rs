use std::convert::TryInto;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Foundation::*;

#[implement(
    Windows::Foundation::Collections::IVectorView<Windows::Foundation::IStringable>,
)]
struct Thing(Vec<IStringable>);

#[allow(non_snake_case)]
impl Thing {
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

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<IStringable> = Thing(vec![
        Uri::CreateUri("http://one/")?.try_into().unwrap(),
        Uri::CreateUri("http://two/")?.try_into().unwrap(),
        Uri::CreateUri("http://three/")?.try_into().unwrap(),
    ])
    .into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
