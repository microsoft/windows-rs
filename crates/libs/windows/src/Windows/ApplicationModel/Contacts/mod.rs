#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAggregateContactManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAggregateContactManager {
    type Vtable = IAggregateContactManager_Vtbl;
}
impl ::core::clone::Clone for IAggregateContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAggregateContactManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0379d5dd_db5a_4fd3_b54e_4df17917a212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindRawContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindRawContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryLinkContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, primarycontact: *mut ::core::ffi::c_void, secondarycontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLinkContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnlinkRawContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlinkRawContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetPreferredSourceForPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aggregatecontact: *mut ::core::ffi::c_void, rawcontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetPreferredSourceForPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAggregateContactManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAggregateContactManager2 {
    type Vtable = IAggregateContactManager2_Vtbl;
}
impl ::core::clone::Clone for IAggregateContactManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAggregateContactManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e8cc2d8_a9cd_4430_9c4b_01348db2ca50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetRemoteIdentificationInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, remotesourceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, accountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRemoteIdentificationInformationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContact {
    type Vtable = IContact_Vtbl;
}
impl ::core::clone::Clone for IContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContact {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec0072f3_2118_4049_9ebc_17f0ab692b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Fields: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContact2 {
    type Vtable = IContact2_Vtbl;
}
impl ::core::clone::Clone for IContact2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContact2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf312f365_bb77_4c94_802d_8328cee40c08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNotes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Phones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Phones: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Addresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Addresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConnectedServiceAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConnectedServiceAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportantDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportantDates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSuppliers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSuppliers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub JobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    JobInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SignificantOthers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SignificantOthers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Websites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Websites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContact3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContact3 {
    type Vtable = IContact3_Vtbl;
}
impl ::core::clone::Clone for IContact3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContact3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48201e67_e08e_42a4_b561_41d08ca9575d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayPictureUserUpdateTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayPictureUserUpdateTime: usize,
    pub IsMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RingToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRingToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDisplayPictureManuallySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LargeDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LargeDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SmallDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmallDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SourceDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceDisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceDisplayPicture: usize,
    pub TextToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTextToneToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsAggregate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayNameOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayNameOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAddress {
    type Vtable = IContactAddress_Vtbl;
}
impl ::core::clone::Clone for IContactAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9739d39a_42ce_4872_8d70_3063aa584b70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAddress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAddressKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAddressKind) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAnnotation {
    type Vtable = IContactAnnotation_Vtbl;
}
impl ::core::clone::Clone for IContactAnnotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAnnotation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x821fc2ef_7d41_44a2_84c3_60a281dd7b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AnnotationListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows_core::HRESULT,
    pub SetSupportedOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows_core::HRESULT,
    pub IsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAnnotation2 {
    type Vtable = IContactAnnotation2_Vtbl;
}
impl ::core::clone::Clone for IContactAnnotation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAnnotation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb691ecf3_4ab7_4a1f_9941_0c9cf3171b75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContactListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAnnotationList {
    type Vtable = IContactAnnotationList_Vtbl;
}
impl ::core::clone::Clone for IContactAnnotationList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAnnotationList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92a486aa_5c88_45b9_aad0_461888e68d8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsByRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAnnotationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAnnotationStore {
    type Vtable = IContactAnnotationStore_Vtbl;
}
impl ::core::clone::Clone for IContactAnnotationStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAnnotationStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23acf4aa_7a77_457d_8203_987f4b31af09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByEmailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByPhoneNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAnnotationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListInAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationlistid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationListAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationListsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAnnotationStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactAnnotationStore2 {
    type Vtable = IContactAnnotationStore2_Vtbl;
}
impl ::core::clone::Clone for IContactAnnotationStore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactAnnotationStore2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ede23fd_61e7_4967_8ec5_bdf280a24063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactListAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactBatch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactBatch {
    type Vtable = IContactBatch_Vtbl;
}
impl ::core::clone::Clone for IContactBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactBatch {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35d1972d_bfce_46bb_93f8_a5b06ec5e201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactBatch_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contacts: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactBatchStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardDelayedDataLoader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_Vtbl;
}
impl ::core::clone::Clone for IContactCardDelayedDataLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactCardDelayedDataLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb60af902_1546_434d_869c_6e3520760ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardDelayedDataLoader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactCardOptions {
    type Vtable = IContactCardOptions_Vtbl;
}
impl ::core::clone::Clone for IContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactCardOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c0a4f7e_6ab6_4f3f_be72_817236eeea5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HeaderKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactCardHeaderKind) -> ::windows_core::HRESULT,
    pub SetHeaderKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactCardHeaderKind) -> ::windows_core::HRESULT,
    pub InitialTabKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactCardTabKind) -> ::windows_core::HRESULT,
    pub SetInitialTabKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactCardTabKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactCardOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactCardOptions2 {
    type Vtable = IContactCardOptions2_Vtbl;
}
impl ::core::clone::Clone for IContactCardOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactCardOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f271ba0_d74b_4cc6_9f53_1b0eb5d1273c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerSearchContactListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerSearchContactListIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChange {
    type Vtable = IContactChange_Vtbl;
}
impl ::core::clone::Clone for IContactChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x951d4b10_6a59_4720_a4e1_363d98c135d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactChangeType) -> ::windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChangeReader {
    type Vtable = IContactChangeReader_Vtbl;
}
impl ::core::clone::Clone for IContactChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChangeReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217319fa_2d0c_42e0_a9da_3ecd56a78a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoaccept: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChangeTracker {
    type Vtable = IContactChangeTracker_Vtbl;
}
impl ::core::clone::Clone for IContactChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChangeTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e992952_309b_404d_9712_b37bd30278aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeTracker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChangeTracker2 {
    type Vtable = IContactChangeTracker2_Vtbl;
}
impl ::core::clone::Clone for IContactChangeTracker2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChangeTracker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f8ad0fc_9321_4d18_9c09_d708c63fcd31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChangedDeferral {
    type Vtable = IContactChangedDeferral_Vtbl;
}
impl ::core::clone::Clone for IContactChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChangedDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5143ae8_1b03_46f8_b694_a523e83cfcb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x525e7fd1_73f3_4b7d_a918_580be4366121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactConnectedServiceAccount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_Vtbl;
}
impl ::core::clone::Clone for IContactConnectedServiceAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactConnectedServiceAccount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6f83553_aa27_4731_8e4a_3dec5ce9eec9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactConnectedServiceAccount_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactDate {
    type Vtable = IContactDate_Vtbl;
}
impl ::core::clone::Clone for IContactDate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactDate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe98ae66_b205_4934_9174_0ff2b0565707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDay: usize,
    #[cfg(feature = "Foundation")]
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Month: usize,
    #[cfg(feature = "Foundation")]
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Year: usize,
    #[cfg(feature = "Foundation")]
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetYear: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactDateKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactDateKind) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactEmail(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactEmail {
    type Vtable = IContactEmail_Vtbl;
}
impl ::core::clone::Clone for IContactEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactEmail {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90a219a9_e3d3_4d63_993b_05b9a5393abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactEmail_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactEmailKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactEmailKind) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactField(::windows_core::IUnknown);
impl IContactField {
    pub fn Type(&self) -> ::windows_core::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactField, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IContactField {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b176486a-d293-492c-a058-db575b3e3c0f}");
}
unsafe impl ::windows_core::Interface for IContactField {
    type Vtable = IContactField_Vtbl;
}
impl ::core::clone::Clone for IContactField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactField {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb176486a_d293_492c_a058_db575b3e3c0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactField_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldCategory) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactFieldFactory(::windows_core::IUnknown);
impl IContactFieldFactory {
    pub fn CreateField_Default(&self, value: &::windows_core::HSTRING, r#type: ContactFieldType) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateField_Category(&self, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateField_Custom(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Custom)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactFieldFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IContactFieldFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{85e2913f-0e4a-4a3e-8994-406ae7ed646e}");
}
unsafe impl ::windows_core::Interface for IContactFieldFactory {
    type Vtable = IContactFieldFactory_Vtbl;
}
impl ::core::clone::Clone for IContactFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactFieldFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85e2913f_0e4a_4a3e_8994_406ae7ed646e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactFieldFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateField_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateField_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateField_Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactGroup {
    type Vtable = IContactGroup_Vtbl;
}
impl ::core::clone::Clone for IContactGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59bdeb01_9e9a_475d_bfe5_a37b806d852c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactInformation {
    type Vtable = IContactInformation_Vtbl;
}
impl ::core::clone::Clone for IContactInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x275eb6d4_6a2e_4278_a914_e460d5f088f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneNumbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstantMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstantMessages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomFields: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryCustomFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryCustomFields: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactInstantMessageField(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactInstantMessageField {
    type Vtable = IContactInstantMessageField_Vtbl;
}
impl ::core::clone::Clone for IContactInstantMessageField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactInstantMessageField {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcce33b37_0d85_41fa_b43d_da599c3eb009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageField_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUri: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactInstantMessageFieldFactory(::windows_core::IUnknown);
impl IContactInstantMessageFieldFactory {
    pub fn CreateInstantMessage_Default(&self, username: &::windows_core::HSTRING) -> ::windows_core::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Category(&self, username: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All<P0>(&self, username: &::windows_core::HSTRING, category: ContactFieldCategory, service: &::windows_core::HSTRING, displaytext: &::windows_core::HSTRING, verb: P0) -> ::windows_core::Result<ContactInstantMessageField>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), verb.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactInstantMessageFieldFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IContactInstantMessageFieldFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ba0b6794-91a3-4bb2-b1b9-69a5dff0ba09}");
}
unsafe impl ::windows_core::Interface for IContactInstantMessageFieldFactory {
    type Vtable = IContactInstantMessageFieldFactory_Vtbl;
}
impl ::core::clone::Clone for IContactInstantMessageFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactInstantMessageFieldFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba0b6794_91a3_4bb2_b1b9_69a5dff0ba09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageFieldFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstantMessage_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInstantMessage_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstantMessage_All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, service: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displaytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, verb: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstantMessage_All: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactJobInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactJobInfo {
    type Vtable = IContactJobInfo_Vtbl;
}
impl ::core::clone::Clone for IContactJobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactJobInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d117b4c_ce50_4b43_9e69_b18258ea5315);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactJobInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CompanyYomiName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompanyYomiName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Office: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOffice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CompanyAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompanyAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactLaunchActionVerbsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactLaunchActionVerbsStatics {
    type Vtable = IContactLaunchActionVerbsStatics_Vtbl;
}
impl ::core::clone::Clone for IContactLaunchActionVerbsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactLaunchActionVerbsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb1232d6_ee73_46e7_8761_11cd0157728f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLaunchActionVerbsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactList {
    type Vtable = IContactList_Vtbl;
}
impl ::core::clone::Clone for IContactList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16ddec75_392c_4845_9dfb_51a3e7ef3e42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SupportsServerSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactFromRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactFromRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactList2 {
    type Vtable = IContactList2_Vtbl;
}
impl ::core::clone::Clone for IContactList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactList2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb3943b4_4550_4dcb_9229_40ff91fb0203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
    pub SetSupportsServerSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SyncConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactList3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactList3 {
    type Vtable = IContactList3_Vtbl;
}
impl ::core::clone::Clone for IContactList3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactList3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1578ee57_26fc_41e8_a850_5aa32514aca9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListLimitedWriteOperations(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_Vtbl;
}
impl ::core::clone::Clone for IContactListLimitedWriteOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactListLimitedWriteOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe19813da_4a0b_44b8_9a1f_a0f3d218175f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListLimitedWriteOperations_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncConstraints(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_Vtbl;
}
impl ::core::clone::Clone for IContactListSyncConstraints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactListSyncConstraints {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2b0bf01_3062_4e2e_969d_018d1987f314);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncConstraints_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanSyncDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanSyncDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBirthdayDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBirthdayDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAnniversaryDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAnniversaryDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherDates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSpouseRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSpouseRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPartnerRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPartnerRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSiblingRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSiblingRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxParentRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxParentRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxChildRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxChildRelationships: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxJobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxJobInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWebsites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWebsites: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWebsites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWebsites: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactListSyncManager {
    type Vtable = IContactListSyncManager_Vtbl;
}
impl ::core::clone::Clone for IContactListSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactListSyncManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x146e83be_7925_4acc_9de5_21ddd06f8674);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactListSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactListSyncManager2 {
    type Vtable = IContactListSyncManager2_Vtbl;
}
impl ::core::clone::Clone for IContactListSyncManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactListSyncManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9591247_bb55_4e23_8128_370134a85d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactListSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactLocationField(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactLocationField {
    type Vtable = IContactLocationField_Vtbl;
}
impl ::core::clone::Clone for IContactLocationField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactLocationField {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ec00f82_ab6e_4b36_89e3_b23bc0a1dacc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationField_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnstructuredAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct IContactLocationFieldFactory(::windows_core::IUnknown);
impl IContactLocationFieldFactory {
    pub fn CreateLocation_Default(&self, unstructuredaddress: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_Category(&self, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_All(&self, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory, street: &::windows_core::HSTRING, city: &::windows_core::HSTRING, region: &::windows_core::HSTRING, country: &::windows_core::HSTRING, postalcode: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactLocationFieldFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IContactLocationFieldFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f79932d7-2fdf-43fe-8f18-41897390bcfe}");
}
unsafe impl ::windows_core::Interface for IContactLocationFieldFactory {
    type Vtable = IContactLocationFieldFactory_Vtbl;
}
impl ::core::clone::Clone for IContactLocationFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactLocationFieldFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf79932d7_2fdf_43fe_8f18_41897390bcfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationFieldFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateLocation_Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLocation_Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLocation_All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, street: ::std::mem::MaybeUninit<::windows_core::HSTRING>, city: ::std::mem::MaybeUninit<::windows_core::HSTRING>, region: ::std::mem::MaybeUninit<::windows_core::HSTRING>, country: ::std::mem::MaybeUninit<::windows_core::HSTRING>, postalcode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerForUser {
    type Vtable = IContactManagerForUser_Vtbl;
}
impl ::core::clone::Clone for IContactManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb74bba57_1076_4bef_aef3_54686d18387d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows_core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows_core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows_core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerForUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerForUser2 {
    type Vtable = IContactManagerForUser2_Vtbl;
}
impl ::core::clone::Clone for IContactManagerForUser2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerForUser2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d469c2e_3b75_4a73_bb30_736645472256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerStatics {
    type Vtable = IContactManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IContactManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81f21ac0_f661_4708_ba4f_d386bd0d622e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowContactCard: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithPlacement: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCard: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerStatics2 {
    type Vtable = IContactManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IContactManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa178e620_47d8_48cc_963c_9592b6e510c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerStatics3 {
    type Vtable = IContactManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IContactManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4cc3d42_7586_492a_930b_7bc138fc2139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsyncWithAccessType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsyncWithAccessType: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub IsShowContactCardSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithOptions: usize,
    pub IsShowDelayLoadedContactCardSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCardWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCardWithOptions: usize,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows_core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows_core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactNameOrder) -> ::windows_core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactNameOrder) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerStatics4 {
    type Vtable = IContactManagerStatics4_Vtbl;
}
impl ::core::clone::Clone for IContactManagerStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24982272_347b_46dc_8d95_51bd41e15aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactManagerStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactManagerStatics5 {
    type Vtable = IContactManagerStatics5_Vtbl;
}
impl ::core::clone::Clone for IContactManagerStatics5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactManagerStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7591a87_acb7_4fad_90f2_a8ab64cdbba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsShowFullContactCardSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShowFullContactCardSupportedAsync: usize,
    pub IncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactMatchReason(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactMatchReason {
    type Vtable = IContactMatchReason_Vtbl;
}
impl ::core::clone::Clone for IContactMatchReason {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactMatchReason {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc922504_e7d8_413e_95f4_b75c54c74077);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMatchReason_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Field: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactMatchReasonKind) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub Segments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    Segments: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactName(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactName {
    type Vtable = IContactName_Vtbl;
}
impl ::core::clone::Clone for IContactName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactName {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf404e97b_9034_453c_8ebf_140a38c86f1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactName_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MiddleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMiddleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YomiGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetYomiGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YomiFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetYomiFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HonorificNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetHonorificNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HonorificNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetHonorificNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub YomiDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPanel {
    type Vtable = IContactPanel_Vtbl;
}
impl ::core::clone::Clone for IContactPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41bf1265_d2ee_4b97_a80a_7d8d64cca6f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ClosePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub HeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    HeaderColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub SetHeaderColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    SetHeaderColor: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullAppRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLaunchFullAppRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanelClosingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactPanelClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPanelClosingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x222174d3_cf4b_46d7_b739_6edc16110bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelClosingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactPanelLaunchFullAppRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPanelLaunchFullAppRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88d61c0e_23b4_4be8_8afc_072c25a4190d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPhone(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPhone {
    type Vtable = IContactPhone_Vtbl;
}
impl ::core::clone::Clone for IContactPhone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPhone {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x467dab65_2712_4f52_b783_9ea8111c63cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPhone_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactPhoneKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactPhoneKind) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPicker {
    type Vtable = IContactPicker_Vtbl;
}
impl ::core::clone::Clone for IContactPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e09fd91_42f8_4055_90a0_896f96738936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactSelectionMode) -> ::windows_core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactSelectionMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFields: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleContactsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPicker2 {
    type Vtable = IContactPicker2_Vtbl;
}
impl ::core::clone::Clone for IContactPicker2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPicker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35011cf_5cef_4d24_aa0c_340c5208725d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
    #[cfg(feature = "Foundation")]
    pub PickContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickContactsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPicker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPicker3 {
    type Vtable = IContactPicker3_Vtbl;
}
impl ::core::clone::Clone for IContactPicker3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPicker3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e723315_b243_4bed_8516_22b1a7ac0ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPickerStatics {
    type Vtable = IContactPickerStatics_Vtbl;
}
impl ::core::clone::Clone for IContactPickerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPickerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7488c029_6a53_4258_a3e9_62dff6784b6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactQueryOptions {
    type Vtable = IContactQueryOptions_Vtbl;
}
impl ::core::clone::Clone for IContactQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactQueryOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4408cc9e_7d7c_42f0_8ac7_f50733ecdbc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TextSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactListIds: usize,
    pub IncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQueryDesiredFields) -> ::windows_core::HRESULT,
    pub SetDesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQueryDesiredFields) -> ::windows_core::HRESULT,
    pub DesiredOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactAnnotationOperations) -> ::windows_core::HRESULT,
    pub SetDesiredOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactAnnotationOperations) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AnnotationListIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AnnotationListIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactQueryOptionsFactory {
    type Vtable = IContactQueryOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for IContactQueryOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactQueryOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543fba47_8ce7_46cb_9dac_9aa42a1bc8e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: ContactQuerySearchFields, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryTextSearch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_Vtbl;
}
impl ::core::clone::Clone for IContactQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactQueryTextSearch {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7e3f9cb_a957_439b_a0b7_1c02a1963ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryTextSearch_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchFields) -> ::windows_core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQuerySearchFields) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQuerySearchScope) -> ::windows_core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQuerySearchScope) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactReader {
    type Vtable = IContactReader_Vtbl;
}
impl ::core::clone::Clone for IContactReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd397e42e_1488_42f2_bf64_253f4884bfed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMatchingPropertiesWithMatchReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMatchingPropertiesWithMatchReason: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactSignificantOther(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactSignificantOther {
    type Vtable = IContactSignificantOther_Vtbl;
}
impl ::core::clone::Clone for IContactSignificantOther {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactSignificantOther {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8873b5ab_c5fb_46d8_93fe_da3ff1934054);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactSignificantOther2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactSignificantOther2 {
    type Vtable = IContactSignificantOther2_Vtbl;
}
impl ::core::clone::Clone for IContactSignificantOther2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactSignificantOther2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d7bd474_3f03_45f8_ba0f_c4ed37d64219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactRelationship) -> ::windows_core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactRelationship) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactStore {
    type Vtable = IContactStore_Vtbl;
}
impl ::core::clone::Clone for IContactStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c220b10_3a6c_4293_b9bc_fe987f6e0d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsWithSearchTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsWithSearchTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactStore2 {
    type Vtable = IContactStore2_Vtbl;
}
impl ::core::clone::Clone for IContactStore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactStore2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18ce1c22_ebd5_4bfb_b690_5f4f27c4f0e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    pub AggregateContactManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactListsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateContactListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateContactListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, userdataaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactStore3 {
    type Vtable = IContactStore3_Vtbl;
}
impl ::core::clone::Clone for IContactStore3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactStore3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb882c6c_004e_4050_87f0_840407ee6818);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStoreNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IContactStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactStoreNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabb298d6_878a_4f8b_a9ce_46bb7d1c84ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactWebsite(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactWebsite {
    type Vtable = IContactWebsite_Vtbl;
}
impl ::core::clone::Clone for IContactWebsite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactWebsite {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f130176_dc1b_4055_ad66_652f39d990e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactWebsite2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactWebsite2 {
    type Vtable = IContactWebsite2_Vtbl;
}
impl ::core::clone::Clone for IContactWebsite2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactWebsite2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf87ee91e_5647_4068_bb5e_4b6f437ce308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullContactCardOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullContactCardOptions {
    type Vtable = IFullContactCardOptions_Vtbl;
}
impl ::core::clone::Clone for IFullContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFullContactCardOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8744436c_5cf9_4683_bdca_a1fdebf8dbce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullContactCardOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownContactFieldStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IKnownContactFieldStatics {
    type Vtable = IKnownContactFieldStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IKnownContactFieldStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IKnownContactFieldStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e0e1b12_d627_4fca_bad4_1faf168c7d14);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownContactFieldStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Email: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Location: usize,
    #[cfg(feature = "deprecated")]
    pub InstantMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InstantMessage: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertNameToType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ContactFieldType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertNameToType: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertTypeToName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ContactFieldType, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertTypeToName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactIdsQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_Vtbl;
}
impl ::core::clone::Clone for IPinnedContactIdsQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPinnedContactIdsQueryResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9b2552_1579_4ddc_871f_a30a3aea9ba1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactIdsQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPinnedContactManager {
    type Vtable = IPinnedContactManager_Vtbl;
}
impl ::core::clone::Clone for IPinnedContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPinnedContactManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcbc740c_e1d6_45c3_b8b6_a35604e167a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub IsPinSurfaceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsContactPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPinContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestPinContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contacts: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestPinContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnpinContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnpinContactAsync: usize,
    pub SignalContactActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPinnedContactIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPinnedContactIdsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinnedContactManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPinnedContactManagerStatics {
    type Vtable = IPinnedContactManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPinnedContactManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPinnedContactManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf65ccc7e_fdf9_486a_ace9_bc311d0ae7f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct AggregateContactManager(::windows_core::IUnknown);
impl AggregateContactManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindRawContactsAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindRawContactsAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryLinkContactsAsync<P0, P1>(&self, primarycontact: P0, secondarycontact: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryLinkContactsAsync)(::windows_core::Interface::as_raw(this), primarycontact.into_param().abi(), secondarycontact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnlinkRawContactAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnlinkRawContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetPreferredSourceForPictureAsync<P0, P1>(&self, aggregatecontact: P0, rawcontact: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetPreferredSourceForPictureAsync)(::windows_core::Interface::as_raw(this), aggregatecontact.into_param().abi(), rawcontact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRemoteIdentificationInformationAsync(&self, contactlistid: &::windows_core::HSTRING, remotesourceid: &::windows_core::HSTRING, accountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IAggregateContactManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetRemoteIdentificationInformationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactlistid), ::core::mem::transmute_copy(remotesourceid), ::core::mem::transmute_copy(accountid), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AggregateContactManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.AggregateContactManager;{0379d5dd-db5a-4fd3-b54e-4df17917a212})");
}
impl ::core::clone::Clone for AggregateContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AggregateContactManager {
    type Vtable = IAggregateContactManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AggregateContactManager {
    const IID: ::windows_core::GUID = <IAggregateContactManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AggregateContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.AggregateContactManager";
}
::windows_core::imp::interface_hierarchy!(AggregateContactManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AggregateContactManager {}
unsafe impl ::core::marker::Sync for AggregateContactManager {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct Contact(::windows_core::IUnknown);
impl Contact {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Contact, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Fields(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Fields)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Notes(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Notes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNotes(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNotes)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Phones(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactPhone>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Phones)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Emails(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactEmail>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Addresses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactAddress>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Addresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConnectedServiceAccounts(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectedServiceAccounts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportantDates(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactDate>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportantDates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DataSuppliers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataSuppliers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn JobInfo(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactJobInfo>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JobInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SignificantOthers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactSignificantOther>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantOthers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Websites(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactWebsite>> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Websites)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactListId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayPictureUserUpdateTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayPictureUserUpdateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDisplayPictureUserUpdateTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayPictureUserUpdateTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMe(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMe)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AggregateId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AggregateId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RingToneToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RingToneToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRingToneToken(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRingToneToken)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDisplayPictureManuallySet(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisplayPictureManuallySet)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LargeDisplayPicture(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LargeDisplayPicture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SmallDisplayPicture(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmallDisplayPicture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceDisplayPicture(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayPicture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceDisplayPicture<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceDisplayPicture)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn TextToneToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextToneToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextToneToken(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTextToneToken)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsAggregate(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAggregate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayNameOverride(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayNameOverride)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayNameOverride(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayNameOverride)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Nickname(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Nickname)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNickname(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNickname)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SortName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SortName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFirstName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFirstName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LastName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLastName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MiddleName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MiddleName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMiddleName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMiddleName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn YomiGivenName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YomiGivenName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYomiGivenName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetYomiGivenName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn YomiFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YomiFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYomiFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetYomiFamilyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificNameSuffix(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HonorificNameSuffix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHonorificNameSuffix(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHonorificNameSuffix)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificNamePrefix(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HonorificNamePrefix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHonorificNamePrefix(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHonorificNamePrefix)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YomiDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YomiDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for Contact {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Contact;{ec0072f3-2118-4049-9ebc-17f0ab692b64})");
}
impl ::core::clone::Clone for Contact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Contact {
    type Vtable = IContact_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Contact {
    const IID: ::windows_core::GUID = <IContact as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Contact {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Contact";
}
::windows_core::imp::interface_hierarchy!(Contact, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Contact {}
unsafe impl ::core::marker::Sync for Contact {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAddress(::windows_core::IUnknown);
impl ContactAddress {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactAddress, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StreetAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreetAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStreetAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreetAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Locality(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Locality)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLocality(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocality)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Region(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Region)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRegion(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCountry(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCountry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPostalCode(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPostalCode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ContactAddressKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactAddressKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactAddress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAddress;{9739d39a-42ce-4872-8d70-3063aa584b70})");
}
impl ::core::clone::Clone for ContactAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactAddress {
    type Vtable = IContactAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactAddress {
    const IID: ::windows_core::GUID = <IContactAddress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAddress";
}
::windows_core::imp::interface_hierarchy!(ContactAddress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactAddress {}
unsafe impl ::core::marker::Sync for ContactAddress {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotation(::windows_core::IUnknown);
impl ContactAnnotation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactAnnotation, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AnnotationListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationListId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContactId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContactId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SupportedOperations(&self) -> ::windows_core::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedOperations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportedOperations(&self, value: ContactAnnotationOperations) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportedOperations)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDisabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactAnnotation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactListId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContactListId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactAnnotation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContactListId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactAnnotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotation;{821fc2ef-7d41-44a2-84c3-60a281dd7b86})");
}
impl ::core::clone::Clone for ContactAnnotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactAnnotation {
    type Vtable = IContactAnnotation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactAnnotation {
    const IID: ::windows_core::GUID = <IContactAnnotation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactAnnotation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotation";
}
::windows_core::imp::interface_hierarchy!(ContactAnnotation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotation {}
unsafe impl ::core::marker::Sync for ContactAnnotation {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationList(::windows_core::IUnknown);
impl ContactAnnotationList {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySaveAnnotationAsync<P0>(&self, annotation: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<ContactAnnotation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySaveAnnotationAsync)(::windows_core::Interface::as_raw(this), annotation.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnnotationAsync(&self, annotationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(annotationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsByRemoteIdAsync(&self, remoteid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAnnotationsByRemoteIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(remoteid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAnnotationsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAnnotationAsync<P0>(&self, annotation: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<ContactAnnotation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAnnotationAsync)(::windows_core::Interface::as_raw(this), annotation.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactAnnotationList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationList;{92a486aa-5c88-45b9-aad0-461888e68d8a})");
}
impl ::core::clone::Clone for ContactAnnotationList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactAnnotationList {
    type Vtable = IContactAnnotationList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactAnnotationList {
    const IID: ::windows_core::GUID = <IContactAnnotationList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactAnnotationList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationList";
}
::windows_core::imp::interface_hierarchy!(ContactAnnotationList, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotationList {}
unsafe impl ::core::marker::Sync for ContactAnnotationList {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationStore(::windows_core::IUnknown);
impl ContactAnnotationStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactIdsByEmailAsync(&self, emailaddress: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindContactIdsByEmailAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(emailaddress), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactIdsByPhoneNumberAsync(&self, phonenumber: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindContactIdsByPhoneNumberAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(phonenumber), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsForContactAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAnnotationsForContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAnnotationAsync<P0>(&self, annotation: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<ContactAnnotation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableAnnotationAsync)(::windows_core::Interface::as_raw(this), annotation.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAnnotationListAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAnnotationListAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAnnotationListInAccountAsync(&self, userdataaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAnnotationListInAccountAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(userdataaccountid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnnotationListAsync(&self, annotationlistid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotationListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(annotationlistid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationListsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAnnotationListsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAnnotationsForContactListAsync(&self, contactlistid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = &::windows_core::ComInterface::cast::<IContactAnnotationStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAnnotationsForContactListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactlistid), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactAnnotationStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationStore;{23acf4aa-7a77-457d-8203-987f4b31af09})");
}
impl ::core::clone::Clone for ContactAnnotationStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactAnnotationStore {
    type Vtable = IContactAnnotationStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactAnnotationStore {
    const IID: ::windows_core::GUID = <IContactAnnotationStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactAnnotationStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationStore";
}
::windows_core::imp::interface_hierarchy!(ContactAnnotationStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactAnnotationStore {}
unsafe impl ::core::marker::Sync for ContactAnnotationStore {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactBatch(::windows_core::IUnknown);
impl ContactBatch {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contacts(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contacts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<ContactBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactBatch {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactBatch;{35d1972d-bfce-46bb-93f8-a5b06ec5e201})");
}
impl ::core::clone::Clone for ContactBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactBatch {
    type Vtable = IContactBatch_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactBatch {
    const IID: ::windows_core::GUID = <IContactBatch as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactBatch";
}
::windows_core::imp::interface_hierarchy!(ContactBatch, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactBatch {}
unsafe impl ::core::marker::Sync for ContactBatch {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(::windows_core::IUnknown);
impl ContactCardDelayedDataLoader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetData<P0>(&self, contact: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), contact.into_param().abi()).ok() }
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
impl ::windows_core::RuntimeType for ContactCardDelayedDataLoader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader;{b60af902-1546-434d-869c-6e3520760ef3})");
}
impl ::core::clone::Clone for ContactCardDelayedDataLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactCardDelayedDataLoader {
    const IID: ::windows_core::GUID = <IContactCardDelayedDataLoader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactCardDelayedDataLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader";
}
::windows_core::imp::interface_hierarchy!(ContactCardDelayedDataLoader, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for ContactCardDelayedDataLoader {}
unsafe impl ::core::marker::Send for ContactCardDelayedDataLoader {}
unsafe impl ::core::marker::Sync for ContactCardDelayedDataLoader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardOptions(::windows_core::IUnknown);
impl ContactCardOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactCardOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HeaderKind(&self) -> ::windows_core::Result<ContactCardHeaderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeaderKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHeaderKind(&self, value: ContactCardHeaderKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHeaderKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialTabKind(&self) -> ::windows_core::Result<ContactCardTabKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialTabKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInitialTabKind(&self, value: ContactCardTabKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialTabKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerSearchContactListIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IContactCardOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerSearchContactListIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactCardOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardOptions;{8c0a4f7e-6ab6-4f3f-be72-817236eeea5b})");
}
impl ::core::clone::Clone for ContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactCardOptions {
    type Vtable = IContactCardOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactCardOptions {
    const IID: ::windows_core::GUID = <IContactCardOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardOptions";
}
::windows_core::imp::interface_hierarchy!(ContactCardOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactCardOptions {}
unsafe impl ::core::marker::Sync for ContactCardOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChange(::windows_core::IUnknown);
impl ContactChange {
    pub fn ChangeType(&self) -> ::windows_core::Result<ContactChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contact(&self) -> ::windows_core::Result<Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChange;{951d4b10-6a59-4720-a4e1-363d98c135d5})");
}
impl ::core::clone::Clone for ContactChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactChange {
    type Vtable = IContactChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactChange {
    const IID: ::windows_core::GUID = <IContactChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactChange {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChange";
}
::windows_core::imp::interface_hierarchy!(ContactChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactChange {}
unsafe impl ::core::marker::Sync for ContactChange {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangeReader(::windows_core::IUnknown);
impl ContactChangeReader {
    pub fn AcceptChanges(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChanges)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough<P0>(&self, lastchangetoaccept: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ContactChange>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChangesThrough)(::windows_core::Interface::as_raw(this), lastchangetoaccept.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactChangeReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeReader;{217319fa-2d0c-42e0-a9da-3ecd56a78a47})");
}
impl ::core::clone::Clone for ContactChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactChangeReader {
    type Vtable = IContactChangeReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactChangeReader {
    const IID: ::windows_core::GUID = <IContactChangeReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeReader";
}
::windows_core::imp::interface_hierarchy!(ContactChangeReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangeReader {}
unsafe impl ::core::marker::Sync for ContactChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangeTracker(::windows_core::IUnknown);
impl ContactChangeTracker {
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows_core::Result<ContactChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsTracking(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IContactChangeTracker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTracking)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactChangeTracker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeTracker;{6e992952-309b-404d-9712-b37bd30278aa})");
}
impl ::core::clone::Clone for ContactChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactChangeTracker {
    type Vtable = IContactChangeTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactChangeTracker {
    const IID: ::windows_core::GUID = <IContactChangeTracker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeTracker";
}
::windows_core::imp::interface_hierarchy!(ContactChangeTracker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangeTracker {}
unsafe impl ::core::marker::Sync for ContactChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangedDeferral(::windows_core::IUnknown);
impl ContactChangedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for ContactChangedDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedDeferral;{c5143ae8-1b03-46f8-b694-a523e83cfcb6})");
}
impl ::core::clone::Clone for ContactChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactChangedDeferral {
    type Vtable = IContactChangedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactChangedDeferral {
    const IID: ::windows_core::GUID = <IContactChangedDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedDeferral";
}
::windows_core::imp::interface_hierarchy!(ContactChangedDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangedDeferral {}
unsafe impl ::core::marker::Sync for ContactChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangedEventArgs(::windows_core::IUnknown);
impl ContactChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<ContactChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedEventArgs;{525e7fd1-73f3-4b7d-a918-580be4366121})");
}
impl ::core::clone::Clone for ContactChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactChangedEventArgs {
    const IID: ::windows_core::GUID = <IContactChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContactChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactConnectedServiceAccount(::windows_core::IUnknown);
impl ContactConnectedServiceAccount {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactConnectedServiceAccount, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetServiceName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServiceName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactConnectedServiceAccount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount;{f6f83553-aa27-4731-8e4a-3dec5ce9eec9})");
}
impl ::core::clone::Clone for ContactConnectedServiceAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactConnectedServiceAccount {
    const IID: ::windows_core::GUID = <IContactConnectedServiceAccount as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactConnectedServiceAccount {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount";
}
::windows_core::imp::interface_hierarchy!(ContactConnectedServiceAccount, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactConnectedServiceAccount {}
unsafe impl ::core::marker::Sync for ContactConnectedServiceAccount {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactDate(::windows_core::IUnknown);
impl ContactDate {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactDate, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Day(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Day)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDay<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDay)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Month(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Month)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMonth<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMonth)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Year(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Year)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetYear<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYear)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ContactDateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactDateKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactDate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactDate;{fe98ae66-b205-4934-9174-0ff2b0565707})");
}
impl ::core::clone::Clone for ContactDate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactDate {
    type Vtable = IContactDate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactDate {
    const IID: ::windows_core::GUID = <IContactDate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactDate {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactDate";
}
::windows_core::imp::interface_hierarchy!(ContactDate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactDate {}
unsafe impl ::core::marker::Sync for ContactDate {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactEmail(::windows_core::IUnknown);
impl ContactEmail {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactEmail, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ContactEmailKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactEmailKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactEmail {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactEmail;{90a219a9-e3d3-4d63-993b-05b9a5393abf})");
}
impl ::core::clone::Clone for ContactEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactEmail {
    type Vtable = IContactEmail_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactEmail {
    const IID: ::windows_core::GUID = <IContactEmail as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactEmail {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactEmail";
}
::windows_core::imp::interface_hierarchy!(ContactEmail, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactEmail {}
unsafe impl ::core::marker::Sync for ContactEmail {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactField(::windows_core::IUnknown);
impl ContactField {
    pub fn Type(&self) -> ::windows_core::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateField_Default(value: &::windows_core::HSTRING, r#type: ContactFieldType) -> ::windows_core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateField_Category(value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateField_Custom(name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Custom)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactFieldFactory<R, F: FnOnce(&IContactFieldFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactField, IContactFieldFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ContactField {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactField;{b176486a-d293-492c-a058-db575b3e3c0f})");
}
impl ::core::clone::Clone for ContactField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactField {
    type Vtable = IContactField_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactField {
    const IID: ::windows_core::GUID = <IContactField as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactField";
}
::windows_core::imp::interface_hierarchy!(ContactField, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IContactField> for ContactField {}
unsafe impl ::core::marker::Send for ContactField {}
unsafe impl ::core::marker::Sync for ContactField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactFieldFactory(::windows_core::IUnknown);
impl ContactFieldFactory {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactFieldFactory, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateField_Default(&self, value: &::windows_core::HSTRING, r#type: ContactFieldType) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateField_Category(&self, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateField_Custom(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateField_Custom)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), r#type, category, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Default(&self, username: &::windows_core::HSTRING) -> ::windows_core::Result<ContactInstantMessageField> {
        let this = &::windows_core::ComInterface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Category(&self, username: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactInstantMessageField> {
        let this = &::windows_core::ComInterface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All<P0>(&self, username: &::windows_core::HSTRING, category: ContactFieldCategory, service: &::windows_core::HSTRING, displaytext: &::windows_core::HSTRING, verb: P0) -> ::windows_core::Result<ContactInstantMessageField>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), verb.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_Default(&self, unstructuredaddress: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        let this = &::windows_core::ComInterface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_Category(&self, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactLocationField> {
        let this = &::windows_core::ComInterface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_All(&self, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory, street: &::windows_core::HSTRING, city: &::windows_core::HSTRING, region: &::windows_core::HSTRING, country: &::windows_core::HSTRING, postalcode: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        let this = &::windows_core::ComInterface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactFieldFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactFieldFactory;{85e2913f-0e4a-4a3e-8994-406ae7ed646e})");
}
impl ::core::clone::Clone for ContactFieldFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactFieldFactory {
    type Vtable = IContactFieldFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactFieldFactory {
    const IID: ::windows_core::GUID = <IContactFieldFactory as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactFieldFactory";
}
::windows_core::imp::interface_hierarchy!(ContactFieldFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IContactFieldFactory> for ContactFieldFactory {}
impl ::windows_core::CanTryInto<IContactInstantMessageFieldFactory> for ContactFieldFactory {}
impl ::windows_core::CanTryInto<IContactLocationFieldFactory> for ContactFieldFactory {}
unsafe impl ::core::marker::Send for ContactFieldFactory {}
unsafe impl ::core::marker::Sync for ContactFieldFactory {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactGroup(::windows_core::IUnknown);
impl ContactGroup {}
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
impl ::windows_core::RuntimeType for ContactGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactGroup;{59bdeb01-9e9a-475d-bfe5-a37b806d852c})");
}
impl ::core::clone::Clone for ContactGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactGroup {
    type Vtable = IContactGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactGroup {
    const IID: ::windows_core::GUID = <IContactGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactGroup";
}
::windows_core::imp::interface_hierarchy!(ContactGroup, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactGroup {}
unsafe impl ::core::marker::Sync for ContactGroup {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactInformation(::windows_core::IUnknown);
impl ContactInformation {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Emails(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Locations(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactLocationField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Locations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstantMessages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstantMessages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomFields(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomFields)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryCustomFields(&self, customname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryCustomFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(customname), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInformation;{275eb6d4-6a2e-4278-a914-e460d5f088f6})");
}
impl ::core::clone::Clone for ContactInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactInformation {
    type Vtable = IContactInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactInformation {
    const IID: ::windows_core::GUID = <IContactInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInformation";
}
::windows_core::imp::interface_hierarchy!(ContactInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactInstantMessageField(::windows_core::IUnknown);
impl ContactInstantMessageField {
    pub fn Type(&self) -> ::windows_core::Result<ContactFieldType> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<ContactFieldCategory> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Service(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Service)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstantMessage_Default(username: &::windows_core::HSTRING) -> ::windows_core::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInstantMessage_Category(username: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstantMessage_All<P0>(username: &::windows_core::HSTRING, category: ContactFieldCategory, service: &::windows_core::HSTRING, displaytext: &::windows_core::HSTRING, verb: P0) -> ::windows_core::Result<ContactInstantMessageField>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstantMessage_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), category, ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(displaytext), verb.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactInstantMessageFieldFactory<R, F: FnOnce(&IContactInstantMessageFieldFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactInstantMessageField, IContactInstantMessageFieldFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ContactInstantMessageField {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInstantMessageField;{cce33b37-0d85-41fa-b43d-da599c3eb009})");
}
impl ::core::clone::Clone for ContactInstantMessageField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactInstantMessageField {
    type Vtable = IContactInstantMessageField_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactInstantMessageField {
    const IID: ::windows_core::GUID = <IContactInstantMessageField as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactInstantMessageField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInstantMessageField";
}
::windows_core::imp::interface_hierarchy!(ContactInstantMessageField, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IContactField> for ContactInstantMessageField {}
unsafe impl ::core::marker::Send for ContactInstantMessageField {}
unsafe impl ::core::marker::Sync for ContactInstantMessageField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactJobInfo(::windows_core::IUnknown);
impl ContactJobInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactJobInfo, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CompanyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompanyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompanyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CompanyYomiName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompanyYomiName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompanyYomiName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyYomiName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Department(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Department)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDepartment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDepartment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Manager(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Manager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetManager(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetManager)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Office(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Office)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOffice(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffice)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CompanyAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompanyAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompanyAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactJobInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactJobInfo;{6d117b4c-ce50-4b43-9e69-b18258ea5315})");
}
impl ::core::clone::Clone for ContactJobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactJobInfo {
    type Vtable = IContactJobInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactJobInfo {
    const IID: ::windows_core::GUID = <IContactJobInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactJobInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactJobInfo";
}
::windows_core::imp::interface_hierarchy!(ContactJobInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactJobInfo {}
unsafe impl ::core::marker::Sync for ContactJobInfo {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
pub struct ContactLaunchActionVerbs;
impl ContactLaunchActionVerbs {
    pub fn Call() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Call)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Message() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Map() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Map)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Post() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Post)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoCall() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoCall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactLaunchActionVerbsStatics<R, F: FnOnce(&IContactLaunchActionVerbsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactLaunchActionVerbs, IContactLaunchActionVerbsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ContactLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLaunchActionVerbs";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactList(::windows_core::IUnknown);
impl ContactList {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHidden)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHidden)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<ContactListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: ContactListOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows_core::Result<ContactListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppWriteAccess)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: ContactListOtherAppWriteAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChangeTracker(&self) -> ::windows_core::Result<ContactChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows_core::Result<ContactListSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportsServerSearch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsServerSearch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContactChanged)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactFromRemoteIdAsync(&self, remoteid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactFromRemoteIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(remoteid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMeContactAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMeContactAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetContactReader(&self) -> ::windows_core::Result<ContactReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetContactReaderWithOptions<P0>(&self, options: P0) -> ::windows_core::Result<ContactReader>
    where
        P0: ::windows_core::IntoParam<ContactQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveContactAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactAsync(&self, contactid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsServerSearch(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportsServerSearch)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SyncConstraints(&self) -> ::windows_core::Result<ContactListSyncConstraints> {
        let this = &::windows_core::ComInterface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncConstraints)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LimitedWriteOperations(&self) -> ::windows_core::Result<ContactListLimitedWriteOperations> {
        let this = &::windows_core::ComInterface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LimitedWriteOperations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows_core::HSTRING) -> ::windows_core::Result<ContactChangeTracker> {
        let this = &::windows_core::ComInterface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeTracker)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactList;{16ddec75-392c-4845-9dfb-51a3e7ef3e42})");
}
impl ::core::clone::Clone for ContactList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactList {
    type Vtable = IContactList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactList {
    const IID: ::windows_core::GUID = <IContactList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactList";
}
::windows_core::imp::interface_hierarchy!(ContactList, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactList {}
unsafe impl ::core::marker::Sync for ContactList {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListLimitedWriteOperations(::windows_core::IUnknown);
impl ContactListLimitedWriteOperations {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateContactAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateOrUpdateContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteContactAsync(&self, contactid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteContactAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactid), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactListLimitedWriteOperations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations;{e19813da-4a0b-44b8-9a1f-a0f3d218175f})");
}
impl ::core::clone::Clone for ContactListLimitedWriteOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactListLimitedWriteOperations {
    const IID: ::windows_core::GUID = <IContactListLimitedWriteOperations as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations";
}
::windows_core::imp::interface_hierarchy!(ContactListLimitedWriteOperations, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactListLimitedWriteOperations {}
unsafe impl ::core::marker::Sync for ContactListLimitedWriteOperations {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListSyncConstraints(::windows_core::IUnknown);
impl ContactListSyncConstraints {
    pub fn CanSyncDescriptions(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSyncDescriptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanSyncDescriptions(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSyncDescriptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomePhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxHomePhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomePhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxHomePhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxMobilePhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxMobilePhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxMobilePhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxMobilePhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxWorkPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxWorkPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOtherPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxOtherPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPagerPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPagerPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPagerPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPagerPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBusinessFaxPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBusinessFaxPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxBusinessFaxPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxBusinessFaxPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomeFaxPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxHomeFaxPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomeFaxPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxHomeFaxPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxCompanyPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxCompanyPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxCompanyPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxCompanyPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAssistantPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAssistantPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAssistantPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxAssistantPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxRadioPhoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxRadioPhoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxRadioPhoneNumbers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxRadioPhoneNumbers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPersonalEmailAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPersonalEmailAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPersonalEmailAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPersonalEmailAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkEmailAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxWorkEmailAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkEmailAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxWorkEmailAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherEmailAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOtherEmailAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherEmailAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxOtherEmailAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxHomeAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxHomeAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxHomeAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxHomeAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWorkAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxWorkAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWorkAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxWorkAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherAddresses(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOtherAddresses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherAddresses<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxOtherAddresses)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBirthdayDates(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBirthdayDates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxBirthdayDates<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxBirthdayDates)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAnniversaryDates(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAnniversaryDates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAnniversaryDates<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxAnniversaryDates)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherDates(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOtherDates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherDates<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxOtherDates)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOtherRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOtherRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxOtherRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxOtherRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSpouseRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSpouseRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSpouseRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSpouseRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPartnerRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPartnerRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPartnerRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPartnerRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSiblingRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSiblingRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSiblingRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSiblingRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxParentRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxParentRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxParentRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxParentRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxChildRelationships(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxChildRelationships)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxChildRelationships<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxChildRelationships)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxJobInfo(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxJobInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxJobInfo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxJobInfo)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxWebsites(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxWebsites)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxWebsites<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxWebsites)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
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
impl ::windows_core::RuntimeType for ContactListSyncConstraints {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncConstraints;{b2b0bf01-3062-4e2e-969d-018d1987f314})");
}
impl ::core::clone::Clone for ContactListSyncConstraints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactListSyncConstraints {
    const IID: ::windows_core::GUID = <IContactListSyncConstraints as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactListSyncConstraints {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncConstraints";
}
::windows_core::imp::interface_hierarchy!(ContactListSyncConstraints, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncConstraints {}
unsafe impl ::core::marker::Sync for ContactListSyncConstraints {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListSyncManager(::windows_core::IUnknown);
impl ContactListSyncManager {
    pub fn Status(&self) -> ::windows_core::Result<ContactListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetStatus(&self, value: ContactListSyncStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for ContactListSyncManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncManager;{146e83be-7925-4acc-9de5-21ddd06f8674})");
}
impl ::core::clone::Clone for ContactListSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactListSyncManager {
    type Vtable = IContactListSyncManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactListSyncManager {
    const IID: ::windows_core::GUID = <IContactListSyncManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncManager";
}
::windows_core::imp::interface_hierarchy!(ContactListSyncManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactListSyncManager {}
unsafe impl ::core::marker::Sync for ContactListSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactLocationField(::windows_core::IUnknown);
impl ContactLocationField {
    pub fn Type(&self) -> ::windows_core::Result<ContactFieldType> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<ContactFieldCategory> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnstructuredAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnstructuredAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Street(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Street)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Region(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Region)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLocation_Default(unstructuredaddress: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Default)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateLocation_Category(unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_Category)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateLocation_All(unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory, street: &::windows_core::HSTRING, city: &::windows_core::HSTRING, region: &::windows_core::HSTRING, country: &::windows_core::HSTRING, postalcode: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocation_All)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(unstructuredaddress), category, ::core::mem::transmute_copy(street), ::core::mem::transmute_copy(city), ::core::mem::transmute_copy(region), ::core::mem::transmute_copy(country), ::core::mem::transmute_copy(postalcode), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactLocationFieldFactory<R, F: FnOnce(&IContactLocationFieldFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactLocationField, IContactLocationFieldFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ContactLocationField {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactLocationField;{9ec00f82-ab6e-4b36-89e3-b23bc0a1dacc})");
}
impl ::core::clone::Clone for ContactLocationField {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactLocationField {
    type Vtable = IContactLocationField_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactLocationField {
    const IID: ::windows_core::GUID = <IContactLocationField as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactLocationField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLocationField";
}
::windows_core::imp::interface_hierarchy!(ContactLocationField, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IContactField> for ContactLocationField {}
unsafe impl ::core::marker::Send for ContactLocationField {}
unsafe impl ::core::marker::Sync for ContactLocationField {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
pub struct ContactManager;
impl ContactManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowContactCard<P0>(contact: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        Self::IContactManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowContactCard)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), selection).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowContactCardWithPlacement<P0>(contact: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        Self::IContactManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowContactCardWithPlacement)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), selection, preferredplacement).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowDelayLoadedContactCard<P0>(contact: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<ContactCardDelayedDataLoader>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        Self::IContactManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowDelayLoadedContactCard)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), selection, preferredplacement, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsync<P0>(contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertContactToVCardAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsyncWithMaxBytes<P0>(contact: P0, maxbytes: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertContactToVCardAsyncWithMaxBytes)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), maxbytes, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertVCardToContactAsync<P0>(vcard: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertVCardToContactAsync)(::windows_core::Interface::as_raw(this), vcard.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsyncWithAccessType(accesstype: ContactStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsyncWithAccessType)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAnnotationStoreAsync(accesstype: ContactAnnotationStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAnnotationStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        })
    }
    pub fn IsShowContactCardSupported() -> ::windows_core::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShowContactCardSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowContactCardWithOptions<P0, P1>(contact: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<ContactCardOptions>,
    {
        Self::IContactManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).ShowContactCardWithOptions)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), selection, preferredplacement, contactcardoptions.into_param().abi()).ok() })
    }
    pub fn IsShowDelayLoadedContactCardSupported() -> ::windows_core::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShowDelayLoadedContactCardSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowDelayLoadedContactCardWithOptions<P0, P1>(contact: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: P1) -> ::windows_core::Result<ContactCardDelayedDataLoader>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<ContactCardOptions>,
    {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowDelayLoadedContactCardWithOptions)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), selection, preferredplacement, contactcardoptions.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowFullContactCard<P0, P1>(contact: P0, fullcontactcardoptions: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<FullContactCardOptions>,
    {
        Self::IContactManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).ShowFullContactCard)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), fullcontactcardoptions.into_param().abi()).ok() })
    }
    pub fn SystemDisplayNameOrder() -> ::windows_core::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemDisplayNameOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetSystemDisplayNameOrder(value: ContactNameOrder) -> ::windows_core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetSystemDisplayNameOrder)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn SystemSortOrder() -> ::windows_core::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSortOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetSystemSortOrder(value: ContactNameOrder) -> ::windows_core::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetSystemSortOrder)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<ContactManagerForUser>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IContactManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsShowFullContactCardSupportedAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShowFullContactCardSupportedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IncludeMiddleNameInSystemDisplayAndSort() -> ::windows_core::Result<bool> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IncludeMiddleNameInSystemDisplayAndSort)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetIncludeMiddleNameInSystemDisplayAndSort(value: bool) -> ::windows_core::Result<()> {
        Self::IContactManagerStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).SetIncludeMiddleNameInSystemDisplayAndSort)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics<R, F: FnOnce(&IContactManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactManager, IContactManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics2<R, F: FnOnce(&IContactManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactManager, IContactManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics3<R, F: FnOnce(&IContactManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactManager, IContactManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics4<R, F: FnOnce(&IContactManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactManager, IContactManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactManagerStatics5<R, F: FnOnce(&IContactManagerStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactManager, IContactManagerStatics5> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManager";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactManagerForUser(::windows_core::IUnknown);
impl ContactManagerForUser {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsync<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertContactToVCardAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertContactToVCardAsyncWithMaxBytes<P0>(&self, contact: P0, maxbytes: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertContactToVCardAsyncWithMaxBytes)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), maxbytes, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ConvertVCardToContactAsync<P0>(&self, vcard: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertVCardToContactAsync)(::windows_core::Interface::as_raw(this), vcard.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: ContactStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAnnotationStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        }
    }
    pub fn SystemDisplayNameOrder(&self) -> ::windows_core::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemDisplayNameOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemDisplayNameOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemSortOrder(&self) -> ::windows_core::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSortOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemSortOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShowFullContactCard<P0, P1>(&self, contact: P0, fullcontactcardoptions: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
        P1: ::windows_core::IntoParam<FullContactCardOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IContactManagerForUser2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ShowFullContactCard)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), fullcontactcardoptions.into_param().abi()).ok() }
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
impl ::windows_core::RuntimeType for ContactManagerForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactManagerForUser;{b74bba57-1076-4bef-aef3-54686d18387d})");
}
impl ::core::clone::Clone for ContactManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactManagerForUser {
    type Vtable = IContactManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactManagerForUser {
    const IID: ::windows_core::GUID = <IContactManagerForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManagerForUser";
}
::windows_core::imp::interface_hierarchy!(ContactManagerForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactManagerForUser {}
unsafe impl ::core::marker::Sync for ContactManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactMatchReason(::windows_core::IUnknown);
impl ContactMatchReason {
    pub fn Field(&self) -> ::windows_core::Result<ContactMatchReasonKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Field)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub fn Segments(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Segments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactMatchReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactMatchReason;{bc922504-e7d8-413e-95f4-b75c54c74077})");
}
impl ::core::clone::Clone for ContactMatchReason {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactMatchReason {
    type Vtable = IContactMatchReason_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactMatchReason {
    const IID: ::windows_core::GUID = <IContactMatchReason as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactMatchReason {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactMatchReason";
}
::windows_core::imp::interface_hierarchy!(ContactMatchReason, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactMatchReason {}
unsafe impl ::core::marker::Sync for ContactMatchReason {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanel(::windows_core::IUnknown);
impl ContactPanel {
    pub fn ClosePanel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClosePanel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn HeaderColor(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeaderColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn SetHeaderColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::UI::Color>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHeaderColor)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFullAppRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullAppRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLaunchFullAppRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLaunchFullAppRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closing<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosing(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosing)(::windows_core::Interface::as_raw(this), token).ok() }
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
impl ::windows_core::RuntimeType for ContactPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanel;{41bf1265-d2ee-4b97-a80a-7d8d64cca6f5})");
}
impl ::core::clone::Clone for ContactPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactPanel {
    type Vtable = IContactPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPanel {
    const IID: ::windows_core::GUID = <IContactPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPanel {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanel";
}
::windows_core::imp::interface_hierarchy!(ContactPanel, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanel {}
unsafe impl ::core::marker::Sync for ContactPanel {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanelClosingEventArgs(::windows_core::IUnknown);
impl ContactPanelClosingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactPanelClosingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs;{222174d3-cf4b-46d7-b739-6edc16110bfb})");
}
impl ::core::clone::Clone for ContactPanelClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPanelClosingEventArgs {
    const IID: ::windows_core::GUID = <IContactPanelClosingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPanelClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactPanelClosingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanelClosingEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelClosingEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(::windows_core::IUnknown);
impl ContactPanelLaunchFullAppRequestedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for ContactPanelLaunchFullAppRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs;{88d61c0e-23b4-4be8-8afc-072c25a4190d})");
}
impl ::core::clone::Clone for ContactPanelLaunchFullAppRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPanelLaunchFullAppRequestedEventArgs {
    const IID: ::windows_core::GUID = <IContactPanelLaunchFullAppRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPanelLaunchFullAppRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactPanelLaunchFullAppRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactPanelLaunchFullAppRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelLaunchFullAppRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPhone(::windows_core::IUnknown);
impl ContactPhone {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactPhone, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Number(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Number)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumber(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumber)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ContactPhoneKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ContactPhoneKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactPhone {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPhone;{467dab65-2712-4f52-b783-9ea8111c63cd})");
}
impl ::core::clone::Clone for ContactPhone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactPhone {
    type Vtable = IContactPhone_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPhone {
    const IID: ::windows_core::GUID = <IContactPhone as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPhone {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPhone";
}
::windows_core::imp::interface_hierarchy!(ContactPhone, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactPhone {}
unsafe impl ::core::marker::Sync for ContactPhone {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPicker(::windows_core::IUnknown);
impl ContactPicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactPicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CommitButtonText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitButtonText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommitButtonText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SelectionMode(&self) -> ::windows_core::Result<ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSelectionMode(&self, value: ContactSelectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFields(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFields)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleContactAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleContactAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickMultipleContactsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickMultipleContactsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ContactFieldType>> {
        let this = &::windows_core::ComInterface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickContactAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows_core::ComInterface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickContactAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickContactsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>> {
        let this = &::windows_core::ComInterface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickContactsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IContactPicker3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> ::windows_core::Result<ContactPicker>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactPickerStatics<R, F: FnOnce(&IContactPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactPicker, IContactPickerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ContactPicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPicker;{0e09fd91-42f8-4055-90a0-896f96738936})");
}
impl ::core::clone::Clone for ContactPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactPicker {
    type Vtable = IContactPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPicker {
    const IID: ::windows_core::GUID = <IContactPicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPicker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPicker";
}
::windows_core::imp::interface_hierarchy!(ContactPicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQueryOptions(::windows_core::IUnknown);
impl ContactQueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactQueryOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextSearch(&self) -> ::windows_core::Result<ContactQueryTextSearch> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextSearch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContactListIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactListIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IncludeContactsFromHiddenLists(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IncludeContactsFromHiddenLists)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeContactsFromHiddenLists(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeContactsFromHiddenLists)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredFields(&self) -> ::windows_core::Result<ContactQueryDesiredFields> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFields)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredFields(&self, value: ContactQueryDesiredFields) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredFields)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredOperations(&self) -> ::windows_core::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredOperations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredOperations(&self, value: ContactAnnotationOperations) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredOperations)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AnnotationListIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationListIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithText(text: &::windows_core::HSTRING) -> ::windows_core::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithTextAndFields(text: &::windows_core::HSTRING, fields: ContactQuerySearchFields) -> ::windows_core::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTextAndFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), fields, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactQueryOptionsFactory<R, F: FnOnce(&IContactQueryOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactQueryOptions, IContactQueryOptionsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ContactQueryOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryOptions;{4408cc9e-7d7c-42f0-8ac7-f50733ecdbc1})");
}
impl ::core::clone::Clone for ContactQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactQueryOptions {
    type Vtable = IContactQueryOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactQueryOptions {
    const IID: ::windows_core::GUID = <IContactQueryOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryOptions";
}
::windows_core::imp::interface_hierarchy!(ContactQueryOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryOptions {}
unsafe impl ::core::marker::Sync for ContactQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQueryTextSearch(::windows_core::IUnknown);
impl ContactQueryTextSearch {
    pub fn Fields(&self) -> ::windows_core::Result<ContactQuerySearchFields> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Fields)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFields(&self, value: ContactQuerySearchFields) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFields)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SearchScope(&self) -> ::windows_core::Result<ContactQuerySearchScope> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchScope)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSearchScope(&self, value: ContactQuerySearchScope) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchScope)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for ContactQueryTextSearch {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryTextSearch;{f7e3f9cb-a957-439b-a0b7-1c02a1963ff0})");
}
impl ::core::clone::Clone for ContactQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactQueryTextSearch {
    const IID: ::windows_core::GUID = <IContactQueryTextSearch as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryTextSearch";
}
::windows_core::imp::interface_hierarchy!(ContactQueryTextSearch, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryTextSearch {}
unsafe impl ::core::marker::Sync for ContactQueryTextSearch {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactReader(::windows_core::IUnknown);
impl ContactReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMatchingPropertiesWithMatchReason<P0>(&self, contact: P0) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ContactMatchReason>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMatchingPropertiesWithMatchReason)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactReader;{d397e42e-1488-42f2-bf64-253f4884bfed})");
}
impl ::core::clone::Clone for ContactReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactReader {
    type Vtable = IContactReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactReader {
    const IID: ::windows_core::GUID = <IContactReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactReader";
}
::windows_core::imp::interface_hierarchy!(ContactReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactReader {}
unsafe impl ::core::marker::Sync for ContactReader {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactSignificantOther(::windows_core::IUnknown);
impl ContactSignificantOther {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactSignificantOther, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Relationship(&self) -> ::windows_core::Result<ContactRelationship> {
        let this = &::windows_core::ComInterface::cast::<IContactSignificantOther2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Relationship)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRelationship(&self, value: ContactRelationship) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactSignificantOther2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelationship)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for ContactSignificantOther {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactSignificantOther;{8873b5ab-c5fb-46d8-93fe-da3ff1934054})");
}
impl ::core::clone::Clone for ContactSignificantOther {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactSignificantOther {
    type Vtable = IContactSignificantOther_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactSignificantOther {
    const IID: ::windows_core::GUID = <IContactSignificantOther as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactSignificantOther {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactSignificantOther";
}
::windows_core::imp::interface_hierarchy!(ContactSignificantOther, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactSignificantOther {}
unsafe impl ::core::marker::Sync for ContactSignificantOther {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactStore(::windows_core::IUnknown);
impl ContactStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindContactsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactsWithSearchTextAsync(&self, searchtext: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindContactsWithSearchTextAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchtext), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactAsync(&self, contactid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactid), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows_core::Result<ContactChangeTracker> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContactChanged)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AggregateContactManager(&self) -> ::windows_core::Result<AggregateContactManager> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AggregateContactManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindContactListsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindContactListsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactListAsync(&self, contactlistid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactlistid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateContactListAsync(&self, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateContactListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMeContactAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMeContactAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetContactReader(&self) -> ::windows_core::Result<ContactReader> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetContactReaderWithOptions<P0>(&self, options: P0) -> ::windows_core::Result<ContactReader>
    where
        P0: ::windows_core::IntoParam<ContactQueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContactReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateContactListInAccountAsync(&self, displayname: &::windows_core::HSTRING, userdataaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows_core::ComInterface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateContactListInAccountAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(userdataaccountid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows_core::HSTRING) -> ::windows_core::Result<ContactChangeTracker> {
        let this = &::windows_core::ComInterface::cast::<IContactStore3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeTracker)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ContactStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStore;{2c220b10-3a6c-4293-b9bc-fe987f6e0d52})");
}
impl ::core::clone::Clone for ContactStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactStore {
    type Vtable = IContactStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactStore {
    const IID: ::windows_core::GUID = <IContactStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStore";
}
::windows_core::imp::interface_hierarchy!(ContactStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactStore {}
unsafe impl ::core::marker::Sync for ContactStore {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactStoreNotificationTriggerDetails(::windows_core::IUnknown);
impl ContactStoreNotificationTriggerDetails {}
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
impl ::windows_core::RuntimeType for ContactStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails;{abb298d6-878a-4f8b-a9ce-46bb7d1c84ce})");
}
impl ::core::clone::Clone for ContactStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactStoreNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IContactStoreNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(ContactStoreNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for ContactStoreNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactWebsite(::windows_core::IUnknown);
impl ContactWebsite {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactWebsite, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RawValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactWebsite2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRawValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContactWebsite2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRawValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows_core::RuntimeType for ContactWebsite {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactWebsite;{9f130176-dc1b-4055-ad66-652f39d990e8})");
}
impl ::core::clone::Clone for ContactWebsite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContactWebsite {
    type Vtable = IContactWebsite_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactWebsite {
    const IID: ::windows_core::GUID = <IContactWebsite as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactWebsite {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactWebsite";
}
::windows_core::imp::interface_hierarchy!(ContactWebsite, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContactWebsite {}
unsafe impl ::core::marker::Sync for ContactWebsite {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct FullContactCardOptions(::windows_core::IUnknown);
impl FullContactCardOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FullContactCardOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for FullContactCardOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.FullContactCardOptions;{8744436c-5cf9-4683-bdca-a1fdebf8dbce})");
}
impl ::core::clone::Clone for FullContactCardOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FullContactCardOptions {
    type Vtable = IFullContactCardOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FullContactCardOptions {
    const IID: ::windows_core::GUID = <IFullContactCardOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FullContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.FullContactCardOptions";
}
::windows_core::imp::interface_hierarchy!(FullContactCardOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FullContactCardOptions {}
unsafe impl ::core::marker::Sync for FullContactCardOptions {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownContactField;
#[cfg(feature = "deprecated")]
impl KnownContactField {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Email() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Email)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PhoneNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Location() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn InstantMessage() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstantMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertNameToType(name: &::windows_core::HSTRING) -> ::windows_core::Result<ContactFieldType> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertNameToType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertTypeToName(r#type: ContactFieldType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertTypeToName)(::windows_core::Interface::as_raw(this), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownContactFieldStatics<R, F: FnOnce(&IKnownContactFieldStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownContactField, IKnownContactFieldStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for KnownContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.KnownContactField";
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct PinnedContactIdsQueryResult(::windows_core::IUnknown);
impl PinnedContactIdsQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContactIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PinnedContactIdsQueryResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult;{7d9b2552-1579-4ddc-871f-a30a3aea9ba1})");
}
impl ::core::clone::Clone for PinnedContactIdsQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PinnedContactIdsQueryResult {
    const IID: ::windows_core::GUID = <IPinnedContactIdsQueryResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PinnedContactIdsQueryResult {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult";
}
::windows_core::imp::interface_hierarchy!(PinnedContactIdsQueryResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PinnedContactIdsQueryResult {}
unsafe impl ::core::marker::Sync for PinnedContactIdsQueryResult {}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct PinnedContactManager(::windows_core::IUnknown);
impl PinnedContactManager {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPinSurfaceSupported(&self, surface: PinnedContactSurface) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPinSurfaceSupported)(::windows_core::Interface::as_raw(this), surface, &mut result__).from_abi(result__)
        }
    }
    pub fn IsContactPinned<P0>(&self, contact: P0, surface: PinnedContactSurface) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContactPinned)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), surface, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinContactAsync<P0>(&self, contact: P0, surface: PinnedContactSurface) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), surface, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestPinContactsAsync<P0>(&self, contacts: P0, surface: PinnedContactSurface) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<Contact>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinContactsAsync)(::windows_core::Interface::as_raw(this), contacts.try_into_param()?.abi(), surface, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnpinContactAsync<P0>(&self, contact: P0, surface: PinnedContactSurface) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnpinContactAsync)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), surface, &mut result__).from_abi(result__)
        }
    }
    pub fn SignalContactActivity<P0>(&self, contact: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Contact>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SignalContactActivity)(::windows_core::Interface::as_raw(this), contact.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPinnedContactIdsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPinnedContactIdsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<PinnedContactManager> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<PinnedContactManager>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPinnedContactManagerStatics<R, F: FnOnce(&IPinnedContactManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PinnedContactManager, IPinnedContactManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PinnedContactManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactManager;{fcbc740c-e1d6-45c3-b8b6-a35604e167a0})");
}
impl ::core::clone::Clone for PinnedContactManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PinnedContactManager {
    type Vtable = IPinnedContactManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PinnedContactManager {
    const IID: ::windows_core::GUID = <IPinnedContactManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PinnedContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactManager";
}
::windows_core::imp::interface_hierarchy!(PinnedContactManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for ContactAddressKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactAddressKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAddressKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactAddressKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAddressKind;i4)");
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
impl ::windows_core::TypeKind for ContactAnnotationOperations {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactAnnotationOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationOperations").field(&self.0).finish()
    }
}
impl ContactAnnotationOperations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for ContactAnnotationOperations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationOperations;u4)");
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
impl ::windows_core::TypeKind for ContactAnnotationStoreAccessType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactAnnotationStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactAnnotationStoreAccessType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactAnnotationStoreAccessType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationStoreAccessType;i4)");
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
impl ::windows_core::TypeKind for ContactBatchStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactBatchStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactBatchStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactBatchStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactBatchStatus;i4)");
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
impl ::windows_core::TypeKind for ContactCardHeaderKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactCardHeaderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardHeaderKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactCardHeaderKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardHeaderKind;i4)");
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
impl ::windows_core::TypeKind for ContactCardTabKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactCardTabKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCardTabKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactCardTabKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardTabKind;i4)");
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
impl ::windows_core::TypeKind for ContactChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactChangeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactChangeType;i4)");
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
impl ::windows_core::TypeKind for ContactDateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactDateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactDateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactDateKind;i4)");
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
impl ::windows_core::TypeKind for ContactEmailKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactEmailKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactEmailKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactEmailKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactEmailKind;i4)");
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
impl ::windows_core::TypeKind for ContactFieldCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactFieldCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactFieldCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactFieldCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldCategory;i4)");
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
impl ::windows_core::TypeKind for ContactFieldType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactFieldType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactFieldType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactFieldType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldType;i4)");
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
impl ::windows_core::TypeKind for ContactListOtherAppReadAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactListOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactListOtherAppReadAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppReadAccess;i4)");
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
impl ::windows_core::TypeKind for ContactListOtherAppWriteAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactListOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactListOtherAppWriteAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppWriteAccess;i4)");
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
impl ::windows_core::TypeKind for ContactListSyncStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactListSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactListSyncStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListSyncStatus;i4)");
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
impl ::windows_core::TypeKind for ContactMatchReasonKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactMatchReasonKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMatchReasonKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactMatchReasonKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactMatchReasonKind;i4)");
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
impl ::windows_core::TypeKind for ContactNameOrder {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactNameOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactNameOrder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactNameOrder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactNameOrder;i4)");
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
impl ::windows_core::TypeKind for ContactPhoneKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactPhoneKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPhoneKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactPhoneKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactPhoneKind;i4)");
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
impl ::windows_core::TypeKind for ContactQueryDesiredFields {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactQueryDesiredFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryDesiredFields").field(&self.0).finish()
    }
}
impl ContactQueryDesiredFields {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for ContactQueryDesiredFields {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQueryDesiredFields;u4)");
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
impl ::windows_core::TypeKind for ContactQuerySearchFields {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactQuerySearchFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQuerySearchFields").field(&self.0).finish()
    }
}
impl ContactQuerySearchFields {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for ContactQuerySearchFields {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchFields;u4)");
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
impl ::windows_core::TypeKind for ContactQuerySearchScope {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactQuerySearchScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQuerySearchScope").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactQuerySearchScope {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchScope;i4)");
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
impl ::windows_core::TypeKind for ContactRelationship {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactRelationship {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactRelationship").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactRelationship {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactRelationship;i4)");
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
impl ::windows_core::TypeKind for ContactSelectionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactSelectionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactSelectionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactSelectionMode;i4)");
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
impl ::windows_core::TypeKind for ContactStoreAccessType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContactStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreAccessType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContactStoreAccessType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactStoreAccessType;i4)");
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
impl ::windows_core::TypeKind for PinnedContactSurface {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PinnedContactSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinnedContactSurface").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PinnedContactSurface {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.PinnedContactSurface;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
