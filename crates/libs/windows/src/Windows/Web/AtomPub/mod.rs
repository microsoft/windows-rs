#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAtomPubClient(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAtomPubClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClient_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub CreateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::HSTRING>, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, mediatype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, description: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, mediatype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAtomPubClientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAtomPubClientFactory {
    type Vtable = IAtomPubClientFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAtomPubClientFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49d55012_57cb_4bde_ab9f_2610b172777b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateAtomPubClientWithCredentials: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Accepts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Accepts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IServiceDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IServiceDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDocument_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Workspaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Workspaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspace(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AtomPubClient(::windows_core::IUnknown);
impl AtomPubClient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AtomPubClient, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveServiceDocumentAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveServiceDocumentAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn RetrieveMediaResourceAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveResourceAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn CreateResourceAsync<P0, P1>(&self, uri: P0, description: &::windows_core::HSTRING, item: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(description), item.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn CreateMediaResourceAsync<P0, P1>(&self, uri: P0, mediatype: &::windows_core::HSTRING, description: &::windows_core::HSTRING, mediastream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(mediatype), ::core::mem::transmute_copy(description), mediastream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn UpdateMediaResourceAsync<P0, P1>(&self, uri: P0, mediatype: &::windows_core::HSTRING, mediastream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(mediatype), mediastream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceAsync<P0, P1>(&self, uri: P0, item: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), item.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceItemAsync<P0>(&self, item: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateResourceItemAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceItemAsync<P0>(&self, item: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::windows_core::IntoParam<super::Syndication::SyndicationItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteResourceItemAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CancelAsyncOperations(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CancelAsyncOperations)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Security_Credentials\"`"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateAtomPubClientWithCredentials<P0>(servercredential: P0) -> ::windows_core::Result<AtomPubClient>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAtomPubClientWithCredentials)(::windows_core::Interface::as_raw(this), servercredential.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetServerCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetRequestHeader(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveFeedAsync<P0>(&self, uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AtomPubClient, IAtomPubClientFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AtomPubClient {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.AtomPubClient;{35392c38-cded-4d4c-9637-05f15c1c9406})");
}
unsafe impl ::windows_core::Interface for AtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AtomPubClient {
    const IID: ::windows_core::GUID = <IAtomPubClient as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
::windows_core::imp::interface_hierarchy!(AtomPubClient, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
impl ::windows_core::CanTryInto<super::Syndication::ISyndicationClient> for AtomPubClient {}
unsafe impl ::core::marker::Send for AtomPubClient {}
unsafe impl ::core::marker::Sync for AtomPubClient {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceCollection(::windows_core::IUnknown);
impl ResourceCollection {
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn Categories(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Accepts(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Accepts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ResourceCollection;{7f5fd609-bc88-41d4-88fa-3de6704d428e})");
}
unsafe impl ::windows_core::Interface for ResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceCollection {
    const IID: ::windows_core::GUID = <IResourceCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
::windows_core::imp::interface_hierarchy!(ResourceCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
impl ::windows_core::CanTryInto<super::Syndication::ISyndicationNode> for ResourceCollection {}
unsafe impl ::core::marker::Send for ResourceCollection {}
unsafe impl ::core::marker::Sync for ResourceCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ServiceDocument(::windows_core::IUnknown);
impl ServiceDocument {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Workspaces(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Workspaces)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ServiceDocument {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ServiceDocument;{8b7ec771-2ab3-4dbe-8bcc-778f92b75e51})");
}
unsafe impl ::windows_core::Interface for ServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ServiceDocument {
    const IID: ::windows_core::GUID = <IServiceDocument as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
::windows_core::imp::interface_hierarchy!(ServiceDocument, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
impl ::windows_core::CanTryInto<super::Syndication::ISyndicationNode> for ServiceDocument {}
unsafe impl ::core::marker::Send for ServiceDocument {}
unsafe impl ::core::marker::Sync for ServiceDocument {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Workspace(::windows_core::IUnknown);
impl Workspace {
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::ComInterface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_Syndication\"`"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Workspace {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.Workspace;{b41da63b-a4b8-4036-89c5-83c31266ba49})");
}
unsafe impl ::windows_core::Interface for Workspace {
    type Vtable = IWorkspace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Workspace {
    const IID: ::windows_core::GUID = <IWorkspace as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
::windows_core::imp::interface_hierarchy!(Workspace, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Web_Syndication")]
impl ::windows_core::CanTryInto<super::Syndication::ISyndicationNode> for Workspace {}
unsafe impl ::core::marker::Send for Workspace {}
unsafe impl ::core::marker::Sync for Workspace {}
