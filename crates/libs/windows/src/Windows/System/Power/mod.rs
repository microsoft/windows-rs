#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[doc = "*Required features: 'System_Power', 'deprecated'*"]
#[cfg(feature = "deprecated")]
pub struct BackgroundEnergyManager {}
#[cfg(feature = "deprecated")]
impl BackgroundEnergyManager {
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn NearTerminationUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn TerminationUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundEnergyManagerStatics<R, F: FnOnce(&IBackgroundEnergyManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundEnergyManager, IBackgroundEnergyManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for BackgroundEnergyManager {
    const NAME: &'static str = "Windows.System.Power.BackgroundEnergyManager";
}
#[doc = "*Required features: 'System_Power'*"]
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl ::core::marker::Copy for BatteryStatus {}
impl ::core::clone::Clone for BatteryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BatteryStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BatteryStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BatteryStatus {}
impl ::core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.BatteryStatus;i4)");
}
impl ::windows::core::DefaultType for BatteryStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'System_Power'*"]
#[repr(transparent)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Disabled: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for EnergySaverStatus {}
impl ::core::clone::Clone for EnergySaverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EnergySaverStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EnergySaverStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnergySaverStatus {}
impl ::core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.EnergySaverStatus;i4)");
}
impl ::windows::core::DefaultType for EnergySaverStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'System_Power', 'deprecated'*"]
#[cfg(feature = "deprecated")]
pub struct ForegroundEnergyManager {}
#[cfg(feature = "deprecated")]
impl ForegroundEnergyManager {
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IForegroundEnergyManagerStatics<R, F: FnOnce(&IForegroundEnergyManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ForegroundEnergyManager, IForegroundEnergyManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ForegroundEnergyManager {
    const NAME: &'static str = "Windows.System.Power.ForegroundEnergyManager";
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundEnergyManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IBackgroundEnergyManagerStatics {
    type Vtable = IBackgroundEnergyManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3161d95_1180_4376_96e1_4095568147ce);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IForegroundEnergyManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IForegroundEnergyManagerStatics {
    type Vtable = IForegroundEnergyManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff86872_e677_4814_9a20_5337ca732b98);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1394825d_62ce_4364_98d5_aa28c7fbd15b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnergySaverStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BatteryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PowerSupplyStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'System_Power'*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: 'System_Power'*"]
    pub fn EnergySaverStatus() -> ::windows::core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: EnergySaverStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnergySaverStatus>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EnergySaverStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnergySaverStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power'*"]
    pub fn BatteryStatus() -> ::windows::core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: BatteryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BatteryStatus>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BatteryStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBatteryStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power'*"]
    pub fn PowerSupplyStatus() -> ::windows::core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSupplyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PowerSupplyStatus>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PowerSupplyStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePowerSupplyStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power'*"]
    pub fn RemainingChargePercent() -> ::windows::core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTime() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTimeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingDischargeTimeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.System.Power.PowerManager";
}
#[doc = "*Required features: 'System_Power'*"]
#[repr(transparent)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl ::core::marker::Copy for PowerSupplyStatus {}
impl ::core::clone::Clone for PowerSupplyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PowerSupplyStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PowerSupplyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PowerSupplyStatus {}
impl ::core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.PowerSupplyStatus;i4)");
}
impl ::windows::core::DefaultType for PowerSupplyStatus {
    type DefaultType = Self;
}
