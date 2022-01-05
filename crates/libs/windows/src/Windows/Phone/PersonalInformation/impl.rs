#[cfg(feature = "implement_exclusive")]
pub trait IContactAddressImpl: Sized {
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Locality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StreetAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeRecordImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ContactChangeType>;
    fn RevisionNumber(&self) -> ::windows::core::Result<u64>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IContactInformationImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificPrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDisplayPictureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn ToVcardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
pub trait IContactInformation2Impl: Sized {
    fn DisplayPictureDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(&self, returnvalue: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactInformationStaticsImpl: Sized {
    fn ParseVcardAsync(&self, vcard: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryOptionsImpl: Sized {
    fn DesiredFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OrderBy(&self) -> ::windows::core::Result<ContactQueryResultOrdering>;
    fn SetOrderBy(&self, value: ContactQueryResultOrdering) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryResultImpl: Sized {
    fn GetContactCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>>;
    fn GetContactsAsyncInRange(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>>;
    fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<ContactQueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreImpl: Sized {
    fn FindContactByRemoteIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
    fn FindContactByIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
    fn DeleteContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateContactQueryDefault(&self) -> ::windows::core::Result<ContactQueryResult>;
    fn CreateContactQueryWithOptions(&self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactQueryResult>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RevisionNumber(&self) -> ::windows::core::Result<u64>;
    fn GetChangesAsync(&self, baserevisionnumber: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChangeRecord>>>;
    fn LoadExtendedPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SaveExtendedPropertiesAsync(&self, data: &::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStore2Impl: Sized {
    fn CreateMeContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreStaticsImpl: Sized {
    fn CreateOrOpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn CreateOrOpenWithOptionsAsync(&self, access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownContactPropertiesStaticsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdditionalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Email(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JobTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Birthdate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Anniversary(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Telephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Url(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Notes(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkFax(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Children(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignificantOther(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompanyTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HomeFax(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manager(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OfficeLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WorkEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiGivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiCompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateMobileTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlternateWorkTelephone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoredContactImpl: Sized + IContactInformationImpl {
    fn Store(&self) -> ::windows::core::Result<ContactStore>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetExtendedPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReplaceExistingContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoredContactFactoryImpl: Sized {
    fn CreateStoredContact(&self, store: &::core::option::Option<ContactStore>) -> ::windows::core::Result<StoredContact>;
    fn CreateStoredContactFromInformation(&self, store: &::core::option::Option<ContactStore>, contact: &::core::option::Option<ContactInformation>) -> ::windows::core::Result<StoredContact>;
}
