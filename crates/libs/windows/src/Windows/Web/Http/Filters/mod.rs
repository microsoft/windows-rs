#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(::windows::core::IUnknown);
impl HttpBaseProtocolFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpBaseProtocolFilter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn AllowAutoRedirect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetAllowAutoRedirect(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn AllowUI(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetAllowUI(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn AutomaticDecompression(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetAutomaticDecompression(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheControl>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn CookieManager(&self) -> ::windows::core::Result<super::HttpCookieManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpCookieManager>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn MaxConnectionsPerServer(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetMaxConnectionsPerServer(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn UseProxy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetUseProxy(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn MaxVersion(&self) -> ::windows::core::Result<super::HttpVersion> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe {
            let mut result__: super::HttpVersion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpVersion>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetMaxVersion(&self, value: super::HttpVersion) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn CookieUsageBehavior(&self) -> ::windows::core::Result<HttpCookieUsageBehavior> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe {
            let mut result__: HttpCookieUsageBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCookieUsageBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn ClearAuthenticationCache(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'System'*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<HttpBaseProtocolFilter> {
        Self::IHttpBaseProtocolFilterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<HttpBaseProtocolFilter>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpFilter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpBaseProtocolFilterStatics<R, F: FnOnce(&IHttpBaseProtocolFilterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpBaseProtocolFilter, IHttpBaseProtocolFilterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpBaseProtocolFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpBaseProtocolFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpBaseProtocolFilter;{71c89b09-e131-4b54-a53c-eb43ff37e9bb})");
}
unsafe impl ::windows::core::Interface for HttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c89b09_e131_4b54_a53c_eb43ff37e9bb);
}
impl ::windows::core::RuntimeName for HttpBaseProtocolFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpBaseProtocolFilter";
}
impl ::core::convert::From<HttpBaseProtocolFilter> for ::windows::core::IUnknown {
    fn from(value: HttpBaseProtocolFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBaseProtocolFilter> for ::windows::core::IUnknown {
    fn from(value: &HttpBaseProtocolFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpBaseProtocolFilter> for ::windows::core::IInspectable {
    fn from(value: HttpBaseProtocolFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBaseProtocolFilter> for ::windows::core::IInspectable {
    fn from(value: &HttpBaseProtocolFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpBaseProtocolFilter> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpBaseProtocolFilter> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpBaseProtocolFilter> for IHttpFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpBaseProtocolFilter> for IHttpFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpFilter> for HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpFilter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpFilter> for &HttpBaseProtocolFilter {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpFilter> {
        ::core::convert::TryInto::<IHttpFilter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpBaseProtocolFilter {}
unsafe impl ::core::marker::Sync for HttpBaseProtocolFilter {}
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
pub struct HttpCacheControl(::windows::core::IUnknown);
impl HttpCacheControl {
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn ReadBehavior(&self) -> ::windows::core::Result<HttpCacheReadBehavior> {
        let this = self;
        unsafe {
            let mut result__: HttpCacheReadBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheReadBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn WriteBehavior(&self) -> ::windows::core::Result<HttpCacheWriteBehavior> {
        let this = self;
        unsafe {
            let mut result__: HttpCacheWriteBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheWriteBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for HttpCacheControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpCacheControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpCacheControl;{c77e1cb4-3cea-4eb5-ac85-04e186e63ab7})");
}
unsafe impl ::windows::core::Interface for HttpCacheControl {
    type Vtable = IHttpCacheControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77e1cb4_3cea_4eb5_ac85_04e186e63ab7);
}
impl ::windows::core::RuntimeName for HttpCacheControl {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpCacheControl";
}
impl ::core::convert::From<HttpCacheControl> for ::windows::core::IUnknown {
    fn from(value: HttpCacheControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheControl> for ::windows::core::IUnknown {
    fn from(value: &HttpCacheControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCacheControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCacheControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCacheControl> for ::windows::core::IInspectable {
    fn from(value: HttpCacheControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheControl> for ::windows::core::IInspectable {
    fn from(value: &HttpCacheControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCacheControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCacheControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpCacheControl {}
unsafe impl ::core::marker::Sync for HttpCacheControl {}
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for HttpCacheReadBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpCacheReadBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheReadBehavior {}
impl ::core::fmt::Debug for HttpCacheReadBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheReadBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCacheReadBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheReadBehavior;i4)");
}
impl ::windows::core::DefaultType for HttpCacheReadBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for HttpCacheWriteBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpCacheWriteBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheWriteBehavior {}
impl ::core::fmt::Debug for HttpCacheWriteBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheWriteBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCacheWriteBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheWriteBehavior;i4)");
}
impl ::windows::core::DefaultType for HttpCacheWriteBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for HttpCookieUsageBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpCookieUsageBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookieUsageBehavior {}
impl ::core::fmt::Debug for HttpCookieUsageBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieUsageBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCookieUsageBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCookieUsageBehavior;i4)");
}
impl ::windows::core::DefaultType for HttpCookieUsageBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
impl HttpServerCustomValidationRequestedEventArgs {
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Networking_Sockets'*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Networking::Sockets::SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters'*"]
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for HttpServerCustomValidationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs;{3165fe32-e7dd-48b7-a361-939c750e63cc})");
}
unsafe impl ::windows::core::Interface for HttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3165fe32_e7dd_48b7_a361_939c750e63cc);
}
impl ::windows::core::RuntimeName for HttpServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs";
}
impl ::core::convert::From<HttpServerCustomValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HttpServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpServerCustomValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HttpServerCustomValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpServerCustomValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HttpServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpServerCustomValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HttpServerCustomValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpServerCustomValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpServerCustomValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for HttpServerCustomValidationRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c89b09_e131_4b54_a53c_eb43ff37e9bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter2 {
    type Vtable = IHttpBaseProtocolFilter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec30013_9427_4900_a017_fa7da3b5c9ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::HttpVersion) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::HttpVersion) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter3 {
    type Vtable = IHttpBaseProtocolFilter3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd43f4d4c_bd42_43ae_8717_ad2c8f4b2937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCookieUsageBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCookieUsageBehavior) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter4 {
    type Vtable = IHttpBaseProtocolFilter4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fe36ccf_2983_4893_941f_eb518ca8cef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter5 {
    type Vtable = IHttpBaseProtocolFilter5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x416e4993_31e3_4816_bf09_e018ee8dc1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilterStatics {
    type Vtable = IHttpBaseProtocolFilterStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4dee0c_e908_494e_b5a3_1263c9b8242a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilterStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCacheControl {
    type Vtable = IHttpCacheControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77e1cb4_3cea_4eb5_ac85_04e186e63ab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheReadBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheReadBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheWriteBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheWriteBehavior) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Web_Http_Filters'*"]
#[repr(transparent)]
pub struct IHttpFilter(::windows::core::IUnknown);
impl IHttpFilter {
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, super::HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Filters', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IHttpFilter> for ::windows::core::IInspectable {
    fn from(value: IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpFilter> for ::windows::core::IInspectable {
    fn from(value: &IHttpFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHttpFilter> for ::windows::core::IUnknown {
    fn from(value: IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpFilter> for ::windows::core::IUnknown {
    fn from(value: &IHttpFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IHttpFilter> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IHttpFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IHttpFilter> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IHttpFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IHttpFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IHttpFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IHttpFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a4cb6dd5-0902-439e-bfd7-e12552b165ce}");
}
unsafe impl ::windows::core::Interface for IHttpFilter {
    type Vtable = IHttpFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4cb6dd5_0902_439e_bfd7_e12552b165ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFilterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3165fe32_e7dd_48b7_a361_939c750e63cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpServerCustomValidationRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
