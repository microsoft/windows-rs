#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundEnergyDiagnosticsStatics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundEnergyDiagnosticsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>() -> IBackgroundEnergyDiagnosticsStaticsVtbl {
        unsafe extern "system" fn DeviceSpecificConversionFactor<Impl: IBackgroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceSpecificConversionFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeTotalEnergyUsage<Impl: IBackgroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeTotalEnergyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetTotalEnergyUsage<Impl: IBackgroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTotalEnergyUsage().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundEnergyDiagnosticsStatics>, ::windows::core::GetTrustLevel, DeviceSpecificConversionFactor::<Impl, OFFSET>, ComputeTotalEnergyUsage::<Impl, OFFSET>, ResetTotalEnergyUsage::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IForegroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IForegroundEnergyDiagnosticsStatics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IForegroundEnergyDiagnosticsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForegroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>() -> IForegroundEnergyDiagnosticsStaticsVtbl {
        unsafe extern "system" fn DeviceSpecificConversionFactor<Impl: IForegroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceSpecificConversionFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeTotalEnergyUsage<Impl: IForegroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeTotalEnergyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetTotalEnergyUsage<Impl: IForegroundEnergyDiagnosticsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTotalEnergyUsage().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IForegroundEnergyDiagnosticsStatics>, ::windows::core::GetTrustLevel, DeviceSpecificConversionFactor::<Impl, OFFSET>, ComputeTotalEnergyUsage::<Impl, OFFSET>, ResetTotalEnergyUsage::<Impl, OFFSET>)
    }
}
