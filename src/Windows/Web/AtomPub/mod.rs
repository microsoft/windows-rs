#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Web_AtomPub`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AtomPubClient(pub ::windows::runtime::IInspectable);
impl AtomPubClient {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AtomPubClient, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn RetrieveServiceDocumentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Storage_Streams`, `Web_Syndication`*"]
    pub fn RetrieveMediaResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn RetrieveResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn CreateResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, description: Param1, item: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), description.into_param().abi(), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Storage_Streams`, `Web_Syndication`*"]
    pub fn CreateMediaResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(
        &self,
        uri: Param0,
        mediatype: Param1,
        description: Param2,
        mediastream: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), mediatype.into_param().abi(), description.into_param().abi(), mediastream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Storage_Streams`, `Web_Syndication`*"]
    pub fn UpdateMediaResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, mediatype: Param1, mediastream: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), uri.into_param().abi(), mediatype.into_param().abi(), mediastream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn UpdateResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, item: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), uri.into_param().abi(), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn UpdateResourceItemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn DeleteResourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn DeleteResourceItemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Web_AtomPub`*"]
    pub fn CancelAsyncOperations(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Security_Credentials`, `Web_Syndication`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Security_Credentials`, `Web_Syndication`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Security_Credentials`, `Web_Syndication`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Security_Credentials`, `Web_Syndication`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn MaxResponseBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Timeout(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetTimeout(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn RetrieveFeedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Web_AtomPub`, `Security_Credentials`*"]
    pub fn CreateAtomPubClientWithCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(servercredential: Param0) -> ::windows::runtime::Result<AtomPubClient> {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), servercredential.into_param().abi(), &mut result__).from_abi::<AtomPubClient>(result__)
        })
    }
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AtomPubClient, IAtomPubClientFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AtomPubClient {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.AtomPubClient;{35392c38-cded-4d4c-9637-05f15c1c9406})");
}
unsafe impl ::windows::runtime::Interface for AtomPubClient {
    type Vtable = IAtomPubClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
impl ::windows::runtime::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
impl ::core::convert::From<AtomPubClient> for ::windows::runtime::IUnknown {
    fn from(value: AtomPubClient) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows::runtime::IUnknown {
    fn from(value: &AtomPubClient) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AtomPubClient> for ::windows::runtime::IInspectable {
    fn from(value: AtomPubClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows::runtime::IInspectable {
    fn from(value: &AtomPubClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AtomPubClient) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AtomPubClient) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationClient> for AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationClient> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationClient> for &AtomPubClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationClient> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationClient>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AtomPubClient {}
unsafe impl ::core::marker::Sync for AtomPubClient {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAtomPubClient(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAtomPubClient {
    type Vtable = IAtomPubClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, description: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, item: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, mediastream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, mediastream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAtomPubClientFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAtomPubClientFactory {
    type Vtable = IAtomPubClientFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49d55012_57cb_4bde_ab9f_2610b172777b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClientFactory_abi(
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
#[doc(hidden)]
pub struct IResourceCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCollection {
    type Vtable = IResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Web_Syndication")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IServiceDocument(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IServiceDocument {
    type Vtable = IServiceDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWorkspace(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWorkspace {
    type Vtable = IWorkspace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Web_Syndication")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Web_AtomPub`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceCollection(pub ::windows::runtime::IInspectable);
impl ResourceCollection {
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn Categories(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`*"]
    pub fn Accepts(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Data_Xml_Dom`, `Web_Syndication`*"]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ResourceCollection;{7f5fd609-bc88-41d4-88fa-3de6704d428e})");
}
unsafe impl ::windows::runtime::Interface for ResourceCollection {
    type Vtable = IResourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
impl ::windows::runtime::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
impl ::core::convert::From<ResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: ResourceCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceCollection> for ::windows::runtime::IInspectable {
    fn from(value: ResourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ResourceCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ResourceCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for &ResourceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceCollection {}
unsafe impl ::core::marker::Sync for ResourceCollection {}
#[doc = "*Required features: `Web_AtomPub`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ServiceDocument(pub ::windows::runtime::IInspectable);
impl ServiceDocument {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`*"]
    pub fn Workspaces(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Workspace>>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Data_Xml_Dom`, `Web_Syndication`*"]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ServiceDocument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ServiceDocument;{8b7ec771-2ab3-4dbe-8bcc-778f92b75e51})");
}
unsafe impl ::windows::runtime::Interface for ServiceDocument {
    type Vtable = IServiceDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
impl ::windows::runtime::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
impl ::core::convert::From<ServiceDocument> for ::windows::runtime::IUnknown {
    fn from(value: ServiceDocument) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows::runtime::IUnknown {
    fn from(value: &ServiceDocument) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ServiceDocument> for ::windows::runtime::IInspectable {
    fn from(value: ServiceDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows::runtime::IInspectable {
    fn from(value: &ServiceDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ServiceDocument) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ServiceDocument) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for &ServiceDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ServiceDocument {}
unsafe impl ::core::marker::Sync for ServiceDocument {}
#[doc = "*Required features: `Web_AtomPub`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Workspace(pub ::windows::runtime::IInspectable);
impl Workspace {
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`*"]
    pub fn Collections(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ResourceCollection>>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn NodeValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetNodeValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_Syndication")]
    #[doc = "*Required features: `Web_AtomPub`, `Web_Syndication`*"]
    pub fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation`, `Web_Syndication`*"]
    pub fn SetBaseUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn AttributeExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Foundation_Collections`, `Web_Syndication`*"]
    pub fn ElementExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    #[doc = "*Required features: `Web_AtomPub`, `Data_Xml_Dom`, `Web_Syndication`*"]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::runtime::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::runtime::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Workspace {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.Workspace;{b41da63b-a4b8-4036-89c5-83c31266ba49})");
}
unsafe impl ::windows::runtime::Interface for Workspace {
    type Vtable = IWorkspace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
impl ::windows::runtime::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
impl ::core::convert::From<Workspace> for ::windows::runtime::IUnknown {
    fn from(value: Workspace) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Workspace> for ::windows::runtime::IUnknown {
    fn from(value: &Workspace) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Workspace> for ::windows::runtime::IInspectable {
    fn from(value: Workspace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Workspace> for ::windows::runtime::IInspectable {
    fn from(value: &Workspace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Workspace) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Workspace) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Syndication::ISyndicationNode> for &Workspace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for Workspace {}
unsafe impl ::core::marker::Sync for Workspace {}
