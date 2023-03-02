#[doc(hidden)]
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISysStorageProviderEventReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISysStorageProviderEventReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe132d1b9_7b9d_5820_9728_4262b5289142);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Json: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISysStorageProviderEventReceivedEventArgsFactory {
    type Vtable = ISysStorageProviderEventReceivedEventArgsFactory_Vtbl;
}
impl ::core::clone::Clone for ISysStorageProviderEventReceivedEventArgsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISysStorageProviderEventReceivedEventArgsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde1a780e_e975_5f68_bcc6_fb46281c6a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderEventSource(::windows::core::IUnknown);
impl ISysStorageProviderEventSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EventReceived(&self, handler: &super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).EventReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEventReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEventReceived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
::windows::imp::interface_hierarchy!(ISysStorageProviderEventSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISysStorageProviderEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderEventSource {}
impl ::core::fmt::Debug for ISysStorageProviderEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderEventSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISysStorageProviderEventSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1f36c476-9546-536a-8381-2f9a2c08cedd}");
}
unsafe impl ::windows::core::Interface for ISysStorageProviderEventSource {
    type Vtable = ISysStorageProviderEventSource_Vtbl;
}
impl ::core::clone::Clone for ISysStorageProviderEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISysStorageProviderEventSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f36c476_9546_536a_8381_2f9a2c08cedd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EventReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEventReceived: usize,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderHandlerFactory(::windows::core::IUnknown);
impl ISysStorageProviderHandlerFactory {
    pub fn GetHttpRequestProvider(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ISysStorageProviderHttpRequestProvider>();
            (::windows::core::Interface::vtable(this).GetHttpRequestProvider)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetEventSource(&self, syncrootid: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderEventSource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ISysStorageProviderEventSource>();
            (::windows::core::Interface::vtable(this).GetEventSource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootid), ::core::mem::transmute_copy(eventname), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISysStorageProviderHandlerFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISysStorageProviderHandlerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderHandlerFactory {}
impl ::core::fmt::Debug for ISysStorageProviderHandlerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderHandlerFactory").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISysStorageProviderHandlerFactory {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ee798431-8213-5e89-a623-14d8c72b8a61}");
}
unsafe impl ::windows::core::Interface for ISysStorageProviderHandlerFactory {
    type Vtable = ISysStorageProviderHandlerFactory_Vtbl;
}
impl ::core::clone::Clone for ISysStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISysStorageProviderHandlerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee798431_8213_5e89_a623_14d8c72b8a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHandlerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetHttpRequestProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderHttpRequestProvider(::windows::core::IUnknown);
impl ISysStorageProviderHttpRequestProvider {
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub fn SendRequestAsync(&self, request: &super::super::super::Web::Http::HttpRequestMessage) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>();
            (::windows::core::Interface::vtable(this).SendRequestAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISysStorageProviderHttpRequestProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISysStorageProviderHttpRequestProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderHttpRequestProvider {}
impl ::core::fmt::Debug for ISysStorageProviderHttpRequestProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderHttpRequestProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISysStorageProviderHttpRequestProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{cb6fefb6-e76a-5c25-a33e-3e78a6e0e0ce}");
}
unsafe impl ::windows::core::Interface for ISysStorageProviderHttpRequestProvider {
    type Vtable = ISysStorageProviderHttpRequestProvider_Vtbl;
}
impl ::core::clone::Clone for ISysStorageProviderHttpRequestProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISysStorageProviderHttpRequestProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb6fefb6_e76a_5c25_a33e_3e78a6e0e0ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHttpRequestProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    SendRequestAsync: usize,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct SysStorageProviderEventReceivedEventArgs(::windows::core::IUnknown);
impl SysStorageProviderEventReceivedEventArgs {
    pub fn Json(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Json)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(json: &::windows::core::HSTRING) -> ::windows::core::Result<SysStorageProviderEventReceivedEventArgs> {
        Self::ISysStorageProviderEventReceivedEventArgsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SysStorageProviderEventReceivedEventArgs>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(json), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISysStorageProviderEventReceivedEventArgsFactory<R, F: FnOnce(&ISysStorageProviderEventReceivedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SysStorageProviderEventReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SysStorageProviderEventReceivedEventArgs {}
impl ::core::fmt::Debug for SysStorageProviderEventReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysStorageProviderEventReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs;{e132d1b9-7b9d-5820-9728-4262b5289142})");
}
impl ::core::clone::Clone for SysStorageProviderEventReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SysStorageProviderEventReceivedEventArgs {
    const IID: ::windows::core::GUID = <ISysStorageProviderEventReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(SysStorageProviderEventReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SysStorageProviderEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SysStorageProviderEventReceivedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
