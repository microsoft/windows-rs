pub trait IContentPrefetcherTaskTriggerImpl: Sized {
    fn TriggerContentPrefetcherTask();
    fn IsRegisteredForContentPrefetch();
}
impl ::windows::core::RuntimeName for IContentPrefetcherTaskTrigger {
    const NAME: &'static str = "Windows.Win32.Networking.WindowsWebServices.IContentPrefetcherTaskTrigger";
}
impl IContentPrefetcherTaskTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: isize>() -> IContentPrefetcherTaskTriggerVtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerContentPrefetcherTask(&*(&packagefullname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRegisteredForContentPrefetch(&*(&packagefullname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isregistered)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentPrefetcherTaskTrigger>, ::windows::core::GetTrustLevel, TriggerContentPrefetcherTask::<Impl, OFFSET>, IsRegisteredForContentPrefetch::<Impl, OFFSET>)
    }
}
