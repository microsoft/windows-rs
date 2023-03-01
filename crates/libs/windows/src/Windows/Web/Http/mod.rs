#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBufferContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBufferContentFactory {
    type Vtable = IHttpBufferContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpBufferContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpBufferContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc20c193_c41f_4ff7_9123_6435736eadc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBufferContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBufferWithOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBufferWithOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClient {
    type Vtable = IHttpClient_Vtbl;
}
impl ::core::clone::Clone for IHttpClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fda1151_3574_4880_a8ba_e6b1e0061f3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWithOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWithOptionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PostAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PostAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendRequestWithOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestWithOptionAsync: usize,
    #[cfg(feature = "Web_Http_Headers")]
    pub DefaultRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    DefaultRequestHeaders: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClient2 {
    type Vtable = IHttpClient2_Vtbl;
}
impl ::core::clone::Clone for IHttpClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpClient2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd83348_e8b7_4cec_b1b0_dc455fe72c92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAsync2: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetStringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPostAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPostAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySendRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySendRequestAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySendRequestAsync2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClient3 {
    type Vtable = IHttpClient3_Vtbl;
}
impl ::core::clone::Clone for IHttpClient3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpClient3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1172fd01_9899_4194_963f_8f9d72a7ec15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DefaultPrivacyAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDefaultPrivacyAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClientFactory {
    type Vtable = IHttpClientFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpClientFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpClientFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc30c4eca_e3fa_4f99_afb4_63cc65009462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClientFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http_Filters")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Filters"))]
    Create: usize,
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct IHttpContent(::windows::core::IUnknown);
impl IHttpContent {
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IHttpContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IHttpContent {}
impl ::core::cmp::PartialEq for IHttpContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpContent {}
impl ::core::fmt::Debug for IHttpContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IHttpContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6b14a441-fba7-4bd2-af0a-839de7c295da}");
}
unsafe impl ::windows::core::Interface for IHttpContent {
    type Vtable = IHttpContent_Vtbl;
}
impl ::core::clone::Clone for IHttpContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    #[cfg(feature = "Foundation")]
    pub BufferAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadAsBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadAsBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadAsInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadAsInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadAsStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsStringAsync: usize,
    pub TryComputeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteToStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookie(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookie {
    type Vtable = IHttpCookie_Vtbl;
}
impl ::core::clone::Clone for IHttpCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f5488e2_cc2d_4779_86a7_88f10687d249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookie_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Expires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Expires: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpires: usize,
    pub HttpOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHttpOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Secure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookieFactory {
    type Vtable = IHttpCookieFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpCookieFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpCookieFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0585a9_931c_4cd1_a96d_c21701785c5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, domain: ::std::mem::MaybeUninit<::windows::core::HSTRING>, path: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookieManager {
    type Vtable = IHttpCookieManager_Vtbl;
}
impl ::core::clone::Clone for IHttpCookieManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpCookieManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a431780_cd4f_4e57_a84a_5b0a53d6bb96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCookieWithThirdParty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: *mut ::core::ffi::c_void, thirdparty: bool, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DeleteCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCookies: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpFormUrlEncodedContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpFormUrlEncodedContentFactory {
    type Vtable = IHttpFormUrlEncodedContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpFormUrlEncodedContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpFormUrlEncodedContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43f0138c_2f73_4302_b5f3_eae9238a5e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFormUrlEncodedContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetBufferResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetBufferResult {
    type Vtable = IHttpGetBufferResult_Vtbl;
}
impl ::core::clone::Clone for IHttpGetBufferResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpGetBufferResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d08e7c_e209_404e_9a49_742d8236fd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetBufferResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetInputStreamResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResult_Vtbl;
}
impl ::core::clone::Clone for IHttpGetInputStreamResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpGetInputStreamResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d63463_13aa_4ee0_be95_a0c39fe91203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetInputStreamResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetStringResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetStringResult {
    type Vtable = IHttpGetStringResult_Vtbl;
}
impl ::core::clone::Clone for IHttpGetStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpGetStringResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bac466d_8509_4775_b16d_8953f47a7f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetStringResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethod(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethod {
    type Vtable = IHttpMethod_Vtbl;
}
impl ::core::clone::Clone for IHttpMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMethod {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728d4022_700d_4fe0_afa5_40299c58dbfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethod_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethodFactory {
    type Vtable = IHttpMethodFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpMethodFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMethodFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c51d10d_36d7_40f8_a86d_e759caf2f83f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethodStatics {
    type Vtable = IHttpMethodStatics_Vtbl;
}
impl ::core::clone::Clone for IHttpMethodStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMethodStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d171f0_d99a_4153_8dc6_d68cc4cce317);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Head: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Patch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartContent {
    type Vtable = IHttpMultipartContent_Vtbl;
}
impl ::core::clone::Clone for IHttpMultipartContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMultipartContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf916aff_9926_4ac9_aaf1_e0d04ef09bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartContentFactory {
    type Vtable = IHttpMultipartContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpMultipartContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMultipartContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eb42e62_0222_4f20_b372_47d5db5d33b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithSubtypeAndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, boundary: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartFormDataContent {
    type Vtable = IHttpMultipartFormDataContent_Vtbl;
}
impl ::core::clone::Clone for IHttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMultipartFormDataContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d337e2_e967_4624_b6d1_cf74604a4a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddWithNameAndFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, filename: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartFormDataContentFactory {
    type Vtable = IHttpMultipartFormDataContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpMultipartFormDataContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpMultipartFormDataContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa04d7311_5017_4622_93a8_49b24a4fcbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundary: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestMessage {
    type Vtable = IHttpRequestMessage_Vtbl;
}
impl ::core::clone::Clone for IHttpRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpRequestMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5762b3c_74d4_4811_b5dc_9f8b4e2f9abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequestUri: usize,
    pub TransportInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestMessage2 {
    type Vtable = IHttpRequestMessage2_Vtbl;
}
impl ::core::clone::Clone for IHttpRequestMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpRequestMessage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c60489_62c2_4a3f_9554_226e7c60bd96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrivacyAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPrivacyAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestMessageFactory {
    type Vtable = IHttpRequestMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpRequestMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpRequestMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bac994e_3886_412e_aec3_52ec7f25616f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestResult {
    type Vtable = IHttpRequestResult_Vtbl;
}
impl ::core::clone::Clone for IHttpRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpRequestResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6acf4da8_b5eb_4a35_a902_4217fbe820c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpResponseMessage {
    type Vtable = IHttpResponseMessage_Vtbl;
}
impl ::core::clone::Clone for IHttpResponseMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpResponseMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee200fb_8664_44e0_95d9_42696199bffc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub IsSuccessStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetReasonPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpResponseMessageSource) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpResponseMessageSource) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpStatusCode) -> ::windows::core::HRESULT,
    pub SetStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpStatusCode) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpVersion) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpVersion) -> ::windows::core::HRESULT,
    pub EnsureSuccessStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpResponseMessageFactory {
    type Vtable = IHttpResponseMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpResponseMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpResponseMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52a8af99_f095_43da_b60f_7cfc2bc7ea2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statuscode: HttpStatusCode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStreamContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpStreamContentFactory {
    type Vtable = IHttpStreamContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpStreamContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpStreamContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3e64d9d_f725_407e_942f_0eda189809f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStreamContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromInputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStringContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpStringContentFactory {
    type Vtable = IHttpStringContentFactory_Vtbl;
}
impl ::core::clone::Clone for IHttpStringContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpStringContentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46649d5b_2e93_48eb_8e61_19677878e57f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStringContentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, encoding: super::super::Storage::Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncoding: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncodingAndMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncodingAndMediaType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransportInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransportInformation {
    type Vtable = IHttpTransportInformation_Vtbl;
}
impl ::core::clone::Clone for IHttpTransportInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHttpTransportInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70127198_c6a7_4ed0_833a_83fd8b8f178d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransportInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ServerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpBufferContent(::windows::core::IUnknown);
impl HttpBufferContent {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(content: P0) -> ::windows::core::Result<HttpBufferContent>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpBufferContent>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBufferWithOffset<P0>(content: P0, offset: u32, count: u32) -> ::windows::core::Result<HttpBufferContent>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpBufferContent>();
            (::windows::core::Interface::vtable(this).CreateFromBufferWithOffset)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), offset, count, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpBufferContentFactory<R, F: FnOnce(&IHttpBufferContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpBufferContent, IHttpBufferContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpBufferContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpBufferContent {}
impl ::core::fmt::Debug for HttpBufferContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpBufferContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpBufferContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpBufferContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpBufferContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpBufferContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpBufferContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpBufferContent {
    const NAME: &'static str = "Windows.Web.Http.HttpBufferContent";
}
::windows::imp::interface_hierarchy!(HttpBufferContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpBufferContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpBufferContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpBufferContent {}
unsafe impl ::core::marker::Send for HttpBufferContent {}
unsafe impl ::core::marker::Sync for HttpBufferContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpClient(::windows::core::IUnknown);
impl HttpClient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpClient, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).GetAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetWithOptionAsync(&self, uri: &super::super::Foundation::Uri, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).GetWithOptionAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), completionoption, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetBufferAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, HttpProgress>>();
            (::windows::core::Interface::vtable(this).GetBufferAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetInputStreamAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, HttpProgress>>();
            (::windows::core::Interface::vtable(this).GetInputStreamAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStringAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, HttpProgress>>();
            (::windows::core::Interface::vtable(this).GetStringAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PostAsync<P0>(&self, uri: &super::super::Foundation::Uri, content: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).PostAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PutAsync<P0>(&self, uri: &super::super::Foundation::Uri, content: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).PutAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync(&self, request: &HttpRequestMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).SendRequestAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestWithOptionAsync(&self, request: &HttpRequestMessage, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>();
            (::windows::core::Interface::vtable(this).SendRequestWithOptionAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), completionoption, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn DefaultRequestHeaders(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpRequestHeaderCollection>();
            (::windows::core::Interface::vtable(this).DefaultRequestHeaders)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryDeleteAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryGetAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetAsync2(&self, uri: &super::super::Foundation::Uri, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryGetAsync2)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), completionoption, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetBufferAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryGetBufferAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetInputStreamAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryGetInputStreamAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetStringAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryGetStringAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPostAsync<P0>(&self, uri: &super::super::Foundation::Uri, content: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryPostAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPutAsync<P0>(&self, uri: &super::super::Foundation::Uri, content: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TryPutAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySendRequestAsync(&self, request: &HttpRequestMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TrySendRequestAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySendRequestAsync2(&self, request: &HttpRequestMessage, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>();
            (::windows::core::Interface::vtable(this).TrySendRequestAsync2)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), completionoption, &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultPrivacyAnnotation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DefaultPrivacyAnnotation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultPrivacyAnnotation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IHttpClient3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultPrivacyAnnotation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Filters\"`*"]
    #[cfg(feature = "Web_Http_Filters")]
    pub fn Create<P0>(filter: P0) -> ::windows::core::Result<HttpClient>
    where
        P0: ::windows::core::TryIntoParam<Filters::IHttpFilter>,
    {
        Self::IHttpClientFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpClient>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), filter.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpClientFactory<R, F: FnOnce(&IHttpClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpClient, IHttpClientFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpClient {}
impl ::core::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpClient").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpClient {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpClient;{7fda1151-3574-4880-a8ba-e6b1e0061f3d})");
}
impl ::core::clone::Clone for HttpClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpClient {
    type Vtable = IHttpClient_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpClient {
    const IID: ::windows::core::GUID = <IHttpClient as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpClient {
    const NAME: &'static str = "Windows.Web.Http.HttpClient";
}
::windows::imp::interface_hierarchy!(HttpClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpClient {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpClient {}
unsafe impl ::core::marker::Send for HttpClient {}
unsafe impl ::core::marker::Sync for HttpClient {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpCookie(::windows::core::IUnknown);
impl HttpCookie {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Domain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Expires(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).Expires)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpires<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExpires)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn HttpOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HttpOnly)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHttpOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHttpOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Secure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Secure)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSecure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSecure)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(name: &::windows::core::HSTRING, domain: &::windows::core::HSTRING, path: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookie> {
        Self::IHttpCookieFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpCookie>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(domain), ::core::mem::transmute_copy(path), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCookieFactory<R, F: FnOnce(&IHttpCookieFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpCookie, IHttpCookieFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookie {}
impl ::core::fmt::Debug for HttpCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookie").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpCookie {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})");
}
impl ::core::clone::Clone for HttpCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpCookie {
    type Vtable = IHttpCookie_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpCookie {
    const IID: ::windows::core::GUID = <IHttpCookie as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpCookie {
    const NAME: &'static str = "Windows.Web.Http.HttpCookie";
}
::windows::imp::interface_hierarchy!(HttpCookie, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpCookie {}
unsafe impl ::core::marker::Send for HttpCookie {}
unsafe impl ::core::marker::Sync for HttpCookie {}
#[doc = "*Required features: `\"Web_Http\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct HttpCookieCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HttpCookieCollection {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<HttpCookie>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<HttpCookie>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<HttpCookie>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpCookie> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpCookie>();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpCookie, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpCookie>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for HttpCookieCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for HttpCookieCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeType for HttpCookieCollection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for HttpCookieCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for HttpCookieCollection {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<HttpCookie>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::ComInterface for HttpCookieCollection {
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVectorView<HttpCookie> as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for HttpCookieCollection {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::windows::core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::imp::interface_hierarchy!(HttpCookieCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<HttpCookie>> for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IVectorView<HttpCookie>> for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for HttpCookieCollection {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpCookieManager(::windows::core::IUnknown);
impl HttpCookieManager {
    pub fn SetCookie(&self, cookie: &HttpCookie) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SetCookie)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cookie), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCookieWithThirdParty(&self, cookie: &HttpCookie, thirdparty: bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SetCookieWithThirdParty)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cookie), thirdparty, &mut result__).from_abi(result__)
        }
    }
    pub fn DeleteCookie(&self, cookie: &HttpCookie) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeleteCookie)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cookie)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCookies(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<HttpCookieCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpCookieCollection>();
            (::windows::core::Interface::vtable(this).GetCookies)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpCookieManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookieManager {}
impl ::core::fmt::Debug for HttpCookieManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpCookieManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieManager;{7a431780-cd4f-4e57-a84a-5b0a53d6bb96})");
}
impl ::core::clone::Clone for HttpCookieManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpCookieManager {
    type Vtable = IHttpCookieManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpCookieManager {
    const IID: ::windows::core::GUID = <IHttpCookieManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpCookieManager {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieManager";
}
::windows::imp::interface_hierarchy!(HttpCookieManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HttpCookieManager {}
unsafe impl ::core::marker::Sync for HttpCookieManager {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpFormUrlEncodedContent(::windows::core::IUnknown);
impl HttpFormUrlEncodedContent {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(content: P0) -> ::windows::core::Result<HttpFormUrlEncodedContent>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>,
    {
        Self::IHttpFormUrlEncodedContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpFormUrlEncodedContent>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpFormUrlEncodedContentFactory<R, F: FnOnce(&IHttpFormUrlEncodedContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpFormUrlEncodedContent, IHttpFormUrlEncodedContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpFormUrlEncodedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpFormUrlEncodedContent {}
impl ::core::fmt::Debug for HttpFormUrlEncodedContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpFormUrlEncodedContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpFormUrlEncodedContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpFormUrlEncodedContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpFormUrlEncodedContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpFormUrlEncodedContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpFormUrlEncodedContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpFormUrlEncodedContent {
    const NAME: &'static str = "Windows.Web.Http.HttpFormUrlEncodedContent";
}
::windows::imp::interface_hierarchy!(HttpFormUrlEncodedContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpFormUrlEncodedContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpFormUrlEncodedContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpFormUrlEncodedContent {}
unsafe impl ::core::marker::Send for HttpFormUrlEncodedContent {}
unsafe impl ::core::marker::Sync for HttpFormUrlEncodedContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpGetBufferResult(::windows::core::IUnknown);
impl HttpGetBufferResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).ResponseMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpGetBufferResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetBufferResult {}
impl ::core::fmt::Debug for HttpGetBufferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetBufferResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpGetBufferResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetBufferResult;{53d08e7c-e209-404e-9a49-742d8236fd3a})");
}
impl ::core::clone::Clone for HttpGetBufferResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpGetBufferResult {
    type Vtable = IHttpGetBufferResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpGetBufferResult {
    const IID: ::windows::core::GUID = <IHttpGetBufferResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpGetBufferResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetBufferResult";
}
::windows::imp::interface_hierarchy!(HttpGetBufferResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpGetBufferResult {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpGetBufferResult {}
unsafe impl ::core::marker::Send for HttpGetBufferResult {}
unsafe impl ::core::marker::Sync for HttpGetBufferResult {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpGetInputStreamResult(::windows::core::IUnknown);
impl HttpGetInputStreamResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).ResponseMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpGetInputStreamResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetInputStreamResult {}
impl ::core::fmt::Debug for HttpGetInputStreamResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetInputStreamResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpGetInputStreamResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetInputStreamResult;{d5d63463-13aa-4ee0-be95-a0c39fe91203})");
}
impl ::core::clone::Clone for HttpGetInputStreamResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpGetInputStreamResult {
    const IID: ::windows::core::GUID = <IHttpGetInputStreamResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpGetInputStreamResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetInputStreamResult";
}
::windows::imp::interface_hierarchy!(HttpGetInputStreamResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpGetInputStreamResult {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpGetInputStreamResult {}
unsafe impl ::core::marker::Send for HttpGetInputStreamResult {}
unsafe impl ::core::marker::Sync for HttpGetInputStreamResult {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpGetStringResult(::windows::core::IUnknown);
impl HttpGetStringResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).ResponseMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpGetStringResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetStringResult {}
impl ::core::fmt::Debug for HttpGetStringResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetStringResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpGetStringResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetStringResult;{9bac466d-8509-4775-b16d-8953f47a7f5f})");
}
impl ::core::clone::Clone for HttpGetStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpGetStringResult {
    type Vtable = IHttpGetStringResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpGetStringResult {
    const IID: ::windows::core::GUID = <IHttpGetStringResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpGetStringResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetStringResult";
}
::windows::imp::interface_hierarchy!(HttpGetStringResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpGetStringResult {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpGetStringResult {}
unsafe impl ::core::marker::Send for HttpGetStringResult {}
unsafe impl ::core::marker::Sync for HttpGetStringResult {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpMethod(::windows::core::IUnknown);
impl HttpMethod {
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Method)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(method: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(method), &mut result__).from_abi(result__)
        })
    }
    pub fn Delete() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Delete)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Get() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Get)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Head() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Head)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Options() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Patch() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Patch)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Post() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Post)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Put() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Put)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMethodFactory<R, F: FnOnce(&IHttpMethodFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMethod, IHttpMethodFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpMethodStatics<R, F: FnOnce(&IHttpMethodStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMethod, IHttpMethodStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethod {}
impl ::core::fmt::Debug for HttpMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMethod").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpMethod {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMethod;{728d4022-700d-4fe0-afa5-40299c58dbfd})");
}
impl ::core::clone::Clone for HttpMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpMethod {
    type Vtable = IHttpMethod_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpMethod {
    const IID: ::windows::core::GUID = <IHttpMethod as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpMethod {
    const NAME: &'static str = "Windows.Web.Http.HttpMethod";
}
::windows::imp::interface_hierarchy!(HttpMethod, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpMethod {}
unsafe impl ::core::marker::Send for HttpMethod {}
unsafe impl ::core::marker::Sync for HttpMethod {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpMultipartContent(::windows::core::IUnknown);
impl HttpMultipartContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMultipartContent, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpMultipartContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Add)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi()).ok() }
    }
    pub fn CreateWithSubtype(subtype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMultipartContent>();
            (::windows::core::Interface::vtable(this).CreateWithSubtype)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithSubtypeAndBoundary(subtype: &::windows::core::HSTRING, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMultipartContent>();
            (::windows::core::Interface::vtable(this).CreateWithSubtypeAndBoundary)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), ::core::mem::transmute_copy(boundary), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<IHttpContent>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMultipartContentFactory<R, F: FnOnce(&IHttpMultipartContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMultipartContent, IHttpMultipartContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpMultipartContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartContent {}
impl ::core::fmt::Debug for HttpMultipartContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMultipartContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpMultipartContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpMultipartContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpMultipartContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpMultipartContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpMultipartContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::imp::interface_hierarchy!(HttpMultipartContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpMultipartContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpMultipartContent {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<IHttpContent>> for HttpMultipartContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpMultipartContent {}
unsafe impl ::core::marker::Send for HttpMultipartContent {}
unsafe impl ::core::marker::Sync for HttpMultipartContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpMultipartFormDataContent(::windows::core::IUnknown);
impl HttpMultipartFormDataContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMultipartFormDataContent, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Add)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi()).ok() }
    }
    pub fn AddWithName<P0>(&self, content: P0, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddWithName)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn AddWithNameAndFileName<P0>(&self, content: P0, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = &::windows::core::ComInterface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddWithNameAndFileName)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(filename)).ok() }
    }
    pub fn CreateWithBoundary(boundary: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartFormDataContent> {
        Self::IHttpMultipartFormDataContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMultipartFormDataContent>();
            (::windows::core::Interface::vtable(this).CreateWithBoundary)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(boundary), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<IHttpContent>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMultipartFormDataContentFactory<R, F: FnOnce(&IHttpMultipartFormDataContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpMultipartFormDataContent, IHttpMultipartFormDataContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpMultipartFormDataContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartFormDataContent {}
impl ::core::fmt::Debug for HttpMultipartFormDataContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMultipartFormDataContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpMultipartFormDataContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartFormDataContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpMultipartFormDataContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpMultipartFormDataContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpMultipartFormDataContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartFormDataContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::imp::interface_hierarchy!(HttpMultipartFormDataContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpMultipartFormDataContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpMultipartFormDataContent {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<IHttpContent>> for HttpMultipartFormDataContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpMultipartFormDataContent {}
unsafe impl ::core::marker::Send for HttpMultipartFormDataContent {}
unsafe impl ::core::marker::Sync for HttpMultipartFormDataContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpRequestMessage(::windows::core::IUnknown);
impl HttpRequestMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpRequestMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows::core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IHttpContent>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpRequestHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Method(&self) -> ::windows::core::Result<HttpMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpMethod>();
            (::windows::core::Interface::vtable(this).Method)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMethod(&self, value: &HttpMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMethod)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).RequestUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequestUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TransportInformation(&self) -> ::windows::core::Result<HttpTransportInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpTransportInformation>();
            (::windows::core::Interface::vtable(this).TransportInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrivacyAnnotation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IHttpRequestMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PrivacyAnnotation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPrivacyAnnotation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IHttpRequestMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrivacyAnnotation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(method: &HttpMethod, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<HttpRequestMessage> {
        Self::IHttpRequestMessageFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(method), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpRequestMessageFactory<R, F: FnOnce(&IHttpRequestMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpRequestMessage, IHttpRequestMessageFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpRequestMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestMessage {}
impl ::core::fmt::Debug for HttpRequestMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpRequestMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestMessage;{f5762b3c-74d4-4811-b5dc-9f8b4e2f9abf})");
}
impl ::core::clone::Clone for HttpRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpRequestMessage {
    type Vtable = IHttpRequestMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpRequestMessage {
    const IID: ::windows::core::GUID = <IHttpRequestMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpRequestMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestMessage";
}
::windows::imp::interface_hierarchy!(HttpRequestMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpRequestMessage {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpRequestMessage {}
unsafe impl ::core::marker::Send for HttpRequestMessage {}
unsafe impl ::core::marker::Sync for HttpRequestMessage {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpRequestResult(::windows::core::IUnknown);
impl HttpRequestResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).ResponseMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestResult {}
impl ::core::fmt::Debug for HttpRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpRequestResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestResult;{6acf4da8-b5eb-4a35-a902-4217fbe820c5})");
}
impl ::core::clone::Clone for HttpRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpRequestResult {
    type Vtable = IHttpRequestResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpRequestResult {
    const IID: ::windows::core::GUID = <IHttpRequestResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpRequestResult {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestResult";
}
::windows::imp::interface_hierarchy!(HttpRequestResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpRequestResult {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpRequestResult {}
unsafe impl ::core::marker::Send for HttpRequestResult {}
unsafe impl ::core::marker::Sync for HttpRequestResult {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpResponseMessage(::windows::core::IUnknown);
impl HttpResponseMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpResponseMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows::core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IHttpContent>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IHttpContent>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpResponseHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpResponseHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSuccessStatusCode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSuccessStatusCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ReasonPhrase)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReasonPhrase(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReasonPhrase)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpRequestMessage>();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestMessage(&self, value: &HttpRequestMessage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<HttpResponseMessageSource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessageSource>();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSource(&self, value: HttpResponseMessageSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn StatusCode(&self) -> ::windows::core::Result<HttpStatusCode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpStatusCode>();
            (::windows::core::Interface::vtable(this).StatusCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatusCode(&self, value: HttpStatusCode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatusCode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> ::windows::core::Result<HttpVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpVersion>();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: HttpVersion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnsureSuccessStatusCode(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).EnsureSuccessStatusCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(statuscode: HttpStatusCode) -> ::windows::core::Result<HttpResponseMessage> {
        Self::IHttpResponseMessageFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpResponseMessage>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), statuscode, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpResponseMessageFactory<R, F: FnOnce(&IHttpResponseMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpResponseMessage, IHttpResponseMessageFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpResponseMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseMessage {}
impl ::core::fmt::Debug for HttpResponseMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpResponseMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpResponseMessage;{fee200fb-8664-44e0-95d9-42696199bffc})");
}
impl ::core::clone::Clone for HttpResponseMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpResponseMessage {
    type Vtable = IHttpResponseMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpResponseMessage {
    const IID: ::windows::core::GUID = <IHttpResponseMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpResponseMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpResponseMessage";
}
::windows::imp::interface_hierarchy!(HttpResponseMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpResponseMessage {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpResponseMessage {}
unsafe impl ::core::marker::Send for HttpResponseMessage {}
unsafe impl ::core::marker::Sync for HttpResponseMessage {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpStreamContent(::windows::core::IUnknown);
impl HttpStreamContent {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromInputStream<P0>(content: P0) -> ::windows::core::Result<HttpStreamContent>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        Self::IHttpStreamContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpStreamContent>();
            (::windows::core::Interface::vtable(this).CreateFromInputStream)(::windows::core::Interface::as_raw(this), content.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpStreamContentFactory<R, F: FnOnce(&IHttpStreamContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpStreamContent, IHttpStreamContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpStreamContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStreamContent {}
impl ::core::fmt::Debug for HttpStreamContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStreamContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpStreamContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStreamContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpStreamContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpStreamContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpStreamContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStreamContent";
}
::windows::imp::interface_hierarchy!(HttpStreamContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpStreamContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpStreamContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpStreamContent {}
unsafe impl ::core::marker::Send for HttpStreamContent {}
unsafe impl ::core::marker::Sync for HttpStreamContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpStringContent(::windows::core::IUnknown);
impl HttpStringContent {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http_Headers\"`*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Headers::HttpContentHeaderCollection>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).BufferAllAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsBufferAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>();
            (::windows::core::Interface::vtable(this).ReadAsStringAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryComputeLength)(::windows::core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<P0>(&self, outputstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>();
            (::windows::core::Interface::vtable(this).WriteToStreamAsync)(::windows::core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromString(content: &::windows::core::HSTRING) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpStringContent>();
            (::windows::core::Interface::vtable(this).CreateFromString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncoding(content: &::windows::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpStringContent>();
            (::windows::core::Interface::vtable(this).CreateFromStringWithEncoding)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), encoding, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncodingAndMediaType(content: &::windows::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HttpStringContent>();
            (::windows::core::Interface::vtable(this).CreateFromStringWithEncodingAndMediaType)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), encoding, ::core::mem::transmute_copy(mediatype), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpStringContentFactory<R, F: FnOnce(&IHttpStringContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HttpStringContent, IHttpStringContentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpStringContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStringContent {}
impl ::core::fmt::Debug for HttpStringContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStringContent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpStringContent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStringContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
impl ::core::clone::Clone for HttpStringContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpStringContent {
    type Vtable = IHttpContent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpStringContent {
    const IID: ::windows::core::GUID = <IHttpContent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpStringContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStringContent";
}
::windows::imp::interface_hierarchy!(HttpStringContent, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for HttpStringContent {}
impl ::windows::core::CanTryInto<IHttpContent> for HttpStringContent {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpStringContent {}
unsafe impl ::core::marker::Send for HttpStringContent {}
unsafe impl ::core::marker::Sync for HttpStringContent {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpTransportInformation(::windows::core::IUnknown);
impl HttpTransportInformation {
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Networking::Sockets::SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpTransportInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransportInformation {}
impl ::core::fmt::Debug for HttpTransportInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransportInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpTransportInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpTransportInformation;{70127198-c6a7-4ed0-833a-83fd8b8f178d})");
}
impl ::core::clone::Clone for HttpTransportInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HttpTransportInformation {
    type Vtable = IHttpTransportInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HttpTransportInformation {
    const IID: ::windows::core::GUID = <IHttpTransportInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HttpTransportInformation {
    const NAME: &'static str = "Windows.Web.Http.HttpTransportInformation";
}
::windows::imp::interface_hierarchy!(HttpTransportInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for HttpTransportInformation {}
unsafe impl ::core::marker::Send for HttpTransportInformation {}
unsafe impl ::core::marker::Sync for HttpTransportInformation {}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: Self = Self(0i32);
    pub const ResponseHeadersRead: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCompletionOption {}
impl ::core::clone::Clone for HttpCompletionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpCompletionOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpCompletionOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpCompletionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCompletionOption").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpCompletionOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpCompletionOption;i4)");
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl ::core::marker::Copy for HttpProgressStage {}
impl ::core::clone::Clone for HttpProgressStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpProgressStage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpProgressStage {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpProgressStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProgressStage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpProgressStage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpProgressStage;i4)");
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: Self = Self(0i32);
    pub const Cache: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for HttpResponseMessageSource {}
impl ::core::clone::Clone for HttpResponseMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpResponseMessageSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpResponseMessageSource {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpResponseMessageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseMessageSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpResponseMessageSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpResponseMessageSource;i4)");
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: Self = Self(0i32);
    pub const Continue: Self = Self(100i32);
    pub const SwitchingProtocols: Self = Self(101i32);
    pub const Processing: Self = Self(102i32);
    pub const Ok: Self = Self(200i32);
    pub const Created: Self = Self(201i32);
    pub const Accepted: Self = Self(202i32);
    pub const NonAuthoritativeInformation: Self = Self(203i32);
    pub const NoContent: Self = Self(204i32);
    pub const ResetContent: Self = Self(205i32);
    pub const PartialContent: Self = Self(206i32);
    pub const MultiStatus: Self = Self(207i32);
    pub const AlreadyReported: Self = Self(208i32);
    pub const IMUsed: Self = Self(226i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const PermanentRedirect: Self = Self(308i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const UnprocessableEntity: Self = Self(422i32);
    pub const Locked: Self = Self(423i32);
    pub const FailedDependency: Self = Self(424i32);
    pub const UpgradeRequired: Self = Self(426i32);
    pub const PreconditionRequired: Self = Self(428i32);
    pub const TooManyRequests: Self = Self(429i32);
    pub const RequestHeaderFieldsTooLarge: Self = Self(431i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
    pub const VariantAlsoNegotiates: Self = Self(506i32);
    pub const InsufficientStorage: Self = Self(507i32);
    pub const LoopDetected: Self = Self(508i32);
    pub const NotExtended: Self = Self(510i32);
    pub const NetworkAuthenticationRequired: Self = Self(511i32);
}
impl ::core::marker::Copy for HttpStatusCode {}
impl ::core::clone::Clone for HttpStatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpStatusCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpStatusCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpStatusCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStatusCode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpStatusCode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpStatusCode;i4)");
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: Self = Self(0i32);
    pub const Http10: Self = Self(1i32);
    pub const Http11: Self = Self(2i32);
    pub const Http20: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpVersion {}
impl ::core::clone::Clone for HttpVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpVersion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HttpVersion {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HttpVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpVersion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HttpVersion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpVersion;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Web_Http\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: ::core::option::Option<super::super::Foundation::IReference<u64>>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: ::core::option::Option<super::super::Foundation::IReference<u64>>,
    pub Retries: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for HttpProgress {
    fn clone(&self) -> Self {
        Self {
            Stage: self.Stage,
            BytesSent: self.BytesSent,
            TotalBytesToSend: self.TotalBytesToSend.clone(),
            BytesReceived: self.BytesReceived,
            TotalBytesToReceive: self.TotalBytesToReceive.clone(),
            Retries: self.Retries,
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for HttpProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HttpProgress").field("Stage", &self.Stage).field("BytesSent", &self.BytesSent).field("TotalBytesToSend", &self.TotalBytesToSend).field("BytesReceived", &self.BytesReceived).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("Retries", &self.Retries).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::TypeKind for HttpProgress {
    type TypeKind = ::windows::core::ValueType;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for HttpProgress {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Web.Http.HttpProgress;enum(Windows.Web.Http.HttpProgressStage;i4);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u4)");
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for HttpProgress {
    fn eq(&self, other: &Self) -> bool {
        self.Stage == other.Stage && self.BytesSent == other.BytesSent && self.TotalBytesToSend == other.TotalBytesToSend && self.BytesReceived == other.BytesReceived && self.TotalBytesToReceive == other.TotalBytesToReceive && self.Retries == other.Retries
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for HttpProgress {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for HttpProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
