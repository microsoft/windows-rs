use windows::core::*;
use windows::Win32::Foundation::E_BOUNDS;
use windows_collections::*;

#[implement(
    IIterator<T>,
)]
struct Iterator<T>(std::cell::UnsafeCell<(IIterable<T>, usize)>)
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq;

impl<T> IIterator_Impl<T> for Iterator_Impl<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn Current(&self) -> Result<T> {
        unsafe {
            let this = self.0.get();
            let owner = (*this).0.as_impl();

            if owner.0.len() > (*this).1 {
                Ok(owner.0[(*this).1].clone())
            } else {
                Err(Error::new(E_BOUNDS, ""))
            }
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        unsafe {
            let this = self.0.get();
            let owner = (*this).0.as_impl();
            Ok(owner.0.len() > (*this).1)
        }
    }

    fn MoveNext(&self) -> Result<bool> {
        unsafe {
            let this = self.0.get();
            let owner = (*this).0.as_impl();
            (*this).1 += 1;
            Ok(owner.0.len() > (*this).1)
        }
    }

    fn GetMany(&self, _items: &mut [T::Default]) -> Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}

#[implement(
    IIterable<T>,
)]
struct Iterable<T>(Vec<T>)
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq;

impl<T> IIterable_Impl<T> for Iterable_Impl<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        Ok(Iterator::<T>((self.to_interface::<IIterable<T>>(), 0).into()).into())
    }
}

#[test]
fn test_collect() -> Result<()> {
    let source: IIterable<i32> = Iterable(vec![10, 20, 30]).into();

    // TODO: not sure why this won't compile.
    // let values: Vec<i32> = source.collect();

    let mut values = Vec::<i32>::new();

    for value in source {
        values.push(value);
    }

    assert_eq!(values, &[10, 20, 30]);

    Ok(())
}

#[test]
fn test_explicit() -> Result<()> {
    let iterable: IIterable<i32> = Iterable(vec![10, 20, 30]).into();
    let it1 = iterable.First()?;

    assert_eq!(it1.Current()?, 10);
    assert!(it1.HasCurrent()?);
    assert!(it1.MoveNext()?);

    assert_eq!(it1.Current()?, 20);
    assert!(it1.HasCurrent()?);
    assert!(it1.MoveNext()?);

    assert_eq!(it1.Current()?, 30);
    assert!(it1.HasCurrent()?);
    assert!(!(it1.MoveNext()?));

    assert!(it1.Current().is_err());
    assert!(!(it1.HasCurrent()?));
    assert!(!(it1.MoveNext()?));

    // The following just validates that iterators are independent and stable.

    let it2 = iterable.First()?;

    assert_eq!(it2.Current()?, 10);
    assert!(it2.HasCurrent()?);
    assert!(it1.Current().is_err());
    assert!(!(it1.HasCurrent()?));

    assert_eq!(it2.Current()?, 10);
    assert!(it2.HasCurrent()?);
    assert!(it1.Current().is_err());
    assert!(!(it1.HasCurrent()?));

    Ok(())
}
