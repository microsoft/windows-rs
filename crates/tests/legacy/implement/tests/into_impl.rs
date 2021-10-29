use test_implement::*;
use windows::runtime::*;
use Windows::Foundation::Collections::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IIterator<T>,
)]
struct Iterator<T>
where
    T: RuntimeType + 'static,
{
    owner: IIterable<T>,
    current: usize,
}

#[allow(non_snake_case)]
impl<T: RuntimeType + 'static> Iterator<T> {
    fn Current(&self) -> Result<T> {
        let owner = unsafe { Iterable::to_impl(&self.owner) };

        if owner.0.len() > self.current {
            Ok(owner.0[self.current].clone())
        } else {
            Err(Error::new(E_BOUNDS, ""))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let owner = unsafe { Iterable::to_impl(&self.owner) };
        Ok(owner.0.len() > self.current)
    }

    fn MoveNext(&mut self) -> Result<bool> {
        let owner = unsafe { Iterable::to_impl(&self.owner) };
        self.current += 1;
        Ok(owner.0.len() > self.current)
    }

    fn GetMany(&self, _items: &mut [<T as Abi>::DefaultType]) -> Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}

#[implement(
    Windows::Foundation::Collections::IIterable<T>,
)]
struct Iterable<T>(Vec<T>)
where
    T: RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: RuntimeType + 'static> Iterable<T> {
    fn First(&mut self) -> Result<IIterator<T>> {
        Ok(Iterator::<T> { owner: self.into(), current: 0 }.into())
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
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, true);

    assert_eq!(it1.Current()?, 20);
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, true);

    assert_eq!(it1.Current()?, 30);
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, false);

    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);
    assert_eq!(it1.MoveNext()?, false);

    // The following just validates that iterators are independent and stable.

    let it2 = iterable.First()?;

    assert_eq!(it2.Current()?, 10);
    assert_eq!(it2.HasCurrent()?, true);
    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);

    assert_eq!(it2.Current()?, 10);
    assert_eq!(it2.HasCurrent()?, true);
    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);

    Ok(())
}
