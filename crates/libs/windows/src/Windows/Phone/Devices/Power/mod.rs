#[doc(hidden)]
#[repr(transparent)]
pub struct IBattery(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBattery {
    type Vtable = IBattery_Vtbl;
}
unsafe impl ::windows::core::Interface for IBattery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972adbdd_6720_4702_a476_b9d38a0070e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBattery_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBatteryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBatteryStatics {
    type Vtable = IBatteryStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBatteryStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf5bc70_6369_11e1_b86c_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Devices_Power\"`*"]
#[repr(transparent)]
pub struct Battery(::windows::core::IUnknown);
impl Battery {
    pub fn RemainingChargePercent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingChargePercent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingDischargeTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingChargePercentChanged(&self, changehandler: &super::super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingChargePercentChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(changehandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingChargePercentChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemainingChargePercentChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBatteryStatics<R, F: FnOnce(&IBatteryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Battery, IBatteryStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Battery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Battery {}
impl ::core::fmt::Debug for Battery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Battery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Battery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Devices.Power.Battery;{972adbdd-6720-4702-a476-b9d38a0070e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Battery {
    type Vtable = IBattery_Vtbl;
}
unsafe impl ::windows::core::Interface for Battery {
    const IID: ::windows::core::GUID = <IBattery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Battery {
    const NAME: &'static str = "Windows.Phone.Devices.Power.Battery";
}
::windows::core::interface_hierarchy!(Battery, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Battery {}
unsafe impl ::core::marker::Sync for Battery {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
