#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct AtomPubClient(::windows::core::IUnknown);
impl AtomPubClient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AtomPubClient, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveServiceDocumentAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveServiceDocumentAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn RetrieveMediaResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveMediaResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn CreateResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, description: Param1, item: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), description.into_param().abi(), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn CreateMediaResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, mediatype: Param1, description: Param2, mediastream: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMediaResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), mediatype.into_param().abi(), description.into_param().abi(), mediastream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn UpdateMediaResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, mediatype: Param1, mediastream: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateMediaResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), mediatype.into_param().abi(), mediastream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, item: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateResourceItemAsync)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteResourceAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceItemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteResourceItemAsync)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`*"]
    pub fn CancelAsyncOperations(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CancelAsyncOperations)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateAtomPubClientWithCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(servercredential: Param0) -> ::windows::core::Result<AtomPubClient> {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAtomPubClientWithCredentials)(::core::mem::transmute_copy(this), servercredential.into_param().abi(), &mut result__).from_abi::<AtomPubClient>(result__)
        })
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServerCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn MaxResponseBufferSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxResponseBufferSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxResponseBufferSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Timeout(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetTimeout(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTimeout)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BypassCacheOnRetrieve)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestHeader)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveFeedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveFeedAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AtomPubClient, IAtomPubClientFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AtomPubClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AtomPubClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AtomPubClient {}
impl ::core::fmt::Debug for AtomPubClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AtomPubClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AtomPubClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.AtomPubClient;{35392c38-cded-4d4c-9637-05f15c1c9406})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
    const IID: ::windows::core::GUID = <IAtomPubClient as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
impl ::core::convert::From<AtomPubClient> for ::windows::core::IUnknown {
    fn from(value: AtomPubClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows::core::IUnknown {
    fn from(value: &AtomPubClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AtomPubClient> for ::windows::core::IInspectable {
    fn from(value: AtomPubClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows::core::IInspectable {
    fn from(value: &AtomPubClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows::core::Error;
    fn try_from(value: AtomPubClient) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows::core::Error;
    fn try_from(value: &AtomPubClient) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationClient> for AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationClient> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationClient> for &AtomPubClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationClient> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationClient>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AtomPubClient {}
unsafe impl ::core::marker::Sync for AtomPubClient {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClient_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub CreateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAtomPubClientFactory {
    type Vtable = IAtomPubClientFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d55012_57cb_4bde_ab9f_2610b172777b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClientFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateAtomPubClientWithCredentials: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCollection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Accepts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Accepts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDocument_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Workspaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Workspaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkspace(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct ResourceCollection(::windows::core::IUnknown);
impl ResourceCollection {
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Categories)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Accepts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Accepts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeNamespace)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBaseUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttributeExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlDocument)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCollection {}
impl ::core::fmt::Debug for ResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ResourceCollection;{7f5fd609-bc88-41d4-88fa-3de6704d428e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
    const IID: ::windows::core::GUID = <IResourceCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
impl ::core::convert::From<ResourceCollection> for ::windows::core::IUnknown {
    fn from(value: ResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows::core::IUnknown {
    fn from(value: &ResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceCollection> for ::windows::core::IInspectable {
    fn from(value: ResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows::core::IInspectable {
    fn from(value: &ResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for &ResourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceCollection {}
unsafe impl ::core::marker::Sync for ResourceCollection {}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct ServiceDocument(::windows::core::IUnknown);
impl ServiceDocument {
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Workspaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Workspaces)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Workspace>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeNamespace)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBaseUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttributeExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlDocument)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::clone::Clone for ServiceDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceDocument {}
impl ::core::fmt::Debug for ServiceDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServiceDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ServiceDocument;{8b7ec771-2ab3-4dbe-8bcc-778f92b75e51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
    const IID: ::windows::core::GUID = <IServiceDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
impl ::core::convert::From<ServiceDocument> for ::windows::core::IUnknown {
    fn from(value: ServiceDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows::core::IUnknown {
    fn from(value: &ServiceDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServiceDocument> for ::windows::core::IInspectable {
    fn from(value: ServiceDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows::core::IInspectable {
    fn from(value: &ServiceDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: ServiceDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &ServiceDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for &ServiceDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ServiceDocument {}
unsafe impl ::core::marker::Sync for ServiceDocument {}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct Workspace(::windows::core::IUnknown);
impl Workspace {
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeNamespace)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNodeValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BaseUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBaseUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttributeExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementExtensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlDocument)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_AtomPub\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Collections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ResourceCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for Workspace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Workspace {}
impl ::core::fmt::Debug for Workspace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Workspace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Workspace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.Workspace;{b41da63b-a4b8-4036-89c5-83c31266ba49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Workspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: ::windows::core::GUID = <IWorkspace as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
impl ::core::convert::From<Workspace> for ::windows::core::IUnknown {
    fn from(value: Workspace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Workspace> for ::windows::core::IUnknown {
    fn from(value: &Workspace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Workspace> for ::windows::core::IInspectable {
    fn from(value: Workspace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Workspace> for ::windows::core::IInspectable {
    fn from(value: &Workspace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: Workspace) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl ::core::convert::TryFrom<&Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &Workspace) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_Syndication")]
impl<'a> ::windows::core::IntoParam<'a, super::Syndication::ISyndicationNode> for &Workspace {
    fn into_param(self) -> ::windows::core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Workspace {}
unsafe impl ::core::marker::Sync for Workspace {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
