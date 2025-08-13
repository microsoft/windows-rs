use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::E_BOUNDS;
use windows_collections::*;

#[implement(
    IVectorView<T>,
)]
struct Thing<T>(Vec<T::Default>)
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq;

impl<T> IVectorView_Impl<T> for Thing_Impl<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        match self.0.get(index as usize) {
            Some(value) => T::from_default(value),
            None => Err(Error::new(E_BOUNDS, "")),
        }
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.0.len() as u32)
    }

    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        match self.0.iter().position(|element| element == &*value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [T::Default]) -> Result<u32> {
        panic!();
    }
}

impl<T> IIterable_Impl<T> for Thing_Impl<T>
where
    T: RuntimeType + 'static,
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
    assert_eq!(3, v.Size()?);

    let mut index = 0;
    assert!(v.IndexOf(20, &mut index)?);
    assert_eq!(1, index);
    assert!(v.IndexOf(30, &mut index)?);
    assert_eq!(2, index);
    assert!(!(v.IndexOf(123, &mut index)?));

    let v: IVectorView<HSTRING> = Thing(vec!["10".into(), "20".into(), "30".into()]).into();
    assert_eq!("10", v.GetAt(0)?);
    assert_eq!("20", v.GetAt(1)?);
    assert_eq!("30", v.GetAt(2)?);
    assert_eq!(3, v.Size()?);

    let mut index = 0;
    assert!(v.IndexOf(&HSTRING::from("20"), &mut index)?);
    assert_eq!(1, index);
    assert!(v.IndexOf(&HSTRING::from("30"), &mut index)?);
    assert_eq!(2, index);
    assert!(!(v.IndexOf(&HSTRING::from("123"), &mut index)?));

    let url1: HSTRING = "http://one/".into();
    let url2: HSTRING = "http://two/".into();
    let url3: HSTRING = "http://three/".into();
    let v: IVectorView<IStringable> = Thing(vec![
        Some(Uri::CreateUri(&url1)?.cast()?),
        Some(Uri::CreateUri(&url2)?.cast()?),
        Some(Uri::CreateUri(&url3)?.cast()?),
    ])
    .into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);
    assert_eq!(3, v.Size()?);

    Ok(())
}
