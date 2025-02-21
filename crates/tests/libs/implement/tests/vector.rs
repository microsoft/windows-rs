#![allow(non_snake_case)]

use std::sync::RwLock;
use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::*;
use windows_collections::*;

pub(crate) fn err_bounds() -> Error {
    E_BOUNDS.into()
}

pub(crate) fn err_memory() -> Error {
    E_OUTOFMEMORY.into()
}

#[implement(
    IVector<T>,
    IVectorView<T>,
)]
struct Vector<T>(std::sync::RwLock<Vec<T::Default>>)
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone;

impl<T> Vector<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone,
{
    fn new(vec: Vec<T::Default>) -> Self {
        Self(RwLock::new(vec))
    }
}

impl<T> Vector_Impl<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone,
{
    // Methods common to IVector and IVectorView:
    fn GetAt(&self, index: u32) -> Result<T> {
        let reader = self.0.read().unwrap();
        let item = reader.get(index as usize).ok_or_else(err_bounds)?;
        T::from_default(item)
    }
    fn Size(&self) -> Result<u32> {
        let reader = self.0.read().unwrap();
        Ok(reader.len() as u32)
    }
    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        let reader = self.0.read().unwrap();
        match reader.iter().position(|element| element == &*value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    fn GetMany(&self, _startindex: u32, _items: &mut [T::Default]) -> Result<u32> {
        unimplemented!();
    }
}

impl<T> IVector_Impl<T> for Vector_Impl<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        self.GetAt(index)
    }
    fn Size(&self) -> Result<u32> {
        self.Size()
    }
    fn GetView(&self) -> Result<IVectorView<T>> {
        Ok(self.to_interface())
    }
    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        self.IndexOf(value, result)
    }
    fn SetAt(&self, index: u32, value: Ref<T>) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        let item = writer.get_mut(index as usize).ok_or_else(err_bounds)?;
        *item = value.clone();
        Ok(())
    }
    fn InsertAt(&self, index: u32, value: Ref<T>) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        let index = index as usize;
        if index > writer.len() {
            Err(err_bounds())
        } else {
            let len = writer.len();
            writer.try_reserve(len + 1).map_err(|_| err_memory())?;
            writer.insert(index, value.clone());
            Ok(())
        }
    }
    fn RemoveAt(&self, index: u32) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        let index = index as usize;
        if index < writer.len() {
            writer.remove(index);
            Ok(())
        } else {
            Err(err_bounds())
        }
    }
    fn Append(&self, value: Ref<T>) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        let len = writer.len();
        writer.try_reserve(len + 1).map_err(|_| err_memory())?;
        writer.insert(len, value.clone());
        Ok(())
    }
    fn RemoveAtEnd(&self) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        if !writer.is_empty() {
            let len = writer.len();
            writer.truncate(len - 1);
        }
        Ok(())
    }
    fn Clear(&self) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer.clear();
        Ok(())
    }
    fn GetMany(&self, startindex: u32, items: &mut [T::Default]) -> Result<u32> {
        self.GetMany(startindex, items)
    }
    fn ReplaceAll(&self, items: &[T::Default]) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer
            .try_reserve(items.len() + 1)
            .map_err(|_| err_memory())?;
        for item in items {
            writer.push(item.clone());
        }
        Ok(())
    }
}

impl<T> IVectorView_Impl<T> for Vector_Impl<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        self.GetAt(index)
    }
    fn Size(&self) -> Result<u32> {
        self.Size()
    }
    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        self.IndexOf(value, result)
    }
    fn GetMany(&self, startindex: u32, items: &mut [T::Default]) -> Result<u32> {
        self.GetMany(startindex, items)
    }
}

impl<T> IIterable_Impl<T> for Vector_Impl<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq + Clone,
{
    fn First(&self) -> Result<IIterator<T>> {
        unimplemented!()
    }
}

#[test]
fn GetAt() -> Result<()> {
    let v: IVector<i32> = Vector::new(vec![123]).into();
    assert_eq!(v.GetAt(0)?, 123);
    assert_eq!(v.GetAt(1).unwrap_err().code(), E_BOUNDS);

    let v: IVector<IStringable> = Vector::new(vec![
        Some(Uri::CreateUri(&HSTRING::from("http://test/"))?.cast()?),
        None,
    ])
    .into();
    assert_eq!(v.GetAt(0)?.ToString()?, "http://test/");
    assert_eq!(v.GetAt(1).unwrap_err().code(), S_OK);

    Ok(())
}

