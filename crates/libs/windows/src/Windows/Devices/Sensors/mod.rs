#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometer {
    type Vtable = IAccelerometer_Vtbl;
}
impl ::core::clone::Clone for IAccelerometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf184548_2711_4da7_8098_4b82205d3c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Shaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Shaken: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShaken: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometer2 {
    type Vtable = IAccelerometer2_Vtbl;
}
impl ::core::clone::Clone for IAccelerometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8f092ee_4964_401a_b602_220d7153c60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometer3 {
    type Vtable = IAccelerometer3_Vtbl;
}
impl ::core::clone::Clone for IAccelerometer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87e0022a_ed80_49eb_bf8a_a4ea31e5cd84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometer4 {
    type Vtable = IAccelerometer4_Vtbl;
}
impl ::core::clone::Clone for IAccelerometer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d373c4f_42d3_45b2_8144_ab7fb665eb59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccelerometerReadingType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometer5 {
    type Vtable = IAccelerometer5_Vtbl;
}
impl ::core::clone::Clone for IAccelerometer5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometer5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e7e7021_def4_53a6_af43_806fd538edf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92c1b68_6320_5577_879e_9942621c3dd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub XAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetXAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub YAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetYAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ZAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetZAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerDeviceId {
    type Vtable = IAccelerometerDeviceId_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eac64a9_97d5_446d_ab5a_917df9b96a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerReading {
    type Vtable = IAccelerometerReading_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9fe7acb_d351_40af_8bb6_7aa9ae641fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AccelerationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AccelerationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AccelerationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerReading2 {
    type Vtable = IAccelerometerReading2_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a864aa2_15ae_4a40_be55_db58d7de7389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0095c65b_b6ac_475a_9f44_8b32d35a3f25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerShakenEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerShakenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerShakenEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerShakenEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerStatics {
    type Vtable = IAccelerometerStatics_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5e28b74_5a87_4a2d_becc_0f906ea061dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerStatics2 {
    type Vtable = IAccelerometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c4842f_d86b_4685_b2d7_3396f798d57b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccelerometerStatics3 {
    type Vtable = IAccelerometerStatics3_Vtbl;
}
impl ::core::clone::Clone for IAccelerometerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccelerometerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9de218cf_455d_4cf3_8200_70e1410340f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensor {
    type Vtable = IActivitySensor_Vtbl;
}
impl ::core::clone::Clone for IActivitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7a630c_fb5f_48eb_b09b_a2708d1c61ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedActivities: usize,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorReading {
    type Vtable = IActivitySensorReading_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85125a96_1472_40a2_b2ae_e1ef29226c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Activity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivityType) -> ::windows::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivitySensorReadingConfidence) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReadingChangeReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorReadingChangeReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorReadingChangeReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f3c2915_d93b_47bd_960a_f20fb2f322b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangeReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorStatics {
    type Vtable = IActivitySensorStatics_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa71e0e9d_ee8b_45d1_b25b_08cc0df92ab6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9e6612_b9ca_4677_b263_243297f79d3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeter {
    type Vtable = IAltimeter_Vtbl;
}
impl ::core::clone::Clone for IAltimeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72f057fd_8f04_49f1_b4a7_f4e363b701a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeter2 {
    type Vtable = IAltimeter2_Vtbl;
}
impl ::core::clone::Clone for IAltimeter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9471bf9_2add_48f5_9f08_3d0c7660d938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeterReading {
    type Vtable = IAltimeterReading_Vtbl;
}
impl ::core::clone::Clone for IAltimeterReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeterReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbe8ef73_7f5e_48c8_aa1a_f1f3befc1144);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AltitudeChangeInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeterReading2 {
    type Vtable = IAltimeterReading2_Vtbl;
}
impl ::core::clone::Clone for IAltimeterReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeterReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543a1bd9_6d0b_42b2_bd69_bc8fae0f782c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAltimeterReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeterReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7069d077_446d_47f7_998c_ebc23b45e4a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAltimeterStatics {
    type Vtable = IAltimeterStatics_Vtbl;
}
impl ::core::clone::Clone for IAltimeterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAltimeterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eb4d7c3_e5ac_47ce_8eef_d3718168c01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometer {
    type Vtable = IBarometer_Vtbl;
}
impl ::core::clone::Clone for IBarometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x934475a8_78bf_452f_b017_f0209ce6dab4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometer2 {
    type Vtable = IBarometer2_Vtbl;
}
impl ::core::clone::Clone for IBarometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32bcc418_3eeb_4d04_9574_7633a8781f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometer3 {
    type Vtable = IBarometer3_Vtbl;
}
impl ::core::clone::Clone for IBarometer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e35f0ea_02b5_5a04_b03d_822084863a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_Vtbl;
}
impl ::core::clone::Clone for IBarometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076b952c_cb62_5a90_a0d1_f85e4a936394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Hectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerReading {
    type Vtable = IBarometerReading_Vtbl;
}
impl ::core::clone::Clone for IBarometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub StationPressureInHectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerReading2 {
    type Vtable = IBarometerReading2_Vtbl;
}
impl ::core::clone::Clone for IBarometerReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a244eb_90c5_4875_891c_3865b4c357e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBarometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d84945f_037b_404f_9bbb_6232d69543c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerStatics {
    type Vtable = IBarometerStatics_Vtbl;
}
impl ::core::clone::Clone for IBarometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x286b270a_02e3_4f86_84fc_fdd892b5940f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarometerStatics2 {
    type Vtable = IBarometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IBarometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc6b1e7_95ff_44ac_878e_d65c8308c34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompass {
    type Vtable = ICompass_Vtbl;
}
impl ::core::clone::Clone for ICompass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x292ffa94_1b45_403c_ba06_b106dba69a64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompass2 {
    type Vtable = ICompass2_Vtbl;
}
impl ::core::clone::Clone for ICompass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompass2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36f26d09_c7d7_434f_b461_979ddfc2322f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompass3 {
    type Vtable = ICompass3_Vtbl;
}
impl ::core::clone::Clone for ICompass3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompass3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa424801b_c5ea_4d45_a0ec_4b791f041a89);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompass4 {
    type Vtable = ICompass4_Vtbl;
}
impl ::core::clone::Clone for ICompass4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompass4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x291e7f11_ec32_5dcc_bfcb_0bb39eba5774);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassDataThreshold {
    type Vtable = ICompassDataThreshold_Vtbl;
}
impl ::core::clone::Clone for ICompassDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Degrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassDeviceId {
    type Vtable = ICompassDeviceId_Vtbl;
}
impl ::core::clone::Clone for ICompassDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd181ca29_b085_4b1d_870a_4ff57ba74fd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassReading {
    type Vtable = ICompassReading_Vtbl;
}
impl ::core::clone::Clone for ICompassReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82911128_513d_4dc9_b781_5eedfbf02d0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub HeadingMagneticNorth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeadingTrueNorth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadingTrueNorth: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassReading2 {
    type Vtable = ICompassReading2_Vtbl;
}
impl ::core::clone::Clone for ICompassReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb13a661e_51bb_4a12_bedd_ad47ff87d2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICompassReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReadingHeadingAccuracy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassReadingHeadingAccuracy {
    type Vtable = ICompassReadingHeadingAccuracy_Vtbl;
}
impl ::core::clone::Clone for ICompassReadingHeadingAccuracy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassReadingHeadingAccuracy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe761354e_8911_40f7_9e16_6ecc7daec5de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingHeadingAccuracy_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HeadingAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassStatics {
    type Vtable = ICompassStatics_Vtbl;
}
impl ::core::clone::Clone for ICompassStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9abc97df_56ec_4c25_b54d_40a68bb5b269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompassStatics2 {
    type Vtable = ICompassStatics2_Vtbl;
}
impl ::core::clone::Clone for ICompassStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompassStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ace0ead_3baa_4990_9ce4_be0913754ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometer {
    type Vtable = IGyrometer_Vtbl;
}
impl ::core::clone::Clone for IGyrometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdb9a9c4_84b1_4ca2_9763_9b589506c70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometer2 {
    type Vtable = IGyrometer2_Vtbl;
}
impl ::core::clone::Clone for IGyrometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63df2443_8ce8_41c3_ac44_8698810b557f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometer3 {
    type Vtable = IGyrometer3_Vtbl;
}
impl ::core::clone::Clone for IGyrometer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d6f88d5_8fbc_4484_914b_528adfd947b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometer4 {
    type Vtable = IGyrometer4_Vtbl;
}
impl ::core::clone::Clone for IGyrometer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0628a60c_4c4b_5096_94e6_c356df68bef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_Vtbl;
}
impl ::core::clone::Clone for IGyrometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8648b31e_6e52_5259_bbad_242a69dc38c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub XAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetXAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub YAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetYAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerDeviceId {
    type Vtable = IGyrometerDeviceId_Vtbl;
}
impl ::core::clone::Clone for IGyrometerDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee5e978_89a2_4275_9e95_7126f4708760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerReading {
    type Vtable = IGyrometerReading_Vtbl;
}
impl ::core::clone::Clone for IGyrometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3d6de5c_1ee4_456f_9de7_e2493b5c8e03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngularVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AngularVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AngularVelocityZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerReading2 {
    type Vtable = IGyrometerReading2_Vtbl;
}
impl ::core::clone::Clone for IGyrometerReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16afe13c_2b89_44bb_822b_d1e1556ff09b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGyrometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fdf1895_6f9e_42ce_8d58_388c0ab8356d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerStatics {
    type Vtable = IGyrometerStatics_Vtbl;
}
impl ::core::clone::Clone for IGyrometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83b6e7c9_e49d_4b39_86e6_cd554be4c5c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGyrometerStatics2 {
    type Vtable = IGyrometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IGyrometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGyrometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef83f7a1_d700_4204_9613_79c6b161df4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHingeAngleReading {
    type Vtable = IHingeAngleReading_Vtbl;
}
impl ::core::clone::Clone for IHingeAngleReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHingeAngleReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHingeAngleSensor {
    type Vtable = IHingeAngleSensor_Vtbl;
}
impl ::core::clone::Clone for IHingeAngleSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHingeAngleSensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9d3be02_bfdf_437f_8c29_88c77393d309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MinReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub ReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHingeAngleSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHingeAngleSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24d9558b_fad0_42b8_a854_78923049a1ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHingeAngleSensorStatics {
    type Vtable = IHingeAngleSensorStatics_Vtbl;
}
impl ::core::clone::Clone for IHingeAngleSensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHingeAngleSensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b63910_fbb1_4123_89ce_4ea34eb0dfca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRelatedToAdjacentPanelsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstpanelid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, secondpanelid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRelatedToAdjacentPanelsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceFeatures(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceFeatures {
    type Vtable = IHumanPresenceFeatures_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceFeatures {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceFeatures {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdb09fda_3244_557a_bd29_8b004f59f2cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceFeatures_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedWakeOrLockDistancesInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedWakeOrLockDistancesInMillimeters: usize,
    pub IsWakeOnApproachSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLockOnLeaveSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAttentionAwareDimmingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSensor {
    type Vtable = IHumanPresenceSensor_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2116788b_e389_5cc3_9a97_cb17be1008bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxDetectableDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDetectableDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub MinDetectableDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDetectableDistanceInMillimeters: usize,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSensorReading {
    type Vtable = IHumanPresenceSensorReading_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSensorReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83533bf5_a85a_5d50_8be4_6072d745a3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Presence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HumanPresence) -> ::windows::core::HRESULT,
    pub Engagement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HumanEngagement) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistanceInMillimeters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSensorReadingChangedEventArgs {
    type Vtable = IHumanPresenceSensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9dc4583_fd69_5c5e_ab1f_942204eae2db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSensorStatics {
    type Vtable = IHumanPresenceSensorStatics_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ae89842_dba9_56b2_9f27_eac69d621004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSettings {
    type Vtable = IHumanPresenceSettings_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef4daf5b_07b7_5eb6_86bb_b7ff49ce44fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSensorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsWakeOnApproachEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsWakeOnApproachEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WakeOnApproachDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub SetWakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWakeOnApproachDistanceInMillimeters: usize,
    pub IsLockOnLeaveEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsLockOnLeaveEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockOnLeaveDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub SetLockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLockOnLeaveDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub LockOnLeaveTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockOnLeaveTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetLockOnLeaveTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLockOnLeaveTimeout: usize,
    pub IsAttentionAwareDimmingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAttentionAwareDimmingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHumanPresenceSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHumanPresenceSettingsStatics {
    type Vtable = IHumanPresenceSettingsStatics_Vtbl;
}
impl ::core::clone::Clone for IHumanPresenceSettingsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHumanPresenceSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f343202_e010_52c4_af0c_04a8f1e033da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSettingsAsync: usize,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSettingsAsync: usize,
    pub UpdateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetSupportedFeaturesForSensorIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSupportedFeaturesForSensorIdAsync: usize,
    pub GetSupportedFeaturesForSensorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockOnLeaveTimeouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockOnLeaveTimeouts: usize,
    #[cfg(feature = "Foundation")]
    pub SettingsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SettingsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSettingsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSettingsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometer {
    type Vtable = IInclinometer_Vtbl;
}
impl ::core::clone::Clone for IInclinometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometer2 {
    type Vtable = IInclinometer2_Vtbl;
}
impl ::core::clone::Clone for IInclinometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x029f3393_28b2_45f8_bb16_61e86a7fae6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometer3 {
    type Vtable = IInclinometer3_Vtbl;
}
impl ::core::clone::Clone for IInclinometer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a095004_d765_4384_a3d7_0283f3abe6ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometer4 {
    type Vtable = IInclinometer4_Vtbl;
}
impl ::core::clone::Clone for IInclinometer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43852618_8fca_548e_bbf5_5c50412b6aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_Vtbl;
}
impl ::core::clone::Clone for IInclinometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf80a4783_7bfe_545e_bb60_a0ebc47bd2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PitchInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPitchInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub RollInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetRollInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetYawInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerDeviceId {
    type Vtable = IInclinometerDeviceId_Vtbl;
}
impl ::core::clone::Clone for IInclinometerDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01e91982_41ff_4406_ae83_62210ff16fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerReading {
    type Vtable = IInclinometerReading_Vtbl;
}
impl ::core::clone::Clone for IInclinometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f44f055_b6f6_497f_b127_1a775e501458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub PitchDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub RollDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub YawDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerReading2 {
    type Vtable = IInclinometerReading2_Vtbl;
}
impl ::core::clone::Clone for IInclinometerReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f164781_e90b_4658_8915_0103e08a805a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IInclinometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ae91dc1_e7eb_4938_8511_ae0d6b440438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReadingYawAccuracy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerReadingYawAccuracy {
    type Vtable = IInclinometerReadingYawAccuracy_Vtbl;
}
impl ::core::clone::Clone for IInclinometerReadingYawAccuracy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerReadingYawAccuracy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb453e880_1fe3_4986_a257_e6ece2723949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingYawAccuracy_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerStatics {
    type Vtable = IInclinometerStatics_Vtbl;
}
impl ::core::clone::Clone for IInclinometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf22ec551_9c30_453a_8b49_3c3eeb33cb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerStatics2 {
    type Vtable = IInclinometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IInclinometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x043f9775_6a1e_499c_86e0_638c1a864b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerStatics3 {
    type Vtable = IInclinometerStatics3_Vtbl;
}
impl ::core::clone::Clone for IInclinometerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd9a4280_b91a_4829_9392_abc0b6bdf2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInclinometerStatics4 {
    type Vtable = IInclinometerStatics4_Vtbl;
}
impl ::core::clone::Clone for IInclinometerStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInclinometerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8ba96f9_6e85_4a83_aed0_d7cdcc9856c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensor {
    type Vtable = ILightSensor_Vtbl;
}
impl ::core::clone::Clone for ILightSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf84c0718_0c54_47ae_922e_789f57fb03a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensor2 {
    type Vtable = ILightSensor2_Vtbl;
}
impl ::core::clone::Clone for ILightSensor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x486b24e8_a94c_4090_8f48_09f782a9f7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensor3 {
    type Vtable = ILightSensor3_Vtbl;
}
impl ::core::clone::Clone for ILightSensor3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensor3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4876d0ff_9f4c_5f72_adbd_a3471b063c00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_Vtbl;
}
impl ::core::clone::Clone for ILightSensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb160afd1_878f_5492_9f2c_33dc3ae584a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LuxPercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorDeviceId {
    type Vtable = ILightSensorDeviceId_Vtbl;
}
impl ::core::clone::Clone for ILightSensorDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fee49f8_0afb_4f51_87f0_6c26375ce94f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorReading {
    type Vtable = ILightSensorReading_Vtbl;
}
impl ::core::clone::Clone for ILightSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffdf6300_227c_4d2b_b302_fc0142485c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IlluminanceInLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorReading2 {
    type Vtable = ILightSensorReading2_Vtbl;
}
impl ::core::clone::Clone for ILightSensorReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7512185_44a3_44c9_8190_9ef6de0a8a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILightSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a2f4cf_258b_420c_b8ab_8edd601ecf50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorStatics {
    type Vtable = ILightSensorStatics_Vtbl;
}
impl ::core::clone::Clone for ILightSensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45db8c84_c3a8_471e_9a53_6457fad87c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILightSensorStatics2 {
    type Vtable = ILightSensorStatics2_Vtbl;
}
impl ::core::clone::Clone for ILightSensorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILightSensorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ec0a650_ddc6_40ab_ace3_ec3359d42c51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometer {
    type Vtable = IMagnetometer_Vtbl;
}
impl ::core::clone::Clone for IMagnetometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometer2 {
    type Vtable = IMagnetometer2_Vtbl;
}
impl ::core::clone::Clone for IMagnetometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4656c85_26f6_444b_a9e2_a23f966cd368);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometer3 {
    type Vtable = IMagnetometer3_Vtbl;
}
impl ::core::clone::Clone for IMagnetometer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe93db7c_a625_48ef_acf7_fac104832671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometer4 {
    type Vtable = IMagnetometer4_Vtbl;
}
impl ::core::clone::Clone for IMagnetometer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfb17901_3e0f_508f_b24b_f2bb75015f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerDataThreshold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd177cb01_9063_5fa5_b596_b445e9dc3401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub XAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetXAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub YAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetYAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub ZAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetZAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerDeviceId {
    type Vtable = IMagnetometerDeviceId_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58b498c2_7e4b_404c_9fc5_5de8b40ebae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerReading {
    type Vtable = IMagnetometerReading_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c2cc40d_ebfd_4e5c_bb11_afc29b3cae61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub MagneticFieldX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MagneticFieldY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MagneticFieldZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub DirectionalAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerReading2 {
    type Vtable = IMagnetometerReading2_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4c95c61_61d9_404b_a328_066f177a1409);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerStatics {
    type Vtable = IMagnetometerStatics_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x853c64cc_0698_4dda_a6df_9cb9cc4ab40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagnetometerStatics2 {
    type Vtable = IMagnetometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IMagnetometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMagnetometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0819f0_ffc6_4f89_a06f_18fa10792933);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensor {
    type Vtable = IOrientationSensor_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e354635_cf6b_4c63_abd8_10252b0bf6ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensor2 {
    type Vtable = IOrientationSensor2_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d924cf9_2f1f_49c9_8042_4a1813d67760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensor3 {
    type Vtable = IOrientationSensor3_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensor3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensor3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cce578d_646b_48c5_b7ee_44fdc4c6aafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorDeviceId {
    type Vtable = IOrientationSensorDeviceId_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a69b648_4c29_49ec_b28f_ea1d117b66f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorReading {
    type Vtable = IOrientationSensorReading_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4756c993_6595_4897_bcc6_d537ee757564);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub RotationMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Quaternion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReading2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorReading2 {
    type Vtable = IOrientationSensorReading2_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorReading2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorReading2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00576e5f_49f8_4c05_9e07_24fac79408c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012c1186_c3ba_46bc_ae65_7a98996cbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReadingYawAccuracy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorReadingYawAccuracy {
    type Vtable = IOrientationSensorReadingYawAccuracy_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorReadingYawAccuracy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorReadingYawAccuracy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1ac9824_3f5a_49a2_bc7b_1180bc38cd2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingYawAccuracy_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics {
    type Vtable = IOrientationSensorStatics_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ef8712_fb4c_428a_898b_2765e409e669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics2 {
    type Vtable = IOrientationSensorStatics2_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59da0d0b_d40a_4c71_9276_8a272a0a6619);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics3 {
    type Vtable = IOrientationSensorStatics3_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82ce920_2777_40ff_9f59_d654b085f12f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOrientationSensorStatics4 {
    type Vtable = IOrientationSensorStatics4_Vtbl;
}
impl ::core::clone::Clone for IOrientationSensorStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOrientationSensorStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa67feb55_2c85_4b28_a0fe_58c4b20495f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometer {
    type Vtable = IPedometer_Vtbl;
}
impl ::core::clone::Clone for IPedometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1e013d_3d98_45f8_8920_8e4ecaca5f97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometer2 {
    type Vtable = IPedometer2_Vtbl;
}
impl ::core::clone::Clone for IPedometer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a406df_2b81_4add_b2ff_77ab6c98ba19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentReadings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerDataThresholdFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometerDataThresholdFactory {
    type Vtable = IPedometerDataThresholdFactory_Vtbl;
}
impl ::core::clone::Clone for IPedometerDataThresholdFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometerDataThresholdFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbad8f50_7a54_466b_9010_77a162fca5d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerDataThresholdFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensor: *mut ::core::ffi::c_void, stepgoal: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometerReading {
    type Vtable = IPedometerReading_Vtbl;
}
impl ::core::clone::Clone for IPedometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometerReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2245dcf4_a8e1_432f_896a_be0dd9b02d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StepKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PedometerStepKind) -> ::windows::core::HRESULT,
    pub CumulativeSteps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation")]
    pub CumulativeStepsDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CumulativeStepsDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPedometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf855e47e_abbc_4456_86a8_25cf2b333742);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometerStatics {
    type Vtable = IPedometerStatics_Vtbl;
}
impl ::core::clone::Clone for IPedometerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82980a2f_4083_4dfb_b411_938ea0f4b946);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPedometerStatics2 {
    type Vtable = IPedometerStatics2_Vtbl;
}
impl ::core::clone::Clone for IPedometerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPedometerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f5c6bb_ce0e_4133_b47e_8627ea72f677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensor {
    type Vtable = IProximitySensor_Vtbl;
}
impl ::core::clone::Clone for IProximitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54c076b8_ecfb_4944_b928_74fc504d47ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub MinDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDistanceInMillimeters: usize,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDisplayOnOffController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDisplayOnOffController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorDataThresholdFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensorDataThresholdFactory {
    type Vtable = IProximitySensorDataThresholdFactory_Vtbl;
}
impl ::core::clone::Clone for IProximitySensorDataThresholdFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensorDataThresholdFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905ac121_6d27_4ad3_9db5_6467f2a5ad9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorDataThresholdFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorReading(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensorReading {
    type Vtable = IProximitySensorReading_Vtbl;
}
impl ::core::clone::Clone for IProximitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensorReading {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71228d59_132d_4d5f_8ff9_2f0db8751ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReading_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IsDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistanceInMillimeters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorReadingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IProximitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensorStatics {
    type Vtable = IProximitySensorStatics_Vtbl;
}
impl ::core::clone::Clone for IProximitySensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29186649_6269_4e57_a5ad_82be80813392);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximitySensorStatics2 {
    type Vtable = IProximitySensorStatics2_Vtbl;
}
impl ::core::clone::Clone for IProximitySensorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProximitySensorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbf473ae_e9ca_422f_ad67_4c3d25df350c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ISensorDataThreshold(::windows::core::IUnknown);
impl ISensorDataThreshold {}
::windows::imp::interface_hierarchy!(ISensorDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorDataThreshold {}
impl ::core::fmt::Debug for ISensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISensorDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{54daec61-fe4b-4e07-b260-3a4cdfbe396e}");
}
unsafe impl ::windows::core::Interface for ISensorDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
}
impl ::core::clone::Clone for ISensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorDataThreshold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThreshold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ISensorDataThresholdTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorDataThresholdTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9106f1b7_e88d_48b1_bc90_619c7b349391);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SensorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorQuaternion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorQuaternion {
    type Vtable = ISensorQuaternion_Vtbl;
}
impl ::core::clone::Clone for ISensorQuaternion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorQuaternion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorQuaternion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub W: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub X: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Z: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorRotationMatrix(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_Vtbl;
}
impl ::core::clone::Clone for ISensorRotationMatrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorRotationMatrix {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a3d5a67_22f4_4392_9538_65d0bd064aa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorRotationMatrix_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub M11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M21: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M23: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M31: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub M33: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff53856_214a_4dee_a3f9_616f1ab06ffd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OrientationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOrientationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensor2 {
    type Vtable = ISimpleOrientationSensor2_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa277a798_8870_453e_8bd6_b8f5d8d7941b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorDeviceId {
    type Vtable = ISimpleOrientationSensorDeviceId_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensorDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensorDeviceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc00acb_3b76_41f6_8091_30efe646d3cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorDeviceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensorOrientationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensorOrientationChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorStatics {
    type Vtable = ISimpleOrientationSensorStatics_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72ed066f_70aa_40c6_9b1b_3433f7459b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleOrientationSensorStatics2 {
    type Vtable = ISimpleOrientationSensorStatics2_Vtbl;
}
impl ::core::clone::Clone for ISimpleOrientationSensorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleOrientationSensorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x848f9c7f_b138_4e11_8910_a2a2a3b56d83);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Accelerometer(::windows::core::IUnknown);
impl Accelerometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AccelerometerReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Shaken(&self, handler: &super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Shaken)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShaken(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShaken)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<AccelerometerReadingType> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AccelerometerReadingType>();
            (::windows::core::Interface::vtable(this).ReadingType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<AccelerometerDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometer5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AccelerometerDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Accelerometer> {
        Self::IAccelerometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Accelerometer>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultWithAccelerometerReadingType(readingtype: AccelerometerReadingType) -> ::windows::core::Result<Accelerometer> {
        Self::IAccelerometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Accelerometer>();
            (::windows::core::Interface::vtable(this).GetDefaultWithAccelerometerReadingType)(::windows::core::Interface::as_raw(this), readingtype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Accelerometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: AccelerometerReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), readingtype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics<R, F: FnOnce(&IAccelerometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Accelerometer, IAccelerometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics2<R, F: FnOnce(&IAccelerometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Accelerometer, IAccelerometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics3<R, F: FnOnce(&IAccelerometerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Accelerometer, IAccelerometerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Accelerometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Accelerometer {}
impl ::core::fmt::Debug for Accelerometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Accelerometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Accelerometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Accelerometer;{df184548-2711-4da7-8098-4b82205d3c7d})");
}
impl ::core::clone::Clone for Accelerometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Accelerometer {
    type Vtable = IAccelerometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Accelerometer {
    const IID: ::windows::core::GUID = <IAccelerometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Accelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Accelerometer";
}
::windows::imp::interface_hierarchy!(Accelerometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Accelerometer {}
unsafe impl ::core::marker::Sync for Accelerometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AccelerometerDataThreshold(::windows::core::IUnknown);
impl AccelerometerDataThreshold {
    pub fn XAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).XAxisInGForce)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetXAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXAxisInGForce)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).YAxisInGForce)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYAxisInGForce)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInGForce(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ZAxisInGForce)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZAxisInGForce(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZAxisInGForce)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AccelerometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerDataThreshold {}
impl ::core::fmt::Debug for AccelerometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AccelerometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerDataThreshold;{f92c1b68-6320-5577-879e-9942621c3dd9})");
}
impl ::core::clone::Clone for AccelerometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccelerometerDataThreshold {
    const IID: ::windows::core::GUID = <IAccelerometerDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerDataThreshold";
}
::windows::imp::interface_hierarchy!(AccelerometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AccelerometerDataThreshold {}
unsafe impl ::core::marker::Sync for AccelerometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AccelerometerReading(::windows::core::IUnknown);
impl AccelerometerReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccelerationX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AccelerationX)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccelerationY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AccelerationY)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccelerationZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AccelerationZ)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AccelerometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerReading {}
impl ::core::fmt::Debug for AccelerometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AccelerometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReading;{b9fe7acb-d351-40af-8bb6-7aa9ae641fb7})");
}
impl ::core::clone::Clone for AccelerometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccelerometerReading {
    type Vtable = IAccelerometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccelerometerReading {
    const IID: ::windows::core::GUID = <IAccelerometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReading";
}
::windows::imp::interface_hierarchy!(AccelerometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AccelerometerReading {}
unsafe impl ::core::marker::Sync for AccelerometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AccelerometerReadingChangedEventArgs(::windows::core::IUnknown);
impl AccelerometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AccelerometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AccelerometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for AccelerometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AccelerometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs;{0095c65b-b6ac-475a-9f44-8b32d35a3f25})");
}
impl ::core::clone::Clone for AccelerometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccelerometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IAccelerometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(AccelerometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AccelerometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AccelerometerShakenEventArgs(::windows::core::IUnknown);
impl AccelerometerShakenEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AccelerometerShakenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerShakenEventArgs {}
impl ::core::fmt::Debug for AccelerometerShakenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerShakenEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AccelerometerShakenEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerShakenEventArgs;{95ff01d1-4a28-4f35-98e8-8178aae4084a})");
}
impl ::core::clone::Clone for AccelerometerShakenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccelerometerShakenEventArgs {
    const IID: ::windows::core::GUID = <IAccelerometerShakenEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerShakenEventArgs";
}
::windows::imp::interface_hierarchy!(AccelerometerShakenEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AccelerometerShakenEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerShakenEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensor(::windows::core::IUnknown);
impl ActivitySensor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>();
            (::windows::core::Interface::vtable(this).GetCurrentReadingAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<ActivityType>>();
            (::windows::core::Interface::vtable(this).SubscribedActivities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).PowerInMilliwatts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ActivityType>>();
            (::windows::core::Interface::vtable(this).SupportedActivities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ActivitySensor>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ActivitySensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>();
            (::windows::core::Interface::vtable(this).GetSystemHistoryAsync)(::windows::core::Interface::as_raw(this), fromtime, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>();
            (::windows::core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(::windows::core::Interface::as_raw(this), fromtime, duration, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorStatics<R, F: FnOnce(&IActivitySensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ActivitySensor, IActivitySensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ActivitySensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensor {}
impl ::core::fmt::Debug for ActivitySensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensor;{cd7a630c-fb5f-48eb-b09b-a2708d1c61ef})");
}
impl ::core::clone::Clone for ActivitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensor {
    type Vtable = IActivitySensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensor {
    const IID: ::windows::core::GUID = <IActivitySensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensor";
}
::windows::imp::interface_hierarchy!(ActivitySensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivitySensor {}
unsafe impl ::core::marker::Sync for ActivitySensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensorReading(::windows::core::IUnknown);
impl ActivitySensorReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Activity(&self) -> ::windows::core::Result<ActivityType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivityType>();
            (::windows::core::Interface::vtable(this).Activity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Confidence(&self) -> ::windows::core::Result<ActivitySensorReadingConfidence> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivitySensorReadingConfidence>();
            (::windows::core::Interface::vtable(this).Confidence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReading {}
impl ::core::fmt::Debug for ActivitySensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReading;{85125a96-1472-40a2-b2ae-e1ef29226c78})");
}
impl ::core::clone::Clone for ActivitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensorReading {
    type Vtable = IActivitySensorReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensorReading {
    const IID: ::windows::core::GUID = <IActivitySensorReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReading";
}
::windows::imp::interface_hierarchy!(ActivitySensorReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivitySensorReading {}
unsafe impl ::core::marker::Sync for ActivitySensorReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensorReadingChangeReport(::windows::core::IUnknown);
impl ActivitySensorReadingChangeReport {
    pub fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivitySensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReadingChangeReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReadingChangeReport {}
impl ::core::fmt::Debug for ActivitySensorReadingChangeReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingChangeReport").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorReadingChangeReport {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangeReport;{4f3c2915-d93b-47bd-960a-f20fb2f322b9})");
}
impl ::core::clone::Clone for ActivitySensorReadingChangeReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensorReadingChangeReport {
    const IID: ::windows::core::GUID = <IActivitySensorReadingChangeReport as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangeReport";
}
::windows::imp::interface_hierarchy!(ActivitySensorReadingChangeReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivitySensorReadingChangeReport {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangeReport {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensorReadingChangedEventArgs(::windows::core::IUnknown);
impl ActivitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivitySensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for ActivitySensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs;{de386717-aeb6-4ec7-946a-d9cc19b951ec})");
}
impl ::core::clone::Clone for ActivitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IActivitySensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(ActivitySensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensorTriggerDetails(::windows::core::IUnknown);
impl ActivitySensorTriggerDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>();
            (::windows::core::Interface::vtable(this).ReadReports)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ActivitySensorTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorTriggerDetails {}
impl ::core::fmt::Debug for ActivitySensorTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorTriggerDetails;{2c9e6612-b9ca-4677-b263-243297f79d3a})");
}
impl ::core::clone::Clone for ActivitySensorTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensorTriggerDetails {
    const IID: ::windows::core::GUID = <IActivitySensorTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorTriggerDetails";
}
::windows::imp::interface_hierarchy!(ActivitySensorTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivitySensorTriggerDetails {}
unsafe impl ::core::marker::Sync for ActivitySensorTriggerDetails {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Altimeter(::windows::core::IUnknown);
impl Altimeter {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AltimeterReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAltimeter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Altimeter> {
        Self::IAltimeterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Altimeter>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAltimeterStatics<R, F: FnOnce(&IAltimeterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Altimeter, IAltimeterStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Altimeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Altimeter {}
impl ::core::fmt::Debug for Altimeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Altimeter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Altimeter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Altimeter;{72f057fd-8f04-49f1-b4a7-f4e363b701a2})");
}
impl ::core::clone::Clone for Altimeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Altimeter {
    type Vtable = IAltimeter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Altimeter {
    const IID: ::windows::core::GUID = <IAltimeter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Altimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.Altimeter";
}
::windows::imp::interface_hierarchy!(Altimeter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Altimeter {}
unsafe impl ::core::marker::Sync for Altimeter {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AltimeterReading(::windows::core::IUnknown);
impl AltimeterReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeChangeInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AltitudeChangeInMeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AltimeterReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AltimeterReading {}
impl ::core::fmt::Debug for AltimeterReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltimeterReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AltimeterReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReading;{fbe8ef73-7f5e-48c8-aa1a-f1f3befc1144})");
}
impl ::core::clone::Clone for AltimeterReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AltimeterReading {
    type Vtable = IAltimeterReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AltimeterReading {
    const IID: ::windows::core::GUID = <IAltimeterReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReading";
}
::windows::imp::interface_hierarchy!(AltimeterReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AltimeterReading {}
unsafe impl ::core::marker::Sync for AltimeterReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AltimeterReadingChangedEventArgs(::windows::core::IUnknown);
impl AltimeterReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AltimeterReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AltimeterReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AltimeterReadingChangedEventArgs {}
impl ::core::fmt::Debug for AltimeterReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltimeterReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AltimeterReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReadingChangedEventArgs;{7069d077-446d-47f7-998c-ebc23b45e4a2})");
}
impl ::core::clone::Clone for AltimeterReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AltimeterReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IAltimeterReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(AltimeterReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AltimeterReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AltimeterReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Barometer(::windows::core::IUnknown);
impl Barometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BarometerReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBarometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<BarometerDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<IBarometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BarometerDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Barometer> {
        Self::IBarometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Barometer>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Barometer>> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Barometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarometerStatics<R, F: FnOnce(&IBarometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Barometer, IBarometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarometerStatics2<R, F: FnOnce(&IBarometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Barometer, IBarometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Barometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Barometer {}
impl ::core::fmt::Debug for Barometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Barometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Barometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Barometer;{934475a8-78bf-452f-b017-f0209ce6dab4})");
}
impl ::core::clone::Clone for Barometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Barometer {
    type Vtable = IBarometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Barometer {
    const IID: ::windows::core::GUID = <IBarometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Barometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Barometer";
}
::windows::imp::interface_hierarchy!(Barometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Barometer {}
unsafe impl ::core::marker::Sync for Barometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct BarometerDataThreshold(::windows::core::IUnknown);
impl BarometerDataThreshold {
    pub fn Hectopascals(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Hectopascals)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHectopascals(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHectopascals)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BarometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerDataThreshold {}
impl ::core::fmt::Debug for BarometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BarometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerDataThreshold;{076b952c-cb62-5a90-a0d1-f85e4a936394})");
}
impl ::core::clone::Clone for BarometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BarometerDataThreshold {
    const IID: ::windows::core::GUID = <IBarometerDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerDataThreshold";
}
::windows::imp::interface_hierarchy!(BarometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarometerDataThreshold {}
unsafe impl ::core::marker::Sync for BarometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct BarometerReading(::windows::core::IUnknown);
impl BarometerReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StationPressureInHectopascals(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).StationPressureInHectopascals)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BarometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerReading {}
impl ::core::fmt::Debug for BarometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BarometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReading;{f5b9d2e6-1df6-4a1a-a7ad-321d4f5db247})");
}
impl ::core::clone::Clone for BarometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BarometerReading {
    type Vtable = IBarometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BarometerReading {
    const IID: ::windows::core::GUID = <IBarometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReading";
}
::windows::imp::interface_hierarchy!(BarometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarometerReading {}
unsafe impl ::core::marker::Sync for BarometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct BarometerReadingChangedEventArgs(::windows::core::IUnknown);
impl BarometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BarometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BarometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for BarometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BarometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReadingChangedEventArgs;{3d84945f-037b-404f-9bbb-6232d69543c3})");
}
impl ::core::clone::Clone for BarometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BarometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IBarometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(BarometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for BarometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Compass(::windows::core::IUnknown);
impl Compass {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CompassReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICompass2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<ICompass2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ICompass3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<CompassDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<ICompass4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CompassDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ICompassDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Compass> {
        Self::ICompassStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Compass>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Compass>> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Compass>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompassStatics<R, F: FnOnce(&ICompassStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Compass, ICompassStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICompassStatics2<R, F: FnOnce(&ICompassStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Compass, ICompassStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Compass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Compass {}
impl ::core::fmt::Debug for Compass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Compass").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Compass {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Compass;{292ffa94-1b45-403c-ba06-b106dba69a64})");
}
impl ::core::clone::Clone for Compass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Compass {
    type Vtable = ICompass_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Compass {
    const IID: ::windows::core::GUID = <ICompass as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Compass {
    const NAME: &'static str = "Windows.Devices.Sensors.Compass";
}
::windows::imp::interface_hierarchy!(Compass, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Compass {}
unsafe impl ::core::marker::Sync for Compass {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct CompassDataThreshold(::windows::core::IUnknown);
impl CompassDataThreshold {
    pub fn Degrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Degrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CompassDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassDataThreshold {}
impl ::core::fmt::Debug for CompassDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompassDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassDataThreshold;{d15b52b3-d39d-5ec8-b2e4-f193e6ab34ed})");
}
impl ::core::clone::Clone for CompassDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompassDataThreshold {
    type Vtable = ICompassDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompassDataThreshold {
    const IID: ::windows::core::GUID = <ICompassDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassDataThreshold";
}
::windows::imp::interface_hierarchy!(CompassDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompassDataThreshold {}
unsafe impl ::core::marker::Sync for CompassDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct CompassReading(::windows::core::IUnknown);
impl CompassReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HeadingMagneticNorth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).HeadingMagneticNorth)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadingTrueNorth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<f64>>();
            (::windows::core::Interface::vtable(this).HeadingTrueNorth)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HeadingAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::ComInterface::cast::<ICompassReadingHeadingAccuracy>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerAccuracy>();
            (::windows::core::Interface::vtable(this).HeadingAccuracy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CompassReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassReading {}
impl ::core::fmt::Debug for CompassReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompassReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReading;{82911128-513d-4dc9-b781-5eedfbf02d0c})");
}
impl ::core::clone::Clone for CompassReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompassReading {
    type Vtable = ICompassReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompassReading {
    const IID: ::windows::core::GUID = <ICompassReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReading";
}
::windows::imp::interface_hierarchy!(CompassReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompassReading {}
unsafe impl ::core::marker::Sync for CompassReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct CompassReadingChangedEventArgs(::windows::core::IUnknown);
impl CompassReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CompassReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CompassReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassReadingChangedEventArgs {}
impl ::core::fmt::Debug for CompassReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompassReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReadingChangedEventArgs;{8f1549b0-e8bc-4c7e-b009-4e41df137072})");
}
impl ::core::clone::Clone for CompassReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompassReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <ICompassReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(CompassReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompassReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for CompassReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Gyrometer(::windows::core::IUnknown);
impl Gyrometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GyrometerReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<GyrometerDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<IGyrometer4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GyrometerDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IGyrometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Gyrometer> {
        Self::IGyrometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Gyrometer>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Gyrometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGyrometerStatics<R, F: FnOnce(&IGyrometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Gyrometer, IGyrometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGyrometerStatics2<R, F: FnOnce(&IGyrometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Gyrometer, IGyrometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Gyrometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Gyrometer {}
impl ::core::fmt::Debug for Gyrometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gyrometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Gyrometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Gyrometer;{fdb9a9c4-84b1-4ca2-9763-9b589506c70c})");
}
impl ::core::clone::Clone for Gyrometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Gyrometer {
    type Vtable = IGyrometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Gyrometer {
    const IID: ::windows::core::GUID = <IGyrometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Gyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Gyrometer";
}
::windows::imp::interface_hierarchy!(Gyrometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Gyrometer {}
unsafe impl ::core::marker::Sync for Gyrometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct GyrometerDataThreshold(::windows::core::IUnknown);
impl GyrometerDataThreshold {
    pub fn XAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).XAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetXAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).YAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ZAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZAxisInDegreesPerSecond)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for GyrometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerDataThreshold {}
impl ::core::fmt::Debug for GyrometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GyrometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerDataThreshold;{8648b31e-6e52-5259-bbad-242a69dc38c8})");
}
impl ::core::clone::Clone for GyrometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GyrometerDataThreshold {
    const IID: ::windows::core::GUID = <IGyrometerDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerDataThreshold";
}
::windows::imp::interface_hierarchy!(GyrometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GyrometerDataThreshold {}
unsafe impl ::core::marker::Sync for GyrometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct GyrometerReading(::windows::core::IUnknown);
impl GyrometerReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AngularVelocityX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AngularVelocityX)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AngularVelocityY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AngularVelocityY)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AngularVelocityZ(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AngularVelocityZ)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GyrometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerReading {}
impl ::core::fmt::Debug for GyrometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GyrometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReading;{b3d6de5c-1ee4-456f-9de7-e2493b5c8e03})");
}
impl ::core::clone::Clone for GyrometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GyrometerReading {
    type Vtable = IGyrometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GyrometerReading {
    const IID: ::windows::core::GUID = <IGyrometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReading";
}
::windows::imp::interface_hierarchy!(GyrometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GyrometerReading {}
unsafe impl ::core::marker::Sync for GyrometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct GyrometerReadingChangedEventArgs(::windows::core::IUnknown);
impl GyrometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GyrometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GyrometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for GyrometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GyrometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReadingChangedEventArgs;{0fdf1895-6f9e-42ce-8d58-388c0ab8356d})");
}
impl ::core::clone::Clone for GyrometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GyrometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IGyrometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(GyrometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GyrometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for GyrometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HingeAngleReading(::windows::core::IUnknown);
impl HingeAngleReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AngleInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AngleInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HingeAngleReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleReading {}
impl ::core::fmt::Debug for HingeAngleReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HingeAngleReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleReading;{a3cd45b9-1bf1-4f65-a704-e2da04f182c0})");
}
impl ::core::clone::Clone for HingeAngleReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HingeAngleReading {
    type Vtable = IHingeAngleReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HingeAngleReading {
    const IID: ::windows::core::GUID = <IHingeAngleReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleReading";
}
::windows::imp::interface_hierarchy!(HingeAngleReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HingeAngleReading {}
unsafe impl ::core::marker::Sync for HingeAngleReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HingeAngleSensor(::windows::core::IUnknown);
impl HingeAngleSensor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HingeAngleReading>>();
            (::windows::core::Interface::vtable(this).GetCurrentReadingAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinReportThresholdInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MinReportThresholdInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThresholdInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ReportThresholdInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportThresholdInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportThresholdInDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRelatedToAdjacentPanelsAsync(firstpanelid: &::windows::core::HSTRING, secondpanelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>();
            (::windows::core::Interface::vtable(this).GetRelatedToAdjacentPanelsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(firstpanelid), ::core::mem::transmute_copy(secondpanelid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHingeAngleSensorStatics<R, F: FnOnce(&IHingeAngleSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HingeAngleSensor, IHingeAngleSensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HingeAngleSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleSensor {}
impl ::core::fmt::Debug for HingeAngleSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleSensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HingeAngleSensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensor;{e9d3be02-bfdf-437f-8c29-88c77393d309})");
}
impl ::core::clone::Clone for HingeAngleSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HingeAngleSensor {
    type Vtable = IHingeAngleSensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HingeAngleSensor {
    const IID: ::windows::core::GUID = <IHingeAngleSensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensor";
}
::windows::imp::interface_hierarchy!(HingeAngleSensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HingeAngleSensor {}
unsafe impl ::core::marker::Sync for HingeAngleSensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HingeAngleSensorReadingChangedEventArgs(::windows::core::IUnknown);
impl HingeAngleSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<HingeAngleReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HingeAngleReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HingeAngleSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for HingeAngleSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs;{24d9558b-fad0-42b8-a854-78923049a1ba})");
}
impl ::core::clone::Clone for HingeAngleSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HingeAngleSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IHingeAngleSensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(HingeAngleSensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HingeAngleSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for HingeAngleSensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HumanPresenceFeatures(::windows::core::IUnknown);
impl HumanPresenceFeatures {
    pub fn SensorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SensorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedWakeOrLockDistancesInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).SupportedWakeOrLockDistancesInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsWakeOnApproachSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWakeOnApproachSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsLockOnLeaveSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsLockOnLeaveSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAttentionAwareDimmingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAttentionAwareDimmingSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HumanPresenceFeatures {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HumanPresenceFeatures {}
impl ::core::fmt::Debug for HumanPresenceFeatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresenceFeatures").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresenceFeatures {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HumanPresenceFeatures;{bdb09fda-3244-557a-bd29-8b004f59f2cc})");
}
impl ::core::clone::Clone for HumanPresenceFeatures {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HumanPresenceFeatures {
    type Vtable = IHumanPresenceFeatures_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HumanPresenceFeatures {
    const IID: ::windows::core::GUID = <IHumanPresenceFeatures as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HumanPresenceFeatures {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceFeatures";
}
::windows::imp::interface_hierarchy!(HumanPresenceFeatures, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HumanPresenceFeatures {}
unsafe impl ::core::marker::Sync for HumanPresenceFeatures {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HumanPresenceSensor(::windows::core::IUnknown);
impl HumanPresenceSensor {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxDetectableDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).MaxDetectableDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinDetectableDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).MinDetectableDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanPresenceSensorReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<HumanPresenceSensor, HumanPresenceSensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensorid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHumanPresenceSensorStatics<R, F: FnOnce(&IHumanPresenceSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HumanPresenceSensor, IHumanPresenceSensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HumanPresenceSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HumanPresenceSensor {}
impl ::core::fmt::Debug for HumanPresenceSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresenceSensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresenceSensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HumanPresenceSensor;{2116788b-e389-5cc3-9a97-cb17be1008bd})");
}
impl ::core::clone::Clone for HumanPresenceSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HumanPresenceSensor {
    type Vtable = IHumanPresenceSensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HumanPresenceSensor {
    const IID: ::windows::core::GUID = <IHumanPresenceSensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HumanPresenceSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensor";
}
::windows::imp::interface_hierarchy!(HumanPresenceSensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HumanPresenceSensor {}
unsafe impl ::core::marker::Sync for HumanPresenceSensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HumanPresenceSensorReading(::windows::core::IUnknown);
impl HumanPresenceSensorReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Presence(&self) -> ::windows::core::Result<HumanPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanPresence>();
            (::windows::core::Interface::vtable(this).Presence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Engagement(&self) -> ::windows::core::Result<HumanEngagement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanEngagement>();
            (::windows::core::Interface::vtable(this).Engagement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).DistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HumanPresenceSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HumanPresenceSensorReading {}
impl ::core::fmt::Debug for HumanPresenceSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresenceSensorReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresenceSensorReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HumanPresenceSensorReading;{83533bf5-a85a-5d50-8be4-6072d745a3bb})");
}
impl ::core::clone::Clone for HumanPresenceSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HumanPresenceSensorReading {
    type Vtable = IHumanPresenceSensorReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HumanPresenceSensorReading {
    const IID: ::windows::core::GUID = <IHumanPresenceSensorReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HumanPresenceSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReading";
}
::windows::imp::interface_hierarchy!(HumanPresenceSensorReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HumanPresenceSensorReading {}
unsafe impl ::core::marker::Sync for HumanPresenceSensorReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HumanPresenceSensorReadingChangedEventArgs(::windows::core::IUnknown);
impl HumanPresenceSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanPresenceSensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HumanPresenceSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HumanPresenceSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for HumanPresenceSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresenceSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresenceSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HumanPresenceSensorReadingChangedEventArgs;{a9dc4583-fd69-5c5e-ab1f-942204eae2db})");
}
impl ::core::clone::Clone for HumanPresenceSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HumanPresenceSensorReadingChangedEventArgs {
    type Vtable = IHumanPresenceSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HumanPresenceSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IHumanPresenceSensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HumanPresenceSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(HumanPresenceSensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HumanPresenceSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for HumanPresenceSensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct HumanPresenceSettings(::windows::core::IUnknown);
impl HumanPresenceSettings {
    pub fn SensorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SensorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSensorId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSensorId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsWakeOnApproachEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWakeOnApproachEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsWakeOnApproachEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsWakeOnApproachEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WakeOnApproachDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).WakeOnApproachDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWakeOnApproachDistanceInMillimeters<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWakeOnApproachDistanceInMillimeters)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsLockOnLeaveEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsLockOnLeaveEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsLockOnLeaveEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLockOnLeaveEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockOnLeaveDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).LockOnLeaveDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLockOnLeaveDistanceInMillimeters<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLockOnLeaveDistanceInMillimeters)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockOnLeaveTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).LockOnLeaveTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLockOnLeaveTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLockOnLeaveTimeout)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAttentionAwareDimmingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAttentionAwareDimmingEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAttentionAwareDimmingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAttentionAwareDimmingEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSettingsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSettings>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HumanPresenceSettings>>();
            (::windows::core::Interface::vtable(this).GetCurrentSettingsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetCurrentSettings() -> ::windows::core::Result<HumanPresenceSettings> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanPresenceSettings>();
            (::windows::core::Interface::vtable(this).GetCurrentSettings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateSettingsAsync(settings: &HumanPresenceSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).UpdateSettingsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(settings), &mut result__).from_abi(result__)
        })
    }
    pub fn UpdateSettings(settings: &HumanPresenceSettings) -> ::windows::core::Result<()> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).UpdateSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(settings)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSupportedFeaturesForSensorIdAsync(sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceFeatures>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<HumanPresenceFeatures>>();
            (::windows::core::Interface::vtable(this).GetSupportedFeaturesForSensorIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensorid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetSupportedFeaturesForSensorId(sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<HumanPresenceFeatures> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<HumanPresenceFeatures>();
            (::windows::core::Interface::vtable(this).GetSupportedFeaturesForSensorId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensorid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockOnLeaveTimeouts() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::TimeSpan>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).GetSupportedLockOnLeaveTimeouts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SettingsChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SettingsChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSettingsChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSettingsChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IHumanPresenceSettingsStatics<R, F: FnOnce(&IHumanPresenceSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<HumanPresenceSettings, IHumanPresenceSettingsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for HumanPresenceSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HumanPresenceSettings {}
impl ::core::fmt::Debug for HumanPresenceSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresenceSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresenceSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HumanPresenceSettings;{ef4daf5b-07b7-5eb6-86bb-b7ff49ce44fb})");
}
impl ::core::clone::Clone for HumanPresenceSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HumanPresenceSettings {
    type Vtable = IHumanPresenceSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HumanPresenceSettings {
    const IID: ::windows::core::GUID = <IHumanPresenceSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HumanPresenceSettings {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSettings";
}
::windows::imp::interface_hierarchy!(HumanPresenceSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HumanPresenceSettings {}
unsafe impl ::core::marker::Sync for HumanPresenceSettings {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Inclinometer(::windows::core::IUnknown);
impl Inclinometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InclinometerReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorReadingType>();
            (::windows::core::Interface::vtable(this).ReadingType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<InclinometerDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<IInclinometer4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InclinometerDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IInclinometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Inclinometer>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Inclinometer>();
            (::windows::core::Interface::vtable(this).GetDefaultForRelativeReadings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows::core::Result<Inclinometer> {
        Self::IInclinometerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Inclinometer>();
            (::windows::core::Interface::vtable(this).GetDefaultWithSensorReadingType)(::windows::core::Interface::as_raw(this), sensorreadingtype, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), readingtype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Inclinometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics<R, F: FnOnce(&IInclinometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Inclinometer, IInclinometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics2<R, F: FnOnce(&IInclinometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Inclinometer, IInclinometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics3<R, F: FnOnce(&IInclinometerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Inclinometer, IInclinometerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics4<R, F: FnOnce(&IInclinometerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Inclinometer, IInclinometerStatics4> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Inclinometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Inclinometer {}
impl ::core::fmt::Debug for Inclinometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Inclinometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Inclinometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Inclinometer;{2648ca6f-2286-406f-9161-f0c4bd806ebf})");
}
impl ::core::clone::Clone for Inclinometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Inclinometer {
    type Vtable = IInclinometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Inclinometer {
    const IID: ::windows::core::GUID = <IInclinometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Inclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Inclinometer";
}
::windows::imp::interface_hierarchy!(Inclinometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Inclinometer {}
unsafe impl ::core::marker::Sync for Inclinometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct InclinometerDataThreshold(::windows::core::IUnknown);
impl InclinometerDataThreshold {
    pub fn PitchInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).PitchInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPitchInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPitchInDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RollInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).RollInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRollInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRollInDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn YawInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).YawInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYawInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYawInDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for InclinometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerDataThreshold {}
impl ::core::fmt::Debug for InclinometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InclinometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerDataThreshold;{f80a4783-7bfe-545e-bb60-a0ebc47bd2fb})");
}
impl ::core::clone::Clone for InclinometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InclinometerDataThreshold {
    const IID: ::windows::core::GUID = <IInclinometerDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerDataThreshold";
}
::windows::imp::interface_hierarchy!(InclinometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InclinometerDataThreshold {}
unsafe impl ::core::marker::Sync for InclinometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct InclinometerReading(::windows::core::IUnknown);
impl InclinometerReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PitchDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).PitchDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RollDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).RollDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YawDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).YawDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::ComInterface::cast::<IInclinometerReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerAccuracy>();
            (::windows::core::Interface::vtable(this).YawAccuracy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InclinometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerReading {}
impl ::core::fmt::Debug for InclinometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InclinometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReading;{9f44f055-b6f6-497f-b127-1a775e501458})");
}
impl ::core::clone::Clone for InclinometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InclinometerReading {
    type Vtable = IInclinometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InclinometerReading {
    const IID: ::windows::core::GUID = <IInclinometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReading";
}
::windows::imp::interface_hierarchy!(InclinometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InclinometerReading {}
unsafe impl ::core::marker::Sync for InclinometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct InclinometerReadingChangedEventArgs(::windows::core::IUnknown);
impl InclinometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InclinometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InclinometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for InclinometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InclinometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReadingChangedEventArgs;{4ae91dc1-e7eb-4938-8511-ae0d6b440438})");
}
impl ::core::clone::Clone for InclinometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InclinometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IInclinometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(InclinometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InclinometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for InclinometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct LightSensor(::windows::core::IUnknown);
impl LightSensor {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LightSensorReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILightSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<LightSensorDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<ILightSensor3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LightSensorDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILightSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<LightSensor> {
        Self::ILightSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LightSensor>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LightSensor>> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LightSensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILightSensorStatics<R, F: FnOnce(&ILightSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LightSensor, ILightSensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILightSensorStatics2<R, F: FnOnce(&ILightSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LightSensor, ILightSensorStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LightSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensor {}
impl ::core::fmt::Debug for LightSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LightSensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensor;{f84c0718-0c54-47ae-922e-789f57fb03a0})");
}
impl ::core::clone::Clone for LightSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LightSensor {
    type Vtable = ILightSensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LightSensor {
    const IID: ::windows::core::GUID = <ILightSensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensor";
}
::windows::imp::interface_hierarchy!(LightSensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LightSensor {}
unsafe impl ::core::marker::Sync for LightSensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct LightSensorDataThreshold(::windows::core::IUnknown);
impl LightSensorDataThreshold {
    pub fn LuxPercentage(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).LuxPercentage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLuxPercentage(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLuxPercentage)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AbsoluteLux(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).AbsoluteLux)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAbsoluteLux(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAbsoluteLux)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for LightSensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorDataThreshold {}
impl ::core::fmt::Debug for LightSensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LightSensorDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorDataThreshold;{b160afd1-878f-5492-9f2c-33dc3ae584a3})");
}
impl ::core::clone::Clone for LightSensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LightSensorDataThreshold {
    const IID: ::windows::core::GUID = <ILightSensorDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorDataThreshold";
}
::windows::imp::interface_hierarchy!(LightSensorDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LightSensorDataThreshold {}
unsafe impl ::core::marker::Sync for LightSensorDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct LightSensorReading(::windows::core::IUnknown);
impl LightSensorReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IlluminanceInLux(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).IlluminanceInLux)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LightSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorReading {}
impl ::core::fmt::Debug for LightSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LightSensorReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReading;{ffdf6300-227c-4d2b-b302-fc0142485c68})");
}
impl ::core::clone::Clone for LightSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LightSensorReading {
    type Vtable = ILightSensorReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LightSensorReading {
    const IID: ::windows::core::GUID = <ILightSensorReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReading";
}
::windows::imp::interface_hierarchy!(LightSensorReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LightSensorReading {}
unsafe impl ::core::marker::Sync for LightSensorReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct LightSensorReadingChangedEventArgs(::windows::core::IUnknown);
impl LightSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LightSensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LightSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for LightSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LightSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReadingChangedEventArgs;{a3a2f4cf-258b-420c-b8ab-8edd601ecf50})");
}
impl ::core::clone::Clone for LightSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LightSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <ILightSensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(LightSensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LightSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for LightSensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Magnetometer(::windows::core::IUnknown);
impl Magnetometer {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows::core::Result<MagnetometerDataThreshold> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometer4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerDataThreshold>();
            (::windows::core::Interface::vtable(this).ReportThreshold)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<Magnetometer> {
        Self::IMagnetometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Magnetometer>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Magnetometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagnetometerStatics<R, F: FnOnce(&IMagnetometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Magnetometer, IMagnetometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMagnetometerStatics2<R, F: FnOnce(&IMagnetometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Magnetometer, IMagnetometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Magnetometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Magnetometer {}
impl ::core::fmt::Debug for Magnetometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Magnetometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Magnetometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Magnetometer;{484f626e-d3c9-4111-b3f6-2cf1faa418d5})");
}
impl ::core::clone::Clone for Magnetometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Magnetometer {
    type Vtable = IMagnetometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Magnetometer {
    const IID: ::windows::core::GUID = <IMagnetometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Magnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Magnetometer";
}
::windows::imp::interface_hierarchy!(Magnetometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Magnetometer {}
unsafe impl ::core::marker::Sync for Magnetometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct MagnetometerDataThreshold(::windows::core::IUnknown);
impl MagnetometerDataThreshold {
    pub fn XAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).XAxisMicroteslas)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetXAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXAxisMicroteslas)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).YAxisMicroteslas)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetYAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYAxisMicroteslas)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisMicroteslas(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).ZAxisMicroteslas)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZAxisMicroteslas)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for MagnetometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerDataThreshold {}
impl ::core::fmt::Debug for MagnetometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MagnetometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerDataThreshold;{d177cb01-9063-5fa5-b596-b445e9dc3401})");
}
impl ::core::clone::Clone for MagnetometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MagnetometerDataThreshold {
    const IID: ::windows::core::GUID = <IMagnetometerDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerDataThreshold";
}
::windows::imp::interface_hierarchy!(MagnetometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagnetometerDataThreshold {}
unsafe impl ::core::marker::Sync for MagnetometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct MagnetometerReading(::windows::core::IUnknown);
impl MagnetometerReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MagneticFieldX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MagneticFieldX)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MagneticFieldY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MagneticFieldY)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MagneticFieldZ(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MagneticFieldZ)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DirectionalAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerAccuracy>();
            (::windows::core::Interface::vtable(this).DirectionalAccuracy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MagnetometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerReading {}
impl ::core::fmt::Debug for MagnetometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MagnetometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReading;{0c2cc40d-ebfd-4e5c-bb11-afc29b3cae61})");
}
impl ::core::clone::Clone for MagnetometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MagnetometerReading {
    type Vtable = IMagnetometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MagnetometerReading {
    const IID: ::windows::core::GUID = <IMagnetometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReading";
}
::windows::imp::interface_hierarchy!(MagnetometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagnetometerReading {}
unsafe impl ::core::marker::Sync for MagnetometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct MagnetometerReadingChangedEventArgs(::windows::core::IUnknown);
impl MagnetometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MagnetometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for MagnetometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MagnetometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs;{17eae872-2eb9-4ee7-8ad0-3127537d949b})");
}
impl ::core::clone::Clone for MagnetometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MagnetometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IMagnetometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(MagnetometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagnetometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for MagnetometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct OrientationSensor(::windows::core::IUnknown);
impl OrientationSensor {
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensorReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorReadingType>();
            (::windows::core::Interface::vtable(this).ReadingType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReportLatency)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportLatency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensor>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensor>();
            (::windows::core::Interface::vtable(this).GetDefaultForRelativeReadings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensor>();
            (::windows::core::Interface::vtable(this).GetDefaultWithSensorReadingType)(::windows::core::Interface::as_raw(this), sensorreadingtype, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensor>();
            (::windows::core::Interface::vtable(this).GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal)(::windows::core::Interface::as_raw(this), sensorreadingtype, optimizationgoal, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), readingtype, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal)(::windows::core::Interface::as_raw(this), readingtype, optimizationgoal, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<OrientationSensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics<R, F: FnOnce(&IOrientationSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics2<R, F: FnOnce(&IOrientationSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics3<R, F: FnOnce(&IOrientationSensorStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics4<R, F: FnOnce(&IOrientationSensorStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics4> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for OrientationSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensor {}
impl ::core::fmt::Debug for OrientationSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OrientationSensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensor;{5e354635-cf6b-4c63-abd8-10252b0bf6ec})");
}
impl ::core::clone::Clone for OrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OrientationSensor {
    type Vtable = IOrientationSensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OrientationSensor {
    const IID: ::windows::core::GUID = <IOrientationSensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensor";
}
::windows::imp::interface_hierarchy!(OrientationSensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OrientationSensor {}
unsafe impl ::core::marker::Sync for OrientationSensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct OrientationSensorReading(::windows::core::IUnknown);
impl OrientationSensorReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RotationMatrix(&self) -> ::windows::core::Result<SensorRotationMatrix> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorRotationMatrix>();
            (::windows::core::Interface::vtable(this).RotationMatrix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Quaternion(&self) -> ::windows::core::Result<SensorQuaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorQuaternion>();
            (::windows::core::Interface::vtable(this).Quaternion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).PerformanceCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy> {
        let this = &::windows::core::ComInterface::cast::<IOrientationSensorReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MagnetometerAccuracy>();
            (::windows::core::Interface::vtable(this).YawAccuracy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OrientationSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensorReading {}
impl ::core::fmt::Debug for OrientationSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensorReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OrientationSensorReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReading;{4756c993-6595-4897-bcc6-d537ee757564})");
}
impl ::core::clone::Clone for OrientationSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OrientationSensorReading {
    type Vtable = IOrientationSensorReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OrientationSensorReading {
    const IID: ::windows::core::GUID = <IOrientationSensorReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReading";
}
::windows::imp::interface_hierarchy!(OrientationSensorReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OrientationSensorReading {}
unsafe impl ::core::marker::Sync for OrientationSensorReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct OrientationSensorReadingChangedEventArgs(::windows::core::IUnknown);
impl OrientationSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OrientationSensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OrientationSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for OrientationSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OrientationSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs;{012c1186-c3ba-46bc-ae65-7a98996cbfb8})");
}
impl ::core::clone::Clone for OrientationSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OrientationSensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IOrientationSensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(OrientationSensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OrientationSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for OrientationSensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct Pedometer(::windows::core::IUnknown);
impl Pedometer {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).PowerInMilliwatts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReportInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentReadings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>> {
        let this = &::windows::core::ComInterface::cast::<IPedometer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>();
            (::windows::core::Interface::vtable(this).GetCurrentReadings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Pedometer>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Pedometer>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>();
            (::windows::core::Interface::vtable(this).GetSystemHistoryAsync)(::windows::core::Interface::as_raw(this), fromtime, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>();
            (::windows::core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(::windows::core::Interface::as_raw(this), fromtime, duration, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails(triggerdetails: &SensorDataThresholdTriggerDetails) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>> {
        Self::IPedometerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<PedometerReading>>();
            (::windows::core::Interface::vtable(this).GetReadingsFromTriggerDetails)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerdetails), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPedometerStatics<R, F: FnOnce(&IPedometerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Pedometer, IPedometerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPedometerStatics2<R, F: FnOnce(&IPedometerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Pedometer, IPedometerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Pedometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Pedometer {}
impl ::core::fmt::Debug for Pedometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Pedometer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Pedometer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Pedometer;{9a1e013d-3d98-45f8-8920-8e4ecaca5f97})");
}
impl ::core::clone::Clone for Pedometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Pedometer {
    type Vtable = IPedometer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Pedometer {
    const IID: ::windows::core::GUID = <IPedometer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Pedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Pedometer";
}
::windows::imp::interface_hierarchy!(Pedometer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Pedometer {}
unsafe impl ::core::marker::Sync for Pedometer {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct PedometerDataThreshold(::windows::core::IUnknown);
impl PedometerDataThreshold {
    pub fn Create(sensor: &Pedometer, stepgoal: i32) -> ::windows::core::Result<PedometerDataThreshold> {
        Self::IPedometerDataThresholdFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PedometerDataThreshold>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensor), stepgoal, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPedometerDataThresholdFactory<R, F: FnOnce(&IPedometerDataThresholdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PedometerDataThreshold, IPedometerDataThresholdFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PedometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerDataThreshold {}
impl ::core::fmt::Debug for PedometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PedometerDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
}
impl ::core::clone::Clone for PedometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PedometerDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PedometerDataThreshold {
    const IID: ::windows::core::GUID = <ISensorDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PedometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerDataThreshold";
}
::windows::imp::interface_hierarchy!(PedometerDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISensorDataThreshold> for PedometerDataThreshold {}
unsafe impl ::core::marker::Send for PedometerDataThreshold {}
unsafe impl ::core::marker::Sync for PedometerDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct PedometerReading(::windows::core::IUnknown);
impl PedometerReading {
    pub fn StepKind(&self) -> ::windows::core::Result<PedometerStepKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PedometerStepKind>();
            (::windows::core::Interface::vtable(this).StepKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CumulativeSteps(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CumulativeSteps)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CumulativeStepsDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).CumulativeStepsDuration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PedometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerReading {}
impl ::core::fmt::Debug for PedometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PedometerReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReading;{2245dcf4-a8e1-432f-896a-be0dd9b02d24})");
}
impl ::core::clone::Clone for PedometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PedometerReading {
    type Vtable = IPedometerReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PedometerReading {
    const IID: ::windows::core::GUID = <IPedometerReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReading";
}
::windows::imp::interface_hierarchy!(PedometerReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PedometerReading {}
unsafe impl ::core::marker::Sync for PedometerReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct PedometerReadingChangedEventArgs(::windows::core::IUnknown);
impl PedometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<PedometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PedometerReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PedometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for PedometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PedometerReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReadingChangedEventArgs;{f855e47e-abbc-4456-86a8-25cf2b333742})");
}
impl ::core::clone::Clone for PedometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PedometerReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IPedometerReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(PedometerReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PedometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for PedometerReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ProximitySensor(::windows::core::IUnknown);
impl ProximitySensor {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).MaxDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).MinDistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ProximitySensorReading>();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDisplayOnOffController(&self) -> ::windows::core::Result<ProximitySensorDisplayOnOffController> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ProximitySensorDisplayOnOffController>();
            (::windows::core::Interface::vtable(this).CreateDisplayOnOffController)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromId(sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximitySensor> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ProximitySensor>();
            (::windows::core::Interface::vtable(this).FromId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensorid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails(triggerdetails: &SensorDataThresholdTriggerDetails) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>> {
        Self::IProximitySensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>();
            (::windows::core::Interface::vtable(this).GetReadingsFromTriggerDetails)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerdetails), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProximitySensorStatics<R, F: FnOnce(&IProximitySensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ProximitySensor, IProximitySensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProximitySensorStatics2<R, F: FnOnce(&IProximitySensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ProximitySensor, IProximitySensorStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ProximitySensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensor {}
impl ::core::fmt::Debug for ProximitySensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProximitySensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensor;{54c076b8-ecfb-4944-b928-74fc504d47ee})");
}
impl ::core::clone::Clone for ProximitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProximitySensor {
    type Vtable = IProximitySensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProximitySensor {
    const IID: ::windows::core::GUID = <IProximitySensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensor";
}
::windows::imp::interface_hierarchy!(ProximitySensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProximitySensor {}
unsafe impl ::core::marker::Sync for ProximitySensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ProximitySensorDataThreshold(::windows::core::IUnknown);
impl ProximitySensorDataThreshold {
    pub fn Create(sensor: &ProximitySensor) -> ::windows::core::Result<ProximitySensorDataThreshold> {
        Self::IProximitySensorDataThresholdFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ProximitySensorDataThreshold>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sensor), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProximitySensorDataThresholdFactory<R, F: FnOnce(&IProximitySensorDataThresholdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ProximitySensorDataThreshold, IProximitySensorDataThresholdFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ProximitySensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorDataThreshold {}
impl ::core::fmt::Debug for ProximitySensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorDataThreshold").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProximitySensorDataThreshold {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
}
impl ::core::clone::Clone for ProximitySensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProximitySensorDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProximitySensorDataThreshold {
    const IID: ::windows::core::GUID = <ISensorDataThreshold as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProximitySensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDataThreshold";
}
::windows::imp::interface_hierarchy!(ProximitySensorDataThreshold, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISensorDataThreshold> for ProximitySensorDataThreshold {}
unsafe impl ::core::marker::Send for ProximitySensorDataThreshold {}
unsafe impl ::core::marker::Sync for ProximitySensorDataThreshold {}
#[doc = "*Required features: `\"Devices_Sensors\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct ProximitySensorDisplayOnOffController(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl ProximitySensorDisplayOnOffController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ProximitySensorDisplayOnOffController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ProximitySensorDisplayOnOffController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorDisplayOnOffController").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for ProximitySensorDisplayOnOffController {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDisplayOnOffController;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProximitySensorDisplayOnOffController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for ProximitySensorDisplayOnOffController {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::ComInterface for ProximitySensorDisplayOnOffController {
    const IID: ::windows::core::GUID = <super::super::Foundation::IClosable as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ProximitySensorDisplayOnOffController {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDisplayOnOffController";
}
#[cfg(feature = "Foundation")]
::windows::imp::interface_hierarchy!(ProximitySensorDisplayOnOffController, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for ProximitySensorDisplayOnOffController {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ProximitySensorReading(::windows::core::IUnknown);
impl ProximitySensorReading {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDetected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDetected)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).DistanceInMillimeters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProximitySensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorReading {}
impl ::core::fmt::Debug for ProximitySensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorReading").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProximitySensorReading {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReading;{71228d59-132d-4d5f-8ff9-2f0db8751ced})");
}
impl ::core::clone::Clone for ProximitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProximitySensorReading {
    type Vtable = IProximitySensorReading_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProximitySensorReading {
    const IID: ::windows::core::GUID = <IProximitySensorReading as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReading";
}
::windows::imp::interface_hierarchy!(ProximitySensorReading, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProximitySensorReading {}
unsafe impl ::core::marker::Sync for ProximitySensorReading {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ProximitySensorReadingChangedEventArgs(::windows::core::IUnknown);
impl ProximitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ProximitySensorReading>();
            (::windows::core::Interface::vtable(this).Reading)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProximitySensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for ProximitySensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProximitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs;{cfc2f366-c3e8-40fd-8cc3-67e289004938})");
}
impl ::core::clone::Clone for ProximitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProximitySensorReadingChangedEventArgs {
    const IID: ::windows::core::GUID = <IProximitySensorReadingChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs";
}
::windows::imp::interface_hierarchy!(ProximitySensorReadingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProximitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ProximitySensorReadingChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorDataThresholdTriggerDetails(::windows::core::IUnknown);
impl SensorDataThresholdTriggerDetails {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SensorType(&self) -> ::windows::core::Result<SensorType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorType>();
            (::windows::core::Interface::vtable(this).SensorType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SensorDataThresholdTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorDataThresholdTriggerDetails {}
impl ::core::fmt::Debug for SensorDataThresholdTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorDataThresholdTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorDataThresholdTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorDataThresholdTriggerDetails;{9106f1b7-e88d-48b1-bc90-619c7b349391})");
}
impl ::core::clone::Clone for SensorDataThresholdTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SensorDataThresholdTriggerDetails {
    const IID: ::windows::core::GUID = <ISensorDataThresholdTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorDataThresholdTriggerDetails";
}
::windows::imp::interface_hierarchy!(SensorDataThresholdTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SensorDataThresholdTriggerDetails {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTriggerDetails {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorQuaternion(::windows::core::IUnknown);
impl SensorQuaternion {
    pub fn W(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).W)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn X(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).X)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Y(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).Y)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Z(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).Z)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SensorQuaternion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorQuaternion {}
impl ::core::fmt::Debug for SensorQuaternion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorQuaternion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorQuaternion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorQuaternion;{c9c5c827-c71c-46e7-9da3-36a193b232bc})");
}
impl ::core::clone::Clone for SensorQuaternion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SensorQuaternion {
    type Vtable = ISensorQuaternion_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SensorQuaternion {
    const IID: ::windows::core::GUID = <ISensorQuaternion as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorQuaternion";
}
::windows::imp::interface_hierarchy!(SensorQuaternion, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SensorQuaternion {}
unsafe impl ::core::marker::Sync for SensorQuaternion {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorRotationMatrix(::windows::core::IUnknown);
impl SensorRotationMatrix {
    pub fn M11(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M11)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M12(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M12)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M13(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M13)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M21(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M21)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M22(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M22)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M23(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M23)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M31(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M31)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M32(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M32)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn M33(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).M33)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SensorRotationMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorRotationMatrix {}
impl ::core::fmt::Debug for SensorRotationMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorRotationMatrix").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorRotationMatrix {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorRotationMatrix;{0a3d5a67-22f4-4392-9538-65d0bd064aa6})");
}
impl ::core::clone::Clone for SensorRotationMatrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SensorRotationMatrix {
    const IID: ::windows::core::GUID = <ISensorRotationMatrix as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorRotationMatrix";
}
::windows::imp::interface_hierarchy!(SensorRotationMatrix, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SensorRotationMatrix {}
unsafe impl ::core::marker::Sync for SensorRotationMatrix {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SimpleOrientationSensor(::windows::core::IUnknown);
impl SimpleOrientationSensor {
    pub fn GetCurrentOrientation(&self) -> ::windows::core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SimpleOrientation>();
            (::windows::core::Interface::vtable(this).GetCurrentOrientation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OrientationChanged(&self, handler: &super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).OrientationChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOrientationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOrientationChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReadingTransform)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display\"`*"]
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows::core::ComInterface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Display::DisplayOrientations>();
            (::windows::core::Interface::vtable(this).ReadingTransform)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISimpleOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SimpleOrientationSensor> {
        Self::ISimpleOrientationSensorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SimpleOrientationSensor>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISimpleOrientationSensorStatics<R, F: FnOnce(&ISimpleOrientationSensorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISimpleOrientationSensorStatics2<R, F: FnOnce(&ISimpleOrientationSensorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SimpleOrientationSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleOrientationSensor {}
impl ::core::fmt::Debug for SimpleOrientationSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientationSensor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SimpleOrientationSensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensor;{5ff53856-214a-4dee-a3f9-616f1ab06ffd})");
}
impl ::core::clone::Clone for SimpleOrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SimpleOrientationSensor {
    const IID: ::windows::core::GUID = <ISimpleOrientationSensor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensor";
}
::windows::imp::interface_hierarchy!(SimpleOrientationSensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SimpleOrientationSensor {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensor {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(::windows::core::IUnknown);
impl SimpleOrientationSensorOrientationChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SimpleOrientation>();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SimpleOrientationSensorOrientationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleOrientationSensorOrientationChangedEventArgs {}
impl ::core::fmt::Debug for SimpleOrientationSensorOrientationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientationSensorOrientationChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs;{bcd5c660-23d4-4b4c-a22e-ba81ade0c601})");
}
impl ::core::clone::Clone for SimpleOrientationSensorOrientationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SimpleOrientationSensorOrientationChangedEventArgs {
    const IID: ::windows::core::GUID = <ISimpleOrientationSensorOrientationChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs";
}
::windows::imp::interface_hierarchy!(SimpleOrientationSensorOrientationChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SimpleOrientationSensorOrientationChangedEventArgs {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensorOrientationChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl ::core::marker::Copy for AccelerometerReadingType {}
impl ::core::clone::Clone for AccelerometerReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AccelerometerReadingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AccelerometerReadingType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AccelerometerReadingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReadingType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AccelerometerReadingType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.AccelerometerReadingType;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for ActivitySensorReadingConfidence {}
impl ::core::clone::Clone for ActivitySensorReadingConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivitySensorReadingConfidence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ActivitySensorReadingConfidence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ActivitySensorReadingConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingConfidence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorReadingConfidence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivitySensorReadingConfidence;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Stationary: Self = Self(2i32);
    pub const Fidgeting: Self = Self(3i32);
    pub const Walking: Self = Self(4i32);
    pub const Running: Self = Self(5i32);
    pub const InVehicle: Self = Self(6i32);
    pub const Biking: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivityType {}
impl ::core::clone::Clone for ActivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ActivityType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ActivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivityType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivityType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivityType;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HumanEngagement(pub i32);
impl HumanEngagement {
    pub const Unknown: Self = Self(0i32);
    pub const Engaged: Self = Self(1i32);
    pub const Unengaged: Self = Self(2i32);
}
impl ::core::marker::Copy for HumanEngagement {}
impl ::core::clone::Clone for HumanEngagement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HumanEngagement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HumanEngagement {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HumanEngagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanEngagement").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanEngagement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanEngagement;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HumanPresence(pub i32);
impl HumanPresence {
    pub const Unknown: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
    pub const NotPresent: Self = Self(2i32);
}
impl ::core::marker::Copy for HumanPresence {}
impl ::core::clone::Clone for HumanPresence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HumanPresence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HumanPresence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HumanPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HumanPresence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for HumanPresence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanPresence;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl ::core::marker::Copy for MagnetometerAccuracy {}
impl ::core::clone::Clone for MagnetometerAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagnetometerAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MagnetometerAccuracy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MagnetometerAccuracy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.MagnetometerAccuracy;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl ::core::marker::Copy for PedometerStepKind {}
impl ::core::clone::Clone for PedometerStepKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PedometerStepKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PedometerStepKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PedometerStepKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerStepKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PedometerStepKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.PedometerStepKind;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorOptimizationGoal {}
impl ::core::clone::Clone for SensorOptimizationGoal {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorOptimizationGoal {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SensorOptimizationGoal {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SensorOptimizationGoal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorOptimizationGoal").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorOptimizationGoal {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorOptimizationGoal;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorReadingType {}
impl ::core::clone::Clone for SensorReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorReadingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SensorReadingType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SensorReadingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorReadingType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorReadingType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorReadingType;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: Self = Self(0i32);
    pub const ActivitySensor: Self = Self(1i32);
    pub const Barometer: Self = Self(2i32);
    pub const Compass: Self = Self(3i32);
    pub const CustomSensor: Self = Self(4i32);
    pub const Gyroscope: Self = Self(5i32);
    pub const ProximitySensor: Self = Self(6i32);
    pub const Inclinometer: Self = Self(7i32);
    pub const LightSensor: Self = Self(8i32);
    pub const OrientationSensor: Self = Self(9i32);
    pub const Pedometer: Self = Self(10i32);
    pub const RelativeInclinometer: Self = Self(11i32);
    pub const RelativeOrientationSensor: Self = Self(12i32);
    pub const SimpleOrientationSensor: Self = Self(13i32);
}
impl ::core::marker::Copy for SensorType {}
impl ::core::clone::Clone for SensorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SensorType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SensorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorType;i4)");
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl ::core::marker::Copy for SimpleOrientation {}
impl ::core::clone::Clone for SimpleOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SimpleOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SimpleOrientation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SimpleOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SimpleOrientation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SimpleOrientation;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
