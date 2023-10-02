#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensor {
    type Vtable = ICustomSensor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa136f9ad_4034_4b4d_99dd_531aac649c09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensor2 {
    type Vtable = ICustomSensor2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20db3111_ec58_4d9f_bfbd_e77825088510);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensorReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensorReading {
    type Vtable = ICustomSensorReading_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensorReading {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64004f4d_446a_4366_a87a_5f963268ec53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensorReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensorReading2 {
    type Vtable = ICustomSensorReading2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensorReading2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x223c98ea_bf73_4992_9a48_d3c897594ccb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensorReadingChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b202023_cffd_4cc1_8ff0_e21823d76fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSensorStatics {
    type Vtable = ICustomSensorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSensorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x992052cf_f422_4c7d_836b_e7dc74a7124b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: ::windows_core::GUID, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CustomSensor(::windows_core::IUnknown);
impl CustomSensor {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<CustomSensor, CustomSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICustomSensor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector(interfaceid: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), interfaceid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(sensorid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<CustomSensor>> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sensorid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomSensorStatics<R, F: FnOnce(&ICustomSensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CustomSensor, ICustomSensorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CustomSensor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensor;{a136f9ad-4034-4b4d-99dd-531aac649c09})");
}
unsafe impl ::windows_core::Interface for CustomSensor {
    type Vtable = ICustomSensor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomSensor {
    const IID: ::windows_core::GUID = <ICustomSensor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensor";
}
::windows_core::imp::interface_hierarchy!(CustomSensor, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CustomSensor {}
unsafe impl ::core::marker::Sync for CustomSensor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CustomSensorReading(::windows_core::IUnknown);
impl CustomSensorReading {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<ICustomSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CustomSensorReading {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensorReading;{64004f4d-446a-4366-a87a-5f963268ec53})");
}
unsafe impl ::windows_core::Interface for CustomSensorReading {
    type Vtable = ICustomSensorReading_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomSensorReading {
    const IID: ::windows_core::GUID = <ICustomSensorReading as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReading";
}
::windows_core::imp::interface_hierarchy!(CustomSensorReading, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CustomSensorReading {}
unsafe impl ::core::marker::Sync for CustomSensorReading {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CustomSensorReadingChangedEventArgs(::windows_core::IUnknown);
impl CustomSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CustomSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs;{6b202023-cffd-4cc1-8ff0-e21823d76fcc})");
}
unsafe impl ::windows_core::Interface for CustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomSensorReadingChangedEventArgs {
    const IID: ::windows_core::GUID = <ICustomSensorReadingChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CustomSensorReadingChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CustomSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for CustomSensorReadingChangedEventArgs {}
