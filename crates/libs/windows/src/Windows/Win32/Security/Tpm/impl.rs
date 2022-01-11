#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManagerImpl: Sized {
    fn CreateVirtualSmartCard();
    fn DestroyVirtualSmartCard();
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManagerVtbl {
        unsafe extern "system" fn CreateVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateVirtualSmartCard::<Impl, IMPL_OFFSET>, DestroyVirtualSmartCard::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager2Impl: Sized + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithPinPolicy();
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManager2Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Impl: ITpmVirtualSmartCardManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateVirtualSmartCard::<Impl, IMPL_OFFSET>, DestroyVirtualSmartCard::<Impl, IMPL_OFFSET>, CreateVirtualSmartCardWithPinPolicy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager3Impl: Sized + ITpmVirtualSmartCardManager2Impl + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithAttestation();
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManager3Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Impl: ITpmVirtualSmartCardManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateVirtualSmartCard::<Impl, IMPL_OFFSET>, DestroyVirtualSmartCard::<Impl, IMPL_OFFSET>, CreateVirtualSmartCardWithPinPolicy::<Impl, IMPL_OFFSET>, CreateVirtualSmartCardWithAttestation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager3 as ::windows::core::Interface>::IID
    }
}
pub trait ITpmVirtualSmartCardManagerStatusCallbackImpl: Sized {
    fn ReportProgress();
    fn ReportError();
}
impl ITpmVirtualSmartCardManagerStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManagerStatusCallbackVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportError<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReportProgress::<Impl, IMPL_OFFSET>, ReportError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Interface>::IID
    }
}
