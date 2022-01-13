#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&mut self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&mut self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundEnergyDiagnosticsStatics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundEnergyDiagnosticsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundEnergyDiagnosticsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundEnergyDiagnosticsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundEnergyDiagnosticsStatics, BASE_OFFSET>(),
            DeviceSpecificConversionFactor: DeviceSpecificConversionFactor::<Impl, IMPL_OFFSET>,
            ComputeTotalEnergyUsage: ComputeTotalEnergyUsage::<Impl, IMPL_OFFSET>,
            ResetTotalEnergyUsage: ResetTotalEnergyUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundEnergyDiagnosticsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IForegroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&mut self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&mut self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IForegroundEnergyDiagnosticsStatics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IForegroundEnergyDiagnosticsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForegroundEnergyDiagnosticsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForegroundEnergyDiagnosticsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IForegroundEnergyDiagnosticsStatics, BASE_OFFSET>(),
            DeviceSpecificConversionFactor: DeviceSpecificConversionFactor::<Impl, IMPL_OFFSET>,
            ComputeTotalEnergyUsage: ComputeTotalEnergyUsage::<Impl, IMPL_OFFSET>,
            ResetTotalEnergyUsage: ResetTotalEnergyUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForegroundEnergyDiagnosticsStatics as ::windows::core::Interface>::IID
    }
}
