#[cfg(feature = "Win32_Foundation")]
pub trait IContentPrefetcherTaskTrigger_Impl: Sized {
    fn TriggerContentPrefetcherTask(&mut self, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsRegisteredForContentPrefetch(&mut self, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContentPrefetcherTaskTrigger {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IContentPrefetcherTaskTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherTaskTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcherTaskTrigger_Vtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TriggerContentPrefetcherTask(::core::mem::transmute_copy(&packagefullname)).into()
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRegisteredForContentPrefetch(::core::mem::transmute_copy(&packagefullname)) {
                ::core::result::Result::Ok(ok__) => {
                    *isregistered = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPrefetcherTaskTrigger, BASE_OFFSET>(),
            TriggerContentPrefetcherTask: TriggerContentPrefetcherTask::<Impl, IMPL_OFFSET>,
            IsRegisteredForContentPrefetch: IsRegisteredForContentPrefetch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcherTaskTrigger as ::windows::core::Interface>::IID
    }
}
