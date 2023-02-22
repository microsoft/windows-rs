#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::Collections::*;

#[implement(IIterable<T>)]
struct Thing<T>(Vec<T::Default>)
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq;

impl<T> IIterable_Impl<T> for Thing<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        assert!(self.0[0] == self.0[1]);
        todo!()
    }
}
