use super::*;

#[doc(hidden)]
pub trait Compose {
    /// # Safety
    unsafe fn compose<'a>(implementation: Self) -> (IInspectable, &'a mut Option<IInspectable>);
}
