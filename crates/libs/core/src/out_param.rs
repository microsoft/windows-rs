use super::*;
use core::mem::{take, transmute_copy};

/// Converts values to Windows API output parameters.
///
/// Implementations are provided for supported Windows types.
/// Interface output storage is `&mut Option<T>`. Optional storage accepts `Some(&mut value)`
/// or `None`, with the same cleanup as a non-optional output.
pub trait OutParam<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    #[doc(hidden)]
    unsafe fn borrow_mut(&self) -> OutRef<'_, T>;
}

impl<T> OutParam<T, CloneType> for &mut T
where
    T: TypeKind<TypeKind = CloneType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            let this: &mut T = transmute_copy(self);
            take(this);
            transmute_copy(self)
        }
    }
}

impl<T> OutParam<T, CopyType> for &mut T
where
    T: TypeKind<TypeKind = CopyType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe { transmute_copy(self) }
    }
}

impl<T> OutParam<T, InterfaceType> for &mut Option<T>
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            let this: &mut Option<T> = transmute_copy(self);
            take(this);
            transmute_copy(self)
        }
    }
}

impl<'a, T> OutParam<T> for Option<&'a mut T::Default>
where
    T: Type<T>,
    &'a mut T::Default: OutParam<T>,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            match self {
                Some(this) => this.borrow_mut(),
                None => OutRef::default(),
            }
        }
    }
}
