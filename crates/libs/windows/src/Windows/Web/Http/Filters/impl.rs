#[cfg(feature = "Foundation")]
pub trait IHttpFilter_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(&mut self, request: &::core::option::Option<super::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
#[cfg(feature = "Foundation")]
impl IHttpFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpFilter_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Impl: IHttpFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendRequestAsync(&*(&request as *const <super::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpFilter, BASE_OFFSET>(), SendRequestAsync: SendRequestAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpFilter as ::windows::core::Interface>::IID
    }
}
