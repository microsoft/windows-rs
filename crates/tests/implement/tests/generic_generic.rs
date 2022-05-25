use core::convert::TryInto;
use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::*;

#[implement(
    IVectorView<T>,
)]
struct Thing<T>(Vec<T>)
where
    T: ::windows::core::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> IVectorView_Impl<T> for Thing<T> {
    fn GetAt(&self, index: u32) -> Result<T> {
        self.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }

    fn Size(&self) -> Result<u32> {
        panic!();
    }

    fn IndexOf(&self, _value: &T::DefaultType, _index: &mut u32) -> Result<bool> {
        panic!();
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [T::DefaultType]) -> Result<u32> {
        panic!();
    }
}

impl<T: ::windows::core::RuntimeType + 'static> IIterable_Impl<T> for Thing<T> {
    fn First(&self) -> Result<IIterator<T>> {
        todo!()
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<i32> = Thing(vec![10, 20, 30]).into();
    assert_eq!(10, v.GetAt(0)?);
    assert_eq!(20, v.GetAt(1)?);
    assert_eq!(30, v.GetAt(2)?);

    let v: IVectorView<HSTRING> = Thing(vec!["10".into(), "20".into(), "30".into()]).into();
    assert_eq!("10", v.GetAt(0)?);
    assert_eq!("20", v.GetAt(1)?);
    assert_eq!("30", v.GetAt(2)?);

    let v: IVectorView<IStringable> = Thing(vec![Uri::CreateUri("http://one/")?.try_into().unwrap(), Uri::CreateUri("http://two/")?.try_into().unwrap(), Uri::CreateUri("http://three/")?.try_into().unwrap()]).into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
