#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterStatics {
    type Vtable = ICachedFileUpdaterStatics_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetUpdateInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, contentid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UIRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUIRequested: usize,
    pub UIStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UIStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI2 {
    type Vtable = ICachedFileUpdaterUI2_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterUI2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterUI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8856a21c_8699_4340_9f49_f7cad7fe8991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UpdateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
}
impl ::core::clone::Clone for IFileUpdateRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileUpdateRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40c82536_c1fe_4d93_a792_1e736bc70837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileUpdateStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FileUpdateStatus) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateLocalFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequest2 {
    type Vtable = IFileUpdateRequest2_Vtbl;
}
impl ::core::clone::Clone for IFileUpdateRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileUpdateRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82484648_bdbe_447b_a2ee_7afe6a032a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IFileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileUpdateRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileUpdateRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderFileTypeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1955b9c1_0184_5a88_87df_4544f464365d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfoFactory {
    type Vtable = IStorageProviderFileTypeInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderFileTypeInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderFileTypeInfoFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextension: ::std::mem::MaybeUninit<::windows::core::HSTRING>, iconresource: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderGetContentInfoForPathResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2564711d_aa89_4d12_82e3_f72a92e33966);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderGetPathForContentUriResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63711a9d_4118_45a6_acb6_22c49d019f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertiesStatics {
    type Vtable = IStorageProviderItemPropertiesStatics_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderItemPropertiesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderItemPropertiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, itemproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderItemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderItemProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476cb558_730b_4188_b7b5_63b716ed476d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertyDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderItemPropertyDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b383bb_ff1f_4298_831e_ff1c08089690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderItemPropertySource(::windows::core::IUnknown);
impl IStorageProviderItemPropertySource {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemProperties(&self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>();
            (::windows::core::Interface::vtable(this).GetItemProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(itempath), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderItemPropertySource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderItemPropertySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderItemPropertySource {}
impl ::core::fmt::Debug for IStorageProviderItemPropertySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderItemPropertySource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8f6f9c3e-f632-4a9b-8d99-d2d7a11df56a}");
}
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertySource {
    type Vtable = IStorageProviderItemPropertySource_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderItemPropertySource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderItemPropertySource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertySource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itempath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderMoreInfoUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderMoreInfoUI {
    type Vtable = IStorageProviderMoreInfoUI_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderMoreInfoUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderMoreInfoUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef38e591_a7cb_5e7d_9b5e_22749842697c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderMoreInfoUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderPropertyCapabilities(::windows::core::IUnknown);
impl IStorageProviderPropertyCapabilities {
    pub fn IsPropertySupported(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPropertySupported)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertycanonicalname), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderPropertyCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderPropertyCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderPropertyCapabilities {}
impl ::core::fmt::Debug for IStorageProviderPropertyCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderPropertyCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{658d2f0e-63b7-4567-acf9-51abe301dda5}");
}
unsafe impl ::windows::core::Interface for IStorageProviderPropertyCapabilities {
    type Vtable = IStorageProviderPropertyCapabilities_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderPropertyCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderPropertyCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658d2f0e_63b7_4567_acf9_51abe301dda5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderPropertyCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycanonicalname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderQuotaUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderQuotaUI {
    type Vtable = IStorageProviderQuotaUI_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderQuotaUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderQuotaUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba6295c3_312e_544f_9fd5_1f81b21f3649);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderQuotaUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub QuotaTotalInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetQuotaTotalInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub QuotaUsedInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetQuotaUsedInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub QuotaUsedLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetQuotaUsedLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub QuotaUsedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    QuotaUsedColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub SetQuotaUsedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    SetQuotaUsedColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderStatusUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderStatusUI {
    type Vtable = IStorageProviderStatusUI_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderStatusUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderStatusUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b6a758_198d_5b80_977f_5ff73da33118);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderState) -> ::windows::core::HRESULT,
    pub SetProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderState) -> ::windows::core::HRESULT,
    pub ProviderStateLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetProviderStateLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProviderStateIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProviderStateIcon: usize,
    #[cfg(feature = "Foundation")]
    pub SetProviderStateIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProviderStateIcon: usize,
    pub SyncStatusCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSyncStatusCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QuotaUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetQuotaUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoreInfoUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMoreInfoUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProviderPrimaryCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProviderPrimaryCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderSecondaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderSecondaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProviderSecondaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProviderSecondaryCommands: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderStatusUISource(::windows::core::IUnknown);
impl IStorageProviderStatusUISource {
    pub fn GetStatusUI(&self) -> ::windows::core::Result<StorageProviderStatusUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderStatusUI>();
            (::windows::core::Interface::vtable(this).GetStatusUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUIChanged(&self, handler: &super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StatusUIChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUIChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUIChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderStatusUISource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderStatusUISource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderStatusUISource {}
impl ::core::fmt::Debug for IStorageProviderStatusUISource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderStatusUISource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderStatusUISource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a306c249-3d66-5e70-9007-e43df96051ff}");
}
unsafe impl ::windows::core::Interface for IStorageProviderStatusUISource {
    type Vtable = IStorageProviderStatusUISource_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderStatusUISource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderStatusUISource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa306c249_3d66_5e70_9007_e43df96051ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUISource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatusUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUIChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUIChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUIChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUIChanged: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderStatusUISourceFactory(::windows::core::IUnknown);
impl IStorageProviderStatusUISourceFactory {
    pub fn GetStatusUISource(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusUISource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IStorageProviderStatusUISource>();
            (::windows::core::Interface::vtable(this).GetStatusUISource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootid), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderStatusUISourceFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderStatusUISourceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderStatusUISourceFactory {}
impl ::core::fmt::Debug for IStorageProviderStatusUISourceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderStatusUISourceFactory").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderStatusUISourceFactory {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{12e46b74-4e5a-58d1-a62f-0376e8ee7dd8}");
}
unsafe impl ::windows::core::Interface for IStorageProviderStatusUISourceFactory {
    type Vtable = IStorageProviderStatusUISourceFactory_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderStatusUISourceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderStatusUISourceFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12e46b74_4e5a_58d1_a62f_0376e8ee7dd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUISourceFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatusUISource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderSyncRootInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c1305c4_99f9_41ac_8904_ab055d654926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderItemPropertyDefinitions: usize,
    #[cfg(feature = "Foundation")]
    pub RecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecycleBinUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecycleBinUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo2 {
    type Vtable = IStorageProviderSyncRootInfo2_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderSyncRootInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf51b023_7cf1_5166_bdba_efd95f529e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo3 {
    type Vtable = IStorageProviderSyncRootInfo3_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderSyncRootInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderSyncRootInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507a6617_bef6_56fd_855e_75ace2e45cf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FallbackFileTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FallbackFileTypeInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics {
    type Vtable = IStorageProviderSyncRootManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderSyncRootManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderSyncRootManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootinformation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentSyncRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentSyncRoots: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics2 {
    type Vtable = IStorageProviderSyncRootManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderSyncRootManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderSyncRootManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderUICommand(::windows::core::IUnknown);
impl IStorageProviderUICommand {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Label)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).Icon)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<StorageProviderUICommandState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderUICommandState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderUICommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderUICommand {}
impl ::core::fmt::Debug for IStorageProviderUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderUICommand").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderUICommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0c3e0760-d846-568f-9484-105cc57b502b}");
}
unsafe impl ::windows::core::Interface for IStorageProviderUICommand {
    type Vtable = IStorageProviderUICommand_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderUICommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c3e0760_d846_568f_9484_105cc57b502b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUICommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Icon: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUICommandState) -> ::windows::core::HRESULT,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderUriSource(::windows::core::IUnknown);
impl IStorageProviderUriSource {
    pub fn GetPathForContentUri(&self, contenturi: &::windows::core::HSTRING, result: &StorageProviderGetPathForContentUriResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetPathForContentUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(contenturi), ::core::mem::transmute_copy(result)).ok() }
    }
    pub fn GetContentInfoForPath(&self, path: &::windows::core::HSTRING, result: &StorageProviderGetContentInfoForPathResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetContentInfoForPath)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(result)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IStorageProviderUriSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IStorageProviderUriSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderUriSource {}
impl ::core::fmt::Debug for IStorageProviderUriSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderUriSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b29806d1-8be0-4962-8bb6-0d4c2e14d47a}");
}
unsafe impl ::windows::core::Interface for IStorageProviderUriSource {
    type Vtable = IStorageProviderUriSource_Vtbl;
}
impl ::core::clone::Clone for IStorageProviderUriSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageProviderUriSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUriSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPathForContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturi: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct CachedFileUpdater;
impl CachedFileUpdater {
    pub fn SetUpdateInformation<P0>(file: P0, contentid: &::windows::core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::IStorageFile>,
    {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetUpdateInformation)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(contentid), readmode, writemode, options).ok() })
    }
    #[doc(hidden)]
    pub fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterUI(::windows::core::IUnknown);
