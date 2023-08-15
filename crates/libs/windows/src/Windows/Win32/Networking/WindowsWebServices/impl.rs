#[doc = "*Required features: `\"Win32_Networking_WindowsWebServices\"`, `\"implement\"`*"]
pub trait IContentPrefetcherTaskTrigger_Impl: Sized {
    fn TriggerContentPrefetcherTask(&self, packagefullname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsRegisteredForContentPrefetch(&self, packagefullname: &::windows_core::PCWSTR) -> ::windows_core::Result<u8>;
}
impl ::windows_core::RuntimeName for IContentPrefetcherTaskTrigger {}
impl IContentPrefetcherTaskTrigger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>() -> IContentPrefetcherTaskTrigger_Vtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TriggerContentPrefetcherTask(::core::mem::transmute(&packagefullname)).into()
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::windows_core::PCWSTR, isregistered: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRegisteredForContentPrefetch(::core::mem::transmute(&packagefullname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isregistered, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IContentPrefetcherTaskTrigger, OFFSET>(),
            TriggerContentPrefetcherTask: TriggerContentPrefetcherTask::<Identity, Impl, OFFSET>,
            IsRegisteredForContentPrefetch: IsRegisteredForContentPrefetch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContentPrefetcherTaskTrigger as ::windows_core::ComInterface>::IID
    }
}
