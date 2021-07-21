use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IIterator<T>,
)]
struct Iterator<T>
where
    T: ::windows::RuntimeType + 'static,
{
    owner: IIterable<T>,
    current: usize,
}

#[allow(non_snake_case)]
impl<T: ::windows::RuntimeType + 'static> Iterator<T> {
    fn Current(&self) -> ::windows::Result<T> {
        let owner = Iterable::to_impl(&self.owner);

        if owner.0.len() > self.current {
            Ok(owner.0[self.current].clone())
        } else {
            Err(Error::new(E_BOUNDS, ""))
        }
    }
    fn HasCurrent(&self) -> ::windows::Result<bool> {
        let owner = Iterable::to_impl(&self.owner);
        Ok(owner.0.len() > self.current)
    }
    fn MoveNext(&mut self) -> ::windows::Result<bool> {
        let owner = Iterable::to_impl(&self.owner);
        self.current += 1;
        Ok(owner.0.len() > self.current)
    }
    fn GetMany(
        &self,
        _items: &mut [<T as ::windows::RuntimeType>::DefaultType],
    ) -> ::windows::Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}

#[implement(
    Windows::Foundation::Collections::IIterable<T>,
)]
struct Iterable<T>(Vec<T>)
where
    T: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: ::windows::RuntimeType + 'static> Iterable<T> {
    fn First(&mut self) -> Result<IIterator<T>> {
        Ok(Iterator::<T> {
            owner: self.into(),
            current: 0,
        }
        .into())
    }
}

#[test]
fn test_impl() -> Result<()> {
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
