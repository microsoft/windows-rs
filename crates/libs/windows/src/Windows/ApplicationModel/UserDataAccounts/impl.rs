#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<UserDataAccountOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: UserDataAccountOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn DeviceAccountTypeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindAppointmentCalendarsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>>;
    fn FindEmailMailboxesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>>;
    fn FindContactListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>>;
    fn FindContactAnnotationListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccount2Impl: Sized + IUserDataAccountImpl {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsProtectedUnderLock(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccount3Impl: Sized {
    fn ExplictReadAccessPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccount4Impl: Sized {
    fn CanShowCreateContactGroup(&self) -> ::windows::core::Result<bool>;
    fn SetCanShowCreateContactGroup(&self, value: bool) -> ::windows::core::Result<()>;
    fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn FindUserDataTaskListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>>;
    fn FindContactGroupsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>>;
    fn TryShowCreateContactGroupAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn SetIsProtectedUnderLock(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetIcon(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountManagerForUserImpl: Sized {
    fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>;
    fn ShowAddAccountAsync(&self, contentkinds: UserDataAccountContentKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ShowAccountSettingsAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAccountErrorResolverAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserDataAccountManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountStoreImpl: Sized {
    fn FindAccountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>>;
    fn GetAccountAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>>;
    fn CreateAccountAsync(&self, userdisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountStore2Impl: Sized + IUserDataAccountStoreImpl {
    fn CreateAccountWithPackageRelativeAppIdAsync(&self, userdisplayname: &::windows::core::HSTRING, packagerelativeappid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>>;
    fn StoreChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserDataAccountStore, UserDataAccountStoreChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStoreChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountStore3Impl: Sized + IUserDataAccountStoreImpl {
    fn CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync(&self, userdisplayname: &::windows::core::HSTRING, packagerelativeappid: &::windows::core::HSTRING, enterpriseid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountStoreChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
