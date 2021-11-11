#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `System_Power_Diagnostics`*"]
pub struct BackgroundEnergyDiagnostics {}
impl BackgroundEnergyDiagnostics {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn DeviceSpecificConversionFactor() -> ::windows::core::Result<f64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ComputeTotalEnergyUsage() -> ::windows::core::Result<u64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ResetTotalEnergyUsage() -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IBackgroundEnergyDiagnosticsStatics<R, F: FnOnce(&IBackgroundEnergyDiagnosticsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundEnergyDiagnostics, IBackgroundEnergyDiagnosticsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BackgroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
}
#[doc = "*Required features: `System_Power_Diagnostics`*"]
pub struct ForegroundEnergyDiagnostics {}
impl ForegroundEnergyDiagnostics {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn DeviceSpecificConversionFactor() -> ::windows::core::Result<f64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ComputeTotalEnergyUsage() -> ::windows::core::Result<u64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `System_Power_Diagnostics`*"]
    pub fn ResetTotalEnergyUsage() -> ::windows::core::Result<()> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IForegroundEnergyDiagnosticsStatics<R, F: FnOnce(&IForegroundEnergyDiagnosticsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ForegroundEnergyDiagnostics, IForegroundEnergyDiagnosticsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ForegroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackgroundEnergyDiagnosticsStatics {
    type Vtable = IBackgroundEnergyDiagnosticsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7663702_d3a6_46e0_8f9b_50b95bb4f9c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IForegroundEnergyDiagnosticsStatics {
    type Vtable = IForegroundEnergyDiagnosticsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23ca0917_cd07_4609_be15_8fe894c5e41e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
