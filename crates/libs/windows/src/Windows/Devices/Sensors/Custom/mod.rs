#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
#[repr(transparent)]
pub struct CustomSensor(::windows::core::IUnknown);
impl CustomSensor {
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CustomSensorReading>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CustomSensor, CustomSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportLatency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn GetDeviceSelector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(interfaceid: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), interfaceid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sensorid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CustomSensor>> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), sensorid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CustomSensor>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomSensorStatics<R, F: FnOnce(&ICustomSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CustomSensor, ICustomSensorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CustomSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSensor {}
impl ::core::fmt::Debug for CustomSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSensor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomSensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensor;{a136f9ad-4034-4b4d-99dd-531aac649c09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomSensor {
    type Vtable = ICustomSensor_Vtbl;
    const IID: ::windows::core::GUID = <ICustomSensor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensor";
}
impl ::core::convert::From<CustomSensor> for ::windows::core::IUnknown {
    fn from(value: CustomSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensor> for ::windows::core::IUnknown {
    fn from(value: &CustomSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CustomSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomSensor> for ::windows::core::IInspectable {
    fn from(value: CustomSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensor> for ::windows::core::IInspectable {
    fn from(value: &CustomSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CustomSensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CustomSensor {}
unsafe impl ::core::marker::Sync for CustomSensor {}
#[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
#[repr(transparent)]
pub struct CustomSensorReading(::windows::core::IUnknown);
impl CustomSensorReading {
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<ICustomSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
}
impl ::core::clone::Clone for CustomSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSensorReading {}
impl ::core::fmt::Debug for CustomSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSensorReading").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomSensorReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensorReading;{64004f4d-446a-4366-a87a-5f963268ec53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomSensorReading {
    type Vtable = ICustomSensorReading_Vtbl;
    const IID: ::windows::core::GUID = <ICustomSensorReading as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReading";
}
impl ::core::convert::From<CustomSensorReading> for ::windows::core::IUnknown {
    fn from(value: CustomSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensorReading> for ::windows::core::IUnknown {
    fn from(value: &CustomSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CustomSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomSensorReading> for ::windows::core::IInspectable {
    fn from(value: CustomSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensorReading> for ::windows::core::IInspectable {
    fn from(value: &CustomSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CustomSensorReading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CustomSensorReading {}
unsafe impl ::core::marker::Sync for CustomSensorReading {}
#[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
#[repr(transparent)]
pub struct CustomSensorReadingChangedEventArgs(::windows::core::IUnknown);
impl CustomSensorReadingChangedEventArgs {
    #[doc = "*Required features: `\"Devices_Sensors_Custom\"`*"]
    pub fn Reading(&self) -> ::windows::core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CustomSensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for CustomSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for CustomSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs;{6b202023-cffd-4cc1-8ff0-e21823d76fcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICustomSensorReadingChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
}
impl ::core::convert::From<CustomSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CustomSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensorReadingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CustomSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CustomSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CustomSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomSensorReadingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CustomSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CustomSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CustomSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for CustomSensorReadingChangedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensor {
    type Vtable = ICustomSensor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa136f9ad_4034_4b4d_99dd_531aac649c09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensor2 {
    type Vtable = ICustomSensor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20db3111_ec58_4d9f_bfbd_e77825088510);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensorReading {
    type Vtable = ICustomSensorReading_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64004f4d_446a_4366_a87a_5f963268ec53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensorReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensorReading2 {
    type Vtable = ICustomSensorReading2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x223c98ea_bf73_4992_9a48_d3c897594ccb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b202023_cffd_4cc1_8ff0_e21823d76fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReadingChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSensorStatics {
    type Vtable = ICustomSensorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x992052cf_f422_4c7d_836b_e7dc74a7124b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
