#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationInfoImpl: Sized {
    fn TenantId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TenantName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationManagerImpl: Sized {
    fn GetRegistrationsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnterpriseKeyCredentialRegistrationManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager>;
}
