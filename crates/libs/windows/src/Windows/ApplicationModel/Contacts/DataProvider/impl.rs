#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnectionImpl: Sized {
    fn SyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServerSearchReadBatchRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerSearchReadBatchRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnection2Impl: Sized {
    fn CreateOrUpdateContactRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateOrUpdateContactRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteContactRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteContactRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<ContactDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Contact(&self) -> ::windows::core::Result<super::Contact>;
    fn ReportCompletedAsync(&self, createdorupdatedcontact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListDeleteContactRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestImpl: Sized {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Options(&self) -> ::windows::core::Result<super::ContactQueryOptions>;
    fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32>;
    fn SaveContactAsync(&self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestImpl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
