#[cfg(feature = "implement_exclusive")]
pub trait IDisplayRequestImpl: Sized {
    fn RequestActive(&self) -> ::windows::core::Result<()>;
    fn RequestRelease(&self) -> ::windows::core::Result<()>;
}
