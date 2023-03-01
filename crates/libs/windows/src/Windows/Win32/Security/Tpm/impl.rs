#[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager_Impl: Sized {
    fn CreateVirtualSmartCard(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyVirtualSmartCard(&self, pszinstanceid: &::windows::core::PCWSTR, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager {}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCard<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVirtualSmartCard(
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
                ::windows::core::from_raw_borrowed(&pstatuscallback),
                ::core::mem::transmute_copy(&ppszinstanceid),
                ::core::mem::transmute_copy(&pfneedreboot),
            )
            .into()
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceid: ::windows::core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestroyVirtualSmartCard(::core::mem::transmute(&pszinstanceid), ::windows::core::from_raw_borrowed(&pstatuscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfneedreboot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualSmartCard: CreateVirtualSmartCard::<Identity, Impl, OFFSET>,
            DestroyVirtualSmartCard: DestroyVirtualSmartCard::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager2_Impl: Sized + ITpmVirtualSmartCardManager_Impl {
    fn CreateVirtualSmartCardWithPinPolicy(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager2 {}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager2_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateVirtualSmartCardWithPinPolicy(
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
                ::windows::core::from_raw_borrowed(&pstatuscallback),
                ::core::mem::transmute_copy(&ppszinstanceid),
                ::core::mem::transmute_copy(&pfneedreboot),
            )
            .into()
        }
        Self {
            base__: ITpmVirtualSmartCardManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateVirtualSmartCardWithPinPolicy: CreateVirtualSmartCardWithPinPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::ComInterface>::IID || iid == &<ITpmVirtualSmartCardManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager3_Impl: Sized + ITpmVirtualSmartCardManager2_Impl {
    fn CreateVirtualSmartCardWithAttestation(&self, pszfriendlyname: &::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager3 {}
#[cfg(feature = "Win32_Foundation")]
impl ITpmVirtualSmartCardManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManager3_Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVirtualSmartCardWithAttestation(
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
                ::windows::core::from_raw_borrowed(&pstatuscallback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszinstanceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITpmVirtualSmartCardManager2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateVirtualSmartCardWithAttestation: CreateVirtualSmartCardWithAttestation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManager3 as ::windows::core::ComInterface>::IID || iid == &<ITpmVirtualSmartCardManager as ::windows::core::ComInterface>::IID || iid == &<ITpmVirtualSmartCardManager2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"implement\"`*"]
pub trait ITpmVirtualSmartCardManagerStatusCallback_Impl: Sized {
    fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()>;
    fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManagerStatusCallback {}
impl ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>() -> ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
        unsafe extern "system" fn ReportProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportProgress(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn ReportError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportError(::core::mem::transmute_copy(&error)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportProgress: ReportProgress::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::ComInterface>::IID
    }
}
