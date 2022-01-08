pub trait INetDiagExtensibleHelperImpl: Sized {
    fn ResolveAttributes();
}
impl ::windows::core::RuntimeName for INetDiagExtensibleHelper {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetworkDiagnosticsFramework.INetDiagExtensibleHelper";
}
impl INetDiagExtensibleHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagExtensibleHelperImpl, const OFFSET: isize>() -> INetDiagExtensibleHelperVtbl {
        unsafe extern "system" fn ResolveAttributes<Impl: INetDiagExtensibleHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveAttributes(celt, &*(&rgkeyattributes as *const <HELPER_ATTRIBUTE as ::windows::core::Abi>::Abi as *const <HELPER_ATTRIBUTE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&prgmatchvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetDiagExtensibleHelper>, ::windows::core::GetTrustLevel, ResolveAttributes::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for INetDiagHelper {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetworkDiagnosticsFramework.INetDiagHelper";
}
impl INetDiagHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperImpl, const OFFSET: isize>() -> INetDiagHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(celt, &*(&rgattributes as *const <HELPER_ATTRIBUTE as ::windows::core::Abi>::Abi as *const <HELPER_ATTRIBUTE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiagnosticsInfo(::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyAttributes<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowHealth<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowHealth(&*(&pwszinstancedescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighUtilization<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighUtilization(&*(&pwszinstancedescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLowerHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDownStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHigherHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHigherHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpStreamHypotheses(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Repair<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Repair(&*(&pinfo as *const <RepairInfo as ::windows::core::Abi>::Abi as *const <RepairInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate(problem, ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRepairInfo<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRepairInfo(problem, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLifeTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLifeTime(::core::mem::transmute_copy(&plifetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLifeTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLifeTime(&*(&lifetime as *const <LIFE_TIME as ::windows::core::Abi>::Abi as *const <LIFE_TIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCacheTime<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCacheTime(::core::mem::transmute_copy(&pcachetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cleanup<Impl: INetDiagHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cleanup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetDiagHelper>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetDiagnosticsInfo::<Impl, OFFSET>,
            GetKeyAttributes::<Impl, OFFSET>,
            LowHealth::<Impl, OFFSET>,
            HighUtilization::<Impl, OFFSET>,
            GetLowerHypotheses::<Impl, OFFSET>,
            GetDownStreamHypotheses::<Impl, OFFSET>,
            GetHigherHypotheses::<Impl, OFFSET>,
            GetUpStreamHypotheses::<Impl, OFFSET>,
            Repair::<Impl, OFFSET>,
            Validate::<Impl, OFFSET>,
            GetRepairInfo::<Impl, OFFSET>,
            GetLifeTime::<Impl, OFFSET>,
            SetLifeTime::<Impl, OFFSET>,
            GetCacheTime::<Impl, OFFSET>,
            GetAttributes::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Cleanup::<Impl, OFFSET>,
        )
    }
}
pub trait INetDiagHelperExImpl: Sized {
    fn ReconfirmLowHealth();
    fn SetUtilities();
    fn ReproduceFailure();
}
impl ::windows::core::RuntimeName for INetDiagHelperEx {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetworkDiagnosticsFramework.INetDiagHelperEx";
}
impl INetDiagHelperExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperExImpl, const OFFSET: isize>() -> INetDiagHelperExVtbl {
        unsafe extern "system" fn ReconfirmLowHealth<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReconfirmLowHealth(celt, &*(&presults as *const <HypothesisResult as ::windows::core::Abi>::Abi as *const <HypothesisResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszupdateddescription), ::core::mem::transmute_copy(&pupdatedstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUtilities<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, putilities: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUtilities(&*(&putilities as *const <INetDiagHelperUtilFactory as ::windows::core::Abi>::Abi as *const <INetDiagHelperUtilFactory as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReproduceFailure<Impl: INetDiagHelperExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReproduceFailure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetDiagHelperEx>, ::windows::core::GetTrustLevel, ReconfirmLowHealth::<Impl, OFFSET>, SetUtilities::<Impl, OFFSET>, ReproduceFailure::<Impl, OFFSET>)
    }
}
pub trait INetDiagHelperInfoImpl: Sized {
    fn GetAttributeInfo();
}
impl ::windows::core::RuntimeName for INetDiagHelperInfo {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetworkDiagnosticsFramework.INetDiagHelperInfo";
}
impl INetDiagHelperInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperInfoImpl, const OFFSET: isize>() -> INetDiagHelperInfoVtbl {
        unsafe extern "system" fn GetAttributeInfo<Impl: INetDiagHelperInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeInfo(::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributeinfos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetDiagHelperInfo>, ::windows::core::GetTrustLevel, GetAttributeInfo::<Impl, OFFSET>)
    }
}
pub trait INetDiagHelperUtilFactoryImpl: Sized {
    fn CreateUtilityInstance();
}
impl ::windows::core::RuntimeName for INetDiagHelperUtilFactory {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetworkDiagnosticsFramework.INetDiagHelperUtilFactory";
}
impl INetDiagHelperUtilFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetDiagHelperUtilFactoryImpl, const OFFSET: isize>() -> INetDiagHelperUtilFactoryVtbl {
        unsafe extern "system" fn CreateUtilityInstance<Impl: INetDiagHelperUtilFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUtilityInstance(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetDiagHelperUtilFactory>, ::windows::core::GetTrustLevel, CreateUtilityInstance::<Impl, OFFSET>)
    }
}
