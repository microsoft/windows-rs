#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_Power\"`*"]
#[repr(transparent)]
pub struct Battery(::windows::core::IUnknown);
impl Battery {
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    pub fn GetReport(&self) -> ::windows::core::Result<BatteryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BatteryReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReportUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReportUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    pub fn AggregateBattery() -> ::windows::core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AggregateBattery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Battery>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Battery>> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Battery>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Power.Battery;{bc894fc6-0072-47c8-8b5d-614aaa7a437e})");
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
    const NAME: &'static str = "Windows.Devices.Power.Battery";
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
#[doc = "*Required features: `\"Devices_Power\"`*"]
#[repr(transparent)]
pub struct BatteryReport(::windows::core::IUnknown);
impl BatteryReport {
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChargeRateInMilliwatts(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChargeRateInMilliwatts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesignCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesignCapacityInMilliwattHours)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FullChargeCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FullChargeCapacityInMilliwattHours)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingCapacityInMilliwattHours)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`, `\"System_Power\"`*"]
    #[cfg(feature = "System_Power")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::System::Power::BatteryStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::Power::BatteryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::Power::BatteryStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for BatteryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BatteryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BatteryReport {}
impl ::core::fmt::Debug for BatteryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Power.BatteryReport;{c9858c3a-4e13-420a-a8d0-24f18f395401})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BatteryReport {
    type Vtable = IBatteryReport_Vtbl;
    const IID: ::windows::core::GUID = <IBatteryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BatteryReport {
    const NAME: &'static str = "Windows.Devices.Power.BatteryReport";
}
impl ::core::convert::From<BatteryReport> for ::windows::core::IUnknown {
    fn from(value: BatteryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BatteryReport> for ::windows::core::IUnknown {
    fn from(value: &BatteryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BatteryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BatteryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BatteryReport> for ::windows::core::IInspectable {
    fn from(value: BatteryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BatteryReport> for ::windows::core::IInspectable {
    fn from(value: &BatteryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BatteryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BatteryReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BatteryReport {}
unsafe impl ::core::marker::Sync for BatteryReport {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBattery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBattery {
    type Vtable = IBattery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc894fc6_0072_47c8_8b5d_614aaa7a437e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBattery_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReportUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBatteryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBatteryReport {
    type Vtable = IBatteryReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9858c3a_4e13_420a_a8d0_24f18f395401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ChargeRateInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChargeRateInMilliwatts: usize,
    #[cfg(feature = "Foundation")]
    pub DesignCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesignCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub FullChargeCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FullChargeCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCapacityInMilliwattHours: usize,
    #[cfg(feature = "System_Power")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::Power::BatteryStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_Power"))]
    Status: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBatteryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBatteryStatics {
    type Vtable = IBatteryStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79cd72b6_9e5e_4452_bea6_dfcd541e597f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AggregateBattery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
