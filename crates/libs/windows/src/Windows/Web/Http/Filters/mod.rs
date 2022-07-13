#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(::windows::core::IUnknown);
impl HttpBaseProtocolFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpBaseProtocolFilter, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn AllowAutoRedirect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllowAutoRedirect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowAutoRedirect(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowAutoRedirect)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUI(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllowUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUI(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowUI)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticDecompression(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AutomaticDecompression)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticDecompression(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutomaticDecompression)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CacheControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCacheControl>(result__)
        }
    }
    pub fn CookieManager(&self) -> ::windows::core::Result<super::HttpCookieManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CookieManager)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HttpCookieManager>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ClientCertificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Security::Cryptography::Certificates::Certificate>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClientCertificate)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    pub fn MaxConnectionsPerServer(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxConnectionsPerServer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxConnectionsPerServer(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxConnectionsPerServer)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Security::Credentials::PasswordCredential>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerCredential)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Security::Credentials::PasswordCredential>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn UseProxy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UseProxy)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseProxy(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUseProxy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVersion(&self) -> ::windows::core::Result<super::HttpVersion> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HttpVersion>(result__)
        }
    }
    pub fn SetMaxVersion(&self, value: super::HttpVersion) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxVersion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CookieUsageBehavior(&self) -> ::windows::core::Result<HttpCookieUsageBehavior> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CookieUsageBehavior)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCookieUsageBehavior>(result__)
        }
    }
    pub fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCookieUsageBehavior)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn ClearAuthenticationCache(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ClearAuthenticationCache)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IHttpBaseProtocolFilter5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<'a, P0>(user: P0) -> ::windows::core::Result<HttpBaseProtocolFilter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::User>>,
    {
        Self::IHttpBaseProtocolFilterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateForUser)(::windows::core::Interface::as_raw(this), user.into().abi(), result__.as_mut_ptr()).from_abi::<HttpBaseProtocolFilter>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, P0>(&self, request: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::HttpRequestMessage>>,
    {
        let this = &::windows::core::Interface::cast::<IHttpFilter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SendRequestAsync)(::windows::core::Interface::as_raw(this), request.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpBaseProtocolFilterStatics<R, F: FnOnce(&IHttpBaseProtocolFilterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpBaseProtocolFilter, IHttpBaseProtocolFilterStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilter_Vtbl;
    const IID: ::windows::core::GUID = <IHttpBaseProtocolFilter as ::windows::core::Interface>::IID;
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
impl ::core::convert::From<&HttpBaseProtocolFilter> for &::windows::core::IUnknown {
    fn from(value: &HttpBaseProtocolFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&HttpBaseProtocolFilter> for &::windows::core::IInspectable {
    fn from(value: &HttpBaseProtocolFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&HttpBaseProtocolFilter> for ::windows::core::InParam<'a, super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&HttpBaseProtocolFilter> for ::windows::core::InParam<'a, IHttpFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBaseProtocolFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpBaseProtocolFilter {}
unsafe impl ::core::marker::Sync for HttpBaseProtocolFilter {}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpCacheControl(::windows::core::IUnknown);
impl HttpCacheControl {
    pub fn ReadBehavior(&self) -> ::windows::core::Result<HttpCacheReadBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReadBehavior)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCacheReadBehavior>(result__)
        }
    }
    pub fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReadBehavior)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBehavior(&self) -> ::windows::core::Result<HttpCacheWriteBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WriteBehavior)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCacheWriteBehavior>(result__)
        }
    }
    pub fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWriteBehavior)(::windows::core::Interface::as_raw(this), value).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpCacheControl {
    type Vtable = IHttpCacheControl_Vtbl;
    const IID: ::windows::core::GUID = <IHttpCacheControl as ::windows::core::Interface>::IID;
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
impl ::core::convert::From<&HttpCacheControl> for &::windows::core::IUnknown {
    fn from(value: &HttpCacheControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&HttpCacheControl> for &::windows::core::IInspectable {
    fn from(value: &HttpCacheControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for HttpCacheControl {}
unsafe impl ::core::marker::Sync for HttpCacheControl {}
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
unsafe impl ::windows::core::Abi for HttpCacheReadBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpCacheReadBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheReadBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCacheReadBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheReadBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for HttpCacheWriteBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpCacheWriteBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheWriteBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCacheWriteBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheWriteBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for HttpCookieUsageBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpCookieUsageBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieUsageBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCookieUsageBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCookieUsageBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
impl HttpServerCustomValidationRequestedEventArgs {
    pub fn RequestMessage(&self) -> ::windows::core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestMessage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reject)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHttpServerCustomValidationRequestedEventArgs as ::windows::core::Interface>::IID;
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
impl ::core::convert::From<&HttpServerCustomValidationRequestedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &HttpServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&HttpServerCustomValidationRequestedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &HttpServerCustomValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for HttpServerCustomValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for HttpServerCustomValidationRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c89b09_e131_4b54_a53c_eb43ff37e9bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AllowAutoRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowAutoRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutomaticDecompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutomaticDecompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CookieManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
    pub MaxConnectionsPerServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxConnectionsPerServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    pub UseProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter2 {
    type Vtable = IHttpBaseProtocolFilter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec30013_9427_4900_a017_fa7da3b5c9ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MaxVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::HttpVersion) -> ::windows::core::HRESULT,
    pub SetMaxVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::HttpVersion) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter3 {
    type Vtable = IHttpBaseProtocolFilter3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd43f4d4c_bd42_43ae_8717_ad2c8f4b2937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CookieUsageBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCookieUsageBehavior) -> ::windows::core::HRESULT,
    pub SetCookieUsageBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCookieUsageBehavior) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter4 {
    type Vtable = IHttpBaseProtocolFilter4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fe36ccf_2983_4893_941f_eb518ca8cef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
    pub ClearAuthenticationCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilter5 {
    type Vtable = IHttpBaseProtocolFilter5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x416e4993_31e3_4816_bf09_e018ee8dc1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilter5_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBaseProtocolFilterStatics {
    type Vtable = IHttpBaseProtocolFilterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4dee0c_e908_494e_b5a3_1263c9b8242a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBaseProtocolFilterStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCacheControl {
    type Vtable = IHttpCacheControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77e1cb4_3cea_4eb5_ac85_04e186e63ab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ReadBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheReadBehavior) -> ::windows::core::HRESULT,
    pub SetReadBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheReadBehavior) -> ::windows::core::HRESULT,
    pub WriteBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheWriteBehavior) -> ::windows::core::HRESULT,
    pub SetWriteBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpCacheWriteBehavior) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct IHttpFilter(::windows::core::IUnknown);
impl IHttpFilter {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, P0>(&self, request: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::HttpRequestMessage>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SendRequestAsync)(::windows::core::Interface::as_raw(this), request.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IHttpFilter> for ::windows::core::IUnknown {
    fn from(value: IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHttpFilter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpFilter> for ::windows::core::IUnknown {
    fn from(value: &IHttpFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHttpFilter> for ::windows::core::IInspectable {
    fn from(value: IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHttpFilter> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHttpFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpFilter> for ::windows::core::IInspectable {
    fn from(value: &IHttpFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IHttpFilter> for ::windows::core::InParam<'a, super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IHttpFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IHttpFilter {
    type Vtable = IHttpFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4cb6dd5_0902_439e_bfd7_e12552b165ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFilter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3165fe32_e7dd_48b7_a361_939c750e63cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpServerCustomValidationRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT,
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
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
