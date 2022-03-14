#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct BackgroundEnergyManager {}
#[cfg(feature = "deprecated")]
impl BackgroundEnergyManager {
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LowUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NearMaxAcceptableUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxAcceptableUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExcessiveUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NearTerminationUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NearTerminationUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TerminationUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TerminationUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> ::windows::core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageIncreased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRecentEnergyUsageIncreased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageReturnedToLow)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRecentEnergyUsageReturnedToLow)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
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
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for BatteryStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BatteryStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.BatteryStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for EnergySaverStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnergySaverStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.EnergySaverStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct ForegroundEnergyManager {}
#[cfg(feature = "deprecated")]
impl ForegroundEnergyManager {
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LowUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NearMaxAcceptableUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxAcceptableUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExcessiveUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> ::windows::core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageIncreased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageIncreased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRecentEnergyUsageIncreased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecentEnergyUsageReturnedToLow)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecentEnergyUsageReturnedToLow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRecentEnergyUsageReturnedToLow)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
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
    type Vtable = IBackgroundEnergyManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3161d95_1180_4376_96e1_4095568147ce);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearTerminationUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearTerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub TerminationUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IForegroundEnergyManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IForegroundEnergyManagerStatics {
    type Vtable = IForegroundEnergyManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff86872_e677_4814_9a20_5337ca732b98);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1394825d_62ce_4364_98d5_aa28c7fbd15b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnergySaverStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnergySaverStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnergySaverStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnergySaverStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnergySaverStatusChanged: usize,
    pub BatteryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BatteryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BatteryStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BatteryStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBatteryStatusChanged: usize,
    pub PowerSupplyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PowerSupplyStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSupplyStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSupplyStatusChanged: usize,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTimeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingDischargeTimeChanged: usize,
}
#[doc = "*Required features: `\"System_Power\"`*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: `\"System_Power\"`*"]
    pub fn EnergySaverStatus() -> ::windows::core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: EnergySaverStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnergySaverStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnergySaverStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnergySaverStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnergySaverStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnergySaverStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveEnergySaverStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`*"]
    pub fn BatteryStatus() -> ::windows::core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: BatteryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatteryStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BatteryStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BatteryStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatteryStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBatteryStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveBatteryStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`*"]
    pub fn PowerSupplyStatus() -> ::windows::core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSupplyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSupplyStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PowerSupplyStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PowerSupplyStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSupplyStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePowerSupplyStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePowerSupplyStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`*"]
    pub fn RemainingChargePercent() -> ::windows::core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingChargePercent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingChargePercentChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTime() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingDischargeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTimeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingDischargeTimeChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingDischargeTimeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRemainingDischargeTimeChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
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
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PowerSupplyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSupplyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Power.PowerSupplyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
