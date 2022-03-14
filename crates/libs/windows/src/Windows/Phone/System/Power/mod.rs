#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25de8fd0_1c5b_11e1_bddb_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PowerSavingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PowerSavingMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSavingModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSavingModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x596236cf_1918_4551_a466_c51aae373bf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System_Power\"`*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: `\"Phone_System_Power\"`*"]
    pub fn PowerSavingMode() -> ::windows::core::Result<PowerSavingMode> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSavingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSavingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PowerSavingMode>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PowerSavingModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(changehandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSavingModeChanged)(::core::mem::transmute_copy(this), changehandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_System_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePowerSavingModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePowerSavingModeChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_System_Power\"`*"]
    pub fn PowerSavingModeEnabled() -> ::windows::core::Result<bool> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerSavingModeEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics2<R, F: FnOnce(&IPowerManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[doc = "*Required features: `\"Phone_System_Power\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSavingMode {}
impl ::core::clone::Clone for PowerSavingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSavingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSavingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSavingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSavingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSavingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
