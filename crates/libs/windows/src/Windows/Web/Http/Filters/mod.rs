#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilter_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71c89b09_e131_4b54_a53c_eb43ff37e9bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowAutoRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowAutoRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutomaticDecompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticDecompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CookieManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
    pub MaxConnectionsPerServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxConnectionsPerServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    pub UseProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUseProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilter2 {
    type Vtable = IHttpBaseProtocolFilter2_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ec30013_9427_4900_a017_fa7da3b5c9ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::HttpVersion) -> ::windows_core::HRESULT,
    pub SetMaxVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::HttpVersion) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilter3 {
    type Vtable = IHttpBaseProtocolFilter3_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilter3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd43f4d4c_bd42_43ae_8717_ad2c8f4b2937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CookieUsageBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCookieUsageBehavior) -> ::windows_core::HRESULT,
    pub SetCookieUsageBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCookieUsageBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilter4 {
    type Vtable = IHttpBaseProtocolFilter4_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilter4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilter4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fe36ccf_2983_4893_941f_eb518ca8cef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
    pub ClearAuthenticationCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilter5 {
    type Vtable = IHttpBaseProtocolFilter5_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilter5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilter5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x416e4993_31e3_4816_bf09_e018ee8dc1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBaseProtocolFilterStatics {
    type Vtable = IHttpBaseProtocolFilterStatics_Vtbl;
}
impl ::core::clone::Clone for IHttpBaseProtocolFilterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpBaseProtocolFilterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d4dee0c_e908_494e_b5a3_1263c9b8242a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCacheControl {
    type Vtable = IHttpCacheControl_Vtbl;
}
impl ::core::clone::Clone for IHttpCacheControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpCacheControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc77e1cb4_3cea_4eb5_ac85_04e186e63ab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReadBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheReadBehavior) -> ::windows_core::HRESULT,
    pub SetReadBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheReadBehavior) -> ::windows_core::HRESULT,
    pub WriteBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheWriteBehavior) -> ::windows_core::HRESULT,
    pub SetWriteBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheWriteBehavior) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct IHttpFilter(::windows_core::IUnknown);
