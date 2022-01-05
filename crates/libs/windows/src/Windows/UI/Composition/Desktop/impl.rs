#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowTargetImpl: Sized {
    fn IsTopmost(&self) -> ::windows::core::Result<bool>;
}
