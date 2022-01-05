#[cfg(feature = "implement_exclusive")]
pub trait IInteractiveSessionStaticsImpl: Sized {
    fn IsRemote(&self) -> ::windows::core::Result<bool>;
}
