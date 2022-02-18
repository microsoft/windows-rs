#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager_Impl: Sized {
    fn CreateVirtualSmartCard(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: &::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyVirtualSmartCard(&self, pszinstanceid: &::windows::core::PCWSTR, pstatuscallback: &::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCard<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .CreateVirtualSmartCard(
                    ::core::mem::transmute(&pszfriendlyname),
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
        unsafe extern "system" fn DestroyVirtualSmartCard<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceid: ::windows::core::PCWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestroyVirtualSmartCard(::core::mem::transmute(&pszinstanceid), ::core::mem::transmute(&pstatuscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfneedreboot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateVirtualSmartCard: CreateVirtualSmartCard::<Identity, Impl, OFFSET>,
            DestroyVirtualSmartCard: DestroyVirtualSmartCard::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager2_Impl: Sized + ITpmVirtualSmartCardManager_Impl {
    fn CreateVirtualSmartCardWithPinPolicy(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: &::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager2_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .CreateVirtualSmartCardWithPinPolicy(
                    ::core::mem::transmute(&pszfriendlyname),
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
            base: ITpmVirtualSmartCardManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateVirtualSmartCardWithPinPolicy: CreateVirtualSmartCardWithPinPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager3_Impl: Sized + ITpmVirtualSmartCardManager_Impl + ITpmVirtualSmartCardManager2_Impl {
    fn CreateVirtualSmartCardWithAttestation(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: &::core::option::Option<ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager3_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVirtualSmartCardWithAttestation(
                ::core::mem::transmute(&pszfriendlyname),
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
            base: ITpmVirtualSmartCardManager2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateVirtualSmartCardWithAttestation: CreateVirtualSmartCardWithAttestation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager3 as ::windows::core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager as ::windows::core::Interface>::IID || iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::Interface>::IID
    }
}
pub trait ITpmVirtualSmartCardManagerStatusCallback_Impl: Sized {
    fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()>;
    fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()>;
}
impl ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
        unsafe extern "system" fn ReportProgress<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportProgress(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn ReportError<Identity: ::windows::core::IUnknownImpl, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportError(::core::mem::transmute_copy(&error)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReportProgress: ReportProgress::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Interface>::IID
    }
}
