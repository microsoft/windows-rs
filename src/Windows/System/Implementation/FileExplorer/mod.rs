#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe132d1b9_7b9d_5820_9728_4262b5289142);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISysStorageProviderEventReceivedEventArgsFactory {
    type Vtable = ISysStorageProviderEventReceivedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde1a780e_e975_5f68_bcc6_fb46281c6a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `System_Implementation_FileExplorer`*"]
pub struct ISysStorageProviderEventSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISysStorageProviderEventSource {
    type Vtable = ISysStorageProviderEventSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f36c476_9546_536a_8381_2f9a2c08cedd);
}
impl ISysStorageProviderEventSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Implementation_FileExplorer`, `Foundation`*"]
    pub fn EventReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Implementation_FileExplorer`, `Foundation`*"]
    pub fn RemoveEventReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ISysStorageProviderEventSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1f36c476-9546-536a-8381-2f9a2c08cedd}");
}
impl ::core::convert::From<ISysStorageProviderEventSource> for ::windows::core::IUnknown {
    fn from(value: ISysStorageProviderEventSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISysStorageProviderEventSource> for ::windows::core::IUnknown {
    fn from(value: &ISysStorageProviderEventSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISysStorageProviderEventSource> for ::windows::core::IInspectable {
    fn from(value: ISysStorageProviderEventSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISysStorageProviderEventSource> for ::windows::core::IInspectable {
    fn from(value: &ISysStorageProviderEventSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `System_Implementation_FileExplorer`*"]
pub struct ISysStorageProviderHandlerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISysStorageProviderHandlerFactory {
    type Vtable = ISysStorageProviderHandlerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee798431_8213_5e89_a623_14d8c72b8a61);
}
impl ISysStorageProviderHandlerFactory {
    #[doc = "*Required features: `System_Implementation_FileExplorer`*"]
    pub fn GetHttpRequestProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, syncrootid: Param0) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), syncrootid.into_param().abi(), &mut result__).from_abi::<ISysStorageProviderHttpRequestProvider>(result__)
        }
    }
    #[doc = "*Required features: `System_Implementation_FileExplorer`*"]
    pub fn GetEventSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, syncrootid: Param0, eventname: Param1) -> ::windows::core::Result<ISysStorageProviderEventSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), syncrootid.into_param().abi(), eventname.into_param().abi(), &mut result__).from_abi::<ISysStorageProviderEventSource>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISysStorageProviderHandlerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee798431-8213-5e89-a623-14d8c72b8a61}");
}
impl ::core::convert::From<ISysStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: ISysStorageProviderHandlerFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISysStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: &ISysStorageProviderHandlerFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISysStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: ISysStorageProviderHandlerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISysStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: &ISysStorageProviderHandlerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHandlerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `System_Implementation_FileExplorer`*"]
pub struct ISysStorageProviderHttpRequestProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISysStorageProviderHttpRequestProvider {
    type Vtable = ISysStorageProviderHttpRequestProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb6fefb6_e76a_5c25_a33e_3e78a6e0e0ce);
}
impl ISysStorageProviderHttpRequestProvider {
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    #[doc = "*Required features: `System_Implementation_FileExplorer`, `Foundation`, `Web_Http`*"]
    pub fn SendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISysStorageProviderHttpRequestProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cb6fefb6-e76a-5c25-a33e-3e78a6e0e0ce}");
}
impl ::core::convert::From<ISysStorageProviderHttpRequestProvider> for ::windows::core::IUnknown {
    fn from(value: ISysStorageProviderHttpRequestProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISysStorageProviderHttpRequestProvider> for ::windows::core::IUnknown {
    fn from(value: &ISysStorageProviderHttpRequestProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISysStorageProviderHttpRequestProvider> for ::windows::core::IInspectable {
    fn from(value: ISysStorageProviderHttpRequestProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISysStorageProviderHttpRequestProvider> for ::windows::core::IInspectable {
    fn from(value: &ISysStorageProviderHttpRequestProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHttpRequestProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))] usize,
);
#[doc = "*Required features: `System_Implementation_FileExplorer`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SysStorageProviderEventReceivedEventArgs(pub ::windows::core::IInspectable);
impl SysStorageProviderEventReceivedEventArgs {
    #[doc = "*Required features: `System_Implementation_FileExplorer`*"]
    pub fn Json(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Implementation_FileExplorer`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(json: Param0) -> ::windows::core::Result<SysStorageProviderEventReceivedEventArgs> {
        Self::ISysStorageProviderEventReceivedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), json.into_param().abi(), &mut result__).from_abi::<SysStorageProviderEventReceivedEventArgs>(result__)
        })
    }
    pub fn ISysStorageProviderEventReceivedEventArgsFactory<R, F: FnOnce(&ISysStorageProviderEventReceivedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs;{e132d1b9-7b9d-5820-9728-4262b5289142})");
}
unsafe impl ::windows::core::Interface for SysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe132d1b9_7b9d_5820_9728_4262b5289142);
}
impl ::windows::core::RuntimeName for SysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
}
impl ::core::convert::From<SysStorageProviderEventReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SysStorageProviderEventReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SysStorageProviderEventReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SysStorageProviderEventReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SysStorageProviderEventReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SysStorageProviderEventReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SysStorageProviderEventReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SysStorageProviderEventReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SysStorageProviderEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SysStorageProviderEventReceivedEventArgs {}
