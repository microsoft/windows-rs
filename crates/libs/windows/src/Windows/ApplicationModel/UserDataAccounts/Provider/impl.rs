#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountPartnerAccountInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Priority(&self) -> ::windows::core::Result<u32>;
    fn AccountKind(&self) -> ::windows::core::Result<UserDataAccountProviderPartnerAccountKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderAddAccountOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn ContentKinds(&self) -> ::windows::core::Result<super::UserDataAccountContentKinds>;
    fn PartnerAccountInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>;
    fn ReportCompleted(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IUserDataAccountProviderOperationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderResolveErrorsOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderSettingsOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