impl CachedFileUpdaterUI {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdateTarget(&self) -> ::windows::core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CachedFileTarget>();
            (::windows::core::Interface::vtable(this).UpdateTarget)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileUpdateRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).FileUpdateRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileUpdateRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileUpdateRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UIRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).UIRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUIRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUIRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn UIStatus(&self) -> ::windows::core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UIStatus>();
            (::windows::core::Interface::vtable(this).UIStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateRequest(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = &::windows::core::ComInterface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FileUpdateRequest>();
            (::windows::core::Interface::vtable(this).UpdateRequest)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = &::windows::core::ComInterface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FileUpdateRequestDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterUI {}
impl ::core::fmt::Debug for CachedFileUpdaterUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterUI").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.CachedFileUpdaterUI;{9e6f41e6-baf2-4a97-b600-9333f5df80fd})");
}
impl ::core::clone::Clone for CachedFileUpdaterUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CachedFileUpdaterUI {
    const IID: ::windows::core::GUID = <ICachedFileUpdaterUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
::windows::imp::interface_hierarchy!(CachedFileUpdaterUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequest(::windows::core::IUnknown);
impl FileUpdateRequest {
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::StorageFile>();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FileUpdateStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatus(&self, value: FileUpdateStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FileUpdateRequestDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateLocalFile<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::IStorageFile>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateLocalFile)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn UserInputNeededMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserInputNeededMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserInputNeededMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUserInputNeededMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequest {}
impl ::core::fmt::Debug for FileUpdateRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequest;{40c82536-c1fe-4d93-a792-1e736bc70837})");
}
impl ::core::clone::Clone for FileUpdateRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileUpdateRequest {
    const IID: ::windows::core::GUID = <IFileUpdateRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
::windows::imp::interface_hierarchy!(FileUpdateRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(::windows::core::IUnknown);
impl FileUpdateRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestDeferral {}
impl ::core::fmt::Debug for FileUpdateRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestDeferral;{ffcedb2b-8ade-44a5-bb00-164c4e72f13a})");
}
impl ::core::clone::Clone for FileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileUpdateRequestDeferral {
    const IID: ::windows::core::GUID = <IFileUpdateRequestDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
::windows::imp::interface_hierarchy!(FileUpdateRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(::windows::core::IUnknown);
impl FileUpdateRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FileUpdateRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for FileUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestedEventArgs;{7b0a9342-3905-438d-aaef-78ae265f8dd2})");
}
impl ::core::clone::Clone for FileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileUpdateRequestedEventArgs {
    const IID: ::windows::core::GUID = <IFileUpdateRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(FileUpdateRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderFileTypeInfo(::windows::core::IUnknown);
impl StorageProviderFileTypeInfo {
    pub fn FileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FileExtension)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).IconResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(fileextension: &::windows::core::HSTRING, iconresource: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderFileTypeInfo>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(fileextension), ::core::mem::transmute_copy(iconresource), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for StorageProviderFileTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderFileTypeInfo {}
impl ::core::fmt::Debug for StorageProviderFileTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderFileTypeInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderFileTypeInfo;{1955b9c1-0184-5a88-87df-4544f464365d})");
}
impl ::core::clone::Clone for StorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderFileTypeInfo {
    const IID: ::windows::core::GUID = <IStorageProviderFileTypeInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
::windows::imp::interface_hierarchy!(StorageProviderFileTypeInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderFileTypeInfo {}
unsafe impl ::core::marker::Sync for StorageProviderFileTypeInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderGetContentInfoForPathResult, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderUriSourceStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentUri(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetContentInfoForPathResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetContentInfoForPathResult {}
impl ::core::fmt::Debug for StorageProviderGetContentInfoForPathResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetContentInfoForPathResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult;{2564711d-aa89-4d12-82e3-f72a92e33966})");
}
impl ::core::clone::Clone for StorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderGetContentInfoForPathResult {
    const IID: ::windows::core::GUID = <IStorageProviderGetContentInfoForPathResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
::windows::imp::interface_hierarchy!(StorageProviderGetContentInfoForPathResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetContentInfoForPathResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderGetPathForContentUriResult, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderUriSourceStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetPathForContentUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetPathForContentUriResult {}
impl ::core::fmt::Debug for StorageProviderGetPathForContentUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetPathForContentUriResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetPathForContentUriResult;{63711a9d-4118-45a6-acb6-22c49d019f40})");
}
impl ::core::clone::Clone for StorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderGetPathForContentUriResult {
    const IID: ::windows::core::GUID = <IStorageProviderGetPathForContentUriResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
::windows::imp::interface_hierarchy!(StorageProviderGetPathForContentUriResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderGetPathForContentUriResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetPathForContentUriResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderItemProperties;
impl StorageProviderItemProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAsync<P0, P1>(item: P0, itemproperties: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows::core::TryIntoParam<super::IStorageItem>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>,
    {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SetAsync)(::windows::core::Interface::as_raw(this), item.try_into_param()?.abi(), itemproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemProperty(::windows::core::IUnknown);
impl StorageProviderItemProperty {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderItemProperty, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).IconResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemProperty {}
impl ::core::fmt::Debug for StorageProviderItemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemProperty").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemProperty;{476cb558-730b-4188-b7b5-63b716ed476d})");
}
impl ::core::clone::Clone for StorageProviderItemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderItemProperty {
    const IID: ::windows::core::GUID = <IStorageProviderItemProperty as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
::windows::imp::interface_hierarchy!(StorageProviderItemProperty, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderItemProperty {}
unsafe impl ::core::marker::Sync for StorageProviderItemProperty {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(::windows::core::IUnknown);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderItemPropertyDefinition, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayNameResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayNameResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemPropertyDefinition {}
impl ::core::fmt::Debug for StorageProviderItemPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemPropertyDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemPropertyDefinition;{c5b383bb-ff1f-4298-831e-ff1c08089690})");
}
impl ::core::clone::Clone for StorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderItemPropertyDefinition {
    const IID: ::windows::core::GUID = <IStorageProviderItemPropertyDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
::windows::imp::interface_hierarchy!(StorageProviderItemPropertyDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderItemPropertyDefinition {}
unsafe impl ::core::marker::Sync for StorageProviderItemPropertyDefinition {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderMoreInfoUI(::windows::core::IUnknown);
impl StorageProviderMoreInfoUI {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderMoreInfoUI, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Command(&self) -> ::windows::core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IStorageProviderUICommand>();
            (::windows::core::Interface::vtable(this).Command)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommand)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderMoreInfoUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderMoreInfoUI {}
impl ::core::fmt::Debug for StorageProviderMoreInfoUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderMoreInfoUI").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderMoreInfoUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderMoreInfoUI;{ef38e591-a7cb-5e7d-9b5e-22749842697c})");
}
impl ::core::clone::Clone for StorageProviderMoreInfoUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderMoreInfoUI {
    type Vtable = IStorageProviderMoreInfoUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderMoreInfoUI {
    const IID: ::windows::core::GUID = <IStorageProviderMoreInfoUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderMoreInfoUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderMoreInfoUI";
}
::windows::imp::interface_hierarchy!(StorageProviderMoreInfoUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderMoreInfoUI {}
unsafe impl ::core::marker::Sync for StorageProviderMoreInfoUI {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderQuotaUI(::windows::core::IUnknown);
impl StorageProviderQuotaUI {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderQuotaUI, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn QuotaTotalInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).QuotaTotalInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQuotaTotalInBytes(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuotaTotalInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).QuotaUsedInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQuotaUsedInBytes(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuotaUsedInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).QuotaUsedLabel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQuotaUsedLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuotaUsedLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn QuotaUsedColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::UI::Color>>();
            (::windows::core::Interface::vtable(this).QuotaUsedColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn SetQuotaUsedColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::UI::Color>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuotaUsedColor)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderQuotaUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderQuotaUI {}
impl ::core::fmt::Debug for StorageProviderQuotaUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderQuotaUI").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderQuotaUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderQuotaUI;{ba6295c3-312e-544f-9fd5-1f81b21f3649})");
}
impl ::core::clone::Clone for StorageProviderQuotaUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderQuotaUI {
    type Vtable = IStorageProviderQuotaUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderQuotaUI {
    const IID: ::windows::core::GUID = <IStorageProviderQuotaUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderQuotaUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderQuotaUI";
}
::windows::imp::interface_hierarchy!(StorageProviderQuotaUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderQuotaUI {}
unsafe impl ::core::marker::Sync for StorageProviderQuotaUI {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderStatusUI(::windows::core::IUnknown);
impl StorageProviderStatusUI {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderStatusUI, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderState(&self) -> ::windows::core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderState>();
            (::windows::core::Interface::vtable(this).ProviderState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProviderState(&self, value: StorageProviderState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProviderStateLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ProviderStateLabel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProviderStateLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderStateLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProviderStateIcon(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).ProviderStateIcon)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProviderStateIcon(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderStateIcon)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SyncStatusCommand(&self) -> ::windows::core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IStorageProviderUICommand>();
            (::windows::core::Interface::vtable(this).SyncStatusCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSyncStatusCommand<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSyncStatusCommand)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn QuotaUI(&self) -> ::windows::core::Result<StorageProviderQuotaUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderQuotaUI>();
            (::windows::core::Interface::vtable(this).QuotaUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQuotaUI(&self, value: &StorageProviderQuotaUI) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuotaUI)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MoreInfoUI(&self) -> ::windows::core::Result<StorageProviderMoreInfoUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderMoreInfoUI>();
            (::windows::core::Interface::vtable(this).MoreInfoUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMoreInfoUI(&self, value: &StorageProviderMoreInfoUI) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMoreInfoUI)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderPrimaryCommand(&self) -> ::windows::core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IStorageProviderUICommand>();
            (::windows::core::Interface::vtable(this).ProviderPrimaryCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProviderPrimaryCommand<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderPrimaryCommand)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderSecondaryCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IStorageProviderUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<IStorageProviderUICommand>>();
            (::windows::core::Interface::vtable(this).ProviderSecondaryCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProviderSecondaryCommands<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IVector<IStorageProviderUICommand>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderSecondaryCommands)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageProviderStatusUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderStatusUI {}
impl ::core::fmt::Debug for StorageProviderStatusUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderStatusUI").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderStatusUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderStatusUI;{d6b6a758-198d-5b80-977f-5ff73da33118})");
}
impl ::core::clone::Clone for StorageProviderStatusUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderStatusUI {
    type Vtable = IStorageProviderStatusUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderStatusUI {
    const IID: ::windows::core::GUID = <IStorageProviderStatusUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderStatusUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatusUI";
}
::windows::imp::interface_hierarchy!(StorageProviderStatusUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderStatusUI {}
unsafe impl ::core::marker::Sync for StorageProviderStatusUI {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(::windows::core::IUnknown);
impl StorageProviderSyncRootInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderSyncRootInfo, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Context(&self) -> ::windows::core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).Context)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContext<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContext)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Path(&self) -> ::windows::core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IStorageFolder>();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPath<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::IStorageFolder>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayNameResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayNameResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).IconResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HydrationPolicy(&self) -> ::windows::core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderHydrationPolicy>();
            (::windows::core::Interface::vtable(this).HydrationPolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHydrationPolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HydrationPolicyModifier(&self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderHydrationPolicyModifier>();
            (::windows::core::Interface::vtable(this).HydrationPolicyModifier)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHydrationPolicyModifier)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PopulationPolicy(&self) -> ::windows::core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderPopulationPolicy>();
            (::windows::core::Interface::vtable(this).PopulationPolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPopulationPolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InSyncPolicy(&self) -> ::windows::core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderInSyncPolicy>();
            (::windows::core::Interface::vtable(this).InSyncPolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInSyncPolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardlinkPolicy(&self) -> ::windows::core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderHardlinkPolicy>();
            (::windows::core::Interface::vtable(this).HardlinkPolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHardlinkPolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowSiblingsAsGroup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ShowSiblingsAsGroup)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowSiblingsAsGroup)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVersion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProtectionMode(&self) -> ::windows::core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderProtectionMode>();
            (::windows::core::Interface::vtable(this).ProtectionMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectionMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowPinning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowPinning)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowPinning(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowPinning)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderItemPropertyDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>();
            (::windows::core::Interface::vtable(this).StorageProviderItemPropertyDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecycleBinUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).RecycleBinUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRecycleBinUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecycleBinUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ProviderId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProviderId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FallbackFileTypeInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &::windows::core::ComInterface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>();
            (::windows::core::Interface::vtable(this).FallbackFileTypeInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StorageProviderSyncRootInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderSyncRootInfo {}
impl ::core::fmt::Debug for StorageProviderSyncRootInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderSyncRootInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderSyncRootInfo;{7c1305c4-99f9-41ac-8904-ab055d654926})");
}
impl ::core::clone::Clone for StorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageProviderSyncRootInfo {
    const IID: ::windows::core::GUID = <IStorageProviderSyncRootInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
::windows::imp::interface_hierarchy!(StorageProviderSyncRootInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderSyncRootInfo {}
unsafe impl ::core::marker::Sync for StorageProviderSyncRootInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderSyncRootManager;
impl StorageProviderSyncRootManager {
    pub fn Register(syncrootinformation: &StorageProviderSyncRootInfo) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootinformation)).ok() })
    }
    pub fn Unregister(id: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id)).ok() })
    }
    pub fn GetSyncRootInformationForFolder<P0>(folder: P0) -> ::windows::core::Result<StorageProviderSyncRootInfo>
    where
        P0: ::windows::core::TryIntoParam<super::IStorageFolder>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderSyncRootInfo>();
            (::windows::core::Interface::vtable(this).GetSyncRootInformationForFolder)(::windows::core::Interface::as_raw(this), folder.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetSyncRootInformationForId(id: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageProviderSyncRootInfo>();
            (::windows::core::Interface::vtable(this).GetSyncRootInformationForId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentSyncRoots() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>();
            (::windows::core::Interface::vtable(this).GetCurrentSyncRoots)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl ::core::marker::Copy for CachedFileOptions {}
impl ::core::clone::Clone for CachedFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CachedFileOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CachedFileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileOptions").field(&self.0).finish()
    }
}
impl CachedFileOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for CachedFileOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl ::core::marker::Copy for CachedFileTarget {}
impl ::core::clone::Clone for CachedFileTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileTarget {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CachedFileTarget {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CachedFileTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileTarget").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CachedFileTarget {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl ::core::marker::Copy for FileUpdateStatus {}
impl ::core::clone::Clone for FileUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FileUpdateStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FileUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl ::core::marker::Copy for ReadActivationMode {}
impl ::core::clone::Clone for ReadActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReadActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ReadActivationMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ReadActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReadActivationMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ReadActivationMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl ::core::marker::Copy for StorageProviderHardlinkPolicy {}
impl ::core::clone::Clone for StorageProviderHardlinkPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHardlinkPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderHardlinkPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderHardlinkPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHardlinkPolicy").field(&self.0).finish()
    }
}
impl StorageProviderHardlinkPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicy {}
impl ::core::clone::Clone for StorageProviderHydrationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderHydrationPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicy").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicyModifier {}
impl ::core::clone::Clone for StorageProviderHydrationPolicyModifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicyModifier {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderHydrationPolicyModifier {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicyModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicyModifier").field(&self.0).finish()
    }
}
impl StorageProviderHydrationPolicyModifier {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for StorageProviderInSyncPolicy {}
impl ::core::clone::Clone for StorageProviderInSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderInSyncPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderInSyncPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderInSyncPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderInSyncPolicy").field(&self.0).finish()
    }
}
impl StorageProviderInSyncPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderPopulationPolicy {}
impl ::core::clone::Clone for StorageProviderPopulationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderPopulationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderPopulationPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderPopulationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderPopulationPolicy").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageProviderProtectionMode {}
impl ::core::clone::Clone for StorageProviderProtectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderProtectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderProtectionMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderProtectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderProtectionMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl ::core::marker::Copy for StorageProviderState {}
impl ::core::clone::Clone for StorageProviderState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderUICommandState(pub i32);
impl StorageProviderUICommandState {
    pub const Enabled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUICommandState {}
impl ::core::clone::Clone for StorageProviderUICommandState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderUICommandState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderUICommandState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderUICommandState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderUICommandState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderUICommandState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUICommandState;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUriSourceStatus {}
impl ::core::clone::Clone for StorageProviderUriSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderUriSourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StorageProviderUriSourceStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StorageProviderUriSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderUriSourceStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl ::core::marker::Copy for UIStatus {}
impl ::core::clone::Clone for UIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UIStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UIStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for WriteActivationMode {}
impl ::core::clone::Clone for WriteActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WriteActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WriteActivationMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WriteActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WriteActivationMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WriteActivationMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
