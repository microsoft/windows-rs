use windows::core::*;
use windows::Foundation::*;
use windows_collections::*;

#[implement(
    IVectorView<T>,
)]
struct Thing<T>(Vec<T>)
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq;

impl<T> IVectorView_Impl<T> for Thing_Impl<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        self.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }

    fn Size(&self) -> Result<u32> {
        panic!();
    }

    fn IndexOf(&self, _value: Ref<T>, _index: &mut u32) -> Result<bool> {
        panic!();
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [T::Default]) -> Result<u32> {
        panic!();
    }
}

impl<T> IIterable_Impl<T> for Thing_Impl<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        unimplemented!()
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

    let url1: HSTRING = "http://one/".into();
    let url2: HSTRING = "http://two/".into();
    let url3: HSTRING = "http://three/".into();
    let v: IVectorView<IStringable> = Thing(vec![
        Uri::CreateUri(&url1)?.cast()?,
        Uri::CreateUri(&url2)?.cast()?,
        Uri::CreateUri(&url3)?.cast()?,
    ])
    .into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
