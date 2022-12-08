#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactAddress {
    type Vtable = IContactAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f24f927_94a9_44a2_a155_2d0b37d1dccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAddress_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactChangeRecord(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactChangeRecord {
    type Vtable = IContactChangeRecord_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactChangeRecord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9d3f78f_513b_4742_be00_cc5c5c236b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeRecord_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactChangeType) -> ::windows::core::HRESULT,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct IContactInformation(::windows::core::IUnknown);
impl IContactInformation {
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
    pub fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGivenName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificPrefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificPrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificPrefix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificSuffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificSuffix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDisplayPictureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetDisplayPictureAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardWithOptionsAsync)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContactInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactInformation {}
impl ::core::fmt::Debug for IContactInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e2b51ffc-e792-4ab7-b15b-f2e078664dea}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactInformation {
    type Vtable = IContactInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b51ffc_e792_4ab7_b15b_f2e078664dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHonorificPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHonorificSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDisplayPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDisplayPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetDisplayPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetDisplayPictureAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DisplayPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DisplayPicture: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ToVcardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ToVcardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ToVcardWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: VCardFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ToVcardWithOptionsAsync: usize,
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct IContactInformation2(::windows::core::IUnknown);
impl IContactInformation2 {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayPictureDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPictureDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDisplayPictureDate(&self, returnvalue: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayPictureDate)(::windows::core::Vtable::as_raw(this), returnvalue).ok() }
    }
}
::windows::core::interface_hierarchy!(IContactInformation2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContactInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactInformation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactInformation2 {}
impl ::core::fmt::Debug for IContactInformation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactInformation2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContactInformation2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3198b20c-621e-4668-ac38-d667b87d06d5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContactInformation2 {
    type Vtable = IContactInformation2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInformation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3198b20c_621e_4668_ac38_d667b87d06d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DisplayPictureDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayPictureDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayPictureDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayPictureDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactInformationStatics {
    type Vtable = IContactInformationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactInformationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f67bb29_03d0_4be6_b2a5_fb13859f1202);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ParseVcardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ParseVcardAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactQueryOptions {
    type Vtable = IContactQueryOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactQueryOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580cab76_3f31_46c1_9a50_424a53dacae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFields: usize,
    pub OrderBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ContactQueryResultOrdering) -> ::windows::core::HRESULT,
    pub SetOrderBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ContactQueryResultOrdering) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactQueryResult {
    type Vtable = IContactQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactQueryResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03db722_ecdb_4700_857e_3e786426b04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetContactCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContactsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContactsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContactsAsyncInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContactsAsyncInRange: usize,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStore {
    type Vtable = IContactStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2cd6fef_2bfd_4fad_8552_4e698097e8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FindContactByRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindContactByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FindContactByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindContactByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactAsync: usize,
    pub CreateContactQueryDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateContactQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetChangesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baserevisionnumber: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetChangesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadExtendedPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SaveExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SaveExtendedPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStore2 {
    type Vtable = IContactStore2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f1b64f_d653_43a7_b236_b30c0f4d7269);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateMeContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMeContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStoreStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactStoreStatics {
    type Vtable = IContactStoreStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactStoreStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa804fe22_4beb_44cc_a572_67a5b92e8567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateOrOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrOpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateOrOpenWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrOpenWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownContactPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownContactPropertiesStatics {
    type Vtable = IKnownContactPropertiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownContactPropertiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5812b01_2ced_4ee6_b1d6_094bf88ef0b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownContactPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HonorificSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdditionalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OtherAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WorkAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WorkTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Birthdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Anniversary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Telephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MobileTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WorkFax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SignificantOther: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompanyTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HomeFax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AlternateTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OfficeLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WorkEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiGivenName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub YomiCompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OtherEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AlternateMobileTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AlternateWorkTelephone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoredContact(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoredContact {
    type Vtable = IStoredContact_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoredContact {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb070b7b1_263d_4e71_abe7_591d2466570e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoredContact_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtendedPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReplaceExistingContactAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReplaceExistingContactAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoredContactFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoredContactFactory {
    type Vtable = IStoredContactFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoredContactFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ede921_c225_4fd9_89c5_cecc2c8a4b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoredContactFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateStoredContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStoredContactFromInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactAddress;{5f24f927-94a9-44a2-a155-2d0b37d1dccd})");
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
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactAddress";
}
::windows::core::interface_hierarchy!(ContactAddress, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactAddress {}
unsafe impl ::core::marker::Sync for ContactAddress {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactChangeRecord(::windows::core::IUnknown);
impl ContactChangeRecord {
    pub fn ChangeType(&self) -> ::windows::core::Result<ContactChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RevisionNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RevisionNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactChangeRecord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactChangeRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactChangeRecord {}
impl ::core::fmt::Debug for ContactChangeRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactChangeRecord").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactChangeRecord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactChangeRecord;{b9d3f78f-513b-4742-be00-cc5c5c236b04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactChangeRecord {
    type Vtable = IContactChangeRecord_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactChangeRecord {
    const IID: ::windows::core::GUID = <IContactChangeRecord as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactChangeRecord {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactChangeRecord";
}
::windows::core::interface_hierarchy!(ContactChangeRecord, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactChangeRecord {}
unsafe impl ::core::marker::Sync for ContactChangeRecord {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactInformation(::windows::core::IUnknown);
impl ContactInformation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactInformation, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGivenName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificPrefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificPrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificPrefix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificSuffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificSuffix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDisplayPictureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetDisplayPictureAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardWithOptionsAsync)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ParseVcardAsync<P0, E0>(vcard: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactInformation>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IContactInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParseVcardAsync)(::windows::core::Vtable::as_raw(this), vcard.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactInformationStatics<R, F: FnOnce(&IContactInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactInformation, IContactInformationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactInformation;{e2b51ffc-e792-4ab7-b15b-f2e078664dea})");
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
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactInformation";
}
::windows::core::interface_hierarchy!(ContactInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ContactInformation> for IContactInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactInformation> for IContactInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ContactInformation> for ::windows::core::InParam<IContactInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ContactInformation {}
unsafe impl ::core::marker::Sync for ContactInformation {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
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
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFields(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredFields)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrderBy(&self) -> ::windows::core::Result<ContactQueryResultOrdering> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrderBy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOrderBy(&self, value: ContactQueryResultOrdering) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOrderBy)(::windows::core::Vtable::as_raw(this), value).ok() }
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactQueryOptions;{580cab76-3f31-46c1-9a50-424a53dacae3})");
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
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactQueryOptions";
}
::windows::core::interface_hierarchy!(ContactQueryOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryOptions {}
unsafe impl ::core::marker::Sync for ContactQueryOptions {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactQueryResult(::windows::core::IUnknown);
impl ContactQueryResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContactCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactCountAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContactsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContactsAsyncInRange(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoredContact>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContactsAsyncInRange)(::windows::core::Vtable::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentQueryOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ContactQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactQueryResult {}
impl ::core::fmt::Debug for ContactQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactQueryResult;{c03db722-ecdb-4700-857e-3e786426b04b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ContactQueryResult {
    type Vtable = IContactQueryResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ContactQueryResult {
    const IID: ::windows::core::GUID = <IContactQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactQueryResult {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactQueryResult";
}
::windows::core::interface_hierarchy!(ContactQueryResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactQueryResult {}
unsafe impl ::core::marker::Sync for ContactQueryResult {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactStore(::windows::core::IUnknown);
impl ContactStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindContactByRemoteIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactByRemoteIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindContactByIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindContactByIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateContactQueryDefault(&self) -> ::windows::core::Result<ContactQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContactQueryDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateContactQueryWithOptions(&self, options: &ContactQueryOptions) -> ::windows::core::Result<ContactQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateContactQueryWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
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
    pub fn RevisionNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RevisionNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetChangesAsync(&self, baserevisionnumber: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChangeRecord>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChangesAsync)(::windows::core::Vtable::as_raw(this), baserevisionnumber, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadExtendedPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadExtendedPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SaveExtendedPropertiesAsync<P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveExtendedPropertiesAsync)(::windows::core::Vtable::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMeContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoredContact>> {
        let this = &::windows::core::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMeContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrOpenAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactStoreStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOrOpenAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrOpenWithOptionsAsync(access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactStoreStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOrOpenWithOptionsAsync)(::windows::core::Vtable::as_raw(this), access, sharing, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactStoreStatics<R, F: FnOnce(&IContactStoreStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactStore, IContactStoreStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.ContactStore;{b2cd6fef-2bfd-4fad-8552-4e698097e8eb})");
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
    const NAME: &'static str = "Windows.Phone.PersonalInformation.ContactStore";
}
::windows::core::interface_hierarchy!(ContactStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ContactStore {}
unsafe impl ::core::marker::Sync for ContactStore {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
pub struct KnownContactProperties;
impl KnownContactProperties {
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FamilyName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GivenName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HonorificPrefix() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificPrefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HonorificSuffix() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificSuffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AdditionalName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdditionalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Address() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Address)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OtherAddress() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OtherAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Email() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Email)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn WorkAddress() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn WorkTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JobTitle() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JobTitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Birthdate() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Birthdate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Anniversary() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Anniversary)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Telephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Telephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MobileTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MobileTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Url() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Url)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Notes() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn WorkFax() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkFax)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Children() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Children)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SignificantOther() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignificantOther)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompanyName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompanyTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HomeFax() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HomeFax)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AlternateTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Manager() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Manager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Nickname() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Nickname)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OfficeLocation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OfficeLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn WorkEmail() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WorkEmail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn YomiGivenName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiGivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn YomiFamilyName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn YomiCompanyName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiCompanyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OtherEmail() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OtherEmail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AlternateMobileTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateMobileTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AlternateWorkTelephone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownContactPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateWorkTelephone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownContactPropertiesStatics<R, F: FnOnce(&IKnownContactPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownContactProperties, IKnownContactPropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownContactProperties {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.KnownContactProperties";
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct StoredContact(::windows::core::IUnknown);
impl StoredContact {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GivenName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GivenName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGivenName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetGivenName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificPrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificPrefix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificPrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificPrefix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HonorificSuffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HonorificSuffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHonorificSuffix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetHonorificSuffix)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDisplayPictureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetDisplayPictureAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDisplayPictureAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DisplayPicture(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPicture)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IContactInformation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToVcardWithOptionsAsync)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayPictureDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IContactInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayPictureDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDisplayPictureDate(&self, returnvalue: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactInformation2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayPictureDate)(::windows::core::Vtable::as_raw(this), returnvalue).ok() }
    }
    pub fn Store(&self) -> ::windows::core::Result<ContactStore> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Store)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
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
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtendedPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetExtendedPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
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
    pub fn ReplaceExistingContactAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReplaceExistingContactAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateStoredContact(store: &ContactStore) -> ::windows::core::Result<StoredContact> {
        Self::IStoredContactFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStoredContact)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(store), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateStoredContactFromInformation(store: &ContactStore, contact: &ContactInformation) -> ::windows::core::Result<StoredContact> {
        Self::IStoredContactFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStoredContactFromInformation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(store), ::core::mem::transmute_copy(contact), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStoredContactFactory<R, F: FnOnce(&IStoredContactFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoredContact, IStoredContactFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StoredContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoredContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoredContact {}
impl ::core::fmt::Debug for StoredContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoredContact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoredContact {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.PersonalInformation.StoredContact;{b070b7b1-263d-4e71-abe7-591d2466570e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoredContact {
    type Vtable = IStoredContact_Vtbl;
}
unsafe impl ::windows::core::Interface for StoredContact {
    const IID: ::windows::core::GUID = <IStoredContact as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoredContact {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.StoredContact";
}
::windows::core::interface_hierarchy!(StoredContact, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<StoredContact> for IContactInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: StoredContact) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StoredContact> for IContactInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &StoredContact) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&StoredContact> for ::windows::core::InParam<IContactInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StoredContact) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<StoredContact> for IContactInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StoredContact) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StoredContact> for IContactInformation2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StoredContact) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&StoredContact> for ::windows::core::InParam<IContactInformation2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StoredContact) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for StoredContact {}
unsafe impl ::core::marker::Sync for StoredContact {}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.PersonalInformation.ContactChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactQueryResultOrdering(pub i32);
impl ContactQueryResultOrdering {
    pub const SystemDefault: Self = Self(0i32);
    pub const GivenNameFamilyName: Self = Self(1i32);
    pub const FamilyNameGivenName: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactQueryResultOrdering {}
impl ::core::clone::Clone for ContactQueryResultOrdering {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactQueryResultOrdering {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactQueryResultOrdering {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactQueryResultOrdering {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactQueryResultOrdering").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactQueryResultOrdering {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.PersonalInformation.ContactQueryResultOrdering;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactStoreApplicationAccessMode(pub i32);
impl ContactStoreApplicationAccessMode {
    pub const LimitedReadOnly: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreApplicationAccessMode {}
impl ::core::clone::Clone for ContactStoreApplicationAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactStoreApplicationAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactStoreApplicationAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactStoreApplicationAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreApplicationAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactStoreApplicationAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.PersonalInformation.ContactStoreApplicationAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContactStoreSystemAccessMode(pub i32);
impl ContactStoreSystemAccessMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreSystemAccessMode {}
impl ::core::clone::Clone for ContactStoreSystemAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContactStoreSystemAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContactStoreSystemAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContactStoreSystemAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreSystemAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactStoreSystemAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.PersonalInformation.ContactStoreSystemAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VCardFormat(pub i32);
impl VCardFormat {
    pub const Version2_1: Self = Self(0i32);
    pub const Version3: Self = Self(1i32);
}
impl ::core::marker::Copy for VCardFormat {}
impl ::core::clone::Clone for VCardFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VCardFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VCardFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for VCardFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VCardFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VCardFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.PersonalInformation.VCardFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
