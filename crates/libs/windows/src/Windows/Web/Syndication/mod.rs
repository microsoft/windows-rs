#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71e8f969_526e_4001_9a91_e84f83161ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttributeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationAttributeFactory {
    type Vtable = ISyndicationAttributeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationAttributeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624f1599_ed3e_420f_be86_640414886e4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttributeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: *mut ::core::ffi::c_void, attributenamespace: *mut ::core::ffi::c_void, attributevalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationCategory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8715626f_0cba_4a7f_89ff_ecb5281423b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Term: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategoryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationCategoryFactory {
    type Vtable = ISyndicationCategoryFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationCategoryFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab42802f_49e0_4525_8ab2_ab45c02528ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategoryFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSyndicationCategoryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, label: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationClient(::windows::core::IUnknown);
impl ISyndicationClient {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetServerCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProxyCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProxyCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRequestHeader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrieveFeedAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveFeedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISyndicationClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationClient {}
impl ::core::fmt::Debug for ISyndicationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISyndicationClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9e18a9b7-7249-4b45-b229-7df895a5a1f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e18a9b7_7249_4b45_b229_7df895a5a1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub MaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RetrieveFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrieveFeedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationClientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationClientFactory {
    type Vtable = ISyndicationClientFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationClientFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec4b32c_a79b_4114_b29a_05dffbafb9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClientFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateSyndicationClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateSyndicationClient: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4641fefe_0e55_40d0_b8d0_6a2ccba9fc7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationContentFactory {
    type Vtable = ISyndicationContentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d2fbb93_9520_4173_9388_7e2df324a8a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationContentWithSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationContentWithSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationErrorStatics {
    type Vtable = ISyndicationErrorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationErrorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fbb2361_45c7_4833_8aa0_be5f3b58a7f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationErrorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SyndicationErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeed(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationFeed {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ffe3cd2_5b66_4d62_8403_1bc10d910d6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeed_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Generator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IconUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetIconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIconUri: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetImageUri: usize,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FirstUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FirstUri: usize,
    #[cfg(feature = "Foundation")]
    pub LastUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUri: usize,
    #[cfg(feature = "Foundation")]
    pub NextUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextUri: usize,
    #[cfg(feature = "Foundation")]
    pub PreviousUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousUri: usize,
    pub SourceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SyndicationFormat) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feed: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feeddocument: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeedFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationFeedFactory {
    type Vtable = ISyndicationFeedFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationFeedFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23472232_8be9_48b7_8934_6205131d9357);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeedFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, subtitle: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationFeed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9768b379_fb2b_4f6d_b41c_088a5868825c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGenerator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationGeneratorFactory {
    type Vtable = ISyndicationGeneratorFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationGeneratorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa34083e3_1e26_4dbc_ba9d_1ab84beff97b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGeneratorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x548db883_c384_45c1_8ae8_a378c4ec486c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub PublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetPublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPublishedDate: usize,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Summary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSummary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetCommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditMediaUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditMediaUri: usize,
    pub ETag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemUri: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemdocument: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationItemFactory {
    type Vtable = ISyndicationItemFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationItemFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x251d434f_7db8_487a_85e4_10d191e66ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItemFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationItem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLink(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationLink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27553abd_a10e_41b5_86bd_9759086eb0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLinkFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationLinkFactory {
    type Vtable = ISyndicationLinkFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationLinkFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ed863d4_5535_48ac_98d4_c190995080b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLinkFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLinkEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, relationship: *mut ::core::ffi::c_void, title: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLinkEx: usize,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationNode(::windows::core::IUnknown);
impl ISyndicationNode {
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISyndicationNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ISyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationNode {}
impl ::core::fmt::Debug for ISyndicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISyndicationNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{753cef78-51f8-45c0-a9f5-f1719dec3fb2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x753cef78_51f8_45c0_a9f5_f1719dec3fb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ElementExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ElementExtensions: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetXmlDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SyndicationFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetXmlDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationNodeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationNodeFactory {
    type Vtable = ISyndicationNodeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationNodeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12902188_4acb_49a8_b777_a5eb92e18a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNodeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodename: *mut ::core::ffi::c_void, nodenamespace: *mut ::core::ffi::c_void, nodevalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPerson(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationPerson {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa1ee5da_a7c6_4517_a096_0143faf29327);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPerson_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPersonFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationPersonFactory {
    type Vtable = ISyndicationPersonFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationPersonFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcf4886d_229d_4b58_a49b_f3d2f0f5c99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPersonFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationPersonEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, email: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationPersonEx: usize,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationText(::windows::core::IUnknown);
impl ISyndicationText {
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
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Xml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml(&self, value: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ISyndicationText, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ISyndicationText> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: ISyndicationText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISyndicationText> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISyndicationText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ISyndicationText> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISyndicationText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ISyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationText {}
impl ::core::fmt::Debug for ISyndicationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISyndicationText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b9cc5e80-313a-4091-a2a6-243e0ee923f9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISyndicationText {
    type Vtable = ISyndicationText_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9cc5e80_313a_4091_a2a6_243e0ee923f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationText_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Xml: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub SetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    SetXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationTextFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISyndicationTextFactory {
    type Vtable = ISyndicationTextFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISyndicationTextFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee7342f7_11c6_4b25_ab62_e596bd162946);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationTextFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateSyndicationText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSyndicationTextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationAttribute(::windows::core::IUnknown);
impl SyndicationAttribute {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationAttribute, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
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
    pub fn Namespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Namespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationAttribute(attributename: &::windows::core::HSTRING, attributenamespace: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationAttribute> {
        Self::ISyndicationAttributeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationAttribute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributename), ::core::mem::transmute_copy(attributenamespace), ::core::mem::transmute_copy(attributevalue), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationAttributeFactory<R, F: FnOnce(&ISyndicationAttributeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationAttribute, ISyndicationAttributeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationAttribute {}
impl ::core::fmt::Debug for SyndicationAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationAttribute;{71e8f969-526e-4001-9a91-e84f83161ab1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationAttribute {
    const IID: ::windows::core::GUID = <ISyndicationAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationAttribute {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationAttribute";
}
::windows::core::interface_hierarchy!(SyndicationAttribute, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SyndicationAttribute {}
unsafe impl ::core::marker::Sync for SyndicationAttribute {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationCategory(::windows::core::IUnknown);
impl SyndicationCategory {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationCategory, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Label)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLabel)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scheme)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScheme(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScheme)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Term(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Term)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTerm(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTerm)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationCategory(term: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationCategory)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(term), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSyndicationCategoryEx(term: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING, label: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationCategoryEx)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(term), ::core::mem::transmute_copy(scheme), ::core::mem::transmute_copy(label), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationCategoryFactory<R, F: FnOnce(&ISyndicationCategoryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationCategory, ISyndicationCategoryFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationCategory {}
impl ::core::fmt::Debug for SyndicationCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationCategory;{8715626f-0cba-4a7f-89ff-ecb5281423b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationCategory {
    const IID: ::windows::core::GUID = <ISyndicationCategory as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationCategory {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationCategory";
}
::windows::core::interface_hierarchy!(SyndicationCategory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationCategory> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationCategory) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationCategory> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationCategory) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationCategory> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationCategory) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationCategory {}
unsafe impl ::core::marker::Sync for SyndicationCategory {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationClient(::windows::core::IUnknown);
impl SyndicationClient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationClient, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetServerCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProxyCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProxyCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRequestHeader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrieveFeedAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveFeedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateSyndicationClient(servercredential: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<SyndicationClient> {
        Self::ISyndicationClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationClient)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(servercredential), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationClientFactory<R, F: FnOnce(&ISyndicationClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationClient, ISyndicationClientFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationClient {}
impl ::core::fmt::Debug for SyndicationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationClient;{9e18a9b7-7249-4b45-b229-7df895a5a1f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationClient {
    const IID: ::windows::core::GUID = <ISyndicationClient as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationClient {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationClient";
}
::windows::core::interface_hierarchy!(SyndicationClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationClient> for ISyndicationClient {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationClient) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationClient> for ISyndicationClient {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationClient) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationClient> for ::windows::core::InParam<ISyndicationClient> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationClient) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationClient {}
unsafe impl ::core::marker::Sync for SyndicationClient {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationContent(::windows::core::IUnknown);
impl SyndicationContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationContent, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSourceUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSourceUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationContent(text: &::windows::core::HSTRING, r#type: SyndicationTextType) -> ::windows::core::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationContent)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationContentWithSourceUri(sourceuri: &super::super::Foundation::Uri) -> ::windows::core::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationContentWithSourceUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sourceuri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Xml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml(&self, value: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc(hidden)]
    pub fn ISyndicationContentFactory<R, F: FnOnce(&ISyndicationContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationContent, ISyndicationContentFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationContent {}
impl ::core::fmt::Debug for SyndicationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationContent;{4641fefe-0e55-40d0-b8d0-6a2ccba9fc7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationContent {
    const IID: ::windows::core::GUID = <ISyndicationContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationContent {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationContent";
}
::windows::core::interface_hierarchy!(SyndicationContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationContent> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SyndicationContent> for ISyndicationText {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ISyndicationText {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ::windows::core::InParam<ISyndicationText> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationContent {}
unsafe impl ::core::marker::Sync for SyndicationContent {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct SyndicationError;
impl SyndicationError {
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<SyndicationErrorStatus> {
        Self::ISyndicationErrorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatus)(::windows::core::Vtable::as_raw(this), hresult, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationErrorStatics<R, F: FnOnce(&ISyndicationErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationError, ISyndicationErrorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SyndicationError {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationError";
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationFeed(::windows::core::IUnknown);
impl SyndicationFeed {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationFeed, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Authors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Authors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Categories)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contributors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contributors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Generator(&self) -> ::windows::core::Result<SyndicationGenerator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Generator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGenerator(&self, value: &SyndicationGenerator) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGenerator)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IconUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIconUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIconUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Items)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastUpdatedTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastUpdatedTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastUpdatedTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Links(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Links)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImageUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetImageUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetImageUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Rights(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rights)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRights<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRights)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Subtitle(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subtitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSubtitle<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubtitle)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FirstUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NextUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreviousUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SourceFormat(&self) -> ::windows::core::Result<SyndicationFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Load(&self, feed: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Load)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feed)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn LoadFromXml(&self, feeddocument: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).LoadFromXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feeddocument)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationFeed(title: &::windows::core::HSTRING, subtitle: &::windows::core::HSTRING, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<SyndicationFeed> {
        Self::ISyndicationFeedFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationFeed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(subtitle), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationFeedFactory<R, F: FnOnce(&ISyndicationFeedFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationFeed, ISyndicationFeedFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationFeed {}
impl ::core::fmt::Debug for SyndicationFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationFeed").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationFeed {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationFeed;{7ffe3cd2-5b66-4d62-8403-1bc10d910d6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationFeed {
    const IID: ::windows::core::GUID = <ISyndicationFeed as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationFeed {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationFeed";
}
::windows::core::interface_hierarchy!(SyndicationFeed, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationFeed> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationFeed) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationFeed> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationFeed) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationFeed> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationFeed) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationFeed {}
unsafe impl ::core::marker::Sync for SyndicationFeed {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationGenerator(::windows::core::IUnknown);
impl SyndicationGenerator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationGenerator, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVersion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationGenerator(text: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationGenerator> {
        Self::ISyndicationGeneratorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationGenerator)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationGeneratorFactory<R, F: FnOnce(&ISyndicationGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationGenerator, ISyndicationGeneratorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationGenerator {}
impl ::core::fmt::Debug for SyndicationGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationGenerator;{9768b379-fb2b-4f6d-b41c-088a5868825c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationGenerator {
    const IID: ::windows::core::GUID = <ISyndicationGenerator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationGenerator {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationGenerator";
}
::windows::core::interface_hierarchy!(SyndicationGenerator, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationGenerator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationGenerator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationGenerator> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationGenerator) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationGenerator {}
unsafe impl ::core::marker::Sync for SyndicationGenerator {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationItem(::windows::core::IUnknown);
impl SyndicationItem {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationItem, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Authors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Authors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Categories)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contributors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contributors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<SyndicationContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContent(&self, value: &SyndicationContent) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContent)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastUpdatedTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastUpdatedTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastUpdatedTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Links(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Links)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishedDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishedDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPublishedDate(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPublishedDate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Rights(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rights)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRights<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRights)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<SyndicationFeed> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSource(&self, value: &SyndicationFeed) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Summary(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Summary)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSummary<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSummary)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ISyndicationText>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommentsUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommentsUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCommentsUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCommentsUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EditUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EditUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EditMediaUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EditMediaUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ETag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ETag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Load(&self, item: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Load)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn LoadFromXml(&self, itemdocument: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).LoadFromXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(itemdocument)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationItem(title: &::windows::core::HSTRING, content: &SyndicationContent, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<SyndicationItem> {
        Self::ISyndicationItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(content), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationItemFactory<R, F: FnOnce(&ISyndicationItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationItem, ISyndicationItemFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationItem {}
impl ::core::fmt::Debug for SyndicationItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationItem;{548db883-c384-45c1-8ae8-a378c4ec486c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationItem {
    const IID: ::windows::core::GUID = <ISyndicationItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationItem {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationItem";
}
::windows::core::interface_hierarchy!(SyndicationItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationItem> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationItem> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationItem> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationItem) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationItem {}
unsafe impl ::core::marker::Sync for SyndicationItem {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationLink(::windows::core::IUnknown);
impl SyndicationLink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationLink, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLength)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMediaType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Relationship(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Relationship)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRelationship(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRelationship)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    pub fn ResourceLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetResourceLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResourceLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationLink(uri: &super::super::Foundation::Uri) -> ::windows::core::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationLink)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationLinkEx(uri: &super::super::Foundation::Uri, relationship: &::windows::core::HSTRING, title: &::windows::core::HSTRING, mediatype: &::windows::core::HSTRING, length: u32) -> ::windows::core::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationLinkEx)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(relationship), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(mediatype), length, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationLinkFactory<R, F: FnOnce(&ISyndicationLinkFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationLink, ISyndicationLinkFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationLink {}
impl ::core::fmt::Debug for SyndicationLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationLink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationLink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationLink;{27553abd-a10e-41b5-86bd-9759086eb0c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationLink {
    const IID: ::windows::core::GUID = <ISyndicationLink as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationLink {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationLink";
}
::windows::core::interface_hierarchy!(SyndicationLink, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationLink> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationLink) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationLink> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationLink) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationLink> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationLink) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationLink {}
unsafe impl ::core::marker::Sync for SyndicationLink {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationNode(::windows::core::IUnknown);
impl SyndicationNode {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationNode, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateSyndicationNode(nodename: &::windows::core::HSTRING, nodenamespace: &::windows::core::HSTRING, nodevalue: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationNode> {
        Self::ISyndicationNodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(nodename), ::core::mem::transmute_copy(nodenamespace), ::core::mem::transmute_copy(nodevalue), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationNodeFactory<R, F: FnOnce(&ISyndicationNodeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationNode, ISyndicationNodeFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationNode {}
impl ::core::fmt::Debug for SyndicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationNode;{753cef78-51f8-45c0-a9f5-f1719dec3fb2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationNode {
    const IID: ::windows::core::GUID = <ISyndicationNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationNode {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationNode";
}
::windows::core::interface_hierarchy!(SyndicationNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationNode> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationNode> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationNode> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationNode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationNode {}
unsafe impl ::core::marker::Sync for SyndicationNode {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationPerson(::windows::core::IUnknown);
impl SyndicationPerson {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationPerson, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Email(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Email)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEmail(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEmail)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    pub fn CreateSyndicationPerson(name: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationPerson)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationPersonEx(name: &::windows::core::HSTRING, email: &::windows::core::HSTRING, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationPersonEx)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(email), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationPersonFactory<R, F: FnOnce(&ISyndicationPersonFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationPerson, ISyndicationPersonFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationPerson {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationPerson {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationPerson {}
impl ::core::fmt::Debug for SyndicationPerson {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationPerson").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationPerson {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationPerson;{fa1ee5da-a7c6-4517-a096-0143faf29327})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationPerson {
    const IID: ::windows::core::GUID = <ISyndicationPerson as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationPerson {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationPerson";
}
::windows::core::interface_hierarchy!(SyndicationPerson, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationPerson> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationPerson) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationPerson> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationPerson) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationPerson> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationPerson) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationPerson {}
unsafe impl ::core::marker::Sync for SyndicationPerson {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationText(::windows::core::IUnknown);
impl SyndicationText {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationText, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
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
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Xml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml(&self, value: &super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetXml)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationText(text: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSyndicationTextEx(text: &::windows::core::HSTRING, r#type: SyndicationTextType) -> ::windows::core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSyndicationTextEx)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationTextFactory<R, F: FnOnce(&ISyndicationTextFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SyndicationText, ISyndicationTextFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationText {}
impl ::core::fmt::Debug for SyndicationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationText;{b9cc5e80-313a-4091-a2a6-243e0ee923f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SyndicationText {
    type Vtable = ISyndicationText_Vtbl;
}
unsafe impl ::windows::core::Interface for SyndicationText {
    const IID: ::windows::core::GUID = <ISyndicationText as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SyndicationText {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationText";
}
::windows::core::interface_hierarchy!(SyndicationText, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SyndicationText> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ::windows::core::InParam<ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SyndicationText> for ISyndicationText {
    type Error = ::windows::core::Error;
    fn try_from(value: SyndicationText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ISyndicationText {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ::windows::core::InParam<ISyndicationText> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SyndicationText) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SyndicationText {}
unsafe impl ::core::marker::Sync for SyndicationText {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SyndicationErrorStatus(pub i32);
impl SyndicationErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const MissingRequiredElement: Self = Self(1i32);
    pub const MissingRequiredAttribute: Self = Self(2i32);
    pub const InvalidXml: Self = Self(3i32);
    pub const UnexpectedContent: Self = Self(4i32);
    pub const UnsupportedFormat: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationErrorStatus {}
impl ::core::clone::Clone for SyndicationErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SyndicationErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: Self = Self(0i32);
    pub const Rss20: Self = Self(1i32);
    pub const Rss10: Self = Self(2i32);
    pub const Rss092: Self = Self(3i32);
    pub const Rss091: Self = Self(4i32);
    pub const Atom03: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationFormat {}
impl ::core::clone::Clone for SyndicationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SyndicationFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
    pub const Xhtml: Self = Self(2i32);
}
impl ::core::marker::Copy for SyndicationTextType {}
impl ::core::clone::Clone for SyndicationTextType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationTextType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SyndicationTextType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationTextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationTextType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SyndicationTextType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationTextType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct RetrievalProgress {
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for RetrievalProgress {}
impl ::core::clone::Clone for RetrievalProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RetrievalProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RetrievalProgress").field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
unsafe impl ::windows::core::Abi for RetrievalProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RetrievalProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.RetrievalProgress;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RetrievalProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesRetrieved == other.BytesRetrieved && self.TotalBytesToRetrieve == other.TotalBytesToRetrieve
    }
}
impl ::core::cmp::Eq for RetrievalProgress {}
impl ::core::default::Default for RetrievalProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct TransferProgress {
    pub BytesSent: u32,
    pub TotalBytesToSend: u32,
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for TransferProgress {}
impl ::core::clone::Clone for TransferProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TransferProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TransferProgress").field("BytesSent", &self.BytesSent).field("TotalBytesToSend", &self.TotalBytesToSend).field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
unsafe impl ::windows::core::Abi for TransferProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TransferProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.TransferProgress;u4;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TransferProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesSent == other.BytesSent && self.TotalBytesToSend == other.TotalBytesToSend && self.BytesRetrieved == other.BytesRetrieved && self.TotalBytesToRetrieve == other.TotalBytesToRetrieve
    }
}
impl ::core::cmp::Eq for TransferProgress {}
impl ::core::default::Default for TransferProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