impl IHttpFilter {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<P0>(&self, request: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: ::windows_core::IntoParam<super::HttpRequestMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IHttpFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for IHttpFilter {}
impl ::core::cmp::PartialEq for IHttpFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpFilter {}
impl ::core::fmt::Debug for IHttpFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpFilter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IHttpFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a4cb6dd5-0902-439e-bfd7-e12552b165ce}");
}
unsafe impl ::windows_core::Interface for IHttpFilter {
    type Vtable = IHttpFilter_Vtbl;
}
impl ::core::clone::Clone for IHttpFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4cb6dd5_0902_439e_bfd7_e12552b165ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHttpServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHttpServerCustomValidationRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3165fe32_e7dd_48b7_a361_939c750e63cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpServerCustomValidationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ServerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(::windows_core::IUnknown);
impl HttpBaseProtocolFilter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HttpBaseProtocolFilter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AllowAutoRedirect(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowAutoRedirect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowAutoRedirect(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowAutoRedirect)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUI(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowUI(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowUI)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticDecompression(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticDecompression)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutomaticDecompression(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticDecompression)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheControl(&self) -> ::windows_core::Result<HttpCacheControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CacheControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CookieManager(&self) -> ::windows_core::Result<super::HttpCookieManager> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CookieManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows_core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientCertificate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClientCertificate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxConnectionsPerServer(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxConnectionsPerServer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxConnectionsPerServer(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxConnectionsPerServer)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
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
        P0: ::windows_core::IntoParam<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
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
        P0: ::windows_core::IntoParam<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UseProxy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UseProxy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUseProxy(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUseProxy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVersion(&self) -> ::windows_core::Result<super::HttpVersion> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVersion(&self, value: super::HttpVersion) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CookieUsageBehavior(&self) -> ::windows_core::Result<HttpCookieUsageBehavior> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CookieUsageBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCookieUsageBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCustomValidationRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServerCustomValidationRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ClearAuthenticationCache(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ClearAuthenticationCache)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IHttpBaseProtocolFilter5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> ::windows_core::Result<HttpBaseProtocolFilter>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IHttpBaseProtocolFilterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<P0>(&self, request: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: ::windows_core::IntoParam<super::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IHttpFilter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpBaseProtocolFilterStatics<R, F: FnOnce(&IHttpBaseProtocolFilterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HttpBaseProtocolFilter, IHttpBaseProtocolFilterStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HttpBaseProtocolFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpBaseProtocolFilter {}
impl ::core::fmt::Debug for HttpBaseProtocolFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpBaseProtocolFilter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpBaseProtocolFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpBaseProtocolFilter;{71c89b09-e131-4b54-a53c-eb43ff37e9bb})");
}
impl ::core::clone::Clone for HttpBaseProtocolFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HttpBaseProtocolFilter {
    const IID: ::windows_core::GUID = <IHttpBaseProtocolFilter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HttpBaseProtocolFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpBaseProtocolFilter";
}
::windows_core::imp::interface_hierarchy!(HttpBaseProtocolFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for HttpBaseProtocolFilter {}
impl ::windows_core::CanTryInto<IHttpFilter> for HttpBaseProtocolFilter {}
unsafe impl ::core::marker::Send for HttpBaseProtocolFilter {}
unsafe impl ::core::marker::Sync for HttpBaseProtocolFilter {}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpCacheControl(::windows_core::IUnknown);
impl HttpCacheControl {
    pub fn ReadBehavior(&self) -> ::windows_core::Result<HttpCacheReadBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReadBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBehavior(&self) -> ::windows_core::Result<HttpCacheWriteBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWriteBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for HttpCacheControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheControl {}
impl ::core::fmt::Debug for HttpCacheControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpCacheControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpCacheControl;{c77e1cb4-3cea-4eb5-ac85-04e186e63ab7})");
}
impl ::core::clone::Clone for HttpCacheControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HttpCacheControl {
    type Vtable = IHttpCacheControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HttpCacheControl {
    const IID: ::windows_core::GUID = <IHttpCacheControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HttpCacheControl {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpCacheControl";
}
::windows_core::imp::interface_hierarchy!(HttpCacheControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for HttpCacheControl {}
unsafe impl ::core::marker::Sync for HttpCacheControl {}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(::windows_core::IUnknown);
impl HttpServerCustomValidationRequestedEventArgs {
    pub fn RequestMessage(&self) -> ::windows_core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows_core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows_core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificateErrors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerIntermediateCertificates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reject(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reject)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HttpServerCustomValidationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpServerCustomValidationRequestedEventArgs {}
impl ::core::fmt::Debug for HttpServerCustomValidationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpServerCustomValidationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpServerCustomValidationRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs;{3165fe32-e7dd-48b7-a361-939c750e63cc})");
}
impl ::core::clone::Clone for HttpServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HttpServerCustomValidationRequestedEventArgs {
    const IID: ::windows_core::GUID = <IHttpServerCustomValidationRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HttpServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(HttpServerCustomValidationRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for HttpServerCustomValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for HttpServerCustomValidationRequestedEventArgs {}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: Self = Self(0i32);
    pub const MostRecent: Self = Self(1i32);
    pub const OnlyFromCache: Self = Self(2i32);
    pub const NoCache: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpCacheReadBehavior {}
impl ::core::clone::Clone for HttpCacheReadBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpCacheReadBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HttpCacheReadBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HttpCacheReadBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheReadBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpCacheReadBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheReadBehavior;i4)");
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCache: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCacheWriteBehavior {}
impl ::core::clone::Clone for HttpCacheWriteBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpCacheWriteBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HttpCacheWriteBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HttpCacheWriteBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheWriteBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpCacheWriteBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheWriteBehavior;i4)");
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCookies: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCookieUsageBehavior {}
impl ::core::clone::Clone for HttpCookieUsageBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpCookieUsageBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HttpCookieUsageBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HttpCookieUsageBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieUsageBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HttpCookieUsageBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCookieUsageBehavior;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
