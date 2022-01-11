#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagExtensibleHelperImpl: Sized {
    fn ResolveAttributes();
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagExtensibleHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagExtensibleHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagExtensibleHelperVtbl {
        unsafe extern "system" fn ResolveAttributes<Impl: INetDiagExtensibleHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ResolveAttributes::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagExtensibleHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperImpl: Sized {
    fn Initialize();
    fn GetDiagnosticsInfo();
    fn GetKeyAttributes();
    fn LowHealth();
    fn HighUtilization();
    fn GetLowerHypotheses();
    fn GetDownStreamHypotheses();
    fn GetHigherHypotheses();
    fn GetUpStreamHypotheses();
    fn Repair();
    fn Validate();
    fn GetRepairInfo();
    fn GetLifeTime();
    fn SetLifeTime();
    fn GetCacheTime();
    fn GetAttributes();
    fn Cancel();
    fn Cleanup();
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyAttributes<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LowHealth<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HighUtilization<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLowerHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHigherHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Repair<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRepairInfo<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLifeTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLifeTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCacheTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributes<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cleanup<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            GetDiagnosticsInfo::<Impl, IMPL_OFFSET>,
            GetKeyAttributes::<Impl, IMPL_OFFSET>,
            LowHealth::<Impl, IMPL_OFFSET>,
            HighUtilization::<Impl, IMPL_OFFSET>,
            GetLowerHypotheses::<Impl, IMPL_OFFSET>,
            GetDownStreamHypotheses::<Impl, IMPL_OFFSET>,
            GetHigherHypotheses::<Impl, IMPL_OFFSET>,
            GetUpStreamHypotheses::<Impl, IMPL_OFFSET>,
            Repair::<Impl, IMPL_OFFSET>,
            Validate::<Impl, IMPL_OFFSET>,
            GetRepairInfo::<Impl, IMPL_OFFSET>,
            GetLifeTime::<Impl, IMPL_OFFSET>,
            SetLifeTime::<Impl, IMPL_OFFSET>,
            GetCacheTime::<Impl, IMPL_OFFSET>,
            GetAttributes::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Cleanup::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperExImpl: Sized {
    fn ReconfirmLowHealth();
    fn SetUtilities();
    fn ReproduceFailure();
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperExVtbl {
        unsafe extern "system" fn ReconfirmLowHealth<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUtilities<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, putilities: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReproduceFailure<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReconfirmLowHealth::<Impl, IMPL_OFFSET>, SetUtilities::<Impl, IMPL_OFFSET>, ReproduceFailure::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperInfoImpl: Sized {
    fn GetAttributeInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl INetDiagHelperInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperInfoVtbl {
        unsafe extern "system" fn GetAttributeInfo<Impl: INetDiagHelperInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAttributeInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperInfo as ::windows::core::Interface>::IID
    }
}
pub trait INetDiagHelperUtilFactoryImpl: Sized {
    fn CreateUtilityInstance();
}
impl INetDiagHelperUtilFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperUtilFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetDiagHelperUtilFactoryVtbl {
        unsafe extern "system" fn CreateUtilityInstance<Impl: INetDiagHelperUtilFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateUtilityInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetDiagHelperUtilFactory as ::windows::core::Interface>::IID
    }
}
