#[cfg(feature = "implement_exclusive")]
pub trait IAggregateContactManagerImpl: Sized {
    fn FindRawContactsAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn TryLinkContactsAsync(&self, primarycontact: &::core::option::Option<Contact>, secondarycontact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn UnlinkRawContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySetPreferredSourceForPictureAsync(&self, aggregatecontact: &::core::option::Option<Contact>, rawcontact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAggregateContactManager2Impl: Sized {
    fn SetRemoteIdentificationInformationAsync(&self, contactlistid: &::windows::core::HSTRING, remotesourceid: &::windows::core::HSTRING, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Fields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IContactField>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContact2Impl: Sized + IContactImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Notes(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNotes(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Phones(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactPhone>>;
    fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactEmail>>;
    fn Addresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactAddress>>;
    fn ConnectedServiceAccounts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>>;
    fn ImportantDates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactDate>>;
    fn DataSuppliers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn JobInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactJobInfo>>;
    fn SignificantOthers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactSignificantOther>>;
    fn Websites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactWebsite>>;
    fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContact3Impl: Sized + IContactImpl + IContact2Impl {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayPictureUserUpdateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureUserUpdateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsMe(&self) -> ::windows::core::Result<bool>;
    fn AggregateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RingToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRingToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDisplayPictureManuallySet(&self) -> ::windows::core::Result<bool>;
    fn LargeDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SmallDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SourceDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSourceDisplayPicture(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn TextToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsAggregate(&self) -> ::windows::core::Result<bool>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayNameOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayNameOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNickname(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SortName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAddressImpl: Sized {
    fn StreetAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Locality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactAddressKind>;
    fn SetKind(&self, value: ContactAddressKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AnnotationListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations>;
    fn SetSupportedOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()>;
    fn IsDisabled(&self) -> ::windows::core::Result<bool>;
    fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotation2Impl: Sized {
    fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactListId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationListImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySaveAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetAnnotationAsync(&self, annotationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotation>>;
    fn FindAnnotationsByRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn FindAnnotationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn DeleteAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationStoreImpl: Sized {
    fn FindContactIdsByEmailAsync(&self, emailaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn FindContactIdsByPhoneNumberAsync(&self, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn FindAnnotationsForContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
    fn DisableAnnotationAsync(&self, annotation: &::core::option::Option<ContactAnnotation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateAnnotationListAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn CreateAnnotationListInAccountAsync(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn GetAnnotationListAsync(&self, annotationlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>;
    fn FindAnnotationListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactAnnotationStore2Impl: Sized {
    fn FindAnnotationsForContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactBatchImpl: Sized {
    fn Contacts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Contact>>;
    fn Status(&self) -> ::windows::core::Result<ContactBatchStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContactCardDelayedDataLoaderImpl: Sized + IClosableImpl {
    fn SetData(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactCardOptionsImpl: Sized {
    fn HeaderKind(&self) -> ::windows::core::Result<ContactCardHeaderKind>;
    fn SetHeaderKind(&self, value: ContactCardHeaderKind) -> ::windows::core::Result<()>;
    fn InitialTabKind(&self) -> ::windows::core::Result<ContactCardTabKind>;
    fn SetInitialTabKind(&self, value: ContactCardTabKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactCardOptions2Impl: Sized + IContactCardOptionsImpl {
    fn ServerSearchContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ContactChangeType>;
    fn Contact(&self) -> ::windows::core::Result<Contact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoaccept: &::core::option::Option<ContactChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeTrackerImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<ContactChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangeTracker2Impl: Sized {
    fn IsTracking(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ContactChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactConnectedServiceAccountImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDateImpl: Sized {
    fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetDay(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetMonth(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Year(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetYear(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactDateKind>;
    fn SetKind(&self, value: ContactDateKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactEmailImpl: Sized {
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactEmailKind>;
    fn SetKind(&self, value: ContactEmailKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IContactFieldImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<ContactFieldType>;
    fn Category(&self) -> ::windows::core::Result<ContactFieldCategory>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IContactFieldFactoryImpl: Sized {
    fn CreateField_Default(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField>;
    fn CreateField_Category(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
    fn CreateField_Custom(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactGroupImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContactInformationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn PhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactLocationField>>;
    fn InstantMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>>;
    fn CustomFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
    fn QueryCustomFields(&self, customname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactInstantMessageFieldImpl: Sized + IContactFieldImpl {
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
pub trait IContactInstantMessageFieldFactoryImpl: Sized {
    fn CreateInstantMessage_Default(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_Category(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_All(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ContactInstantMessageField>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactJobInfoImpl: Sized {
    fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompanyYomiName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyYomiName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Department(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDepartment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Manager(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetManager(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Office(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOffice(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompanyAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactLaunchActionVerbsStaticsImpl: Sized {
    fn Call(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Map(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Post(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoCall(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHidden(&self) -> ::windows::core::Result<bool>;
    fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<ContactListOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: ContactListOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<ContactListOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: ContactListOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker>;
    fn SyncManager(&self) -> ::windows::core::Result<ContactListSyncManager>;
    fn SupportsServerSearch(&self) -> ::windows::core::Result<bool>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetContactFromRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetContactReader(&self) -> ::windows::core::Result<ContactReader>;
    fn GetContactReaderWithOptions(&self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactReader>;
    fn SaveContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactList2Impl: Sized {
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetSupportsServerSearch(&self, value: bool) -> ::windows::core::Result<()>;
    fn SyncConstraints(&self) -> ::windows::core::Result<ContactListSyncConstraints>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactList3Impl: Sized {
    fn LimitedWriteOperations(&self) -> ::windows::core::Result<ContactListLimitedWriteOperations>;
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListLimitedWriteOperationsImpl: Sized {
    fn TryCreateOrUpdateContactAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDeleteContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncConstraintsImpl: Sized {
    fn CanSyncDescriptions(&self) -> ::windows::core::Result<bool>;
    fn SetCanSyncDescriptions(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxHomePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomePhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxMobilePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxMobilePhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPagerPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPagerPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxBusinessFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxBusinessFaxPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxHomeFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomeFaxPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxCompanyPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxCompanyPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxAssistantPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxAssistantPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxRadioPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxRadioPhoneNumbers(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPersonalEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPersonalEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherEmailAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxHomeAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxHomeAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWorkAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWorkAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherAddresses(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxBirthdayDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxBirthdayDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxAnniversaryDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxAnniversaryDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherDates(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxOtherRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxOtherRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxSpouseRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxSpouseRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxPartnerRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxPartnerRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxSiblingRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxSiblingRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxParentRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxParentRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxChildRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxChildRelationships(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxJobInfo(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxJobInfo(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxWebsites(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMaxWebsites(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ContactListSyncStatus>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManager2Impl: Sized {
    fn SetStatus(&self, value: ContactListSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactLocationFieldImpl: Sized + IContactFieldImpl {
    fn UnstructuredAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IContactLocationFieldFactoryImpl: Sized {
    fn CreateLocation_Default(&self, unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_Category(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_All(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerForUserImpl: Sized {
    fn ConvertContactToVCardAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertContactToVCardAsyncWithMaxBytes(&self, contact: &::core::option::Option<Contact>, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertVCardToContactAsync(&self, vcard: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn RequestStoreAsync(&self, accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>;
    fn SystemDisplayNameOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn SystemSortOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerForUser2Impl: Sized {
    fn ShowFullContactCard(&self, contact: &::core::option::Option<Contact>, fullcontactcardoptions: &::core::option::Option<FullContactCardOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStaticsImpl: Sized {
    fn ShowContactCard(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowContactCardWithPlacement(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn ShowDelayLoadedContactCard(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<ContactCardDelayedDataLoader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics2Impl: Sized + IContactManagerStaticsImpl {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics3Impl: Sized + IContactManagerStaticsImpl + IContactManagerStatics2Impl {
    fn ConvertContactToVCardAsync(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertContactToVCardAsyncWithMaxBytes(&self, contact: &::core::option::Option<Contact>, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn ConvertVCardToContactAsync(&self, vcard: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn RequestStoreAsyncWithAccessType(&self, accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>>;
    fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>;
    fn IsShowContactCardSupported(&self) -> ::windows::core::Result<bool>;
    fn ShowContactCardWithOptions(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &::core::option::Option<ContactCardOptions>) -> ::windows::core::Result<()>;
    fn IsShowDelayLoadedContactCardSupported(&self) -> ::windows::core::Result<bool>;
    fn ShowDelayLoadedContactCardWithOptions(&self, contact: &::core::option::Option<Contact>, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &::core::option::Option<ContactCardOptions>) -> ::windows::core::Result<ContactCardDelayedDataLoader>;
    fn ShowFullContactCard(&self, contact: &::core::option::Option<Contact>, fullcontactcardoptions: &::core::option::Option<FullContactCardOptions>) -> ::windows::core::Result<()>;
    fn SystemDisplayNameOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
    fn SystemSortOrder(&self) -> ::windows::core::Result<ContactNameOrder>;
    fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics4Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ContactManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactManagerStatics5Impl: Sized {
    fn IsShowFullContactCardSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IncludeMiddleNameInSystemDisplayAndSort(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeMiddleNameInSystemDisplayAndSort(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactMatchReasonImpl: Sized {
    fn Field(&self) -> ::windows::core::Result<ContactMatchReasonKind>;
    fn Segments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactNameImpl: Sized {
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFirstName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLastName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MiddleName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMiddleName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YomiGivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYomiGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn YomiFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetYomiFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificNameSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificNameSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HonorificNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHonorificNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn YomiDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelImpl: Sized {
    fn ClosePanel(&self) -> ::windows::core::Result<()>;
    fn HeaderColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>>;
    fn SetHeaderColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::UI::Color>>) -> ::windows::core::Result<()>;
    fn LaunchFullAppRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLaunchFullAppRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelClosingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPanelLaunchFullAppRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPhoneImpl: Sized {
    fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<ContactPhoneKind>;
    fn SetKind(&self, value: ContactPhoneKind) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerImpl: Sized {
    fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectionMode(&self) -> ::windows::core::Result<ContactSelectionMode>;
    fn SetSelectionMode(&self, value: ContactSelectionMode) -> ::windows::core::Result<()>;
    fn DesiredFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PickSingleContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>>;
    fn PickMultipleContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPicker2Impl: Sized {
    fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactFieldType>>;
    fn PickContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn PickContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPicker3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ContactPicker>;
    fn IsSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryOptionsImpl: Sized {
    fn TextSearch(&self) -> ::windows::core::Result<ContactQueryTextSearch>;
    fn ContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IncludeContactsFromHiddenLists(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeContactsFromHiddenLists(&self, value: bool) -> ::windows::core::Result<()>;
    fn DesiredFields(&self) -> ::windows::core::Result<ContactQueryDesiredFields>;
    fn SetDesiredFields(&self, value: ContactQueryDesiredFields) -> ::windows::core::Result<()>;
    fn DesiredOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations>;
    fn SetDesiredOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()>;
    fn AnnotationListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryOptionsFactoryImpl: Sized {
    fn CreateWithText(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<ContactQueryOptions>;
    fn CreateWithTextAndFields(&self, text: &::windows::core::HSTRING, fields: ContactQuerySearchFields) -> ::windows::core::Result<ContactQueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactQueryTextSearchImpl: Sized {
    fn Fields(&self) -> ::windows::core::Result<ContactQuerySearchFields>;
    fn SetFields(&self, value: ContactQuerySearchFields) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SearchScope(&self) -> ::windows::core::Result<ContactQuerySearchScope>;
    fn SetSearchScope(&self, value: ContactQuerySearchScope) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactBatch>>;
    fn GetMatchingPropertiesWithMatchReason(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactMatchReason>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactSignificantOtherImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactSignificantOther2Impl: Sized + IContactSignificantOtherImpl {
    fn Relationship(&self) -> ::windows::core::Result<ContactRelationship>;
    fn SetRelationship(&self, value: ContactRelationship) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreImpl: Sized {
    fn FindContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn FindContactsWithSearchTextAsync(&self, searchtext: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>;
    fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStore2Impl: Sized + IContactStoreImpl {
    fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker>;
    fn ContactChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AggregateContactManager(&self) -> ::windows::core::Result<AggregateContactManager>;
    fn FindContactListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>>;
    fn GetContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
    fn CreateContactListAsync(&self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
    fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>;
    fn GetContactReader(&self) -> ::windows::core::Result<ContactReader>;
    fn GetContactReaderWithOptions(&self, options: &::core::option::Option<ContactQueryOptions>) -> ::windows::core::Result<ContactReader>;
    fn CreateContactListInAccountAsync(&self, displayname: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStore3Impl: Sized {
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreNotificationTriggerDetailsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IContactWebsiteImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactWebsite2Impl: Sized + IContactWebsiteImpl {
    fn RawValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRawValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullContactCardOptionsImpl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownContactFieldStaticsImpl: Sized {
    fn Email(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstantMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertNameToType(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ContactFieldType>;
    fn ConvertTypeToName(&self, r#type: ContactFieldType) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactIdsQueryResultImpl: Sized {
    fn ContactIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn IsPinSurfaceSupported(&self, surface: PinnedContactSurface) -> ::windows::core::Result<bool>;
    fn IsContactPinned(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<bool>;
    fn RequestPinContactAsync(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinContactsAsync(&self, contacts: &::core::option::Option<super::super::Foundation::Collections::IIterable<Contact>>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestUnpinContactAsync(&self, contact: &::core::option::Option<Contact>, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SignalContactActivity(&self, contact: &::core::option::Option<Contact>) -> ::windows::core::Result<()>;
    fn GetPinnedContactIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinnedContactManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PinnedContactManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PinnedContactManager>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
