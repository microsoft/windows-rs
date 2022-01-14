#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagExtensibleHelper_Impl: Sized {
    fn ResolveAttributes(&mut self, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagExtensibleHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagExtensibleHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagExtensibleHelper_Vtbl {
        unsafe extern "system" fn ResolveAttributes<Impl: INetDiagExtensibleHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveAttributes(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgkeyattributes), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&prgmatchvalues)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ResolveAttributes: ResolveAttributes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagExtensibleHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelper_Impl: Sized {
    fn Initialize(&mut self, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn GetDiagnosticsInfo(&mut self) -> ::windows::core::Result<*mut DiagnosticsInfo>;
    fn GetKeyAttributes(&mut self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn LowHealth(&mut self, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn HighUtilization(&mut self, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn GetLowerHypotheses(&mut self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetDownStreamHypotheses(&mut self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetHigherHypotheses(&mut self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetUpStreamHypotheses(&mut self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn Repair(&mut self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()>;
    fn Validate(&mut self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()>;
    fn GetRepairInfo(&mut self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::Result<()>;
    fn GetLifeTime(&mut self) -> ::windows::core::Result<LIFE_TIME>;
    fn SetLifeTime(&mut self, lifetime: &LIFE_TIME) -> ::windows::core::Result<()>;
    fn GetCacheTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetAttributes(&mut self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Cleanup(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelper_Vtbl {
        unsafe extern "system" fn Initialize<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgattributes)).into()
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiagnosticsInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyAttributes<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into()
        }
        unsafe extern "system" fn LowHealth<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LowHealth(::core::mem::transmute_copy(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn HighUtilization<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HighUtilization(::core::mem::transmute_copy(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetLowerHypotheses<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLowerHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDownStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetHigherHypotheses<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHigherHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn Repair<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Repair(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn Validate<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate(::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetRepairInfo<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRepairInfo(::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&ppinfo)).into()
        }
        unsafe extern "system" fn GetLifeTime<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLifeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plifetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLifeTime<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLifeTime(::core::mem::transmute_copy(&lifetime)).into()
        }
        unsafe extern "system" fn GetCacheTime<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCacheTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcachetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into()
        }
        unsafe extern "system" fn Cancel<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Cleanup<Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cleanup().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetDiagnosticsInfo: GetDiagnosticsInfo::<Impl, IMPL_OFFSET>,
            GetKeyAttributes: GetKeyAttributes::<Impl, IMPL_OFFSET>,
            LowHealth: LowHealth::<Impl, IMPL_OFFSET>,
            HighUtilization: HighUtilization::<Impl, IMPL_OFFSET>,
            GetLowerHypotheses: GetLowerHypotheses::<Impl, IMPL_OFFSET>,
            GetDownStreamHypotheses: GetDownStreamHypotheses::<Impl, IMPL_OFFSET>,
            GetHigherHypotheses: GetHigherHypotheses::<Impl, IMPL_OFFSET>,
            GetUpStreamHypotheses: GetUpStreamHypotheses::<Impl, IMPL_OFFSET>,
            Repair: Repair::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
            GetRepairInfo: GetRepairInfo::<Impl, IMPL_OFFSET>,
            GetLifeTime: GetLifeTime::<Impl, IMPL_OFFSET>,
            SetLifeTime: SetLifeTime::<Impl, IMPL_OFFSET>,
            GetCacheTime: GetCacheTime::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Cleanup: Cleanup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperEx_Impl: Sized {
    fn ReconfirmLowHealth(&mut self, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn SetUtilities(&mut self, putilities: &::core::option::Option<INetDiagHelperUtilFactory>) -> ::windows::core::Result<()>;
    fn ReproduceFailure(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperEx_Vtbl {
        unsafe extern "system" fn ReconfirmLowHealth<Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReconfirmLowHealth(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&presults), ::core::mem::transmute_copy(&ppwszupdateddescription), ::core::mem::transmute_copy(&pupdatedstatus)).into()
        }
        unsafe extern "system" fn SetUtilities<Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, putilities: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUtilities(::core::mem::transmute(&putilities)).into()
        }
        unsafe extern "system" fn ReproduceFailure<Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReproduceFailure().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReconfirmLowHealth: ReconfirmLowHealth::<Impl, IMPL_OFFSET>,
            SetUtilities: SetUtilities::<Impl, IMPL_OFFSET>,
            ReproduceFailure: ReproduceFailure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperInfo_Impl: Sized {
    fn GetAttributeInfo(&mut self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperInfo_Vtbl {
        unsafe extern "system" fn GetAttributeInfo<Impl: INetDiagHelperInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeInfo(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributeinfos)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetAttributeInfo: GetAttributeInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperInfo as ::windows::core::Interface>::IID
    }
}
pub trait INetDiagHelperUtilFactory_Impl: Sized {
    fn CreateUtilityInstance(&mut self, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl INetDiagHelperUtilFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperUtilFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperUtilFactory_Vtbl {
        unsafe extern "system" fn CreateUtilityInstance<Impl: INetDiagHelperUtilFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateUtilityInstance(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateUtilityInstance: CreateUtilityInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperUtilFactory as ::windows::core::Interface>::IID
    }
}
