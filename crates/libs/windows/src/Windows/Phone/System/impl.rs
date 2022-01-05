#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionStaticsImpl: Sized {
    fn ScreenLocked(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemProtectionUnlockStaticsImpl: Sized {
    fn RequestScreenUnlock(&self) -> ::windows::core::Result<()>;
}
