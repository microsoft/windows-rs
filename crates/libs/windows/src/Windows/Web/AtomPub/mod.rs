#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
}
unsafe impl ::windows::core::Interface for IAtomPubClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub CreateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, description: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, description: *mut ::core::ffi::c_void, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAtomPubClientFactory {
    type Vtable = IAtomPubClientFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAtomPubClientFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d55012_57cb_4bde_ab9f_2610b172777b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClientFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateAtomPubClientWithCredentials: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Accepts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Accepts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IServiceDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Workspaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Workspaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkspace(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
}
unsafe impl ::windows::core::Interface for IWorkspace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct AtomPubClient(::windows::core::IUnknown);
impl AtomPubClient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AtomPubClient, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveServiceDocumentAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveServiceDocumentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn RetrieveMediaResourceAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveMediaResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveResourceAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn CreateResourceAsync(&self, uri: &super::super::Foundation::Uri, description: &::windows::core::HSTRING, item: &super::Syndication::SyndicationItem) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(description), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn CreateMediaResourceAsync<P0, E0>(&self, uri: &super::super::Foundation::Uri, mediatype: &::windows::core::HSTRING, description: &::windows::core::HSTRING, mediastream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMediaResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(mediatype), ::core::mem::transmute_copy(description), mediastream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub fn UpdateMediaResourceAsync<P0, E0>(&self, uri: &super::super::Foundation::Uri, mediatype: &::windows::core::HSTRING, mediastream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateMediaResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(mediatype), mediastream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceAsync(&self, uri: &super::super::Foundation::Uri, item: &super::Syndication::SyndicationItem) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn UpdateResourceItemAsync(&self, item: &super::Syndication::SyndicationItem) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateResourceItemAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteResourceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn DeleteResourceItemAsync(&self, item: &super::Syndication::SyndicationItem) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteResourceItemAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CancelAsyncOperations(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CancelAsyncOperations)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateAtomPubClientWithCredentials(servercredential: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<AtomPubClient> {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAtomPubClientWithCredentials)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(servercredential), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetServerCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProxyCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Security_Credentials", feature = "Web_Syndication"))]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetProxyCredential)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn MaxResponseBufferSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxResponseBufferSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Timeout(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetTimeout(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBypassCacheOnRetrieve)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetRequestHeader(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRequestHeader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn RetrieveFeedAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveFeedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AtomPubClient, IAtomPubClientFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for AtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
}
unsafe impl ::windows::core::Interface for AtomPubClient {
    const IID: ::windows::core::GUID = <IAtomPubClient as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
::windows::core::interface_hierarchy!(AtomPubClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&AtomPubClient> for ::windows::core::InParam<super::Syndication::ISyndicationClient> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AtomPubClient) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AtomPubClient {}
unsafe impl ::core::marker::Sync for AtomPubClient {}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct ResourceCollection(::windows::core::IUnknown);
impl ResourceCollection {
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
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
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Categories)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Accepts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Accepts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for ResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceCollection {
    const IID: ::windows::core::GUID = <IResourceCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
::windows::core::interface_hierarchy!(ResourceCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&ResourceCollection> for ::windows::core::InParam<super::Syndication::ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ResourceCollection {}
unsafe impl ::core::marker::Sync for ResourceCollection {}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct ServiceDocument(::windows::core::IUnknown);
impl ServiceDocument {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Workspaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Workspaces)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for ServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for ServiceDocument {
    const IID: ::windows::core::GUID = <IServiceDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
::windows::core::interface_hierarchy!(ServiceDocument, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&ServiceDocument> for ::windows::core::InParam<super::Syndication::ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ServiceDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ServiceDocument {}
unsafe impl ::core::marker::Sync for ServiceDocument {}
#[doc = "*Required features: `\"Web_AtomPub\"`*"]
#[repr(transparent)]
pub struct Workspace(::windows::core::IUnknown);
impl Workspace {
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeNamespace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeNamespace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNodeValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub fn SetBaseUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaseUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttributeExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`, `\"Web_Syndication\"`*"]
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Web_Syndication"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlDocument)(::windows::core::Vtable::as_raw(this), format, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Syndication\"`*"]
    #[cfg(feature = "Web_Syndication")]
    pub fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Collections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for Workspace {
    type Vtable = IWorkspace_Vtbl;
}
unsafe impl ::windows::core::Interface for Workspace {
    const IID: ::windows::core::GUID = <IWorkspace as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
::windows::core::interface_hierarchy!(Workspace, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&Workspace> for ::windows::core::InParam<super::Syndication::ISyndicationNode> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Workspace) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Workspace {}
unsafe impl ::core::marker::Sync for Workspace {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
