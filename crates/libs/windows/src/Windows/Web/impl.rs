pub trait IUriToStreamResolverImpl: Sized {
    fn UriToStreamAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
impl ::windows::core::RuntimeName for IUriToStreamResolver {
    const NAME: &'static str = "Windows.Web.IUriToStreamResolver";
}
impl IUriToStreamResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriToStreamResolverImpl, const OFFSET: isize>() -> IUriToStreamResolverVtbl {
        unsafe extern "system" fn UriToStreamAsync<Impl: IUriToStreamResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UriToStreamAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUriToStreamResolver>, ::windows::core::GetTrustLevel, UriToStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebErrorStatics {
    const NAME: &'static str = "Windows.Web.IWebErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebErrorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebErrorStaticsImpl, const OFFSET: isize>() -> IWebErrorStaticsVtbl {
        unsafe extern "system" fn GetStatus<Impl: IWebErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut WebErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebErrorStatics>, ::windows::core::GetTrustLevel, GetStatus::<Impl, OFFSET>)
    }
}
