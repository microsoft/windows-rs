use super::*;

/// Provides automatic parameter conversion in cases where the Windows API expects implicit conversion support.
///
/// This is a mutable version of [Param] meant to support out parameters.
/// There is no need to implement this trait. Blanket implementations are provided for all applicable Windows types.
pub trait ParamMut<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    #[doc(hidden)]
    unsafe fn borrow_mut(&self) -> RefMut<'_, T>;
}

impl<T> ParamMut<T, CloneType> for &mut T
where
    T: TypeKind<TypeKind = CloneType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        let this: &mut T = std::mem::transmute_copy(self);
        std::mem::take(this);
        std::mem::transmute_copy(self)
    }
}

impl<T> ParamMut<T, CopyType> for &mut T
where
    T: TypeKind<TypeKind = CopyType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        std::mem::transmute_copy(self)
    }
}

impl<T> ParamMut<T, InterfaceType> for &mut Option<T>
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        let this: &mut Option<T> = std::mem::transmute_copy(self);
        std::mem::take(this);
        std::mem::transmute_copy(self)
    }
}

impl<T> ParamMut<T> for Option<&mut T>
where
    T: Type<T>,
{
    unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        match self {
            Some(this) => std::mem::transmute_copy(this),
            None => std::mem::zeroed(),
        }
    }
}
