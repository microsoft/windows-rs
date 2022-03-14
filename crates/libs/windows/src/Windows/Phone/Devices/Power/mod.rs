#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Phone_Devices_Power\"`*"]
#[repr(transparent)]
pub struct Battery(::windows::core::IUnknown);
impl Battery {
    #[doc = "*Required features: `\"Phone_Devices_Power\"`*"]
    pub fn RemainingChargePercent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingChargePercent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingDischargeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingDischargeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, changehandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingChargePercentChanged)(::core::mem::transmute_copy(this), changehandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemainingChargePercentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Phone_Devices_Power\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Battery>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBatteryStatics<R, F: FnOnce(&IBatteryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Battery, IBatteryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for Battery {
    type Vtable = IBattery_Vtbl;
    const IID: ::windows::core::GUID = <IBattery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Battery {
    const NAME: &'static str = "Windows.Phone.Devices.Power.Battery";
}
impl ::core::convert::From<Battery> for ::windows::core::IUnknown {
    fn from(value: Battery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Battery> for ::windows::core::IUnknown {
    fn from(value: &Battery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Battery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Battery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Battery> for ::windows::core::IInspectable {
    fn from(value: Battery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Battery> for ::windows::core::IInspectable {
    fn from(value: &Battery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Battery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Battery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Battery {}
unsafe impl ::core::marker::Sync for Battery {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBattery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBattery {
    type Vtable = IBattery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972adbdd_6720_4702_a476_b9d38a0070e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBattery_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IBatteryStatics {
    type Vtable = IBatteryStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf5bc70_6369_11e1_b86c_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
