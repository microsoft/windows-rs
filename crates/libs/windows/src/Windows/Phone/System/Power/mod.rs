#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPowerManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPowerManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25de8fd0_1c5b_11e1_bddb_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PowerSavingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PowerSavingMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSavingModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSavingModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IPowerManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPowerManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x596236cf_1918_4551_a466_c51aae373bf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System_Power\"`*"]
pub struct PowerManager;
impl PowerManager {
    pub fn PowerSavingMode() -> ::windows_core::Result<PowerSavingMode> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PowerSavingModeChanged<P0>(changehandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingModeChanged)(::windows_core::Interface::as_raw(this), changehandler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePowerSavingModeChanged(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePowerSavingModeChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn PowerSavingModeEnabled() -> ::windows_core::Result<bool> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingModeEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics2<R, F: FnOnce(&IPowerManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[doc = "*Required features: `\"Phone_System_Power\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for PowerSavingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PowerSavingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSavingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PowerSavingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
}
