#[cfg(feature = "Foundation")]
pub trait IHttpFilter_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
#[cfg(feature = "Foundation")]
impl IHttpFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpFilter_Impl, const OFFSET: isize>() -> IHttpFilter_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendRequestAsync(::core::mem::transmute(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpFilter, OFFSET>(), SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpFilter as ::windows::core::Interface>::IID
    }
}
