#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerProviderInfoStaticsImpl: Sized {
    fn GetParentProviderId(&self, provider: &::core::option::Option<super::Custom::IGameControllerProvider>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetProviderId(&self, provider: &::core::option::Option<super::Custom::IGameControllerProvider>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
