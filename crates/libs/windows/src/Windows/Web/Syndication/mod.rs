#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttribute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
}
impl ::core::clone::Clone for ISyndicationAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationAttribute {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71e8f969_526e_4001_9a91_e84f83161ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttribute_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttributeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationAttributeFactory {
    type Vtable = ISyndicationAttributeFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationAttributeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationAttributeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x624f1599_ed3e_420f_be86_640414886e4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttributeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, attributenamespace: ::std::mem::MaybeUninit<::windows_core::HSTRING>, attributevalue: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationCategory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8715626f_0cba_4a7f_89ff_ecb5281423b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Term: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategoryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationCategoryFactory {
    type Vtable = ISyndicationCategoryFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationCategoryFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationCategoryFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab42802f_49e0_4525_8ab2_ab45c02528ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategoryFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSyndicationCategoryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: ::std::mem::MaybeUninit<::windows_core::HSTRING>, scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>, label: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationClient(::windows_core::IUnknown);
impl ISyndicationClient {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrieveFeedAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISyndicationClient, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for ISyndicationClient {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9e18a9b7-7249-4b45-b229-7df895a5a1f5}");
}
unsafe impl ::windows_core::Interface for ISyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
}
impl ::core::clone::Clone for ISyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e18a9b7_7249_4b45_b229_7df895a5a1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClient_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub MaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RetrieveFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrieveFeedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationClientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationClientFactory {
    type Vtable = ISyndicationClientFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationClientFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationClientFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ec4b32c_a79b_4114_b29a_05dffbafb9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateSyndicationClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateSyndicationClient: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
}
impl ::core::clone::Clone for ISyndicationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4641fefe_0e55_40d0_b8d0_6a2ccba9fc7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationContentFactory {
    type Vtable = ISyndicationContentFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationContentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d2fbb93_9520_4173_9388_7e2df324a8a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationContentWithSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationContentWithSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationErrorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationErrorStatics {
    type Vtable = ISyndicationErrorStatics_Vtbl;
}
impl ::core::clone::Clone for ISyndicationErrorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationErrorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fbb2361_45c7_4833_8aa0_be5f3b58a7f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationErrorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SyndicationErrorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeed(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
}
impl ::core::clone::Clone for ISyndicationFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationFeed {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ffe3cd2_5b66_4d62_8403_1bc10d910d6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeed_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Generator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IconUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetIconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIconUri: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetImageUri: usize,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FirstUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FirstUri: usize,
    #[cfg(feature = "Foundation")]
    pub LastUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUri: usize,
    #[cfg(feature = "Foundation")]
    pub NextUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextUri: usize,
    #[cfg(feature = "Foundation")]
    pub PreviousUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousUri: usize,
    pub SourceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SyndicationFormat) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feed: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feeddocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeedFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationFeedFactory {
    type Vtable = ISyndicationFeedFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationFeedFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationFeedFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23472232_8be9_48b7_8934_6205131d9357);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeedFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, subtitle: ::std::mem::MaybeUninit<::windows_core::HSTRING>, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationFeed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
}
impl ::core::clone::Clone for ISyndicationGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationGenerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9768b379_fb2b_4f6d_b41c_088a5868825c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGenerator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGeneratorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationGeneratorFactory {
    type Vtable = ISyndicationGeneratorFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationGeneratorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationGeneratorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa34083e3_1e26_4dbc_ba9d_1ab84beff97b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGeneratorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
}
impl ::core::clone::Clone for ISyndicationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x548db883_c384_45c1_8ae8_a378c4ec486c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub PublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetPublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPublishedDate: usize,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Summary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSummary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetCommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditMediaUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditMediaUri: usize,
    pub ETag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemUri: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemdocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationItemFactory {
    type Vtable = ISyndicationItemFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationItemFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationItemFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x251d434f_7db8_487a_85e4_10d191e66ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, content: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationItem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLink(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
}
impl ::core::clone::Clone for ISyndicationLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationLink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27553abd_a10e_41b5_86bd_9759086eb0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLinkFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationLinkFactory {
    type Vtable = ISyndicationLinkFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationLinkFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationLinkFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ed863d4_5535_48ac_98d4_c190995080b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLinkFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLinkEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, relationship: ::std::mem::MaybeUninit<::windows_core::HSTRING>, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mediatype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLinkEx: usize,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationNode(::windows_core::IUnknown);
impl ISyndicationNode {
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISyndicationNode, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for ISyndicationNode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{753cef78-51f8-45c0-a9f5-f1719dec3fb2}");
}
unsafe impl ::windows_core::Interface for ISyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
}
impl ::core::clone::Clone for ISyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationNode {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x753cef78_51f8_45c0_a9f5_f1719dec3fb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ElementExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ElementExtensions: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetXmlDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SyndicationFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetXmlDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationNodeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationNodeFactory {
    type Vtable = ISyndicationNodeFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationNodeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationNodeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12902188_4acb_49a8_b777_a5eb92e18a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNodeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, nodenamespace: ::std::mem::MaybeUninit<::windows_core::HSTRING>, nodevalue: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPerson(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
}
impl ::core::clone::Clone for ISyndicationPerson {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationPerson {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa1ee5da_a7c6_4517_a096_0143faf29327);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPerson_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPersonFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationPersonFactory {
    type Vtable = ISyndicationPersonFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationPersonFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationPersonFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcf4886d_229d_4b58_a49b_f3d2f0f5c99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPersonFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationPersonEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, email: ::std::mem::MaybeUninit<::windows_core::HSTRING>, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationPersonEx: usize,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct ISyndicationText(::windows_core::IUnknown);
impl ISyndicationText {
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
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISyndicationText, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for ISyndicationText {}
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
impl ::windows_core::RuntimeType for ISyndicationText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b9cc5e80-313a-4091-a2a6-243e0ee923f9}");
}
unsafe impl ::windows_core::Interface for ISyndicationText {
    type Vtable = ISyndicationText_Vtbl;
}
impl ::core::clone::Clone for ISyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationText {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9cc5e80_313a_4091_a2a6_243e0ee923f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationText_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Xml: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub SetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    SetXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationTextFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationTextFactory {
    type Vtable = ISyndicationTextFactory_Vtbl;
}
impl ::core::clone::Clone for ISyndicationTextFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISyndicationTextFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee7342f7_11c6_4b25_ab62_e596bd162946);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationTextFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSyndicationText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSyndicationTextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationAttribute(::windows_core::IUnknown);
impl SyndicationAttribute {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationAttribute, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
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
    pub fn Namespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Namespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationAttribute(attributename: &::windows_core::HSTRING, attributenamespace: &::windows_core::HSTRING, attributevalue: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationAttribute> {
        Self::ISyndicationAttributeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationAttribute)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(attributename), ::core::mem::transmute_copy(attributenamespace), ::core::mem::transmute_copy(attributevalue), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationAttributeFactory<R, F: FnOnce(&ISyndicationAttributeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationAttribute, ISyndicationAttributeFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationAttribute {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationAttribute;{71e8f969-526e-4001-9a91-e84f83161ab1})");
}
impl ::core::clone::Clone for SyndicationAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationAttribute {
    const IID: ::windows_core::GUID = <ISyndicationAttribute as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationAttribute {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationAttribute";
}
::windows_core::imp::interface_hierarchy!(SyndicationAttribute, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SyndicationAttribute {}
unsafe impl ::core::marker::Sync for SyndicationAttribute {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationCategory(::windows_core::IUnknown);
impl SyndicationCategory {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationCategory, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Scheme(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scheme)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScheme(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScheme)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Term(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Term)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTerm(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTerm)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationCategory(term: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationCategory)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(term), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSyndicationCategoryEx(term: &::windows_core::HSTRING, scheme: &::windows_core::HSTRING, label: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationCategoryEx)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(term), ::core::mem::transmute_copy(scheme), ::core::mem::transmute_copy(label), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationCategoryFactory<R, F: FnOnce(&ISyndicationCategoryFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationCategory, ISyndicationCategoryFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationCategory;{8715626f-0cba-4a7f-89ff-ecb5281423b6})");
}
impl ::core::clone::Clone for SyndicationCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationCategory {
    const IID: ::windows_core::GUID = <ISyndicationCategory as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationCategory {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationCategory";
}
::windows_core::imp::interface_hierarchy!(SyndicationCategory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationCategory {}
unsafe impl ::core::marker::Send for SyndicationCategory {}
unsafe impl ::core::marker::Sync for SyndicationCategory {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationClient(::windows_core::IUnknown);
impl SyndicationClient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationClient, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrieveFeedAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateSyndicationClient<P0>(servercredential: P0) -> ::windows_core::Result<SyndicationClient>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        Self::ISyndicationClientFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationClient)(::windows_core::Interface::as_raw(this), servercredential.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationClientFactory<R, F: FnOnce(&ISyndicationClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationClient, ISyndicationClientFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationClient {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationClient;{9e18a9b7-7249-4b45-b229-7df895a5a1f5})");
}
impl ::core::clone::Clone for SyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationClient {
    const IID: ::windows_core::GUID = <ISyndicationClient as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationClient {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationClient";
}
::windows_core::imp::interface_hierarchy!(SyndicationClient, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationClient> for SyndicationClient {}
unsafe impl ::core::marker::Send for SyndicationClient {}
unsafe impl ::core::marker::Sync for SyndicationClient {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationContent(::windows_core::IUnknown);
impl SyndicationContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationContent, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSourceUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationContent(text: &::windows_core::HSTRING, r#type: SyndicationTextType) -> ::windows_core::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationContent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationContentWithSourceUri<P0>(sourceuri: P0) -> ::windows_core::Result<SyndicationContent>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationContentWithSourceUri)(::windows_core::Interface::as_raw(this), sourceuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn ISyndicationContentFactory<R, F: FnOnce(&ISyndicationContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationContent, ISyndicationContentFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationContent;{4641fefe-0e55-40d0-b8d0-6a2ccba9fc7c})");
}
impl ::core::clone::Clone for SyndicationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationContent {
    const IID: ::windows_core::GUID = <ISyndicationContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationContent {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationContent";
}
::windows_core::imp::interface_hierarchy!(SyndicationContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationContent {}
impl ::windows_core::CanTryInto<ISyndicationText> for SyndicationContent {}
unsafe impl ::core::marker::Send for SyndicationContent {}
unsafe impl ::core::marker::Sync for SyndicationContent {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct SyndicationError;
impl SyndicationError {
    pub fn GetStatus(hresult: i32) -> ::windows_core::Result<SyndicationErrorStatus> {
        Self::ISyndicationErrorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), hresult, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationErrorStatics<R, F: FnOnce(&ISyndicationErrorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationError, ISyndicationErrorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SyndicationError {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationError";
}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationFeed(::windows_core::IUnknown);
impl SyndicationFeed {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationFeed, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Authors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Authors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Categories(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contributors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contributors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Generator(&self) -> ::windows_core::Result<SyndicationGenerator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Generator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGenerator<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SyndicationGenerator>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGenerator)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IconUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIconUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIconUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastUpdatedTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastUpdatedTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Links(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Links)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetImageUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImageUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Rights(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rights)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRights<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRights)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtitle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FirstUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NextUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreviousUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceFormat(&self) -> ::windows_core::Result<SyndicationFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Load(&self, feed: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Load)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(feed)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn LoadFromXml<P0>(&self, feeddocument: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromXml)(::windows_core::Interface::as_raw(this), feeddocument.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationFeed<P0>(title: &::windows_core::HSTRING, subtitle: &::windows_core::HSTRING, uri: P0) -> ::windows_core::Result<SyndicationFeed>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationFeedFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationFeed)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(subtitle), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationFeedFactory<R, F: FnOnce(&ISyndicationFeedFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationFeed, ISyndicationFeedFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationFeed {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationFeed;{7ffe3cd2-5b66-4d62-8403-1bc10d910d6b})");
}
impl ::core::clone::Clone for SyndicationFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationFeed {
    const IID: ::windows_core::GUID = <ISyndicationFeed as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationFeed {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationFeed";
}
::windows_core::imp::interface_hierarchy!(SyndicationFeed, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationFeed {}
unsafe impl ::core::marker::Send for SyndicationFeed {}
unsafe impl ::core::marker::Sync for SyndicationFeed {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationGenerator(::windows_core::IUnknown);
impl SyndicationGenerator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationGenerator, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Version(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateSyndicationGenerator(text: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationGenerator> {
        Self::ISyndicationGeneratorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationGenerator)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationGeneratorFactory<R, F: FnOnce(&ISyndicationGeneratorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationGenerator, ISyndicationGeneratorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationGenerator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationGenerator;{9768b379-fb2b-4f6d-b41c-088a5868825c})");
}
impl ::core::clone::Clone for SyndicationGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationGenerator {
    const IID: ::windows_core::GUID = <ISyndicationGenerator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationGenerator {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationGenerator";
}
::windows_core::imp::interface_hierarchy!(SyndicationGenerator, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationGenerator {}
unsafe impl ::core::marker::Send for SyndicationGenerator {}
unsafe impl ::core::marker::Sync for SyndicationGenerator {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationItem(::windows_core::IUnknown);
impl SyndicationItem {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationItem, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Authors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Authors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Categories(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Contributors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contributors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<SyndicationContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SyndicationContent>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastUpdatedTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastUpdatedTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Links(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Links)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishedDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublishedDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPublishedDate(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPublishedDate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rights(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rights)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRights<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRights)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<SyndicationFeed> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SyndicationFeed>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Summary(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Summary)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSummary<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSummary)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ISyndicationText>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommentsUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommentsUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCommentsUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommentsUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EditUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EditUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EditMediaUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EditMediaUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ETag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ETag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Load(&self, item: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Load)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(item)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn LoadFromXml<P0>(&self, itemdocument: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromXml)(::windows_core::Interface::as_raw(this), itemdocument.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationItem<P0, P1>(title: &::windows_core::HSTRING, content: P0, uri: P1) -> ::windows_core::Result<SyndicationItem>
    where
        P0: ::windows_core::IntoParam<SyndicationContent>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationItemFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(title), content.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationItemFactory<R, F: FnOnce(&ISyndicationItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationItem, ISyndicationItemFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationItem;{548db883-c384-45c1-8ae8-a378c4ec486c})");
}
impl ::core::clone::Clone for SyndicationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationItem {
    const IID: ::windows_core::GUID = <ISyndicationItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationItem {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationItem";
}
::windows_core::imp::interface_hierarchy!(SyndicationItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationItem {}
unsafe impl ::core::marker::Send for SyndicationItem {}
unsafe impl ::core::marker::Sync for SyndicationItem {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationLink(::windows_core::IUnknown);
impl SyndicationLink {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationLink, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMediaType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Relationship(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Relationship)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRelationship(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelationship)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    pub fn ResourceLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResourceLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResourceLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationLink<P0>(uri: P0) -> ::windows_core::Result<SyndicationLink>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationLink)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationLinkEx<P0>(uri: P0, relationship: &::windows_core::HSTRING, title: &::windows_core::HSTRING, mediatype: &::windows_core::HSTRING, length: u32) -> ::windows_core::Result<SyndicationLink>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationLinkEx)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(relationship), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(mediatype), length, &mut result__).from_abi(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISyndicationLinkFactory<R, F: FnOnce(&ISyndicationLinkFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationLink, ISyndicationLinkFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationLink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationLink;{27553abd-a10e-41b5-86bd-9759086eb0c5})");
}
impl ::core::clone::Clone for SyndicationLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationLink {
    const IID: ::windows_core::GUID = <ISyndicationLink as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationLink {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationLink";
}
::windows_core::imp::interface_hierarchy!(SyndicationLink, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationLink {}
unsafe impl ::core::marker::Send for SyndicationLink {}
unsafe impl ::core::marker::Sync for SyndicationLink {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationNode(::windows_core::IUnknown);
impl SyndicationNode {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationNode, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateSyndicationNode(nodename: &::windows_core::HSTRING, nodenamespace: &::windows_core::HSTRING, nodevalue: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationNode> {
        Self::ISyndicationNodeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationNode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(nodename), ::core::mem::transmute_copy(nodenamespace), ::core::mem::transmute_copy(nodevalue), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationNodeFactory<R, F: FnOnce(&ISyndicationNodeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationNode, ISyndicationNodeFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationNode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationNode;{753cef78-51f8-45c0-a9f5-f1719dec3fb2})");
}
impl ::core::clone::Clone for SyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationNode {
    const IID: ::windows_core::GUID = <ISyndicationNode as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationNode {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationNode";
}
::windows_core::imp::interface_hierarchy!(SyndicationNode, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationNode {}
unsafe impl ::core::marker::Send for SyndicationNode {}
unsafe impl ::core::marker::Sync for SyndicationNode {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationPerson(::windows_core::IUnknown);
impl SyndicationPerson {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationPerson, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    pub fn Email(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Email)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmail(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmail)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
    pub fn CreateSyndicationPerson(name: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationPerson)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSyndicationPersonEx<P0>(name: &::windows_core::HSTRING, email: &::windows_core::HSTRING, uri: P0) -> ::windows_core::Result<SyndicationPerson>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationPersonEx)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(email), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationPersonFactory<R, F: FnOnce(&ISyndicationPersonFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationPerson, ISyndicationPersonFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationPerson {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationPerson;{fa1ee5da-a7c6-4517-a096-0143faf29327})");
}
impl ::core::clone::Clone for SyndicationPerson {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationPerson {
    const IID: ::windows_core::GUID = <ISyndicationPerson as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationPerson {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationPerson";
}
::windows_core::imp::interface_hierarchy!(SyndicationPerson, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationPerson {}
unsafe impl ::core::marker::Send for SyndicationPerson {}
unsafe impl ::core::marker::Sync for SyndicationPerson {}
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationText(::windows_core::IUnknown);
impl SyndicationText {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationText, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
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
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Xml(&self) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn SetXml<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationText(text: &::windows_core::HSTRING) -> ::windows_core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSyndicationTextEx(text: &::windows_core::HSTRING, r#type: SyndicationTextType) -> ::windows_core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationTextEx)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISyndicationTextFactory<R, F: FnOnce(&ISyndicationTextFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SyndicationText, ISyndicationTextFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SyndicationText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationText;{b9cc5e80-313a-4091-a2a6-243e0ee923f9})");
}
impl ::core::clone::Clone for SyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SyndicationText {
    type Vtable = ISyndicationText_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SyndicationText {
    const IID: ::windows_core::GUID = <ISyndicationText as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationText {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationText";
}
::windows_core::imp::interface_hierarchy!(SyndicationText, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ISyndicationNode> for SyndicationText {}
impl ::windows_core::CanTryInto<ISyndicationText> for SyndicationText {}
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
impl ::windows_core::TypeKind for SyndicationErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SyndicationErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SyndicationErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationErrorStatus;i4)");
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
impl ::windows_core::TypeKind for SyndicationFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SyndicationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SyndicationFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationFormat;i4)");
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
impl ::windows_core::TypeKind for SyndicationTextType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SyndicationTextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationTextType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SyndicationTextType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationTextType;i4)");
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
impl ::windows_core::TypeKind for RetrievalProgress {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for RetrievalProgress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.RetrievalProgress;u4;u4)");
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
impl ::windows_core::TypeKind for TransferProgress {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TransferProgress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.TransferProgress;u4;u4;u4;u4)");
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
