#[cfg(feature = "implement_exclusive")]
pub trait IContactPartnerProvisioningManagerStaticsImpl: Sized {
    fn AssociateNetworkAccountAsync(&self, store: &::core::option::Option<super::ContactStore>, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportVcardToSystemAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPartnerProvisioningManagerStatics2Impl: Sized {
    fn AssociateSocialNetworkAccountAsync(&self, store: &::core::option::Option<super::ContactStore>, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMessagePartnerProvisioningManagerStaticsImpl: Sized {
    fn ImportSmsToSystemAsync(&self, incoming: bool, read: bool, body: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, deliverytime: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportMmsToSystemAsync(&self, incoming: bool, read: bool, subject: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, deliverytime: &super::super::super::Foundation::DateTime, attachments: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
