use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Foundation::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IVectorView<T>,
)]
struct Thing<T>(Vec<T::DefaultType>)
where
    T: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: ::windows::RuntimeType + 'static> Thing<T> {
    fn GetAt(&self, index: u32) -> Result<T> {
        match self.0.get(index as usize) {
            Some(value) => unsafe { <T as Abi>::from_default(value) },
            None => Err(Error::new(E_BOUNDS, "")),
        }
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.0.len() as _)
    }

    fn IndexOf(&self, value: &T::DefaultType, result: &mut u32) -> Result<bool> {
        match self.0.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as _;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn GetMany(&self, _startindex: u32, _items: &mut [T]) -> Result<u32> {
        panic!();
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
    assert_eq!(true, v.IndexOf(20, &mut index)?);
    assert_eq!(1, index);
    assert_eq!(true, v.IndexOf(30, &mut index)?);
    assert_eq!(2, index);
    assert_eq!(false, v.IndexOf(123, &mut index)?);

    let v: IVectorView<HSTRING> = Thing(vec!["10".into(), "20".into(), "30".into()]).into();
    assert_eq!("10", v.GetAt(0)?);
    assert_eq!("20", v.GetAt(1)?);
    assert_eq!("30", v.GetAt(2)?);
    assert_eq!(3, v.Size()?);

    let mut index = 0;
    assert_eq!(true, v.IndexOf("20", &mut index)?);
    assert_eq!(1, index);
    assert_eq!(true, v.IndexOf("30", &mut index)?);
    assert_eq!(2, index);
    assert_eq!(false, v.IndexOf("123", &mut index)?);

    let v: IVectorView<IStringable> = Thing(vec![
        Some(Uri::CreateUri("http://one/")?.into()),
        Some(Uri::CreateUri("http://two/")?.into()),
        Some(Uri::CreateUri("http://three/")?.into()),
    ])
    .into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);
    assert_eq!(3, v.Size()?);

    Ok(())
}