#[test]
fn Size() -> Result<()> {
    let v: IVector<i32> = Vector::new(vec![]).into();
    assert_eq!(v.Size()?, 0);
    v.Append(123)?;
    assert_eq!(v.Size()?, 1);
    Ok(())
}

#[test]
fn IndexOf() -> Result<()> {
    let v: IVector<i32> = Vector::new(vec![123, 456]).into();
    let mut index = 0;
    assert!(v.IndexOf(123, &mut index)?);
    assert_eq!(index, 0);
    assert!(v.IndexOf(456, &mut index)?);
    assert_eq!(index, 1);
    assert!(!(v.IndexOf(789, &mut index)?));

    let uri = Uri::CreateUri(&HSTRING::from("http://test/"))?;
    let v: IVector<IStringable> = Vector::new(vec![Some(uri.cast()?), None]).into();
    assert!(v.IndexOf(&uri.cast::<IStringable>()?, &mut index)?);
    assert_eq!(index, 0);
    assert!(v.IndexOf(None, &mut index)?);
    assert_eq!(index, 1);
    assert!(
        !(v.IndexOf(
            &Uri::CreateUri(&HSTRING::from("http://test/"))?.cast::<IStringable>()?,
            &mut index
        )?)
    );

    Ok(())
}

#[test]
fn GetView() -> Result<()> {
    let vector: IVector<i32> = Vector::new(vec![123, 456, 789]).into();
    let view: IVectorView<i32> = vector.GetView()?;
    assert_eq!(view.Size()?, 3);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let v: IVector<i32> = Vector::new(vec![10, 20, 30]).into();
    assert_eq!(10, v.GetAt(0)?);
    assert_eq!(20, v.GetAt(1)?);
    assert_eq!(30, v.GetAt(2)?);
    assert!(v.GetAt(20).is_err());
    assert_eq!(3, v.Size()?);
    let c: &IInspectable = unsafe { std::mem::transmute(&v) };
    assert_eq!(
        c.GetRuntimeClassName()?,
        "Windows.Foundation.Collections.IVector"
    ); // TODO: needs to have `1<Int32>

    let mut index = 0;
    assert!(v.IndexOf(20, &mut index)?);
    assert_eq!(1, index);
    assert!(v.IndexOf(30, &mut index)?);
    assert_eq!(2, index);
    assert!(!(v.IndexOf(123, &mut index)?));

    let v: IVectorView<HSTRING> = Vector::new(vec!["10".into(), "20".into(), "30".into()]).into();
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

    let v: IVectorView<IStringable> = Vector::new(vec![
        Some(Uri::CreateUri(&HSTRING::from("http://one/"))?.cast()?),
        Some(Uri::CreateUri(&HSTRING::from("http://two/"))?.cast()?),
        Some(Uri::CreateUri(&HSTRING::from("http://three/"))?.cast()?),
    ])
    .into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);
    assert_eq!(3, v.Size()?);

    Ok(())
}

// Test for https://github.com/microsoft/windows-rs/issues/2759
#[test]
fn test_2759() -> Result<()> {
    let v: IVector<IStringable> = Vector::new(vec![]).into();
    let uri = Uri::CreateUri(h!("https://github.com/"))?;
    v.Append(&uri)?;
    let uri = Uri::CreateUri(h!("https://microsoft.com/"))?;
    v.Append(&uri)?;
    v.Append(&uri.cast::<IStringable>()?)?;

    assert_eq!(&v.GetAt(0)?.ToString()?, h!("https://github.com/"));
    assert_eq!(&v.GetAt(1)?.ToString()?, h!("https://microsoft.com/"));

    Ok(())
}

#[test]
fn test_into_param() -> Result<()> {
    let v: IVector<i32> = Vector::new(vec![]).into();
    v.Append(1)?;
    v.Append(Some(&2))?;
    v.Append(None)?;

    assert_eq!(v.GetAt(0)?, 1);
    assert_eq!(v.GetAt(1)?, 2);
    assert_eq!(v.GetAt(2)?, 0);

    Ok(())
}
