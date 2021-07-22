use crate::*;

#[doc(hidden)]
pub trait Compose {
    unsafe fn compose<'a>(implementation: Self) -> (IInspectable, &'a mut Option<IInspectable>);
}
