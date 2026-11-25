#![cfg(windows)]
use test_implement::E_BOUNDS;
use windows_collections::*;
use windows_core::imp::Type;
use windows_core::*;

#[implement(
    IIterator<T>,
)]
struct Iterator<T>
where
    T: RuntimeType + Send + Sync + Clone + Into<<T as Type>::Default> + 'static,
    <T as Type>::Default: PartialEq,
{
    owner: ComObject<Iterable<T>>,
    current: std::sync::Mutex<usize>,
}

impl<T> IIterator_Impl<T> for Iterator_Impl<T>
where
    T: RuntimeType + Send + Sync + Clone + Into<<T as Type>::Default> + 'static,
    <T as Type>::Default: PartialEq,
{
    fn Current(&self) -> Result<T> {
        let interface: IIterable<T> = self.owner.to_interface();
        let owner: &Iterable<T> = unsafe { interface.as_impl() };
        let current = *self.current.lock().unwrap();
        owner
            .0
            .get(current)
            .cloned()
            .ok_or_else(|| Error::new(E_BOUNDS, ""))
    }

    fn HasCurrent(&self) -> Result<bool> {
        let interface: IIterable<T> = self.owner.to_interface();
        let owner: &Iterable<T> = unsafe { interface.as_impl() };
        Ok(owner.0.len() > *self.current.lock().unwrap())
    }

    fn MoveNext(&self) -> Result<bool> {
        let interface: IIterable<T> = self.owner.to_interface();
        let owner: &Iterable<T> = unsafe { interface.as_impl() };
        let mut current = self.current.lock().unwrap();
        *current += 1;
        Ok(owner.0.len() > *current)
    }

    fn GetMany(&self, items: &mut [T::Default]) -> Result<u32> {
        let interface: IIterable<T> = self.owner.to_interface();
        let owner: &Iterable<T> = unsafe { interface.as_impl() };
        let range = {
            let mut current = self.current.lock().unwrap();
            let start = (*current).min(owner.0.len());
            let actual = (owner.0.len() - start).min(items.len());
            *current = start + actual;
            start..start + actual
        };
        let actual = range.len();
        for (src, dst) in owner.0[range].iter().zip(items.iter_mut()) {
            *dst = src.clone().into();
        }
        Ok(actual as u32)
    }
}

#[implement(
    IIterable<T>,
)]
struct Iterable<T>(Vec<T>)
where
    T: RuntimeType + Send + Sync + Clone + Into<<T as Type>::Default> + 'static,
    <T as Type>::Default: PartialEq;

impl<T> IIterable_Impl<T> for Iterable_Impl<T>
where
    T: RuntimeType + Send + Sync + Clone + Into<<T as Type>::Default> + 'static,
    <T as Type>::Default: PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        Ok(Iterator::<T> {
            owner: self.to_object(),
            current: std::sync::Mutex::new(0),
        }
        .into())
    }
}

#[test]
fn test_collect() -> Result<()> {
    let source: IIterable<i32> = Iterable(vec![10, 20, 30]).into();

    let values: Vec<i32> = source.into_iter().collect();

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

#[test]
fn test_get_many() -> Result<()> {
    let iterable: IIterable<i32> = Iterable(vec![10, 20, 30]).into();
    let iterator = iterable.First()?;
    let mut values = [0; 2];
    assert_eq!(iterator.GetMany(&mut values)?, 2);
    assert_eq!(values, [10, 20]);
    assert_eq!(iterator.Current()?, 30);
    assert_eq!(iterator.GetMany(&mut values)?, 1);
    assert_eq!(values[0], 30);
    assert!(!iterator.MoveNext()?);
    assert_eq!(iterator.GetMany(&mut values)?, 0);
    assert!(!iterator.HasCurrent()?);
    Ok(())
}
