pub trait ITpmVirtualSmartCardManagerImpl: Sized {
    fn CreateVirtualSmartCard();
    fn DestroyVirtualSmartCard();
}
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager {
    const NAME: &'static str = "Windows.Win32.Security.Tpm.ITpmVirtualSmartCardManager";
}
impl ITpmVirtualSmartCardManagerVtbl {
    pub const fn new<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITpmVirtualSmartCardManagerVtbl {
        unsafe extern "system" fn CreateVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSmartCard(
                &*(&pszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                badminalgid,
                pbadminkey,
                cbadminkey,
                pbadminkcv,
                cbadminkcv,
                pbpuk,
                cbpuk,
                pbpin,
                cbpin,
                &*(&fgenerate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pstatuscallback as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Abi>::Abi as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszinstanceid),
                ::core::mem::transmute_copy(&pfneedreboot),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Impl: ITpmVirtualSmartCardManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestroyVirtualSmartCard(&*(&pszinstanceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pstatuscallback as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Abi>::Abi as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfneedreboot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITpmVirtualSmartCardManager>, base.5, CreateVirtualSmartCard::<Impl, OFFSET>, DestroyVirtualSmartCard::<Impl, OFFSET>)
    }
}
pub trait ITpmVirtualSmartCardManager2Impl: Sized + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithPinPolicy();
}
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager2 {
    const NAME: &'static str = "Windows.Win32.Security.Tpm.ITpmVirtualSmartCardManager2";
}
impl ITpmVirtualSmartCardManager2Vtbl {
    pub const fn new<Impl: ITpmVirtualSmartCardManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITpmVirtualSmartCardManager2Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Impl: ITpmVirtualSmartCardManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSmartCardWithPinPolicy(
                &*(&pszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                badminalgid,
                pbadminkey,
                cbadminkey,
                pbadminkcv,
                cbadminkcv,
                pbpuk,
                cbpuk,
                pbpin,
                cbpin,
                pbpinpolicy,
                cbpinpolicy,
                &*(&fgenerate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pstatuscallback as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Abi>::Abi as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszinstanceid),
                ::core::mem::transmute_copy(&pfneedreboot),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITpmVirtualSmartCardManager2>, base.5, CreateVirtualSmartCardWithPinPolicy::<Impl, OFFSET>)
    }
}
pub trait ITpmVirtualSmartCardManager3Impl: Sized + ITpmVirtualSmartCardManager2Impl + ITpmVirtualSmartCardManagerImpl {
    fn CreateVirtualSmartCardWithAttestation();
}
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManager3 {
    const NAME: &'static str = "Windows.Win32.Security.Tpm.ITpmVirtualSmartCardManager3";
}
impl ITpmVirtualSmartCardManager3Vtbl {
    pub const fn new<Impl: ITpmVirtualSmartCardManager3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITpmVirtualSmartCardManager3Vtbl {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Impl: ITpmVirtualSmartCardManager3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSmartCardWithAttestation(
                &*(&pszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                badminalgid,
                pbadminkey,
                cbadminkey,
                pbadminkcv,
                cbadminkcv,
                pbpuk,
                cbpuk,
                pbpin,
                cbpin,
                pbpinpolicy,
                cbpinpolicy,
                attestationtype,
                &*(&fgenerate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pstatuscallback as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::Abi>::Abi as *const <ITpmVirtualSmartCardManagerStatusCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszinstanceid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITpmVirtualSmartCardManager3>, base.5, CreateVirtualSmartCardWithAttestation::<Impl, OFFSET>)
    }
}
pub trait ITpmVirtualSmartCardManagerStatusCallbackImpl: Sized {
    fn ReportProgress();
    fn ReportError();
}
impl ::windows::core::RuntimeName for ITpmVirtualSmartCardManagerStatusCallback {
    const NAME: &'static str = "Windows.Win32.Security.Tpm.ITpmVirtualSmartCardManagerStatusCallback";
}
impl ITpmVirtualSmartCardManagerStatusCallbackVtbl {
    pub const fn new<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITpmVirtualSmartCardManagerStatusCallbackVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportProgress(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportError<Impl: ITpmVirtualSmartCardManagerStatusCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportError(error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITpmVirtualSmartCardManagerStatusCallback>, base.5, ReportProgress::<Impl, OFFSET>, ReportError::<Impl, OFFSET>)
    }
}
