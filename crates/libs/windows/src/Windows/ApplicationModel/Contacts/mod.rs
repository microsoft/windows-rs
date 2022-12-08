#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAggregateContactManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAggregateContactManager {
    type Vtable = IAggregateContactManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAggregateContactManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0379d5dd_db5a_4fd3_b54e_4df17917a212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindRawContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindRawContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryLinkContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primarycontact: *mut ::core::ffi::c_void, secondarycontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLinkContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnlinkRawContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlinkRawContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetPreferredSourceForPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aggregatecontact: *mut ::core::ffi::c_void, rawcontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetPreferredSourceForPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAggregateContactManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAggregateContactManager2 {
    type Vtable = IAggregateContactManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAggregateContactManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e8cc2d8_a9cd_4430_9c4b_01348db2ca50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetRemoteIdentificationInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: *mut ::core::ffi::c_void, remotesourceid: *mut ::core::ffi::c_void, accountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRemoteIdentificationInformationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContact {
    type Vtable = IContact_Vtbl;
}
unsafe impl ::windows::core::Interface for IContact {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec0072f3_2118_4049_9ebc_17f0ab692b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Fields: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContact2 {
    type Vtable = IContact2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContact2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf312f365_bb77_4c94_802d_8328cee40c08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNotes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Phones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Phones: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Addresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Addresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConnectedServiceAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConnectedServiceAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportantDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportantDates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSuppliers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSuppliers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub JobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    JobInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SignificantOthers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SignificantOthers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Websites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Websites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContact3 {
    type Vtable = IContact3_Vtbl;
}
unsafe impl ::windows::core::Interface for IContact3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48201e67_e08e_42a4_b561_41d08ca9575d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayPictureUserUpdateTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayPictureUserUpdateTime: usize,
    pub IsMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RingToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRingToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsDisplayPictureManuallySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LargeDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LargeDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SmallDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmallDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SourceDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceDisplayPicture: usize,
    pub TextToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTextToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayNameOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayNameOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAddress {
    type Vtable = IContactAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9739d39a_42ce_4872_8d70_3063aa584b70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAddress_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAddressKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAddressKind) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAnnotation {
    type Vtable = IContactAnnotation_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAnnotation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x821fc2ef_7d41_44a2_84c3_60a281dd7b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AnnotationListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows::core::HRESULT,
    pub SetSupportedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows::core::HRESULT,
    pub IsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAnnotation2 {
    type Vtable = IContactAnnotation2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAnnotation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb691ecf3_4ab7_4a1f_9941_0c9cf3171b75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAnnotationList {
    type Vtable = IContactAnnotationList_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAnnotationList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a486aa_5c88_45b9_aad0_461888e68d8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProviderPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsByRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAnnotationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAnnotationStore {
    type Vtable = IContactAnnotationStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAnnotationStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23acf4aa_7a77_457d_8203_987f4b31af09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByEmailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByPhoneNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListInAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationlistid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationListAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationListsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAnnotationStore2 {
    type Vtable = IContactAnnotationStore2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAnnotationStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ede23fd_61e7_4967_8ec5_bdf280a24063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactListAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactBatch(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactBatch {
    type Vtable = IContactBatch_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35d1972d_bfce_46bb_93f8_a5b06ec5e201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactBatch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contacts: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactBatchStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardDelayedDataLoader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactCardDelayedDataLoader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb60af902_1546_434d_869c_6e3520760ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardDelayedDataLoader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactCardOptions {
    type Vtable = IContactCardOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactCardOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c0a4f7e_6ab6_4f3f_be72_817236eeea5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HeaderKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactCardHeaderKind) -> ::windows::core::HRESULT,
    pub SetHeaderKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactCardHeaderKind) -> ::windows::core::HRESULT,
    pub InitialTabKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactCardTabKind) -> ::windows::core::HRESULT,
    pub SetInitialTabKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactCardTabKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactCardOptions2 {
    type Vtable = IContactCardOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactCardOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f271ba0_d74b_4cc6_9f53_1b0eb5d1273c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerSearchContactListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerSearchContactListIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChange {
    type Vtable = IContactChange_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x951d4b10_6a59_4720_a4e1_363d98c135d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChange_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactChangeType) -> ::windows::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangeReader {
    type Vtable = IContactChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x217319fa_2d0c_42e0_a9da_3ecd56a78a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoaccept: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangeTracker {
    type Vtable = IContactChangeTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangeTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e992952_309b_404d_9712_b37bd30278aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeTracker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangeTracker2 {
    type Vtable = IContactChangeTracker2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangeTracker2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f8ad0fc_9321_4d18_9c09_d708c63fcd31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangedDeferral {
    type Vtable = IContactChangedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangedDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5143ae8_1b03_46f8_b694_a523e83cfcb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x525e7fd1_73f3_4b7d_a918_580be4366121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactConnectedServiceAccount(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactConnectedServiceAccount {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6f83553_aa27_4731_8e4a_3dec5ce9eec9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactConnectedServiceAccount_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactDate {
    type Vtable = IContactDate_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactDate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe98ae66_b205_4934_9174_0ff2b0565707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDay: usize,
    #[cfg(feature = "Foundation")]
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Month: usize,
    #[cfg(feature = "Foundation")]
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Year: usize,
    #[cfg(feature = "Foundation")]
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetYear: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactDateKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactDateKind) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactEmail(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactEmail {
    type Vtable = IContactEmail_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactEmail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a219a9_e3d3_4d63_993b_05b9a5393abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactEmail_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactEmailKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactEmailKind) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactField(::windows::core::IUnknown);
impl IContactField {
    pub fn Type(&self) -> ::windows::core::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContactField, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactField {}
impl ::core::fmt::Debug for IContactField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactField").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactField {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b176486a-d293-492c-a058-db575b3e3c0f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactField {
    type Vtable = IContactField_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactField {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb176486a_d293_492c_a058_db575b3e3c0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactField_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows::core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldCategory) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactFieldFactory(::windows::core::IUnknown);
impl IContactFieldFactory {
    pub fn CreateField_Default(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateField_Category(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateField_Custom(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Custom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContactFieldFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactFieldFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactFieldFactory {}
impl ::core::fmt::Debug for IContactFieldFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactFieldFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactFieldFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{85e2913f-0e4a-4a3e-8994-406ae7ed646e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactFieldFactory {
    type Vtable = IContactFieldFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactFieldFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85e2913f_0e4a_4a3e_8994_406ae7ed646e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactFieldFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateField_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateField_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateField_Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactGroup {
    type Vtable = IContactGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59bdeb01_9e9a_475d_bfe5_a37b806d852c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactInformation {
    type Vtable = IContactInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x275eb6d4_6a2e_4278_a914_e460d5f088f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneNumbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstantMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstantMessages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomFields: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryCustomFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryCustomFields: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactInstantMessageField(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactInstantMessageField {
    type Vtable = IContactInstantMessageField_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInstantMessageField {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce33b37_0d85_41fa_b43d_da599c3eb009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageField_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUri: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactInstantMessageFieldFactory(::windows::core::IUnknown);
impl IContactInstantMessageFieldFactory {
    pub fn CreateInstantMessage_Default(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Category(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &super::super::Foundation::Uri) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContactInstantMessageFieldFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactInstantMessageFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactInstantMessageFieldFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactInstantMessageFieldFactory {}
impl ::core::fmt::Debug for IContactInstantMessageFieldFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactInstantMessageFieldFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactInstantMessageFieldFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ba0b6794-91a3-4bb2-b1b9-69a5dff0ba09}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactInstantMessageFieldFactory {
    type Vtable = IContactInstantMessageFieldFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInstantMessageFieldFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0b6794_91a3_4bb2_b1b9_69a5dff0ba09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageFieldFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstantMessage_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstantMessage_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::core::ffi::c_void, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstantMessage_All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::core::ffi::c_void, category: ContactFieldCategory, service: *mut ::core::ffi::c_void, displaytext: *mut ::core::ffi::c_void, verb: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstantMessage_All: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactJobInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactJobInfo {
    type Vtable = IContactJobInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactJobInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d117b4c_ce50_4b43_9e69_b18258ea5315);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactJobInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompanyYomiName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompanyYomiName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Office: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompanyAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompanyAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactLaunchActionVerbsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactLaunchActionVerbsStatics {
    type Vtable = IContactLaunchActionVerbsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactLaunchActionVerbsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb1232d6_ee73_46e7_8761_11cd0157728f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLaunchActionVerbsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactList {
    type Vtable = IContactList_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ddec75_392c_4845_9dfb_51a3e7ef3e42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportsServerSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactFromRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactFromRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactList2 {
    type Vtable = IContactList2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactList2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3943b4_4550_4dcb_9229_40ff91fb0203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
    pub SetSupportsServerSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SyncConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactList3 {
    type Vtable = IContactList3_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactList3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1578ee57_26fc_41e8_a850_5aa32514aca9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListLimitedWriteOperations(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListLimitedWriteOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe19813da_4a0b_44b8_9a1f_a0f3d218175f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListLimitedWriteOperations_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncConstraints(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListSyncConstraints {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b0bf01_3062_4e2e_969d_018d1987f314);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncConstraints_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanSyncDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanSyncDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBirthdayDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBirthdayDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAnniversaryDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAnniversaryDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSpouseRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSpouseRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPartnerRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPartnerRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSiblingRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSiblingRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxParentRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxParentRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxChildRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxChildRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxJobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxJobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWebsites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWebsites: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWebsites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWebsites: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListSyncManager {
    type Vtable = IContactListSyncManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListSyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146e83be_7925_4acc_9de5_21ddd06f8674);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactListSyncManager2 {
    type Vtable = IContactListSyncManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactListSyncManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9591247_bb55_4e23_8128_370134a85d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactLocationField(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactLocationField {
    type Vtable = IContactLocationField_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactLocationField {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ec00f82_ab6e_4b36_89e3_b23bc0a1dacc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationField_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnstructuredAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactLocationFieldFactory(::windows::core::IUnknown);
impl IContactLocationFieldFactory {
    pub fn CreateLocation_Default(&self, unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_Category(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_All(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContactLocationFieldFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactLocationFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactLocationFieldFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactLocationFieldFactory {}
impl ::core::fmt::Debug for IContactLocationFieldFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactLocationFieldFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactLocationFieldFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f79932d7-2fdf-43fe-8f18-41897390bcfe}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactLocationFieldFactory {
    type Vtable = IContactLocationFieldFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactLocationFieldFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79932d7_2fdf_43fe_8f18_41897390bcfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationFieldFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateLocation_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLocation_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: *mut ::core::ffi::c_void, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLocation_All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: *mut ::core::ffi::c_void, category: ContactFieldCategory, street: *mut ::core::ffi::c_void, city: *mut ::core::ffi::c_void, region: *mut ::core::ffi::c_void, country: *mut ::core::ffi::c_void, postalcode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerForUser {
    type Vtable = IContactManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb74bba57_1076_4bef_aef3_54686d18387d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerForUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerForUser2 {
    type Vtable = IContactManagerForUser2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerForUser2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d469c2e_3b75_4a73_bb30_736645472256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerStatics {
    type Vtable = IContactManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81f21ac0_f661_4708_ba4f_d386bd0d622e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowContactCard: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithPlacement: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCard: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerStatics2 {
    type Vtable = IContactManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa178e620_47d8_48cc_963c_9592b6e510c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerStatics3 {
    type Vtable = IContactManagerStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4cc3d42_7586_492a_930b_7bc138fc2139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsyncWithAccessType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsyncWithAccessType: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub IsShowContactCardSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithOptions: usize,
    pub IsShowDelayLoadedContactCardSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCardWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCardWithOptions: usize,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows::core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerStatics4 {
    type Vtable = IContactManagerStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24982272_347b_46dc_8d95_51bd41e15aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactManagerStatics5 {
    type Vtable = IContactManagerStatics5_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactManagerStatics5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7591a87_acb7_4fad_90f2_a8ab64cdbba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsShowFullContactCardSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShowFullContactCardSupportedAsync: usize,
    pub IncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactMatchReason(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactMatchReason {
    type Vtable = IContactMatchReason_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactMatchReason {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc922504_e7d8_413e_95f4_b75c54c74077);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMatchReason_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Field: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactMatchReasonKind) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub Segments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    Segments: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactName(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactName {
    type Vtable = IContactName_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf404e97b_9034_453c_8ebf_140a38c86f1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactName_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MiddleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMiddleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetYomiGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetYomiFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHonorificNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHonorificNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPanel {
    type Vtable = IContactPanel_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPanel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41bf1265_d2ee_4b97_a80a_7d8d64cca6f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ClosePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub HeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    HeaderColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub SetHeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    SetHeaderColor: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullAppRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLaunchFullAppRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanelClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPanelClosingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x222174d3_cf4b_46d7_b739_6edc16110bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelClosingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPanelLaunchFullAppRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88d61c0e_23b4_4be8_8afc_072c25a4190d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPhone(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPhone {
    type Vtable = IContactPhone_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPhone {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x467dab65_2712_4f52_b783_9ea8111c63cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPhone_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactPhoneKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactPhoneKind) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPicker {
    type Vtable = IContactPicker_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e09fd91_42f8_4055_90a0_896f96738936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactSelectionMode) -> ::windows::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactSelectionMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFields: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleContactsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPicker2 {
    type Vtable = IContactPicker2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPicker2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35011cf_5cef_4d24_aa0c_340c5208725d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
    #[cfg(feature = "Foundation")]
    pub PickContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickContactsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPicker3 {
    type Vtable = IContactPicker3_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPicker3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e723315_b243_4bed_8516_22b1a7ac0ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPickerStatics {
    type Vtable = IContactPickerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPickerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7488c029_6a53_4258_a3e9_62dff6784b6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactQueryOptions {
    type Vtable = IContactQueryOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactQueryOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4408cc9e_7d7c_42f0_8ac7_f50733ecdbc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactListIds: usize,
    pub IncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQueryDesiredFields) -> ::windows::core::HRESULT,
    pub SetDesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQueryDesiredFields) -> ::windows::core::HRESULT,
    pub DesiredOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows::core::HRESULT,
    pub SetDesiredOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AnnotationListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AnnotationListIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactQueryOptionsFactory {
    type Vtable = IContactQueryOptionsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactQueryOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543fba47_8ce7_46cb_9dac_9aa42a1bc8e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, fields: ContactQuerySearchFields, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryTextSearch(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactQueryTextSearch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7e3f9cb_a957_439b_a0b7_1c02a1963ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryTextSearch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchFields) -> ::windows::core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQuerySearchFields) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchScope) -> ::windows::core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQuerySearchScope) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactReader {
    type Vtable = IContactReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd397e42e_1488_42f2_bf64_253f4884bfed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMatchingPropertiesWithMatchReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMatchingPropertiesWithMatchReason: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactSignificantOther(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactSignificantOther {
    type Vtable = IContactSignificantOther_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactSignificantOther {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8873b5ab_c5fb_46d8_93fe_da3ff1934054);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactSignificantOther2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactSignificantOther2 {
    type Vtable = IContactSignificantOther2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactSignificantOther2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d7bd474_3f03_45f8_ba0f_c4ed37d64219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactRelationship) -> ::windows::core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactRelationship) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStore {
    type Vtable = IContactStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c220b10_3a6c_4293_b9bc_fe987f6e0d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsWithSearchTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsWithSearchTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStore2 {
    type Vtable = IContactStore2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ce1c22_ebd5_4bfb_b690_5f4f27c4f0e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    pub AggregateContactManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactListsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateContactListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, userdataaccountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStore3 {
    type Vtable = IContactStore3_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStore3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb882c6c_004e_4050_87f0_840407ee6818);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStoreNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStoreNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabb298d6_878a_4f8b_a9ce_46bb7d1c84ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactWebsite(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactWebsite {
    type Vtable = IContactWebsite_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactWebsite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f130176_dc1b_4055_ad66_652f39d990e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactWebsite2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactWebsite2 {
    type Vtable = IContactWebsite2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactWebsite2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf87ee91e_5647_4068_bb5e_4b6f437ce308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullContactCardOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFullContactCardOptions {
    type Vtable = IFullContactCardOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IFullContactCardOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8744436c_5cf9_4683_bdca_a1fdebf8dbce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullContactCardOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownContactFieldStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IKnownContactFieldStatics {
    type Vtable = IKnownContactFieldStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownContactFieldStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e0e1b12_d627_4fca_bad4_1faf168c7d14);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownContactFieldStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Email: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Location: usize,
    #[cfg(feature = "deprecated")]
    pub InstantMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InstantMessage: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertNameToType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertNameToType: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertTypeToName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertTypeToName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactIdsQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPinnedContactIdsQueryResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9b2552_1579_4ddc_871f_a30a3aea9ba1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactIdsQueryResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPinnedContactManager {
    type Vtable = IPinnedContactManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IPinnedContactManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcbc740c_e1d6_45c3_b8b6_a35604e167a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub IsPinSurfaceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsContactPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPinContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestPinContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contacts: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestPinContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnpinContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnpinContactAsync: usize,
    pub SignalContactActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPinnedContactIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPinnedContactIdsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPinnedContactManagerStatics {
    type Vtable = IPinnedContactManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPinnedContactManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf65ccc7e_fdf9_486a_ace9_bc311d0ae7f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct AggregateContactManager(::windows::core::IUnknown);
impl AggregateContactManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindRawContactsAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindRawContactsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryLinkContactsAsync(&self, primarycontact: &Contact, secondarycontact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryLinkContactsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(primarycontact), ::core::mem::transmute_copy(secondarycontact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnlinkRawContactAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnlinkRawContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetPreferredSourceForPictureAsync(&self, aggregatecontact: &Contact, rawcontact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetPreferredSourceForPictureAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(aggregatecontact), ::core::mem::transmute_copy(rawcontact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRemoteIdentificationInformationAsync(&self, contactlistid: &::windows::core::HSTRING, remotesourceid: &::windows::core::HSTRING, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IAggregateContactManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetRemoteIdentificationInformationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactlistid), ::core::mem::transmute_copy(remotesourceid), ::core::mem::transmute_copy(accountid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AggregateContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AggregateContactManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AggregateContactManager {}
impl ::core::fmt::Debug for AggregateContactManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AggregateContactManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AggregateContactManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.AggregateContactManager;{0379d5dd-db5a-4fd3-b54e-4df17917a212})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AggregateContactManager {
    type Vtable = IAggregateContactManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AggregateContactManager {
    const IID: ::windows::core::GUID = <IAggregateContactManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AggregateContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.AggregateContactManager";
}
::windows::core::interface_hierarchy!(AggregateContactManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AggregateContactManager {}
unsafe impl ::core::marker::Sync for AggregateContactManager {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct Contact(::windows::core::IUnknown);
impl Contact {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Contact, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thumbnail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnail)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Fields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Fields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Notes(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNotes(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNotes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Phones(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactPhone>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Phones)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactEmail>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Emails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Addresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactAddress>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Addresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConnectedServiceAccounts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectedServiceAccounts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportantDates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactDate>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportantDates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DataSuppliers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataSuppliers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn JobInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactJobInfo>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JobInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SignificantOthers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactSignificantOther>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignificantOthers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Websites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactWebsite>> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Websites)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayPictureUserUpdateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPictureUserUpdateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDisplayPictureUserUpdateTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayPictureUserUpdateTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsMe(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMe)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AggregateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AggregateId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RingToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RingToneToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRingToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRingToneToken)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDisplayPictureManuallySet(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisplayPictureManuallySet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LargeDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LargeDisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SmallDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SmallDisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceDisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceDisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceDisplayPicture<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSourceDisplayPicture)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn TextToneToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextToneToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTextToneToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTextToneToken)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsAggregate(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAggregate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayNameOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayNameOverride)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayNameOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayNameOverride)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Nickname)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNickname(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNickname)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SortName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SortName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFirstName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetFirstName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLastName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MiddleName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MiddleName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMiddleName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMiddleName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn YomiGivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiGivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetYomiGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetYomiGivenName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn YomiFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetYomiFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetYomiFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificNameSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificNameSuffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificNameSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificNameSuffix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificNamePrefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificNamePrefix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn YomiDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for Contact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Contact {}
impl ::core::fmt::Debug for Contact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Contact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Contact {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Contact;{ec0072f3-2118-4049-9ebc-17f0ab692b64})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Contact {
    type Vtable = IContact_Vtbl;
}
unsafe impl ::windows::core::Interface for Contact {
    const IID: ::windows::core::GUID = <IContact as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Contact {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Contact";
}
::windows::core::interface_hierarchy!(Contact, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Contact {}
unsafe impl ::core::marker::Sync for Contact {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAddress(::windows::core::IUnknown);
impl ContactAddress {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactAddress, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StreetAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StreetAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStreetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStreetAddress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Locality(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Locality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocality)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Region)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRegion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Country)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCountry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostalCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPostalCode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ContactAddressKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactAddressKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactAddress {}
impl ::core::fmt::Debug for ContactAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAddress;{9739d39a-42ce-4872-8d70-3063aa584b70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactAddress {
    type Vtable = IContactAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactAddress {
    const IID: ::windows::core::GUID = <IContactAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAddress";
}
::windows::core::interface_hierarchy!(ContactAddress, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactAddress {}
unsafe impl ::core::marker::Sync for ContactAddress {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotation(::windows::core::IUnknown);
impl ContactAnnotation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactAnnotation, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AnnotationListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AnnotationListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContactId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SupportedOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedOperations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSupportedOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportedOperations)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactAnnotation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContactListId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactAnnotation2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetContactListId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactAnnotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactAnnotation {}
impl ::core::fmt::Debug for ContactAnnotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAnnotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotation;{821fc2ef-7d41-44a2-84c3-60a281dd7b86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactAnnotation {
    type Vtable = IContactAnnotation_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactAnnotation {
    const IID: ::windows::core::GUID = <IContactAnnotation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactAnnotation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotation";
}
::windows::core::interface_hierarchy!(ContactAnnotation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotation {}
unsafe impl ::core::marker::Sync for ContactAnnotation {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationList(::windows::core::IUnknown);
impl ContactAnnotationList {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProviderPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderPackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserDataAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySaveAnnotationAsync(&self, annotation: &ContactAnnotation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySaveAnnotationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(annotation), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnnotationAsync(&self, annotationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnnotationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(annotationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsByRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAnnotationsByRemoteIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remoteid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAnnotationsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAnnotationAsync(&self, annotation: &ContactAnnotation) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAnnotationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(annotation), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactAnnotationList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactAnnotationList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactAnnotationList {}
impl ::core::fmt::Debug for ContactAnnotationList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAnnotationList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationList;{92a486aa-5c88-45b9-aad0-461888e68d8a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactAnnotationList {
    type Vtable = IContactAnnotationList_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactAnnotationList {
    const IID: ::windows::core::GUID = <IContactAnnotationList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactAnnotationList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationList";
}
::windows::core::interface_hierarchy!(ContactAnnotationList, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotationList {}
unsafe impl ::core::marker::Sync for ContactAnnotationList {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationStore(::windows::core::IUnknown);
impl ContactAnnotationStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactIdsByEmailAsync(&self, emailaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactIdsByEmailAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(emailaddress), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactIdsByPhoneNumberAsync(&self, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactIdsByPhoneNumberAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(phonenumber), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsForContactAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAnnotationsForContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAnnotationAsync(&self, annotation: &ContactAnnotation) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableAnnotationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(annotation), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAnnotationListAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAnnotationListAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAnnotationListInAccountAsync(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAnnotationListInAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(userdataaccountid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnnotationListAsync(&self, annotationlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnnotationListAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(annotationlistid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAnnotationListsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsForContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = &::windows::core::Interface::cast::<IContactAnnotationStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAnnotationsForContactListAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactlistid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactAnnotationStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactAnnotationStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactAnnotationStore {}
impl ::core::fmt::Debug for ContactAnnotationStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAnnotationStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationStore;{23acf4aa-7a77-457d-8203-987f4b31af09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactAnnotationStore {
    type Vtable = IContactAnnotationStore_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactAnnotationStore {
    const IID: ::windows::core::GUID = <IContactAnnotationStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactAnnotationStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationStore";
}
::windows::core::interface_hierarchy!(ContactAnnotationStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotationStore {}
unsafe impl ::core::marker::Sync for ContactAnnotationStore {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactBatch(::windows::core::IUnknown);
impl ContactBatch {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contacts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contacts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<ContactBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactBatch {}
impl ::core::fmt::Debug for ContactBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactBatch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactBatch;{35d1972d-bfce-46bb-93f8-a5b06ec5e201})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactBatch {
    type Vtable = IContactBatch_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactBatch {
    const IID: ::windows::core::GUID = <IContactBatch as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactBatch";
}
::windows::core::interface_hierarchy!(ContactBatch, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactBatch {}
unsafe impl ::core::marker::Sync for ContactBatch {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(::windows::core::IUnknown);
impl ContactCardDelayedDataLoader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetData(&self, contact: &Contact) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact)).ok() }
    }
}
impl ::core::clone::Clone for ContactCardDelayedDataLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactCardDelayedDataLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCardDelayedDataLoader {}
impl ::core::fmt::Debug for ContactCardDelayedDataLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardDelayedDataLoader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCardDelayedDataLoader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader;{b60af902-1546-434d-869c-6e3520760ef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactCardDelayedDataLoader {
    const IID: ::windows::core::GUID = <IContactCardDelayedDataLoader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactCardDelayedDataLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader";
}
::windows::core::interface_hierarchy!(ContactCardDelayedDataLoader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ContactCardDelayedDataLoader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCardDelayedDataLoader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ContactCardDelayedDataLoader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCardDelayedDataLoader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ContactCardDelayedDataLoader> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCardDelayedDataLoader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactCardDelayedDataLoader {}
unsafe impl ::core::marker::Sync for ContactCardDelayedDataLoader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardOptions(::windows::core::IUnknown);
impl ContactCardOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactCardOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HeaderKind(&self) -> ::windows::core::Result<ContactCardHeaderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHeaderKind(&self, value: ContactCardHeaderKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHeaderKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InitialTabKind(&self) -> ::windows::core::Result<ContactCardTabKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InitialTabKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInitialTabKind(&self, value: ContactCardTabKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInitialTabKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerSearchContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IContactCardOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerSearchContactListIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactCardOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCardOptions {}
impl ::core::fmt::Debug for ContactCardOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCardOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardOptions;{8c0a4f7e-6ab6-4f3f-be72-817236eeea5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactCardOptions {
    type Vtable = IContactCardOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactCardOptions {
    const IID: ::windows::core::GUID = <IContactCardOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardOptions";
}
::windows::core::interface_hierarchy!(ContactCardOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactCardOptions {}
unsafe impl ::core::marker::Sync for ContactCardOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChange(::windows::core::IUnknown);
impl ContactChange {
    pub fn ChangeType(&self) -> ::windows::core::Result<ContactChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Contact(&self) -> ::windows::core::Result<Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contact)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChange {}
impl ::core::fmt::Debug for ContactChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChange;{951d4b10-6a59-4720-a4e1-363d98c135d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChange {
    type Vtable = IContactChange_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChange {
    const IID: ::windows::core::GUID = <IContactChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChange {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChange";
}
::windows::core::interface_hierarchy!(ContactChange, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChange {}
unsafe impl ::core::marker::Sync for ContactChange {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangeReader(::windows::core::IUnknown);
impl ContactChangeReader {
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcceptChanges)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough(&self, lastchangetoaccept: &ContactChange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcceptChangesThrough)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(lastchangetoaccept)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChangeReader {}
impl ::core::fmt::Debug for ContactChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeReader;{217319fa-2d0c-42e0-a9da-3ecd56a78a47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChangeReader {
    type Vtable = IContactChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChangeReader {
    const IID: ::windows::core::GUID = <IContactChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeReader";
}
::windows::core::interface_hierarchy!(ContactChangeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangeReader {}
unsafe impl ::core::marker::Sync for ContactChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangeTracker(::windows::core::IUnknown);
impl ContactChangeTracker {
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Enable)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows::core::Result<ContactChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChangeReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Reset)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn IsTracking(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContactChangeTracker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTracking)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChangeTracker {}
impl ::core::fmt::Debug for ContactChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeTracker;{6e992952-309b-404d-9712-b37bd30278aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChangeTracker {
    type Vtable = IContactChangeTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChangeTracker {
    const IID: ::windows::core::GUID = <IContactChangeTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeTracker";
}
::windows::core::interface_hierarchy!(ContactChangeTracker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangeTracker {}
unsafe impl ::core::marker::Sync for ContactChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangedDeferral(::windows::core::IUnknown);
impl ContactChangedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ContactChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChangedDeferral {}
impl ::core::fmt::Debug for ContactChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedDeferral;{c5143ae8-1b03-46f8-b694-a523e83cfcb6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChangedDeferral {
    type Vtable = IContactChangedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChangedDeferral {
    const IID: ::windows::core::GUID = <IContactChangedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedDeferral";
}
::windows::core::interface_hierarchy!(ContactChangedDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangedDeferral {}
unsafe impl ::core::marker::Sync for ContactChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangedEventArgs(::windows::core::IUnknown);
impl ContactChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<ContactChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChangedEventArgs {}
impl ::core::fmt::Debug for ContactChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedEventArgs;{525e7fd1-73f3-4b7d-a918-580be4366121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChangedEventArgs {
    const IID: ::windows::core::GUID = <IContactChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedEventArgs";
}
::windows::core::interface_hierarchy!(ContactChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContactChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactConnectedServiceAccount(::windows::core::IUnknown);
impl ContactConnectedServiceAccount {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactConnectedServiceAccount, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetServiceName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactConnectedServiceAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactConnectedServiceAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactConnectedServiceAccount {}
impl ::core::fmt::Debug for ContactConnectedServiceAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactConnectedServiceAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactConnectedServiceAccount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount;{f6f83553-aa27-4731-8e4a-3dec5ce9eec9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactConnectedServiceAccount {
    const IID: ::windows::core::GUID = <IContactConnectedServiceAccount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactConnectedServiceAccount {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount";
}
::windows::core::interface_hierarchy!(ContactConnectedServiceAccount, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactConnectedServiceAccount {}
unsafe impl ::core::marker::Sync for ContactConnectedServiceAccount {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactDate(::windows::core::IUnknown);
impl ContactDate {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactDate, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Day)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDay<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDay)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Month)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMonth<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMonth)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Year(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Year)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetYear<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetYear)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ContactDateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactDateKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactDate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactDate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactDate {}
impl ::core::fmt::Debug for ContactDate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactDate;{fe98ae66-b205-4934-9174-0ff2b0565707})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactDate {
    type Vtable = IContactDate_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactDate {
    const IID: ::windows::core::GUID = <IContactDate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactDate {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactDate";
}
::windows::core::interface_hierarchy!(ContactDate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactDate {}
unsafe impl ::core::marker::Sync for ContactDate {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactEmail(::windows::core::IUnknown);
impl ContactEmail {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactEmail, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Address)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAddress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ContactEmailKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactEmailKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactEmail {}
impl ::core::fmt::Debug for ContactEmail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactEmail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactEmail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactEmail;{90a219a9-e3d3-4d63-993b-05b9a5393abf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactEmail {
    type Vtable = IContactEmail_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactEmail {
    const IID: ::windows::core::GUID = <IContactEmail as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactEmail {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactEmail";
}
::windows::core::interface_hierarchy!(ContactEmail, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactEmail {}
unsafe impl ::core::marker::Sync for ContactEmail {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactField(::windows::core::IUnknown);
impl ContactField {
    pub fn Type(&self) -> ::windows::core::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateField_Default(value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateField_Category(value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateField_Custom(name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Custom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactFieldFactory<R, F: FnOnce(&IContactFieldFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactField, IContactFieldFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ContactField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactField {}
impl ::core::fmt::Debug for ContactField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactField").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactField {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactField;{b176486a-d293-492c-a058-db575b3e3c0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactField {
    type Vtable = IContactField_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactField {
    const IID: ::windows::core::GUID = <IContactField as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactField";
}
::windows::core::interface_hierarchy!(ContactField, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ContactField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactField) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactField) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactField> for ::windows::core::InParam<IContactField> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactField) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactField {}
unsafe impl ::core::marker::Sync for ContactField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactFieldFactory(::windows::core::IUnknown);
impl ContactFieldFactory {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactFieldFactory, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateField_Default(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateField_Category(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateField_Custom(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateField_Custom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Default(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = &::windows::core::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Category(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = &::windows::core::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &super::super::Foundation::Uri) -> ::windows::core::Result<ContactInstantMessageField> {
        let this = &::windows::core::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_Default(&self, unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        let this = &::windows::core::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_Category(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField> {
        let this = &::windows::core::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_All(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        let this = &::windows::core::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactFieldFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactFieldFactory {}
impl ::core::fmt::Debug for ContactFieldFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactFieldFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactFieldFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactFieldFactory;{85e2913f-0e4a-4a3e-8994-406ae7ed646e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactFieldFactory {
    type Vtable = IContactFieldFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactFieldFactory {
    const IID: ::windows::core::GUID = <IContactFieldFactory as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactFieldFactory";
}
::windows::core::interface_hierarchy!(ContactFieldFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ContactFieldFactory> for IContactFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for IContactFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for ::windows::core::InParam<IContactFieldFactory> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactFieldFactory> for IContactInstantMessageFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for IContactInstantMessageFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for ::windows::core::InParam<IContactInstantMessageFieldFactory> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ContactFieldFactory> for IContactLocationFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for IContactLocationFieldFactory {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactFieldFactory> for ::windows::core::InParam<IContactLocationFieldFactory> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactFieldFactory {}
unsafe impl ::core::marker::Sync for ContactFieldFactory {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactGroup(::windows::core::IUnknown);
impl ContactGroup {}
impl ::core::clone::Clone for ContactGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactGroup {}
impl ::core::fmt::Debug for ContactGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactGroup;{59bdeb01-9e9a-475d-bfe5-a37b806d852c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactGroup {
    type Vtable = IContactGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactGroup {
    const IID: ::windows::core::GUID = <IContactGroup as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactGroup";
}
::windows::core::interface_hierarchy!(ContactGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactGroup {}
unsafe impl ::core::marker::Sync for ContactGroup {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactInformation(::windows::core::IUnknown);
impl ContactInformation {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Emails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Emails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactLocationField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Locations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstantMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstantMessages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomFields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryCustomFields(&self, customname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryCustomFields)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(customname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactInformation {}
impl ::core::fmt::Debug for ContactInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInformation;{275eb6d4-6a2e-4278-a914-e460d5f088f6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactInformation {
    type Vtable = IContactInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactInformation {
    const IID: ::windows::core::GUID = <IContactInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInformation";
}
::windows::core::interface_hierarchy!(ContactInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactInstantMessageField(::windows::core::IUnknown);
impl ContactInstantMessageField {
    pub fn Type(&self) -> ::windows::core::Result<ContactFieldType> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<ContactFieldCategory> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Service)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Default(username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInstantMessage_Category(username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All(username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &super::super::Foundation::Uri) -> ::windows::core::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstantMessage_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactInstantMessageFieldFactory<R, F: FnOnce(&IContactInstantMessageFieldFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactInstantMessageField, IContactInstantMessageFieldFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ContactInstantMessageField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactInstantMessageField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactInstantMessageField {}
impl ::core::fmt::Debug for ContactInstantMessageField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactInstantMessageField").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactInstantMessageField {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInstantMessageField;{cce33b37-0d85-41fa-b43d-da599c3eb009})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactInstantMessageField {
    type Vtable = IContactInstantMessageField_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactInstantMessageField {
    const IID: ::windows::core::GUID = <IContactInstantMessageField as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactInstantMessageField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInstantMessageField";
}
::windows::core::interface_hierarchy!(ContactInstantMessageField, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ContactInstantMessageField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactInstantMessageField) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactInstantMessageField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactInstantMessageField) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactInstantMessageField> for ::windows::core::InParam<IContactField> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactInstantMessageField) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactInstantMessageField {}
unsafe impl ::core::marker::Sync for ContactInstantMessageField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactJobInfo(::windows::core::IUnknown);
impl ContactJobInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactJobInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompanyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompanyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CompanyYomiName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyYomiName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompanyYomiName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompanyYomiName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Department(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Department)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDepartment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDepartment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Manager(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Manager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetManager(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetManager)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Office(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Office)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOffice(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOffice)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CompanyAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompanyAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompanyAddress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactJobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactJobInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactJobInfo {}
impl ::core::fmt::Debug for ContactJobInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactJobInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactJobInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactJobInfo;{6d117b4c-ce50-4b43-9e69-b18258ea5315})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactJobInfo {
    type Vtable = IContactJobInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactJobInfo {
    const IID: ::windows::core::GUID = <IContactJobInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactJobInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactJobInfo";
}
::windows::core::interface_hierarchy!(ContactJobInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactJobInfo {}
unsafe impl ::core::marker::Sync for ContactJobInfo {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
pub struct ContactLaunchActionVerbs;
impl ContactLaunchActionVerbs {
    pub fn Call() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Call)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Message() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Map() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Map)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Post() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Post)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn VideoCall() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoCall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactLaunchActionVerbsStatics<R, F: FnOnce(&IContactLaunchActionVerbsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactLaunchActionVerbs, IContactLaunchActionVerbsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ContactLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLaunchActionVerbs";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactList(::windows::core::IUnknown);
impl ContactList {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHidden)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsHidden)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<ContactListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OtherAppReadAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: ContactListOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOtherAppReadAccess)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<ContactListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OtherAppWriteAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: ContactListOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOtherAppWriteAccess)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeTracker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows::core::Result<ContactListSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportsServerSearch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsServerSearch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserDataAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactChanged(&self, value: &super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContactChanged)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactFromRemoteIdAsync(&self, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactFromRemoteIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remoteid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMeContactAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetContactReader(&self) -> ::windows::core::Result<ContactReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetContactReaderWithOptions(&self, options: &ContactQueryOptions) -> ::windows::core::Result<ContactReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactReaderWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveContactAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterSyncManagerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSupportsServerSearch(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactList2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportsServerSearch)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SyncConstraints(&self) -> ::windows::core::Result<ContactListSyncConstraints> {
        let this = &::windows::core::Interface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncConstraints)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LimitedWriteOperations(&self) -> ::windows::core::Result<ContactListLimitedWriteOperations> {
        let this = &::windows::core::Interface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LimitedWriteOperations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker> {
        let this = &::windows::core::Interface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChangeTracker)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(identity), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactList {}
impl ::core::fmt::Debug for ContactList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactList;{16ddec75-392c-4845-9dfb-51a3e7ef3e42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactList {
    type Vtable = IContactList_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactList {
    const IID: ::windows::core::GUID = <IContactList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactList";
}
::windows::core::interface_hierarchy!(ContactList, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactList {}
unsafe impl ::core::marker::Sync for ContactList {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListLimitedWriteOperations(::windows::core::IUnknown);
impl ContactListLimitedWriteOperations {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateContactAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateOrUpdateContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDeleteContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListLimitedWriteOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListLimitedWriteOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListLimitedWriteOperations {}
impl ::core::fmt::Debug for ContactListLimitedWriteOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListLimitedWriteOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListLimitedWriteOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations;{e19813da-4a0b-44b8-9a1f-a0f3d218175f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListLimitedWriteOperations {
    const IID: ::windows::core::GUID = <IContactListLimitedWriteOperations as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations";
}
::windows::core::interface_hierarchy!(ContactListLimitedWriteOperations, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListLimitedWriteOperations {}
unsafe impl ::core::marker::Sync for ContactListLimitedWriteOperations {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListSyncConstraints(::windows::core::IUnknown);
impl ContactListSyncConstraints {
    pub fn CanSyncDescriptions(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSyncDescriptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCanSyncDescriptions(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCanSyncDescriptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxHomePhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomePhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxHomePhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxMobilePhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxMobilePhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxMobilePhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxMobilePhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxWorkPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxWorkPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOtherPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxOtherPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPagerPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPagerPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPagerPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPagerPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBusinessFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxBusinessFaxPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxBusinessFaxPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxBusinessFaxPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomeFaxPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxHomeFaxPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomeFaxPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxHomeFaxPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxCompanyPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxCompanyPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxCompanyPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxCompanyPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAssistantPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAssistantPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAssistantPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxAssistantPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxRadioPhoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxRadioPhoneNumbers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxRadioPhoneNumbers<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxRadioPhoneNumbers)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPersonalEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPersonalEmailAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPersonalEmailAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPersonalEmailAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxWorkEmailAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkEmailAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxWorkEmailAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherEmailAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOtherEmailAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherEmailAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxOtherEmailAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomeAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxHomeAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomeAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxHomeAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxWorkAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxWorkAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherAddresses(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOtherAddresses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherAddresses<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxOtherAddresses)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBirthdayDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxBirthdayDates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxBirthdayDates<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxBirthdayDates)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAnniversaryDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAnniversaryDates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAnniversaryDates<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxAnniversaryDates)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherDates(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOtherDates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherDates<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxOtherDates)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOtherRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxOtherRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSpouseRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxSpouseRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSpouseRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxSpouseRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPartnerRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPartnerRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPartnerRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPartnerRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSiblingRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxSiblingRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSiblingRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxSiblingRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxParentRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxParentRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxParentRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxParentRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxChildRelationships(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxChildRelationships)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxChildRelationships<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxChildRelationships)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxJobInfo(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxJobInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxJobInfo<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxJobInfo)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWebsites(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxWebsites)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWebsites<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxWebsites)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for ContactListSyncConstraints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncConstraints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncConstraints {}
impl ::core::fmt::Debug for ContactListSyncConstraints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncConstraints").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncConstraints {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncConstraints;{b2b0bf01-3062-4e2e-969d-018d1987f314})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListSyncConstraints {
    const IID: ::windows::core::GUID = <IContactListSyncConstraints as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListSyncConstraints {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncConstraints";
}
::windows::core::interface_hierarchy!(ContactListSyncConstraints, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncConstraints {}
unsafe impl ::core::marker::Sync for ContactListSyncConstraints {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListSyncManager(::windows::core::IUnknown);
impl ContactListSyncManager {
    pub fn Status(&self) -> ::windows::core::Result<ContactListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastSuccessfulSyncTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastAttemptedSyncTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncStatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSyncStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn SetStatus(&self, value: ContactListSyncStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastSuccessfulSyncTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastAttemptedSyncTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ContactListSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncManager {}
impl ::core::fmt::Debug for ContactListSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncManager;{146e83be-7925-4acc-9de5-21ddd06f8674})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactListSyncManager {
    type Vtable = IContactListSyncManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactListSyncManager {
    const IID: ::windows::core::GUID = <IContactListSyncManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncManager";
}
::windows::core::interface_hierarchy!(ContactListSyncManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncManager {}
unsafe impl ::core::marker::Sync for ContactListSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactLocationField(::windows::core::IUnknown);
impl ContactLocationField {
    pub fn Type(&self) -> ::windows::core::Result<ContactFieldType> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<ContactFieldCategory> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UnstructuredAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnstructuredAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Street)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).City)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Region)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Country)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostalCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLocation_Default(unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Default)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateLocation_Category(unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_Category)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateLocation_All(unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocation_All)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactLocationFieldFactory<R, F: FnOnce(&IContactLocationFieldFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactLocationField, IContactLocationFieldFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ContactLocationField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactLocationField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactLocationField {}
impl ::core::fmt::Debug for ContactLocationField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactLocationField").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactLocationField {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactLocationField;{9ec00f82-ab6e-4b36-89e3-b23bc0a1dacc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactLocationField {
    type Vtable = IContactLocationField_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactLocationField {
    const IID: ::windows::core::GUID = <IContactLocationField as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactLocationField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLocationField";
}
::windows::core::interface_hierarchy!(ContactLocationField, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ContactLocationField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactLocationField) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactLocationField> for IContactField {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactLocationField) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactLocationField> for ::windows::core::InParam<IContactField> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactLocationField) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactLocationField {}
unsafe impl ::core::marker::Sync for ContactLocationField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
pub struct ContactManager;
impl ContactManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowContactCard(contact: &Contact, selection: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowContactCard)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), selection).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowContactCardWithPlacement(contact: &Contact, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowContactCardWithPlacement)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), selection, preferredplacement).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowDelayLoadedContactCard(contact: &Contact, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<ContactCardDelayedDataLoader> {
        Self::IContactManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowDelayLoadedContactCard)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), selection, preferredplacement, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsync(contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertContactToVCardAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsyncWithMaxBytes(contact: &Contact, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertContactToVCardAsyncWithMaxBytes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), maxbytes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertVCardToContactAsync<P0, E0>(vcard: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertVCardToContactAsync)(::windows::core::Vtable::as_raw(this), vcard.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsyncWithAccessType(accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsyncWithAccessType)(::windows::core::Vtable::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAnnotationStoreAsync(accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAnnotationStoreAsync)(::windows::core::Vtable::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsShowContactCardSupported() -> ::windows::core::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsShowContactCardSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowContactCardWithOptions(contact: &Contact, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &ContactCardOptions) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowContactCardWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), selection, preferredplacement, ::core::mem::transmute_copy(contactcardoptions)).ok() })
    }
    pub fn IsShowDelayLoadedContactCardSupported() -> ::windows::core::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsShowDelayLoadedContactCardSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowDelayLoadedContactCardWithOptions(contact: &Contact, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: &ContactCardOptions) -> ::windows::core::Result<ContactCardDelayedDataLoader> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowDelayLoadedContactCardWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), selection, preferredplacement, ::core::mem::transmute_copy(contactcardoptions), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShowFullContactCard(contact: &Contact, fullcontactcardoptions: &FullContactCardOptions) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowFullContactCard)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), ::core::mem::transmute_copy(fullcontactcardoptions)).ok() })
    }
    pub fn SystemDisplayNameOrder() -> ::windows::core::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemDisplayNameOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetSystemDisplayNameOrder(value: ContactNameOrder) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).SetSystemDisplayNameOrder)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn SystemSortOrder() -> ::windows::core::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemSortOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetSystemSortOrder(value: ContactNameOrder) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).SetSystemSortOrder)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<ContactManagerForUser> {
        Self::IContactManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsShowFullContactCardSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsShowFullContactCardSupportedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IncludeMiddleNameInSystemDisplayAndSort() -> ::windows::core::Result<bool> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeMiddleNameInSystemDisplayAndSort)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetIncludeMiddleNameInSystemDisplayAndSort(value: bool) -> ::windows::core::Result<()> {
        Self::IContactManagerStatics5(|this| unsafe { (::windows::core::Vtable::vtable(this).SetIncludeMiddleNameInSystemDisplayAndSort)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics<R, F: FnOnce(&IContactManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactManager, IContactManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics2<R, F: FnOnce(&IContactManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactManager, IContactManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics3<R, F: FnOnce(&IContactManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactManager, IContactManagerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics4<R, F: FnOnce(&IContactManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactManager, IContactManagerStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics5<R, F: FnOnce(&IContactManagerStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactManager, IContactManagerStatics5> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManager";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactManagerForUser(::windows::core::IUnknown);
impl ContactManagerForUser {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsync(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertContactToVCardAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsyncWithMaxBytes(&self, contact: &Contact, maxbytes: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertContactToVCardAsyncWithMaxBytes)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), maxbytes, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertVCardToContactAsync<P0, E0>(&self, vcard: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertVCardToContactAsync)(::windows::core::Vtable::as_raw(this), vcard.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: ContactStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsync)(::windows::core::Vtable::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAnnotationStoreAsync)(::windows::core::Vtable::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemDisplayNameOrder(&self) -> ::windows::core::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemDisplayNameOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSystemDisplayNameOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SystemSortOrder(&self) -> ::windows::core::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemSortOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSystemSortOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShowFullContactCard(&self, contact: &Contact, fullcontactcardoptions: &FullContactCardOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactManagerForUser2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ShowFullContactCard)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), ::core::mem::transmute_copy(fullcontactcardoptions)).ok() }
    }
}
impl ::core::clone::Clone for ContactManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactManagerForUser {}
impl ::core::fmt::Debug for ContactManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactManagerForUser;{b74bba57-1076-4bef-aef3-54686d18387d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactManagerForUser {
    type Vtable = IContactManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactManagerForUser {
    const IID: ::windows::core::GUID = <IContactManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManagerForUser";
}
::windows::core::interface_hierarchy!(ContactManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactManagerForUser {}
unsafe impl ::core::marker::Sync for ContactManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactMatchReason(::windows::core::IUnknown);
impl ContactMatchReason {
    pub fn Field(&self) -> ::windows::core::Result<ContactMatchReasonKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Field)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub fn Segments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Segments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMatchReason {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMatchReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMatchReason {}
impl ::core::fmt::Debug for ContactMatchReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMatchReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMatchReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactMatchReason;{bc922504-e7d8-413e-95f4-b75c54c74077})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactMatchReason {
    type Vtable = IContactMatchReason_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactMatchReason {
    const IID: ::windows::core::GUID = <IContactMatchReason as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactMatchReason {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactMatchReason";
}
::windows::core::interface_hierarchy!(ContactMatchReason, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactMatchReason {}
unsafe impl ::core::marker::Sync for ContactMatchReason {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanel(::windows::core::IUnknown);
impl ContactPanel {
    pub fn ClosePanel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClosePanel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn HeaderColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn SetHeaderColor<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::UI::Color>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHeaderColor)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullAppRequested(&self, handler: &super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchFullAppRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLaunchFullAppRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLaunchFullAppRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closing(&self, handler: &super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosing)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for ContactPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanel {}
impl ::core::fmt::Debug for ContactPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPanel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanel;{41bf1265-d2ee-4b97-a80a-7d8d64cca6f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactPanel {
    type Vtable = IContactPanel_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactPanel {
    const IID: ::windows::core::GUID = <IContactPanel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPanel {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanel";
}
::windows::core::interface_hierarchy!(ContactPanel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanel {}
unsafe impl ::core::marker::Sync for ContactPanel {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanelClosingEventArgs(::windows::core::IUnknown);
impl ContactPanelClosingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPanelClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanelClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelClosingEventArgs {}
impl ::core::fmt::Debug for ContactPanelClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanelClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPanelClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs;{222174d3-cf4b-46d7-b739-6edc16110bfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactPanelClosingEventArgs {
    const IID: ::windows::core::GUID = <IContactPanelClosingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPanelClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs";
}
::windows::core::interface_hierarchy!(ContactPanelClosingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanelClosingEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelClosingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(::windows::core::IUnknown);
impl ContactPanelLaunchFullAppRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ContactPanelLaunchFullAppRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanelLaunchFullAppRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelLaunchFullAppRequestedEventArgs {}
impl ::core::fmt::Debug for ContactPanelLaunchFullAppRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanelLaunchFullAppRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPanelLaunchFullAppRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs;{88d61c0e-23b4-4be8-8afc-072c25a4190d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactPanelLaunchFullAppRequestedEventArgs {
    const IID: ::windows::core::GUID = <IContactPanelLaunchFullAppRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPanelLaunchFullAppRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs";
}
::windows::core::interface_hierarchy!(ContactPanelLaunchFullAppRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanelLaunchFullAppRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelLaunchFullAppRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPhone(::windows::core::IUnknown);
impl ContactPhone {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactPhone, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Number)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNumber)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ContactPhoneKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactPhoneKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactPhone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPhone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPhone {}
impl ::core::fmt::Debug for ContactPhone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPhone").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPhone {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPhone;{467dab65-2712-4f52-b783-9ea8111c63cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactPhone {
    type Vtable = IContactPhone_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactPhone {
    const IID: ::windows::core::GUID = <IContactPhone as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPhone {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPhone";
}
::windows::core::interface_hierarchy!(ContactPhone, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactPhone {}
unsafe impl ::core::marker::Sync for ContactPhone {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPicker(::windows::core::IUnknown);
impl ContactPicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactPicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CommitButtonText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommitButtonText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCommitButtonText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SelectionMode(&self) -> ::windows::core::Result<ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSelectionMode(&self, value: ContactSelectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelectionMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredFields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickSingleContactAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickMultipleContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickMultipleContactsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ContactFieldType>> {
        let this = &::windows::core::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredFieldsWithContactFieldType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows::core::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickContactAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>> {
        let this = &::windows::core::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickContactsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IContactPicker3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser(user: &super::super::System::User) -> ::windows::core::Result<ContactPicker> {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupportedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactPickerStatics<R, F: FnOnce(&IContactPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactPicker, IContactPickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ContactPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPicker {}
impl ::core::fmt::Debug for ContactPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPicker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPicker;{0e09fd91-42f8-4055-90a0-896f96738936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactPicker {
    type Vtable = IContactPicker_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactPicker {
    const IID: ::windows::core::GUID = <IContactPicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPicker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPicker";
}
::windows::core::interface_hierarchy!(ContactPicker, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQueryOptions(::windows::core::IUnknown);
impl ContactQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactQueryOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextSearch(&self) -> ::windows::core::Result<ContactQueryTextSearch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextSearch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContactListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactListIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IncludeContactsFromHiddenLists(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeContactsFromHiddenLists)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeContactsFromHiddenLists(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeContactsFromHiddenLists)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DesiredFields(&self) -> ::windows::core::Result<ContactQueryDesiredFields> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredFields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDesiredFields(&self, value: ContactQueryDesiredFields) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredFields)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DesiredOperations(&self) -> ::windows::core::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredOperations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDesiredOperations(&self, value: ContactAnnotationOperations) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredOperations)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AnnotationListIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AnnotationListIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateWithText(text: &::windows::core::HSTRING) -> ::windows::core::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithTextAndFields(text: &::windows::core::HSTRING, fields: ContactQuerySearchFields) -> ::windows::core::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithTextAndFields)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), fields, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactQueryOptionsFactory<R, F: FnOnce(&IContactQueryOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactQueryOptions, IContactQueryOptionsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ContactQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactQueryOptions {}
impl ::core::fmt::Debug for ContactQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryOptions;{4408cc9e-7d7c-42f0-8ac7-f50733ecdbc1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactQueryOptions {
    type Vtable = IContactQueryOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactQueryOptions {
    const IID: ::windows::core::GUID = <IContactQueryOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryOptions";
}
::windows::core::interface_hierarchy!(ContactQueryOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryOptions {}
unsafe impl ::core::marker::Sync for ContactQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQueryTextSearch(::windows::core::IUnknown);
impl ContactQueryTextSearch {
    pub fn Fields(&self) -> ::windows::core::Result<ContactQuerySearchFields> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Fields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFields(&self, value: ContactQuerySearchFields) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFields)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SearchScope(&self) -> ::windows::core::Result<ContactQuerySearchScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchScope)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSearchScope(&self, value: ContactQuerySearchScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSearchScope)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ContactQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactQueryTextSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactQueryTextSearch {}
impl ::core::fmt::Debug for ContactQueryTextSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryTextSearch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQueryTextSearch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryTextSearch;{f7e3f9cb-a957-439b-a0b7-1c02a1963ff0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactQueryTextSearch {
    const IID: ::windows::core::GUID = <IContactQueryTextSearch as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryTextSearch";
}
::windows::core::interface_hierarchy!(ContactQueryTextSearch, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryTextSearch {}
unsafe impl ::core::marker::Sync for ContactQueryTextSearch {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactReader(::windows::core::IUnknown);
impl ContactReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMatchingPropertiesWithMatchReason(&self, contact: &Contact) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ContactMatchReason>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMatchingPropertiesWithMatchReason)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactReader {}
impl ::core::fmt::Debug for ContactReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactReader;{d397e42e-1488-42f2-bf64-253f4884bfed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactReader {
    type Vtable = IContactReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactReader {
    const IID: ::windows::core::GUID = <IContactReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactReader";
}
::windows::core::interface_hierarchy!(ContactReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactReader {}
unsafe impl ::core::marker::Sync for ContactReader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactSignificantOther(::windows::core::IUnknown);
impl ContactSignificantOther {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactSignificantOther, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Relationship(&self) -> ::windows::core::Result<ContactRelationship> {
        let this = &::windows::core::Interface::cast::<IContactSignificantOther2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Relationship)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRelationship(&self, value: ContactRelationship) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactSignificantOther2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRelationship)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ContactSignificantOther {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactSignificantOther {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactSignificantOther {}
impl ::core::fmt::Debug for ContactSignificantOther {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactSignificantOther").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactSignificantOther {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactSignificantOther;{8873b5ab-c5fb-46d8-93fe-da3ff1934054})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactSignificantOther {
    type Vtable = IContactSignificantOther_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactSignificantOther {
    const IID: ::windows::core::GUID = <IContactSignificantOther as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactSignificantOther {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactSignificantOther";
}
::windows::core::interface_hierarchy!(ContactSignificantOther, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactSignificantOther {}
unsafe impl ::core::marker::Sync for ContactSignificantOther {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactStore(::windows::core::IUnknown);
impl ContactStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactsWithSearchTextAsync(&self, searchtext: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactsWithSearchTextAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(searchtext), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactAsync(&self, contactid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows::core::Result<ContactChangeTracker> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeTracker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactChanged(&self, value: &super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContactChanged)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AggregateContactManager(&self) -> ::windows::core::Result<AggregateContactManager> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AggregateContactManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactListsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactListAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactListAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contactlistid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateContactListAsync(&self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContactListAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMeContactAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMeContactAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetContactReader(&self) -> ::windows::core::Result<ContactReader> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetContactReaderWithOptions(&self, options: &ContactQueryOptions) -> ::windows::core::Result<ContactReader> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactReaderWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateContactListInAccountAsync(&self, displayname: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContactListInAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(userdataaccountid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<ContactChangeTracker> {
        let this = &::windows::core::Interface::cast::<IContactStore3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChangeTracker)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(identity), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactStore {}
impl ::core::fmt::Debug for ContactStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStore;{2c220b10-3a6c-4293-b9bc-fe987f6e0d52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactStore {
    type Vtable = IContactStore_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactStore {
    const IID: ::windows::core::GUID = <IContactStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStore";
}
::windows::core::interface_hierarchy!(ContactStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactStore {}
unsafe impl ::core::marker::Sync for ContactStore {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactStoreNotificationTriggerDetails(::windows::core::IUnknown);
impl ContactStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for ContactStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for ContactStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails;{abb298d6-878a-4f8b-a9ce-46bb7d1c84ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactStoreNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IContactStoreNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(ContactStoreNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for ContactStoreNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactWebsite(::windows::core::IUnknown);
impl ContactWebsite {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactWebsite, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RawValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactWebsite2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRawValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactWebsite2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRawValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ContactWebsite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactWebsite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactWebsite {}
impl ::core::fmt::Debug for ContactWebsite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactWebsite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactWebsite {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactWebsite;{9f130176-dc1b-4055-ad66-652f39d990e8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactWebsite {
    type Vtable = IContactWebsite_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactWebsite {
    const IID: ::windows::core::GUID = <IContactWebsite as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactWebsite {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactWebsite";
}
::windows::core::interface_hierarchy!(ContactWebsite, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactWebsite {}
unsafe impl ::core::marker::Sync for ContactWebsite {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct FullContactCardOptions(::windows::core::IUnknown);
impl FullContactCardOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FullContactCardOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredRemainingView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredRemainingView)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FullContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullContactCardOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullContactCardOptions {}
impl ::core::fmt::Debug for FullContactCardOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullContactCardOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullContactCardOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.FullContactCardOptions;{8744436c-5cf9-4683-bdca-a1fdebf8dbce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FullContactCardOptions {
    type Vtable = IFullContactCardOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for FullContactCardOptions {
    const IID: ::windows::core::GUID = <IFullContactCardOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FullContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.FullContactCardOptions";
}
::windows::core::interface_hierarchy!(FullContactCardOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FullContactCardOptions {}
unsafe impl ::core::marker::Sync for FullContactCardOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownContactField;
#[cfg(feature = "deprecated")]
impl KnownContactField {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Email() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Email)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Location() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Location)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn InstantMessage() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstantMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertNameToType(name: &::windows::core::HSTRING) -> ::windows::core::Result<ContactFieldType> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertNameToType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertTypeToName(r#type: ContactFieldType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertTypeToName)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownContactFieldStatics<R, F: FnOnce(&IKnownContactFieldStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownContactField, IKnownContactFieldStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.KnownContactField";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct PinnedContactIdsQueryResult(::windows::core::IUnknown);
impl PinnedContactIdsQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContactIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PinnedContactIdsQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PinnedContactIdsQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PinnedContactIdsQueryResult {}
impl ::core::fmt::Debug for PinnedContactIdsQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinnedContactIdsQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PinnedContactIdsQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult;{7d9b2552-1579-4ddc-871f-a30a3aea9ba1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PinnedContactIdsQueryResult {
    const IID: ::windows::core::GUID = <IPinnedContactIdsQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PinnedContactIdsQueryResult {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult";
}
::windows::core::interface_hierarchy!(PinnedContactIdsQueryResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PinnedContactIdsQueryResult {}
unsafe impl ::core::marker::Sync for PinnedContactIdsQueryResult {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct PinnedContactManager(::windows::core::IUnknown);
impl PinnedContactManager {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPinSurfaceSupported(&self, surface: PinnedContactSurface) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPinSurfaceSupported)(::windows::core::Vtable::as_raw(this), surface, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsContactPinned(&self, contact: &Contact, surface: PinnedContactSurface) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsContactPinned)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), surface, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinContactAsync(&self, contact: &Contact, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPinContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), surface, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestPinContactsAsync<P0, E0>(&self, contacts: P0, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<Contact>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestPinContactsAsync)(::windows::core::Vtable::as_raw(this), contacts.try_into().map_err(|e| e.into())?.abi(), surface, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnpinContactAsync(&self, contact: &Contact, surface: PinnedContactSurface) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestUnpinContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact), surface, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SignalContactActivity(&self, contact: &Contact) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SignalContactActivity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contact)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPinnedContactIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPinnedContactIdsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<PinnedContactManager> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<PinnedContactManager> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPinnedContactManagerStatics<R, F: FnOnce(&IPinnedContactManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PinnedContactManager, IPinnedContactManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PinnedContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PinnedContactManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PinnedContactManager {}
impl ::core::fmt::Debug for PinnedContactManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinnedContactManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PinnedContactManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactManager;{fcbc740c-e1d6-45c3-b8b6-a35604e167a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PinnedContactManager {
    type Vtable = IPinnedContactManager_Vtbl;
}
unsafe impl ::windows::core::Interface for PinnedContactManager {
    const IID: ::windows::core::GUID = <IPinnedContactManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PinnedContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactManager";
}
::windows::core::interface_hierarchy!(PinnedContactManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PinnedContactManager {}
unsafe impl ::core::marker::Sync for PinnedContactManager {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactAddressKind(pub i32);
impl ContactAddressKind {
    pub const Home: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactAddressKind {}
impl ::core::clone::Clone for ContactAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactAddressKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactAddressKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactAddressKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAddressKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAddressKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAddressKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactAnnotationOperations(pub u32);
impl ContactAnnotationOperations {
    pub const None: Self = Self(0u32);
    pub const ContactProfile: Self = Self(1u32);
    pub const Message: Self = Self(2u32);
    pub const AudioCall: Self = Self(4u32);
    pub const VideoCall: Self = Self(8u32);
    pub const SocialFeeds: Self = Self(16u32);
    pub const Share: Self = Self(32u32);
}
impl ::core::marker::Copy for ContactAnnotationOperations {}
impl ::core::clone::Clone for ContactAnnotationOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactAnnotationOperations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactAnnotationOperations {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactAnnotationOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationOperations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ContactAnnotationOperations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ContactAnnotationOperations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ContactAnnotationOperations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ContactAnnotationOperations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ContactAnnotationOperations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAnnotationOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationOperations;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: Self = Self(0i32);
    pub const AllAnnotationsReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactAnnotationStoreAccessType {}
impl ::core::clone::Clone for ContactAnnotationStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactAnnotationStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactAnnotationStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactAnnotationStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactAnnotationStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactBatchStatus {}
impl ::core::clone::Clone for ContactBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactBatchStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactBatchStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactBatchStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactBatchStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactBatchStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactBatchStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactCardHeaderKind {}
impl ::core::clone::Clone for ContactCardHeaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactCardHeaderKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactCardHeaderKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactCardHeaderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardHeaderKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCardHeaderKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardHeaderKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactCardTabKind(pub i32);
impl ContactCardTabKind {
    pub const Default: Self = Self(0i32);
    pub const Email: Self = Self(1i32);
    pub const Messaging: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const OrganizationalHierarchy: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactCardTabKind {}
impl ::core::clone::Clone for ContactCardTabKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactCardTabKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactCardTabKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactCardTabKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardTabKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCardTabKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardTabKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactChangeType {}
impl ::core::clone::Clone for ContactChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactDateKind(pub i32);
impl ContactDateKind {
    pub const Birthday: Self = Self(0i32);
    pub const Anniversary: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactDateKind {}
impl ::core::clone::Clone for ContactDateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactDateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactDateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactDateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactDateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactEmailKind {}
impl ::core::clone::Clone for ContactEmailKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactEmailKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactEmailKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactEmailKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactEmailKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactEmailKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactEmailKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Mobile: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
impl ::core::marker::Copy for ContactFieldCategory {}
impl ::core::clone::Clone for ContactFieldCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactFieldCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactFieldCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactFieldCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactFieldCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactFieldCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactFieldType(pub i32);
impl ContactFieldType {
    pub const Email: Self = Self(0i32);
    pub const PhoneNumber: Self = Self(1i32);
    pub const Location: Self = Self(2i32);
    pub const InstantMessage: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const ConnectedServiceAccount: Self = Self(5i32);
    pub const ImportantDate: Self = Self(6i32);
    pub const Address: Self = Self(7i32);
    pub const SignificantOther: Self = Self(8i32);
    pub const Notes: Self = Self(9i32);
    pub const Website: Self = Self(10i32);
    pub const JobInfo: Self = Self(11i32);
}
impl ::core::marker::Copy for ContactFieldType {}
impl ::core::clone::Clone for ContactFieldType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactFieldType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactFieldType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactFieldType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactFieldType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactFieldType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactListOtherAppReadAccess(pub i32);
impl ContactListOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactListOtherAppReadAccess {}
impl ::core::clone::Clone for ContactListOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactListOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactListOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactListOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactListOtherAppWriteAccess {}
impl ::core::clone::Clone for ContactListOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactListOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactListOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactListOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListOtherAppWriteAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactListSyncStatus(pub i32);
impl ContactListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for ContactListSyncStatus {}
impl ::core::clone::Clone for ContactListSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactListSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactListSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactListSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactMatchReasonKind(pub i32);
impl ContactMatchReasonKind {
    pub const Name: Self = Self(0i32);
    pub const EmailAddress: Self = Self(1i32);
    pub const PhoneNumber: Self = Self(2i32);
    pub const JobInfo: Self = Self(3i32);
    pub const YomiName: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactMatchReasonKind {}
impl ::core::clone::Clone for ContactMatchReasonKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactMatchReasonKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactMatchReasonKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactMatchReasonKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMatchReasonKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMatchReasonKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactMatchReasonKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: Self = Self(0i32);
    pub const LastNameFirstName: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactNameOrder {}
impl ::core::clone::Clone for ContactNameOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactNameOrder {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactNameOrder {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactNameOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactNameOrder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactNameOrder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactNameOrder;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactPhoneKind(pub i32);
impl ContactPhoneKind {
    pub const Home: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const Pager: Self = Self(4i32);
    pub const BusinessFax: Self = Self(5i32);
    pub const HomeFax: Self = Self(6i32);
    pub const Company: Self = Self(7i32);
    pub const Assistant: Self = Self(8i32);
    pub const Radio: Self = Self(9i32);
}
impl ::core::marker::Copy for ContactPhoneKind {}
impl ::core::clone::Clone for ContactPhoneKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactPhoneKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactPhoneKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactPhoneKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPhoneKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPhoneKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactPhoneKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: Self = Self(0u32);
    pub const PhoneNumber: Self = Self(1u32);
    pub const EmailAddress: Self = Self(2u32);
    pub const PostalAddress: Self = Self(4u32);
}
impl ::core::marker::Copy for ContactQueryDesiredFields {}
impl ::core::clone::Clone for ContactQueryDesiredFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactQueryDesiredFields {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactQueryDesiredFields {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactQueryDesiredFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryDesiredFields").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ContactQueryDesiredFields {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ContactQueryDesiredFields {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ContactQueryDesiredFields {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ContactQueryDesiredFields {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ContactQueryDesiredFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQueryDesiredFields {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQueryDesiredFields;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Name: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Phone: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ContactQuerySearchFields {}
impl ::core::clone::Clone for ContactQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactQuerySearchFields {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactQuerySearchFields {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactQuerySearchFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQuerySearchFields").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ContactQuerySearchFields {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ContactQuerySearchFields {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ContactQuerySearchFields {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ContactQuerySearchFields {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ContactQuerySearchFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQuerySearchFields {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchFields;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactQuerySearchScope {}
impl ::core::clone::Clone for ContactQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactQuerySearchScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactQuerySearchScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactQuerySearchScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQuerySearchScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQuerySearchScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactRelationship(pub i32);
impl ContactRelationship {
    pub const Other: Self = Self(0i32);
    pub const Spouse: Self = Self(1i32);
    pub const Partner: Self = Self(2i32);
    pub const Sibling: Self = Self(3i32);
    pub const Parent: Self = Self(4i32);
    pub const Child: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactRelationship {}
impl ::core::clone::Clone for ContactRelationship {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactRelationship {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactRelationship {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactRelationship {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactRelationship").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactRelationship {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactRelationship;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: Self = Self(0i32);
    pub const Fields: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactSelectionMode {}
impl ::core::clone::Clone for ContactSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: Self = Self(0i32);
    pub const AllContactsReadOnly: Self = Self(1i32);
    pub const AllContactsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactStoreAccessType {}
impl ::core::clone::Clone for ContactStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PinnedContactSurface(pub i32);
impl PinnedContactSurface {
    pub const StartMenu: Self = Self(0i32);
    pub const Taskbar: Self = Self(1i32);
}
impl ::core::marker::Copy for PinnedContactSurface {}
impl ::core::clone::Clone for PinnedContactSurface {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PinnedContactSurface {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PinnedContactSurface {
    type Abi = Self;
}
impl ::core::fmt::Debug for PinnedContactSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinnedContactSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PinnedContactSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.PinnedContactSurface;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
