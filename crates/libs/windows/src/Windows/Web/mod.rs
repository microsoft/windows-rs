#[cfg(feature = "Web_AtomPub")]
pub mod AtomPub;
#[cfg(feature = "Web_Http")]
pub mod Http;
#[cfg(feature = "Web_Syndication")]
pub mod Syndication;
#[cfg(feature = "Web_UI")]
pub mod UI;
windows_core::imp::define_interface!(IUriToStreamResolver, IUriToStreamResolver_Vtbl, 0xb0aba86a_9aeb_4d3a_9590_003e3ca7e290);
impl windows_core::RuntimeType for IUriToStreamResolver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Web.IUriToStreamResolver");
}
windows_core::imp::interface_hierarchy!(IUriToStreamResolver, windows_core::IUnknown, windows_core::IInspectable);
impl IUriToStreamResolver {
    #[cfg(feature = "Storage_Streams")]
    pub fn UriToStreamAsync<P0>(&self, uri: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::Storage::Streams::IInputStream>>
    where
        P0: windows_core::Param<super::Foundation::Uri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UriToStreamAsync)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IUriToStreamResolver {
    const NAME: &'static str = "Windows.Web.IUriToStreamResolver";
}
#[cfg(feature = "Storage_Streams")]
pub trait IUriToStreamResolver_Impl: windows_core::IUnknownImpl {
    fn UriToStreamAsync(&self, uri: windows_core::Ref<super::Foundation::Uri>) -> windows_core::Result<windows_future::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(feature = "Storage_Streams")]
impl IUriToStreamResolver_Vtbl {
    pub const fn new<Identity: IUriToStreamResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UriToStreamAsync<Identity: IUriToStreamResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriToStreamResolver_Impl::UriToStreamAsync(this, core::mem::transmute_copy(&uri)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUriToStreamResolver, OFFSET>(), UriToStreamAsync: UriToStreamAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriToStreamResolver as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriToStreamResolver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub UriToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UriToStreamAsync: usize,
}
windows_core::imp::define_interface!(IWebErrorStatics, IWebErrorStatics_Vtbl, 0xfe616766_bf27_4064_87b7_6563bb11ce2e);
impl windows_core::RuntimeType for IWebErrorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Web.IWebErrorStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebErrorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut WebErrorStatus) -> windows_core::HRESULT,
}
pub struct WebError;
impl WebError {
    pub fn GetStatus(hresult: i32) -> windows_core::Result<WebErrorStatus> {
        Self::IWebErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    fn IWebErrorStatics<R, F: FnOnce(&IWebErrorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebError, IWebErrorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebError {
    const NAME: &'static str = "Windows.Web.WebError";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebErrorStatus(pub i32);
impl WebErrorStatus {
    pub const Unknown: Self = Self(0);
    pub const CertificateCommonNameIsIncorrect: Self = Self(1);
    pub const CertificateExpired: Self = Self(2);
    pub const CertificateContainsErrors: Self = Self(3);
    pub const CertificateRevoked: Self = Self(4);
    pub const CertificateIsInvalid: Self = Self(5);
    pub const ServerUnreachable: Self = Self(6);
    pub const Timeout: Self = Self(7);
    pub const ErrorHttpInvalidServerResponse: Self = Self(8);
    pub const ConnectionAborted: Self = Self(9);
    pub const ConnectionReset: Self = Self(10);
    pub const Disconnected: Self = Self(11);
    pub const HttpToHttpsOnRedirection: Self = Self(12);
    pub const HttpsToHttpOnRedirection: Self = Self(13);
    pub const CannotConnect: Self = Self(14);
    pub const HostNameNotResolved: Self = Self(15);
    pub const OperationCanceled: Self = Self(16);
    pub const RedirectFailed: Self = Self(17);
    pub const UnexpectedStatusCode: Self = Self(18);
    pub const UnexpectedRedirection: Self = Self(19);
    pub const UnexpectedClientError: Self = Self(20);
    pub const UnexpectedServerError: Self = Self(21);
    pub const InsufficientRangeSupport: Self = Self(22);
    pub const MissingContentLengthSupport: Self = Self(23);
    pub const MultipleChoices: Self = Self(300);
    pub const MovedPermanently: Self = Self(301);
    pub const Found: Self = Self(302);
    pub const SeeOther: Self = Self(303);
    pub const NotModified: Self = Self(304);
    pub const UseProxy: Self = Self(305);
    pub const TemporaryRedirect: Self = Self(307);
    pub const BadRequest: Self = Self(400);
    pub const Unauthorized: Self = Self(401);
    pub const PaymentRequired: Self = Self(402);
    pub const Forbidden: Self = Self(403);
    pub const NotFound: Self = Self(404);
    pub const MethodNotAllowed: Self = Self(405);
    pub const NotAcceptable: Self = Self(406);
    pub const ProxyAuthenticationRequired: Self = Self(407);
    pub const RequestTimeout: Self = Self(408);
    pub const Conflict: Self = Self(409);
    pub const Gone: Self = Self(410);
    pub const LengthRequired: Self = Self(411);
    pub const PreconditionFailed: Self = Self(412);
    pub const RequestEntityTooLarge: Self = Self(413);
    pub const RequestUriTooLong: Self = Self(414);
    pub const UnsupportedMediaType: Self = Self(415);
    pub const RequestedRangeNotSatisfiable: Self = Self(416);
    pub const ExpectationFailed: Self = Self(417);
    pub const InternalServerError: Self = Self(500);
    pub const NotImplemented: Self = Self(501);
    pub const BadGateway: Self = Self(502);
    pub const ServiceUnavailable: Self = Self(503);
    pub const GatewayTimeout: Self = Self(504);
    pub const HttpVersionNotSupported: Self = Self(505);
}
impl windows_core::TypeKind for WebErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.WebErrorStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Web.WebErrorStatus");
}
