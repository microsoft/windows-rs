#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Web_AtomPub")]
pub mod AtomPub;
#[cfg(feature = "Web_Http")]
pub mod Http;
#[cfg(feature = "Web_Syndication")]
pub mod Syndication;
#[cfg(feature = "Web_UI")]
pub mod UI;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Web`*"]
pub struct IUriToStreamResolver(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriToStreamResolver {
    type Vtable = IUriToStreamResolver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2964039786, 39659, 19770, [149, 144, 0, 62, 60, 167, 226, 144]);
}
impl IUriToStreamResolver {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Web`, `Foundation`, `Storage_Streams`*"]
    pub fn UriToStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUriToStreamResolver {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b0aba86a-9aeb-4d3a-9590-003e3ca7e290}");
}
impl ::std::convert::From<IUriToStreamResolver> for ::windows::runtime::IUnknown {
    fn from(value: IUriToStreamResolver) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUriToStreamResolver> for ::windows::runtime::IUnknown {
    fn from(value: &IUriToStreamResolver) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUriToStreamResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUriToStreamResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IUriToStreamResolver> for ::windows::runtime::IInspectable {
    fn from(value: IUriToStreamResolver) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IUriToStreamResolver> for ::windows::runtime::IInspectable {
    fn from(value: &IUriToStreamResolver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IUriToStreamResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IUriToStreamResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriToStreamResolver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebErrorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebErrorStatics {
    type Vtable = IWebErrorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4267796326, 48935, 16484, [135, 183, 101, 99, 187, 17, 206, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebErrorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: i32, result__: *mut WebErrorStatus) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Web`*"]
pub struct WebError {}
impl WebError {
    #[doc = "*Required features: `Web`*"]
    pub fn GetStatus(hresult: i32) -> ::windows::runtime::Result<WebErrorStatus> {
        Self::IWebErrorStatics(|this| unsafe {
            let mut result__: WebErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), hresult, &mut result__).from_abi::<WebErrorStatus>(result__)
        })
    }
    pub fn IWebErrorStatics<R, F: FnOnce(&IWebErrorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebError, IWebErrorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebError {
    const NAME: &'static str = "Windows.Web.WebError";
}
#[doc = "*Required features: `Web`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebErrorStatus(pub i32);
impl WebErrorStatus {
    pub const Unknown: WebErrorStatus = WebErrorStatus(0i32);
    pub const CertificateCommonNameIsIncorrect: WebErrorStatus = WebErrorStatus(1i32);
    pub const CertificateExpired: WebErrorStatus = WebErrorStatus(2i32);
    pub const CertificateContainsErrors: WebErrorStatus = WebErrorStatus(3i32);
    pub const CertificateRevoked: WebErrorStatus = WebErrorStatus(4i32);
    pub const CertificateIsInvalid: WebErrorStatus = WebErrorStatus(5i32);
    pub const ServerUnreachable: WebErrorStatus = WebErrorStatus(6i32);
    pub const Timeout: WebErrorStatus = WebErrorStatus(7i32);
    pub const ErrorHttpInvalidServerResponse: WebErrorStatus = WebErrorStatus(8i32);
    pub const ConnectionAborted: WebErrorStatus = WebErrorStatus(9i32);
    pub const ConnectionReset: WebErrorStatus = WebErrorStatus(10i32);
    pub const Disconnected: WebErrorStatus = WebErrorStatus(11i32);
    pub const HttpToHttpsOnRedirection: WebErrorStatus = WebErrorStatus(12i32);
    pub const HttpsToHttpOnRedirection: WebErrorStatus = WebErrorStatus(13i32);
    pub const CannotConnect: WebErrorStatus = WebErrorStatus(14i32);
    pub const HostNameNotResolved: WebErrorStatus = WebErrorStatus(15i32);
    pub const OperationCanceled: WebErrorStatus = WebErrorStatus(16i32);
    pub const RedirectFailed: WebErrorStatus = WebErrorStatus(17i32);
    pub const UnexpectedStatusCode: WebErrorStatus = WebErrorStatus(18i32);
    pub const UnexpectedRedirection: WebErrorStatus = WebErrorStatus(19i32);
    pub const UnexpectedClientError: WebErrorStatus = WebErrorStatus(20i32);
    pub const UnexpectedServerError: WebErrorStatus = WebErrorStatus(21i32);
    pub const InsufficientRangeSupport: WebErrorStatus = WebErrorStatus(22i32);
    pub const MissingContentLengthSupport: WebErrorStatus = WebErrorStatus(23i32);
    pub const MultipleChoices: WebErrorStatus = WebErrorStatus(300i32);
    pub const MovedPermanently: WebErrorStatus = WebErrorStatus(301i32);
    pub const Found: WebErrorStatus = WebErrorStatus(302i32);
    pub const SeeOther: WebErrorStatus = WebErrorStatus(303i32);
    pub const NotModified: WebErrorStatus = WebErrorStatus(304i32);
    pub const UseProxy: WebErrorStatus = WebErrorStatus(305i32);
    pub const TemporaryRedirect: WebErrorStatus = WebErrorStatus(307i32);
    pub const BadRequest: WebErrorStatus = WebErrorStatus(400i32);
    pub const Unauthorized: WebErrorStatus = WebErrorStatus(401i32);
    pub const PaymentRequired: WebErrorStatus = WebErrorStatus(402i32);
    pub const Forbidden: WebErrorStatus = WebErrorStatus(403i32);
    pub const NotFound: WebErrorStatus = WebErrorStatus(404i32);
    pub const MethodNotAllowed: WebErrorStatus = WebErrorStatus(405i32);
    pub const NotAcceptable: WebErrorStatus = WebErrorStatus(406i32);
    pub const ProxyAuthenticationRequired: WebErrorStatus = WebErrorStatus(407i32);
    pub const RequestTimeout: WebErrorStatus = WebErrorStatus(408i32);
    pub const Conflict: WebErrorStatus = WebErrorStatus(409i32);
    pub const Gone: WebErrorStatus = WebErrorStatus(410i32);
    pub const LengthRequired: WebErrorStatus = WebErrorStatus(411i32);
    pub const PreconditionFailed: WebErrorStatus = WebErrorStatus(412i32);
    pub const RequestEntityTooLarge: WebErrorStatus = WebErrorStatus(413i32);
    pub const RequestUriTooLong: WebErrorStatus = WebErrorStatus(414i32);
    pub const UnsupportedMediaType: WebErrorStatus = WebErrorStatus(415i32);
    pub const RequestedRangeNotSatisfiable: WebErrorStatus = WebErrorStatus(416i32);
    pub const ExpectationFailed: WebErrorStatus = WebErrorStatus(417i32);
    pub const InternalServerError: WebErrorStatus = WebErrorStatus(500i32);
    pub const NotImplemented: WebErrorStatus = WebErrorStatus(501i32);
    pub const BadGateway: WebErrorStatus = WebErrorStatus(502i32);
    pub const ServiceUnavailable: WebErrorStatus = WebErrorStatus(503i32);
    pub const GatewayTimeout: WebErrorStatus = WebErrorStatus(504i32);
    pub const HttpVersionNotSupported: WebErrorStatus = WebErrorStatus(505i32);
}
impl ::std::convert::From<i32> for WebErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebErrorStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebErrorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Web.WebErrorStatus;i4)");
}
