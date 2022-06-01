#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagExtensibleHelper_Impl: Sized {
    fn ResolveAttributes(&self, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetDiagExtensibleHelper {}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagExtensibleHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagExtensibleHelper_Impl, const OFFSET: isize>() -> INetDiagExtensibleHelper_Vtbl {
        unsafe extern "system" fn ResolveAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagExtensibleHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveAttributes(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgkeyattributes), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&prgmatchvalues)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ResolveAttributes: ResolveAttributes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagExtensibleHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelper_Impl: Sized {
    fn Initialize(&self, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn GetDiagnosticsInfo(&self) -> ::windows::core::Result<*mut DiagnosticsInfo>;
    fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn LowHealth(&self, pwszinstancedescription: &::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn HighUtilization(&self, pwszinstancedescription: &::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()>;
    fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()>;
    fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()>;
    fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::Result<()>;
    fn GetLifeTime(&self) -> ::windows::core::Result<LIFE_TIME>;
    fn SetLifeTime(&self, lifetime: &LIFE_TIME) -> ::windows::core::Result<()>;
    fn GetCacheTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Cleanup(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetDiagHelper {}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>() -> INetDiagHelper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgattributes)).into()
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDiagnosticsInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKeyAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into()
        }
        unsafe extern "system" fn LowHealth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LowHealth(::core::mem::transmute(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn HighUtilization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HighUtilization(::core::mem::transmute(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetLowerHypotheses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLowerHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDownStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetHigherHypotheses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHigherHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into()
        }
        unsafe extern "system" fn Repair<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Repair(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Validate(::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetRepairInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRepairInfo(::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&ppinfo)).into()
        }
        unsafe extern "system" fn GetLifeTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLifeTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plifetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLifeTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLifeTime(::core::mem::transmute(&lifetime)).into()
        }
        unsafe extern "system" fn GetCacheTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCacheTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcachetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn Cleanup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cleanup().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDiagnosticsInfo: GetDiagnosticsInfo::<Identity, Impl, OFFSET>,
            GetKeyAttributes: GetKeyAttributes::<Identity, Impl, OFFSET>,
            LowHealth: LowHealth::<Identity, Impl, OFFSET>,
            HighUtilization: HighUtilization::<Identity, Impl, OFFSET>,
            GetLowerHypotheses: GetLowerHypotheses::<Identity, Impl, OFFSET>,
            GetDownStreamHypotheses: GetDownStreamHypotheses::<Identity, Impl, OFFSET>,
            GetHigherHypotheses: GetHigherHypotheses::<Identity, Impl, OFFSET>,
            GetUpStreamHypotheses: GetUpStreamHypotheses::<Identity, Impl, OFFSET>,
            Repair: Repair::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            GetRepairInfo: GetRepairInfo::<Identity, Impl, OFFSET>,
            GetLifeTime: GetLifeTime::<Identity, Impl, OFFSET>,
            SetLifeTime: SetLifeTime::<Identity, Impl, OFFSET>,
            GetCacheTime: GetCacheTime::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Cleanup: Cleanup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperEx_Impl: Sized {
    fn ReconfirmLowHealth(&self, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows::core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>;
    fn SetUtilities(&self, putilities: &::core::option::Option<INetDiagHelperUtilFactory>) -> ::windows::core::Result<()>;
    fn ReproduceFailure(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetDiagHelperEx {}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: isize>() -> INetDiagHelperEx_Vtbl {
        unsafe extern "system" fn ReconfirmLowHealth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows::core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReconfirmLowHealth(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&presults), ::core::mem::transmute_copy(&ppwszupdateddescription), ::core::mem::transmute_copy(&pupdatedstatus)).into()
        }
        unsafe extern "system" fn SetUtilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, putilities: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUtilities(::core::mem::transmute(&putilities)).into()
        }
        unsafe extern "system" fn ReproduceFailure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReproduceFailure().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReconfirmLowHealth: ReconfirmLowHealth::<Identity, Impl, OFFSET>,
            SetUtilities: SetUtilities::<Identity, Impl, OFFSET>,
            ReproduceFailure: ReproduceFailure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperEx as ::windows::core::Interface>::IID
    }
}
pub trait INetDiagHelperInfo_Impl: Sized {
    fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetDiagHelperInfo {}
impl INetDiagHelperInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperInfo_Impl, const OFFSET: isize>() -> INetDiagHelperInfo_Vtbl {
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeInfo(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributeinfos)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperInfo as ::windows::core::Interface>::IID
    }
}
pub trait INetDiagHelperUtilFactory_Impl: Sized {
    fn CreateUtilityInstance(&self, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetDiagHelperUtilFactory {}
impl INetDiagHelperUtilFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperUtilFactory_Impl, const OFFSET: isize>() -> INetDiagHelperUtilFactory_Vtbl {
        unsafe extern "system" fn CreateUtilityInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetDiagHelperUtilFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateUtilityInstance(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateUtilityInstance: CreateUtilityInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperUtilFactory as ::windows::core::Interface>::IID
    }
}
