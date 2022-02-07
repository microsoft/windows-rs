#![allow(non_snake_case)]

use std::convert::TryInto;
use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::*;
use windows::Win32::Foundation::*;

pub(crate) fn err_bounds() -> Error {
    Error::fast_error(E_BOUNDS)
}

pub(crate) fn err_memory() -> Error {
    Error::fast_error(E_OUTOFMEMORY)
}

#[implement(
    IVector<T>,
    IVectorView<T>,
)]
struct Vector<T>(Vec<T::DefaultType>)
where
    T: ::windows::core::RuntimeType;

impl<T: ::windows::core::RuntimeType + 'static> Vector<T> {
    // Methods common to IVector and IVectorView:
    fn GetAt(&mut self, index: u32) -> Result<T> {
        let item = self.0.get(index as usize).ok_or_else(|| err_bounds())?;
        T::from_default(item)
    }
    fn Size(&mut self) -> Result<u32> {
        Ok(self.0.len() as _)
    }
    fn IndexOf(&mut self, value: &T::DefaultType, result: &mut u32) -> Result<bool> {
        match self.0.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as _;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    fn GetMany(&mut self, _startindex: u32, _items: &mut [T::DefaultType]) -> Result<u32> {
        todo!();
    }
}

impl<T: ::windows::core::RuntimeType + 'static> IVector_Impl<T> for Vector<T> {
    fn GetAt(&mut self, index: u32) -> Result<T> {
        self.GetAt(index)
    }
    fn Size(&mut self) -> Result<u32> {
        self.Size()
    }
    fn GetView(&mut self) -> Result<windows::Foundation::Collections::IVectorView<T>> {
        self.cast()
    }
    fn IndexOf(&mut self, value: &T::DefaultType, result: &mut u32) -> Result<bool> {
        self.IndexOf(value, result)
    }
    fn SetAt(&mut self, index: u32, value: &T::DefaultType) -> Result<()> {
        let item = self.0.get_mut(index as usize).ok_or_else(|| err_bounds())?;
        *item = value.clone();
        Ok(())
    }
    fn InsertAt(&mut self, index: u32, value: &T::DefaultType) -> Result<()> {
        let index = index as usize;
        if index > self.0.len() {
            Err(err_bounds())
        } else {
            self.0.try_reserve(self.0.len() + 1).map_err(|_| err_memory())?;
            self.0.insert(index as _, value.clone());
            Ok(())
        }
    }
    fn RemoveAt(&mut self, index: u32) -> Result<()> {
        let index = index as usize;
        if index < self.0.len() {
            self.0.remove(index);
            Ok(())
        } else {
            Err(err_bounds())
        }
    }
    fn Append(&mut self, value: &T::DefaultType) -> Result<()> {
        self.InsertAt(self.0.len() as _, value)
    }
    fn RemoveAtEnd(&mut self) -> Result<()> {
        self.RemoveAt(self.0.len() as u32 - 1)
    }
    fn Clear(&mut self) -> Result<()> {
        self.0.clear();
        Ok(())
    }
    fn GetMany(&mut self, startindex: u32, items: &mut [T::DefaultType]) -> Result<u32> {
        self.GetMany(startindex, items)
    }
    fn ReplaceAll(&mut self, items: &[T::DefaultType]) -> Result<()> {
        self.0.try_reserve(items.len() + 1).map_err(|_| err_memory())?;
        for item in items {
            self.0.push(item.clone());
        }
        Ok(())
    }
}

impl<T: ::windows::core::RuntimeType + 'static> IVectorView_Impl<T> for Vector<T> {
    fn GetAt(&mut self, index: u32) -> Result<T> {
        self.GetAt(index)
    }
    fn Size(&mut self) -> Result<u32> {
        self.Size()
    }
    fn IndexOf(&mut self, value: &T::DefaultType, result: &mut u32) -> Result<bool> {
        self.IndexOf(value, result)
    }
    fn GetMany(&mut self, startindex: u32, items: &mut [T::DefaultType]) -> Result<u32> {
        self.GetMany(startindex, items)
    }
}

impl<T: ::windows::core::RuntimeType + 'static> IIterable_Impl<T> for Vector<T> {
    fn First(&mut self) -> Result<IIterator<T>> {
        todo!()
    }
}

#[test]
fn GetAt() -> Result<()> {
    let v: IVector<i32> = Vector(vec![123]).into();
    assert_eq!(v.GetAt(0)?, 123);
    assert_eq!(v.GetAt(1).unwrap_err().code(), E_BOUNDS);

    let v: IVector<IStringable> = Vector(vec![Some(Uri::CreateUri("http://test/")?.cast()?), None]).into();
    assert_eq!(v.GetAt(0)?.ToString()?, "http://test/");
    assert_eq!(v.GetAt(1).unwrap_err().code(), S_OK);

    Ok(())
}

#[test]
fn Size() -> Result<()> {
    let v: IVector<i32> = Vector(vec![]).into();
    assert_eq!(v.Size()?, 0);
    v.Append(123)?;
    assert_eq!(v.Size()?, 1);
    Ok(())
}

#[test]
fn IndexOf() -> Result<()> {
    let v: IVector<i32> = Vector(vec![123, 456]).into();
    let mut index = 0;
    assert_eq!(v.IndexOf(123, &mut index)?, true);
    assert_eq!(index, 0);
    assert_eq!(v.IndexOf(456, &mut index)?, true);
    assert_eq!(index, 1);
    assert_eq!(v.IndexOf(789, &mut index)?, false);

    let uri = Uri::CreateUri("http://test/")?;
    let v: IVector<IStringable> = Vector(vec![Some(uri.cast()?), None]).into();
    assert_eq!(v.IndexOf(uri, &mut index)?, true);
    assert_eq!(index, 0);
    assert_eq!(v.IndexOf(None, &mut index)?, true);
    assert_eq!(index, 1);
    assert_eq!(v.IndexOf(Uri::CreateUri("http://test/")?, &mut index)?, false);

    Ok(())
}

#[test]
fn GetView() -> Result<()> {
    let vector: IVector<i32> = Vector(vec![123, 456, 789]).into();
    let view: IVectorView<i32> = vector.GetView()?;
    assert_eq!(view.Size()?, 3);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let v: IVector<i32> = Vector(vec![10, 20, 30]).into();
    assert_eq!(10, v.GetAt(0)?);
    assert_eq!(20, v.GetAt(1)?);
    assert_eq!(30, v.GetAt(2)?);
    assert!(v.GetAt(20).is_err());
    assert_eq!(3, v.Size()?);
    let c: IInspectable = (&v).into();
    assert_eq!(c.GetRuntimeClassName()?, "Windows.Foundation.Collections.IVector"); // TODO: needs to have `1<Int32>

    let mut index = 0;
    assert_eq!(true, v.IndexOf(20, &mut index)?);
    assert_eq!(1, index);
    assert_eq!(true, v.IndexOf(30, &mut index)?);
    assert_eq!(2, index);
    assert_eq!(false, v.IndexOf(123, &mut index)?);

    let v: IVectorView<HSTRING> = Vector(vec!["10".into(), "20".into(), "30".into()]).into();
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

    let v: IVectorView<IStringable> = Vector(vec![Some(Uri::CreateUri("http://one/")?.try_into().unwrap()), Some(Uri::CreateUri("http://two/")?.try_into().unwrap()), Some(Uri::CreateUri("http://three/")?.try_into().unwrap())]).into();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);
    assert_eq!(3, v.Size()?);

    Ok(())
}
