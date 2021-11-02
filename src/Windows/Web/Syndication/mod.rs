#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationAttribute(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationAttribute {
    type Vtable = ISyndicationAttribute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1911093609, 21102, 16385, [154, 145, 232, 79, 131, 22, 26, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttribute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationAttributeFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationAttributeFactory {
    type Vtable = ISyndicationAttributeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1649350041, 60734, 16911, [190, 134, 100, 4, 20, 136, 110, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttributeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, attributenamespace: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, attributevalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationCategory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationCategory {
    type Vtable = ISyndicationCategory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2266325615, 3258, 19071, [137, 255, 236, 181, 40, 20, 35, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationCategoryFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationCategoryFactory {
    type Vtable = ISyndicationCategoryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2873262127, 18912, 17701, [138, 178, 171, 69, 192, 37, 40, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategoryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, scheme: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Web_Syndication`*"]
pub struct ISyndicationClient(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationClient {
    type Vtable = ISyndicationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2652416439, 29257, 19269, [178, 41, 125, 248, 149, 165, 161, 245]);
}
impl ISyndicationClient {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn MaxResponseBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Timeout(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTimeout(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn RetrieveFeedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISyndicationClient {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9e18a9b7-7249-4b45-b229-7df895a5a1f5}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationClientFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationClientFactory {
    type Vtable = ISyndicationClientFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(784642860, 42907, 16660, [178, 154, 5, 223, 251, 175, 185, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClientFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servercredential: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationContent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationContent {
    type Vtable = ISyndicationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178730238, 3669, 16592, [184, 208, 106, 44, 203, 169, 252, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationContentFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationContentFactory {
    type Vtable = ISyndicationContentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1026538387, 38176, 16755, [147, 136, 126, 45, 243, 36, 168, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, r#type: SyndicationTextType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceuri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationErrorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationErrorStatics {
    type Vtable = ISyndicationErrorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(532357985, 17863, 18483, [138, 160, 190, 95, 59, 88, 167, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationErrorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: i32, result__: *mut SyndicationErrorStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationFeed(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationFeed {
    type Vtable = ISyndicationFeed_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2147368146, 23398, 19810, [132, 3, 27, 193, 13, 145, 13, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeed_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SyndicationFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, feed: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, feeddocument: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationFeedFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationFeedFactory {
    type Vtable = ISyndicationFeedFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(591864370, 35817, 18615, [137, 52, 98, 5, 19, 29, 147, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeedFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, subtitle: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationGenerator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationGenerator {
    type Vtable = ISyndicationGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2540221305, 64299, 20333, [180, 28, 8, 138, 88, 104, 130, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationGeneratorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationGeneratorFactory {
    type Vtable = ISyndicationGeneratorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2738914275, 7718, 19900, [186, 157, 26, 184, 75, 239, 249, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationItem {
    type Vtable = ISyndicationItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1418573955, 50052, 17857, [138, 232, 163, 120, 196, 236, 72, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemdocument: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationItemFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationItemFactory {
    type Vtable = ISyndicationItemFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(622674767, 32184, 18554, [133, 228, 16, 209, 145, 230, 110, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItemFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, content: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationLink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationLink {
    type Vtable = ISyndicationLink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(659897021, 41230, 16821, [134, 189, 151, 89, 8, 110, 176, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationLinkFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationLinkFactory {
    type Vtable = ISyndicationLinkFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1591239636, 21813, 18604, [152, 212, 193, 144, 153, 80, 128, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLinkFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, relationship: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, mediatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, length: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Web_Syndication`*"]
pub struct ISyndicationNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationNode {
    type Vtable = ISyndicationNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1966927736, 20984, 17856, [169, 245, 241, 113, 157, 236, 63, 178]);
}
impl ISyndicationNode {
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISyndicationNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{753cef78-51f8-45c0-a9f5-f1719dec3fb2}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: SyndicationFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationNodeFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationNodeFactory {
    type Vtable = ISyndicationNodeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(311435656, 19147, 18856, [183, 119, 165, 235, 146, 225, 138, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNodeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, nodenamespace: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, nodevalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationPerson(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationPerson {
    type Vtable = ISyndicationPerson_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4196328922, 42950, 17687, [160, 150, 1, 67, 250, 242, 147, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPerson_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationPersonFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationPersonFactory {
    type Vtable = ISyndicationPersonFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3707013229, 8861, 19288, [164, 155, 243, 210, 240, 245, 201, 159]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPersonFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, email: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Web_Syndication`*"]
pub struct ISyndicationText(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationText {
    type Vtable = ISyndicationText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3117178496, 12602, 16529, [162, 166, 36, 62, 14, 233, 35, 249]);
}
impl ISyndicationText {
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn Xml(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn SetXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISyndicationText {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b9cc5e80-313a-4091-a2a6-243e0ee923f9}");
}
impl ::std::convert::TryFrom<ISyndicationText> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ISyndicationText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ISyndicationText> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ISyndicationText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for ISyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &ISyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationText_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISyndicationTextFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISyndicationTextFactory {
    type Vtable = ISyndicationTextFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4000531191, 4550, 19237, [171, 98, 229, 150, 189, 22, 41, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationTextFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, r#type: SyndicationTextType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Web_Syndication`*"]
pub struct RetrievalProgress {
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl RetrievalProgress {}
impl ::std::default::Default for RetrievalProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RetrievalProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RetrievalProgress").field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
impl ::std::cmp::PartialEq for RetrievalProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesRetrieved == other.BytesRetrieved && self.TotalBytesToRetrieve == other.TotalBytesToRetrieve
    }
}
impl ::std::cmp::Eq for RetrievalProgress {}
unsafe impl ::windows::runtime::Abi for RetrievalProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RetrievalProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.RetrievalProgress;u4;u4)");
}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationAttribute(::windows::runtime::IInspectable);
impl SyndicationAttribute {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationAttribute, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Namespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(attributename: Param0, attributenamespace: Param1, attributevalue: Param2) -> ::windows::runtime::Result<SyndicationAttribute> {
        Self::ISyndicationAttributeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), attributename.into_param().abi(), attributenamespace.into_param().abi(), attributevalue.into_param().abi(), &mut result__).from_abi::<SyndicationAttribute>(result__)
        })
    }
    pub fn ISyndicationAttributeFactory<R, F: FnOnce(&ISyndicationAttributeFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationAttribute, ISyndicationAttributeFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationAttribute {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationAttribute;{71e8f969-526e-4001-9a91-e84f83161ab1})");
}
unsafe impl ::windows::runtime::Interface for SyndicationAttribute {
    type Vtable = ISyndicationAttribute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1911093609, 21102, 16385, [154, 145, 232, 79, 131, 22, 26, 177]);
}
impl ::windows::runtime::RuntimeName for SyndicationAttribute {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationAttribute";
}
impl ::std::convert::From<SyndicationAttribute> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationAttribute) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationAttribute> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationAttribute) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationAttribute> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationAttribute) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationAttribute> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationAttribute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationAttribute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SyndicationAttribute {}
unsafe impl ::std::marker::Sync for SyndicationAttribute {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationCategory(::windows::runtime::IInspectable);
impl SyndicationCategory {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationCategory, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Scheme(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetScheme<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Term(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTerm<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationCategory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(term: Param0) -> ::windows::runtime::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), term.into_param().abi(), &mut result__).from_abi::<SyndicationCategory>(result__)
        })
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationCategoryEx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(term: Param0, scheme: Param1, label: Param2) -> ::windows::runtime::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), term.into_param().abi(), scheme.into_param().abi(), label.into_param().abi(), &mut result__).from_abi::<SyndicationCategory>(result__)
        })
    }
    pub fn ISyndicationCategoryFactory<R, F: FnOnce(&ISyndicationCategoryFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationCategory, ISyndicationCategoryFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationCategory;{8715626f-0cba-4a7f-89ff-ecb5281423b6})");
}
unsafe impl ::windows::runtime::Interface for SyndicationCategory {
    type Vtable = ISyndicationCategory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2266325615, 3258, 19071, [137, 255, 236, 181, 40, 20, 35, 182]);
}
impl ::windows::runtime::RuntimeName for SyndicationCategory {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationCategory";
}
impl ::std::convert::From<SyndicationCategory> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationCategory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationCategory> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationCategory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationCategory> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationCategory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationCategory> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationCategory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationCategory> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationCategory) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationCategory> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationCategory) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationCategory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationCategory {}
unsafe impl ::std::marker::Sync for SyndicationCategory {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationClient(::windows::runtime::IInspectable);
impl SyndicationClient {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationClient, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn MaxResponseBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Timeout(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTimeout(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn RetrieveFeedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_Syndication`, `Security_Credentials`*"]
    pub fn CreateSyndicationClient<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(servercredential: Param0) -> ::windows::runtime::Result<SyndicationClient> {
        Self::ISyndicationClientFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), servercredential.into_param().abi(), &mut result__).from_abi::<SyndicationClient>(result__)
        })
    }
    pub fn ISyndicationClientFactory<R, F: FnOnce(&ISyndicationClientFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationClient, ISyndicationClientFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationClient {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationClient;{9e18a9b7-7249-4b45-b229-7df895a5a1f5})");
}
unsafe impl ::windows::runtime::Interface for SyndicationClient {
    type Vtable = ISyndicationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2652416439, 29257, 19269, [178, 41, 125, 248, 149, 165, 161, 245]);
}
impl ::windows::runtime::RuntimeName for SyndicationClient {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationClient";
}
impl ::std::convert::From<SyndicationClient> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationClient> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationClient> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationClient) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationClient> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SyndicationClient> for ISyndicationClient {
    fn from(value: SyndicationClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationClient> for ISyndicationClient {
    fn from(value: &SyndicationClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationClient> for SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationClient> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationClient>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationClient> for &SyndicationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationClient> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationClient>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SyndicationClient {}
unsafe impl ::std::marker::Sync for SyndicationClient {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationContent(::windows::runtime::IInspectable);
impl SyndicationContent {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationContent, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SourceUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetSourceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn Xml(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn SetXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationContent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0, r#type: SyndicationTextType) -> ::windows::runtime::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), r#type, &mut result__).from_abi::<SyndicationContent>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationContentWithSourceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(sourceuri: Param0) -> ::windows::runtime::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), sourceuri.into_param().abi(), &mut result__).from_abi::<SyndicationContent>(result__)
        })
    }
    pub fn ISyndicationContentFactory<R, F: FnOnce(&ISyndicationContentFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationContent, ISyndicationContentFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationContent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationContent;{4641fefe-0e55-40d0-b8d0-6a2ccba9fc7c})");
}
unsafe impl ::windows::runtime::Interface for SyndicationContent {
    type Vtable = ISyndicationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178730238, 3669, 16592, [184, 208, 106, 44, 203, 169, 252, 124]);
}
impl ::windows::runtime::RuntimeName for SyndicationContent {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationContent";
}
impl ::std::convert::From<SyndicationContent> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationContent> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationContent> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationContent> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationContent> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationContent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationContent> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SyndicationContent> for ISyndicationText {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationContent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationContent> for ISyndicationText {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationContent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationText> for SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationText> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationText> for &SyndicationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationText> {
        ::std::convert::TryInto::<ISyndicationText>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationContent {}
unsafe impl ::std::marker::Sync for SyndicationContent {}
#[doc = "*Required features: `Web_Syndication`*"]
pub struct SyndicationError {}
impl SyndicationError {
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn GetStatus(hresult: i32) -> ::windows::runtime::Result<SyndicationErrorStatus> {
        Self::ISyndicationErrorStatics(|this| unsafe {
            let mut result__: SyndicationErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), hresult, &mut result__).from_abi::<SyndicationErrorStatus>(result__)
        })
    }
    pub fn ISyndicationErrorStatics<R, F: FnOnce(&ISyndicationErrorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationError, ISyndicationErrorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SyndicationError {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationError";
}
#[doc = "*Required features: `Web_Syndication`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SyndicationErrorStatus(pub i32);
impl SyndicationErrorStatus {
    pub const Unknown: SyndicationErrorStatus = SyndicationErrorStatus(0i32);
    pub const MissingRequiredElement: SyndicationErrorStatus = SyndicationErrorStatus(1i32);
    pub const MissingRequiredAttribute: SyndicationErrorStatus = SyndicationErrorStatus(2i32);
    pub const InvalidXml: SyndicationErrorStatus = SyndicationErrorStatus(3i32);
    pub const UnexpectedContent: SyndicationErrorStatus = SyndicationErrorStatus(4i32);
    pub const UnsupportedFormat: SyndicationErrorStatus = SyndicationErrorStatus(5i32);
}
impl ::std::convert::From<i32> for SyndicationErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SyndicationErrorStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationErrorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationErrorStatus;i4)");
}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationFeed(::windows::runtime::IInspectable);
impl SyndicationFeed {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationFeed, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Authors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Categories(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Contributors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Generator(&self) -> ::windows::runtime::Result<SyndicationGenerator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SyndicationGenerator>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetGenerator<'a, Param0: ::windows::runtime::IntoParam<'a, SyndicationGenerator>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn IconUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetIconUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn LastUpdatedTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetLastUpdatedTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Links(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationLink>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn ImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetImageUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Rights(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetRights<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn FirstUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn LastUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn NextUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn PreviousUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SourceFormat(&self) -> ::windows::runtime::Result<SyndicationFormat> {
        let this = self;
        unsafe {
            let mut result__: SyndicationFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SyndicationFormat>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, feed: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), feed.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn LoadFromXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(&self, feeddocument: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), feeddocument.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationFeed<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(title: Param0, subtitle: Param1, uri: Param2) -> ::windows::runtime::Result<SyndicationFeed> {
        Self::ISyndicationFeedFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), title.into_param().abi(), subtitle.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<SyndicationFeed>(result__)
        })
    }
    pub fn ISyndicationFeedFactory<R, F: FnOnce(&ISyndicationFeedFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationFeed, ISyndicationFeedFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationFeed {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationFeed;{7ffe3cd2-5b66-4d62-8403-1bc10d910d6b})");
}
unsafe impl ::windows::runtime::Interface for SyndicationFeed {
    type Vtable = ISyndicationFeed_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2147368146, 23398, 19810, [132, 3, 27, 193, 13, 145, 13, 107]);
}
impl ::windows::runtime::RuntimeName for SyndicationFeed {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationFeed";
}
impl ::std::convert::From<SyndicationFeed> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationFeed) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationFeed> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationFeed) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationFeed> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationFeed) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationFeed> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationFeed) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationFeed> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationFeed) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationFeed> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationFeed) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationFeed {}
unsafe impl ::std::marker::Sync for SyndicationFeed {}
#[doc = "*Required features: `Web_Syndication`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: SyndicationFormat = SyndicationFormat(0i32);
    pub const Rss20: SyndicationFormat = SyndicationFormat(1i32);
    pub const Rss10: SyndicationFormat = SyndicationFormat(2i32);
    pub const Rss092: SyndicationFormat = SyndicationFormat(3i32);
    pub const Rss091: SyndicationFormat = SyndicationFormat(4i32);
    pub const Atom03: SyndicationFormat = SyndicationFormat(5i32);
}
impl ::std::convert::From<i32> for SyndicationFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SyndicationFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationFormat;i4)");
}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationGenerator(::windows::runtime::IInspectable);
impl SyndicationGenerator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationGenerator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationGenerator<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0) -> ::windows::runtime::Result<SyndicationGenerator> {
        Self::ISyndicationGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<SyndicationGenerator>(result__)
        })
    }
    pub fn ISyndicationGeneratorFactory<R, F: FnOnce(&ISyndicationGeneratorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationGenerator, ISyndicationGeneratorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationGenerator;{9768b379-fb2b-4f6d-b41c-088a5868825c})");
}
unsafe impl ::windows::runtime::Interface for SyndicationGenerator {
    type Vtable = ISyndicationGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2540221305, 64299, 20333, [180, 28, 8, 138, 88, 104, 130, 92]);
}
impl ::windows::runtime::RuntimeName for SyndicationGenerator {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationGenerator";
}
impl ::std::convert::From<SyndicationGenerator> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationGenerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationGenerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationGenerator> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationGenerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationGenerator) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationGenerator) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationGenerator {}
unsafe impl ::std::marker::Sync for SyndicationGenerator {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationItem(::windows::runtime::IInspectable);
impl SyndicationItem {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationItem, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Authors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Categories(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Contributors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<SyndicationContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SyndicationContent>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, SyndicationContent>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn LastUpdatedTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetLastUpdatedTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn Links(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationLink>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn PublishedDate(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetPublishedDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Rights(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetRights<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<SyndicationFeed> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SyndicationFeed>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, SyndicationFeed>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Summary(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetSummary<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CommentsUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetCommentsUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn EditUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn EditMediaUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn ETag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn ItemUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), item.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn LoadFromXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(&self, itemdocument: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), itemdocument.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationItem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, SyndicationContent>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(title: Param0, content: Param1, uri: Param2) -> ::windows::runtime::Result<SyndicationItem> {
        Self::ISyndicationItemFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), title.into_param().abi(), content.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<SyndicationItem>(result__)
        })
    }
    pub fn ISyndicationItemFactory<R, F: FnOnce(&ISyndicationItemFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationItem, ISyndicationItemFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationItem;{548db883-c384-45c1-8ae8-a378c4ec486c})");
}
unsafe impl ::windows::runtime::Interface for SyndicationItem {
    type Vtable = ISyndicationItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1418573955, 50052, 17857, [138, 232, 163, 120, 196, 236, 72, 108]);
}
impl ::windows::runtime::RuntimeName for SyndicationItem {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationItem";
}
impl ::std::convert::From<SyndicationItem> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationItem> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationItem> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationItem> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationItem> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationItem) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationItem> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationItem) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationItem {}
unsafe impl ::std::marker::Sync for SyndicationItem {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationLink(::windows::runtime::IInspectable);
impl SyndicationLink {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationLink, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn MediaType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetMediaType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Relationship(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetRelationship<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn ResourceLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetResourceLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<SyndicationLink>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationLinkEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(uri: Param0, relationship: Param1, title: Param2, mediatype: Param3, length: u32) -> ::windows::runtime::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), relationship.into_param().abi(), title.into_param().abi(), mediatype.into_param().abi(), length, &mut result__).from_abi::<SyndicationLink>(result__)
        })
    }
    pub fn ISyndicationLinkFactory<R, F: FnOnce(&ISyndicationLinkFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationLink, ISyndicationLinkFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationLink {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationLink;{27553abd-a10e-41b5-86bd-9759086eb0c5})");
}
unsafe impl ::windows::runtime::Interface for SyndicationLink {
    type Vtable = ISyndicationLink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(659897021, 41230, 16821, [134, 189, 151, 89, 8, 110, 176, 197]);
}
impl ::windows::runtime::RuntimeName for SyndicationLink {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationLink";
}
impl ::std::convert::From<SyndicationLink> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationLink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationLink> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationLink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationLink> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationLink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationLink> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationLink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationLink> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationLink) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationLink> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationLink) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationLink {}
unsafe impl ::std::marker::Sync for SyndicationLink {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationNode(::windows::runtime::IInspectable);
impl SyndicationNode {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationNode, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(nodename: Param0, nodenamespace: Param1, nodevalue: Param2) -> ::windows::runtime::Result<SyndicationNode> {
        Self::ISyndicationNodeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), nodename.into_param().abi(), nodenamespace.into_param().abi(), nodevalue.into_param().abi(), &mut result__).from_abi::<SyndicationNode>(result__)
        })
    }
    pub fn ISyndicationNodeFactory<R, F: FnOnce(&ISyndicationNodeFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationNode, ISyndicationNodeFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationNode;{753cef78-51f8-45c0-a9f5-f1719dec3fb2})");
}
unsafe impl ::windows::runtime::Interface for SyndicationNode {
    type Vtable = ISyndicationNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1966927736, 20984, 17856, [169, 245, 241, 113, 157, 236, 63, 178]);
}
impl ::windows::runtime::RuntimeName for SyndicationNode {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationNode";
}
impl ::std::convert::From<SyndicationNode> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationNode> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationNode> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationNode> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SyndicationNode> for ISyndicationNode {
    fn from(value: SyndicationNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationNode> for ISyndicationNode {
    fn from(value: &SyndicationNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SyndicationNode {}
unsafe impl ::std::marker::Sync for SyndicationNode {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationPerson(::windows::runtime::IInspectable);
impl SyndicationPerson {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationPerson, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Email(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetEmail<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationPerson<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<SyndicationPerson>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn CreateSyndicationPersonEx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(name: Param0, email: Param1, uri: Param2) -> ::windows::runtime::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), email.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<SyndicationPerson>(result__)
        })
    }
    pub fn ISyndicationPersonFactory<R, F: FnOnce(&ISyndicationPersonFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationPerson, ISyndicationPersonFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationPerson {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationPerson;{fa1ee5da-a7c6-4517-a096-0143faf29327})");
}
unsafe impl ::windows::runtime::Interface for SyndicationPerson {
    type Vtable = ISyndicationPerson_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4196328922, 42950, 17687, [160, 150, 1, 67, 250, 242, 147, 39]);
}
impl ::windows::runtime::RuntimeName for SyndicationPerson {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationPerson";
}
impl ::std::convert::From<SyndicationPerson> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationPerson) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationPerson> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationPerson) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationPerson> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationPerson) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationPerson> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationPerson) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SyndicationPerson> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationPerson) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationPerson> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationPerson) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationPerson {}
unsafe impl ::std::marker::Sync for SyndicationPerson {}
#[doc = "*Required features: `Web_Syndication`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SyndicationText(::windows::runtime::IInspectable);
impl SyndicationText {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationText, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn Xml(&self) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn SetXml<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Syndication`, `Foundation_Collections`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Web_Syndication`, `Data_Xml_Dom`*"]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0) -> ::windows::runtime::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<SyndicationText>(result__)
        })
    }
    #[doc = "*Required features: `Web_Syndication`*"]
    pub fn CreateSyndicationTextEx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0, r#type: SyndicationTextType) -> ::windows::runtime::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), r#type, &mut result__).from_abi::<SyndicationText>(result__)
        })
    }
    pub fn ISyndicationTextFactory<R, F: FnOnce(&ISyndicationTextFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SyndicationText, ISyndicationTextFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationText {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationText;{b9cc5e80-313a-4091-a2a6-243e0ee923f9})");
}
unsafe impl ::windows::runtime::Interface for SyndicationText {
    type Vtable = ISyndicationText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3117178496, 12602, 16529, [162, 166, 36, 62, 14, 233, 35, 249]);
}
impl ::windows::runtime::RuntimeName for SyndicationText {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationText";
}
impl ::std::convert::From<SyndicationText> for ::windows::runtime::IUnknown {
    fn from(value: SyndicationText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationText> for ::windows::runtime::IUnknown {
    fn from(value: &SyndicationText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SyndicationText> for ::windows::runtime::IInspectable {
    fn from(value: SyndicationText) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SyndicationText> for ::windows::runtime::IInspectable {
    fn from(value: &SyndicationText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SyndicationText> for ISyndicationText {
    fn from(value: SyndicationText) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SyndicationText> for ISyndicationText {
    fn from(value: &SyndicationText) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationText> for SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationText> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationText>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationText> for &SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationText> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyndicationText>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<SyndicationText> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SyndicationText) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SyndicationText> for ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SyndicationText) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyndicationNode> for &SyndicationText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyndicationNode> {
        ::std::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SyndicationText {}
unsafe impl ::std::marker::Sync for SyndicationText {}
#[doc = "*Required features: `Web_Syndication`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: SyndicationTextType = SyndicationTextType(0i32);
    pub const Html: SyndicationTextType = SyndicationTextType(1i32);
    pub const Xhtml: SyndicationTextType = SyndicationTextType(2i32);
}
impl ::std::convert::From<i32> for SyndicationTextType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SyndicationTextType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SyndicationTextType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationTextType;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Web_Syndication`*"]
pub struct TransferProgress {
    pub BytesSent: u32,
    pub TotalBytesToSend: u32,
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl TransferProgress {}
impl ::std::default::Default for TransferProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TransferProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TransferProgress").field("BytesSent", &self.BytesSent).field("TotalBytesToSend", &self.TotalBytesToSend).field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
impl ::std::cmp::PartialEq for TransferProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesSent == other.BytesSent && self.TotalBytesToSend == other.TotalBytesToSend && self.BytesRetrieved == other.BytesRetrieved && self.TotalBytesToRetrieve == other.TotalBytesToRetrieve
    }
}
impl ::std::cmp::Eq for TransferProgress {}
unsafe impl ::windows::runtime::Abi for TransferProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TransferProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.TransferProgress;u4;u4;u4;u4)");
}
