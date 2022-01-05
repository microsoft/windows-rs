#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IRetailModeStaticsImpl: Sized {
    fn RetailModeEnabled(&self) -> ::windows::core::Result<bool>;
}
