pub trait IContentPrefetcherTaskTriggerImpl: Sized {
    fn TriggerContentPrefetcherTask();
    fn IsRegisteredForContentPrefetch();
}
impl ::windows::core::RuntimeName for IContentPrefetcherTaskTrigger {
    const NAME: &'static str = "Windows.Win32.Networking.WindowsWebServices.IContentPrefetcherTaskTrigger";
}
impl IContentPrefetcherTaskTriggerVtbl {
    pub const fn new<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentPrefetcherTaskTriggerVtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerContentPrefetcherTask(&*(&packagefullname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRegisteredForContentPrefetch(&*(&packagefullname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isregistered)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentPrefetcherTaskTrigger>, base.5, TriggerContentPrefetcherTask::<Impl, OFFSET>, IsRegisteredForContentPrefetch::<Impl, OFFSET>)
    }
}
