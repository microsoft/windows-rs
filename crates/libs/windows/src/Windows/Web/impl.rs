#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IUriToStreamResolver_Impl: Sized {
    fn UriToStreamAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IUriToStreamResolver {
    const NAME: &'static str = "Windows.Web.IUriToStreamResolver";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IUriToStreamResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriToStreamResolver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriToStreamResolver_Vtbl {
        unsafe extern "system" fn UriToStreamAsync<Impl: IUriToStreamResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriToStreamResolver, BASE_OFFSET>(),
            UriToStreamAsync: UriToStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriToStreamResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebErrorStatics_Impl: Sized {
    fn GetStatus(&mut self, hresult: i32) -> ::windows::core::Result<WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebErrorStatics {
    const NAME: &'static str = "Windows.Web.IWebErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebErrorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebErrorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebErrorStatics_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IWebErrorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut WebErrorStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebErrorStatics, BASE_OFFSET>(), GetStatus: GetStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebErrorStatics as ::windows::core::Interface>::IID
    }
}
