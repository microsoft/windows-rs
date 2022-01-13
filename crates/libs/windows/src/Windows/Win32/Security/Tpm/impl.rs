#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManagerImpl: Sized {
    fn CreateVirtualSmartCard(&mut self, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyVirtualSmartCard(&mut self, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManagerVtbl {
        unsafe extern "system" fn CreateVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateVirtualSmartCard(
                    ::core::mem::transmute_copy(&pszfriendlyname),
                    ::core::mem::transmute_copy(&badminalgid),
                    ::core::mem::transmute_copy(&pbadminkey),
                    ::core::mem::transmute_copy(&cbadminkey),
                    ::core::mem::transmute_copy(&pbadminkcv),
                    ::core::mem::transmute_copy(&cbadminkcv),
                    ::core::mem::transmute_copy(&pbpuk),
                    ::core::mem::transmute_copy(&cbpuk),
                    ::core::mem::transmute_copy(&pbpin),
                    ::core::mem::transmute_copy(&cbpin),
                    ::core::mem::transmute_copy(&fgenerate),
                    ::core::mem::transmute(&pstatuscallback),
                    ::core::mem::transmute_copy(&ppszinstanceid),
                    ::core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyVirtualSmartCard(::core::mem::transmute_copy(&pszinstanceid), ::core::mem::transmute(&pstatuscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfneedreboot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateVirtualSmartCard: CreateVirtualSmartCard::<Impl, IMPL_OFFSET>,
            DestroyVirtualSmartCard: DestroyVirtualSmartCard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager2Impl: Sized + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithPinPolicy(&mut self, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManager2Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Impl: ITpmVirtualSmartCardManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateVirtualSmartCardWithPinPolicy(
                    ::core::mem::transmute_copy(&pszfriendlyname),
                    ::core::mem::transmute_copy(&badminalgid),
                    ::core::mem::transmute_copy(&pbadminkey),
                    ::core::mem::transmute_copy(&cbadminkey),
                    ::core::mem::transmute_copy(&pbadminkcv),
                    ::core::mem::transmute_copy(&cbadminkcv),
                    ::core::mem::transmute_copy(&pbpuk),
                    ::core::mem::transmute_copy(&cbpuk),
                    ::core::mem::transmute_copy(&pbpin),
                    ::core::mem::transmute_copy(&cbpin),
                    ::core::mem::transmute_copy(&pbpinpolicy),
                    ::core::mem::transmute_copy(&cbpinpolicy),
                    ::core::mem::transmute_copy(&fgenerate),
                    ::core::mem::transmute(&pstatuscallback),
                    ::core::mem::transmute_copy(&ppszinstanceid),
                    ::core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
        }
        Self {
            base: ITpmVirtualSmartCardManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVirtualSmartCardWithPinPolicy: CreateVirtualSmartCardWithPinPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager3Impl: Sized + ITpmVirtualSmartCardManagerImpl + ITpmVirtualSmartCardManager2Impl {
    fn CreateVirtualSmartCardWithAttestation(&mut self, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManager3Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Impl: ITpmVirtualSmartCardManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSmartCardWithAttestation(
                ::core::mem::transmute_copy(&pszfriendlyname),
                ::core::mem::transmute_copy(&badminalgid),
                ::core::mem::transmute_copy(&pbadminkey),
                ::core::mem::transmute_copy(&cbadminkey),
                ::core::mem::transmute_copy(&pbadminkcv),
                ::core::mem::transmute_copy(&cbadminkcv),
                ::core::mem::transmute_copy(&pbpuk),
                ::core::mem::transmute_copy(&cbpuk),
                ::core::mem::transmute_copy(&pbpin),
                ::core::mem::transmute_copy(&cbpin),
                ::core::mem::transmute_copy(&pbpinpolicy),
                ::core::mem::transmute_copy(&cbpinpolicy),
                ::core::mem::transmute_copy(&attestationtype),
                ::core::mem::transmute_copy(&fgenerate),
                ::core::mem::transmute(&pstatuscallback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITpmVirtualSmartCardManager2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVirtualSmartCardWithAttestation: CreateVirtualSmartCardWithAttestation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager3 as ::windows::core::Interface>::IID
    }
}
pub trait ITpmVirtualSmartCardManagerStatusCallbackImpl: Sized {
    fn ReportProgress(&mut self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()>;
    fn ReportError(&mut self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()>;
}
impl ITpmVirtualSmartCardManagerStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITpmVirtualSmartCardManagerStatusCallbackVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProgress(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn ReportError<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(::core::mem::transmute_copy(&error)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReportProgress: ReportProgress::<Impl, IMPL_OFFSET>,
            ReportError: ReportError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Interface>::IID
    }
}
