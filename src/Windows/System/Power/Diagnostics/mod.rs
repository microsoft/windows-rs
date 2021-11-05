#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `System_Power_Diagnostics`*"]
pub struct BackgroundEnergyDiagnostics {}
impl BackgroundEnergyDiagnostics {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn DeviceSpecificConversionFactor() -> ::windows::runtime::Result<f64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ComputeTotalEnergyUsage() -> ::windows::runtime::Result<u64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ResetTotalEnergyUsage() -> ::windows::runtime::Result<()> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn IBackgroundEnergyDiagnosticsStatics<R, F: FnOnce(&IBackgroundEnergyDiagnosticsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundEnergyDiagnostics, IBackgroundEnergyDiagnosticsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BackgroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
}
#[doc = "*Required features: `System_Power_Diagnostics`*"]
pub struct ForegroundEnergyDiagnostics {}
impl ForegroundEnergyDiagnostics {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn DeviceSpecificConversionFactor() -> ::windows::runtime::Result<f64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ComputeTotalEnergyUsage() -> ::windows::runtime::Result<u64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ResetTotalEnergyUsage() -> ::windows::runtime::Result<()> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn IForegroundEnergyDiagnosticsStatics<R, F: FnOnce(&IForegroundEnergyDiagnosticsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ForegroundEnergyDiagnostics, IForegroundEnergyDiagnosticsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ForegroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundEnergyDiagnosticsStatics {
    type Vtable = IBackgroundEnergyDiagnosticsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3613800194, 54182, 18144, [143, 155, 80, 185, 91, 180, 249, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IForegroundEnergyDiagnosticsStatics {
    type Vtable = IForegroundEnergyDiagnosticsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(600443159, 52487, 17929, [190, 21, 143, 232, 148, 197, 228, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
