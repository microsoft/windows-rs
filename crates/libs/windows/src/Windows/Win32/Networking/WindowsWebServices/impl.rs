#[cfg(feature = "Win32_Foundation")]
pub trait IContentPrefetcherTaskTriggerImpl: Sized {
    fn TriggerContentPrefetcherTask();
    fn IsRegisteredForContentPrefetch();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContentPrefetcherTaskTrigger {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IContentPrefetcherTaskTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetcherTaskTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetcherTaskTriggerVtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Impl: IContentPrefetcherTaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: super::super::Foundation::PWSTR, isregistered: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentPrefetcherTaskTrigger>, ::windows::core::GetTrustLevel, TriggerContentPrefetcherTask::<Impl, IMPL_OFFSET>, IsRegisteredForContentPrefetch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetcherTaskTrigger as ::windows::core::Interface>::IID
    }
}
