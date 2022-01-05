#[cfg(feature = "implement_exclusive")]
pub trait IApplicationProfileStaticsImpl: Sized {
    fn Modes(&self) -> ::windows::core::Result<ApplicationProfileModes>;
}
