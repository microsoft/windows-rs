#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Accelerometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Accelerometer, windows_core::IUnknown, windows_core::IInspectable);
impl Accelerometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Shaken<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shaken)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShaken(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShaken)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<AccelerometerReadingType> {
        let this = &windows_core::Interface::cast::<IAccelerometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<AccelerometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IAccelerometer5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccelerometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithAccelerometerReadingType(readingtype: AccelerometerReadingType) -> windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithAccelerometerReadingType)(windows_core::Interface::as_raw(this), readingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Accelerometer>> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: AccelerometerReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn IAccelerometerStatics<R, F: FnOnce(&IAccelerometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAccelerometerStatics2<R, F: FnOnce(&IAccelerometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAccelerometerStatics3<R, F: FnOnce(&IAccelerometerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Accelerometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometer>();
}
unsafe impl windows_core::Interface for Accelerometer {
    type Vtable = <IAccelerometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccelerometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Accelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Accelerometer";
}
unsafe impl Send for Accelerometer {}
unsafe impl Sync for Accelerometer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccelerometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerDataThreshold {
    pub fn XAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AccelerometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerDataThreshold>();
}
unsafe impl windows_core::Interface for AccelerometerDataThreshold {
    type Vtable = <IAccelerometerDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccelerometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerDataThreshold";
}
unsafe impl Send for AccelerometerDataThreshold {}
unsafe impl Sync for AccelerometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccelerometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationZ(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AccelerometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerReading>();
}
unsafe impl windows_core::Interface for AccelerometerReading {
    type Vtable = <IAccelerometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccelerometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReading";
}
unsafe impl Send for AccelerometerReading {}
unsafe impl Sync for AccelerometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccelerometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AccelerometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for AccelerometerReadingChangedEventArgs {
    type Vtable = <IAccelerometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccelerometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs";
}
unsafe impl Send for AccelerometerReadingChangedEventArgs {}
unsafe impl Sync for AccelerometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl windows_core::TypeKind for AccelerometerReadingType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AccelerometerReadingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.AccelerometerReadingType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccelerometerShakenEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerShakenEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerShakenEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AccelerometerShakenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerShakenEventArgs>();
}
unsafe impl windows_core::Interface for AccelerometerShakenEventArgs {
    type Vtable = <IAccelerometerShakenEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccelerometerShakenEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerShakenEventArgs";
}
unsafe impl Send for AccelerometerShakenEventArgs {}
unsafe impl Sync for AccelerometerShakenEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensor, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensor {
    pub fn GetCurrentReadingAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ActivitySensorReading>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubscribedActivities(&self) -> windows_core::Result<windows_collections::IVector<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PowerInMilliwatts(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerInMilliwatts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SupportedActivities(&self) -> windows_core::Result<windows_collections::IVectorView<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryAsync)(windows_core::Interface::as_raw(this), fromtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(windows_core::Interface::as_raw(this), fromtime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IActivitySensorStatics<R, F: FnOnce(&IActivitySensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ActivitySensor, IActivitySensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ActivitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensor>();
}
unsafe impl windows_core::Interface for ActivitySensor {
    type Vtable = <IActivitySensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivitySensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensor";
}
unsafe impl Send for ActivitySensor {}
unsafe impl Sync for ActivitySensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Activity(&self) -> windows_core::Result<ActivityType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Confidence(&self) -> windows_core::Result<ActivitySensorReadingConfidence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Confidence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReading>();
}
unsafe impl windows_core::Interface for ActivitySensorReading {
    type Vtable = <IActivitySensorReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivitySensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReading";
}
unsafe impl Send for ActivitySensorReading {}
unsafe impl Sync for ActivitySensorReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySensorReadingChangeReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReadingChangeReport, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReadingChangeReport {
    pub fn Reading(&self) -> windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReadingChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReadingChangeReport>();
}
unsafe impl windows_core::Interface for ActivitySensorReadingChangeReport {
    type Vtable = <IActivitySensorReadingChangeReport as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivitySensorReadingChangeReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangeReport";
}
unsafe impl Send for ActivitySensorReadingChangeReport {}
unsafe impl Sync for ActivitySensorReadingChangeReport {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for ActivitySensorReadingChangedEventArgs {
    type Vtable = <IActivitySensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivitySensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs";
}
unsafe impl Send for ActivitySensorReadingChangedEventArgs {}
unsafe impl Sync for ActivitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl windows_core::TypeKind for ActivitySensorReadingConfidence {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActivitySensorReadingConfidence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivitySensorReadingConfidence;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySensorTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorTriggerDetails {
    pub fn ReadReports(&self) -> windows_core::Result<windows_collections::IVectorView<ActivitySensorReadingChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorTriggerDetails>();
}
unsafe impl windows_core::Interface for ActivitySensorTriggerDetails {
    type Vtable = <IActivitySensorTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivitySensorTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorTriggerDetails";
}
unsafe impl Send for ActivitySensorTriggerDetails {}
unsafe impl Sync for ActivitySensorTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for ActivityType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActivityType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivityType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptiveDimmingOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveDimmingOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveDimmingOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveDimmingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveDimmingOptions>();
}
unsafe impl windows_core::Interface for AdaptiveDimmingOptions {
    type Vtable = <IAdaptiveDimmingOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAdaptiveDimmingOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveDimmingOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.AdaptiveDimmingOptions";
}
unsafe impl Send for AdaptiveDimmingOptions {}
unsafe impl Sync for AdaptiveDimmingOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Altimeter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Altimeter, windows_core::IUnknown, windows_core::IInspectable);
impl Altimeter {
    pub fn GetCurrentReading(&self) -> windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDefault() -> windows_core::Result<Altimeter> {
        Self::IAltimeterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAltimeterStatics<R, F: FnOnce(&IAltimeterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Altimeter, IAltimeterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Altimeter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeter>();
}
unsafe impl windows_core::Interface for Altimeter {
    type Vtable = <IAltimeter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAltimeter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Altimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.Altimeter";
}
unsafe impl Send for Altimeter {}
unsafe impl Sync for Altimeter {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AltimeterReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AltimeterReading, windows_core::IUnknown, windows_core::IInspectable);
impl AltimeterReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeChangeInMeters(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeChangeInMeters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AltimeterReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeterReading>();
}
unsafe impl windows_core::Interface for AltimeterReading {
    type Vtable = <IAltimeterReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAltimeterReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReading";
}
unsafe impl Send for AltimeterReading {}
unsafe impl Sync for AltimeterReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AltimeterReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AltimeterReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AltimeterReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AltimeterReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeterReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for AltimeterReadingChangedEventArgs {
    type Vtable = <IAltimeterReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAltimeterReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReadingChangedEventArgs";
}
unsafe impl Send for AltimeterReadingChangedEventArgs {}
unsafe impl Sync for AltimeterReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Barometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Barometer, windows_core::IUnknown, windows_core::IInspectable);
impl Barometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<BarometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IBarometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Barometer> {
        Self::IBarometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Barometer>> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn IBarometerStatics<R, F: FnOnce(&IBarometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Barometer, IBarometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBarometerStatics2<R, F: FnOnce(&IBarometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Barometer, IBarometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Barometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometer>();
}
unsafe impl windows_core::Interface for Barometer {
    type Vtable = <IBarometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBarometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Barometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Barometer";
}
unsafe impl Send for Barometer {}
unsafe impl Sync for Barometer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerDataThreshold {
    pub fn Hectopascals(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hectopascals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHectopascals(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHectopascals)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BarometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerDataThreshold>();
}
unsafe impl windows_core::Interface for BarometerDataThreshold {
    type Vtable = <IBarometerDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBarometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerDataThreshold";
}
unsafe impl Send for BarometerDataThreshold {}
unsafe impl Sync for BarometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StationPressureInHectopascals(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StationPressureInHectopascals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerReading>();
}
unsafe impl windows_core::Interface for BarometerReading {
    type Vtable = <IBarometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBarometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReading";
}
unsafe impl Send for BarometerReading {}
unsafe impl Sync for BarometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for BarometerReadingChangedEventArgs {
    type Vtable = <IBarometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBarometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReadingChangedEventArgs";
}
unsafe impl Send for BarometerReadingChangedEventArgs {}
unsafe impl Sync for BarometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compass(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Compass, windows_core::IUnknown, windows_core::IInspectable);
impl Compass {
    pub fn GetCurrentReading(&self) -> windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<CompassDataThreshold> {
        let this = &windows_core::Interface::cast::<ICompass4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICompassDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Compass> {
        Self::ICompassStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Compass>> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICompassStatics<R, F: FnOnce(&ICompassStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Compass, ICompassStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ICompassStatics2<R, F: FnOnce(&ICompassStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Compass, ICompassStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Compass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompass>();
}
unsafe impl windows_core::Interface for Compass {
    type Vtable = <ICompass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Compass {
    const NAME: &'static str = "Windows.Devices.Sensors.Compass";
}
unsafe impl Send for Compass {}
unsafe impl Sync for Compass {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompassDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl CompassDataThreshold {
    pub fn Degrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Degrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CompassDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassDataThreshold>();
}
unsafe impl windows_core::Interface for CompassDataThreshold {
    type Vtable = <ICompassDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompassDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassDataThreshold";
}
unsafe impl Send for CompassDataThreshold {}
unsafe impl Sync for CompassDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompassReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassReading, windows_core::IUnknown, windows_core::IInspectable);
impl CompassReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HeadingMagneticNorth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingMagneticNorth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HeadingTrueNorth(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingTrueNorth)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadingAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<ICompassReadingHeadingAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CompassReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassReading>();
}
unsafe impl windows_core::Interface for CompassReading {
    type Vtable = <ICompassReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompassReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReading";
}
unsafe impl Send for CompassReading {}
unsafe impl Sync for CompassReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompassReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CompassReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CompassReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for CompassReadingChangedEventArgs {
    type Vtable = <ICompassReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompassReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReadingChangedEventArgs";
}
unsafe impl Send for CompassReadingChangedEventArgs {}
unsafe impl Sync for CompassReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetectedPerson(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DetectedPerson, windows_core::IUnknown, windows_core::IInspectable);
impl DetectedPerson {
    pub fn Engagement(&self) -> windows_core::Result<HumanEngagement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadOrientation(&self) -> windows_core::Result<HeadOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadOrientation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadPosition(&self) -> windows_core::Result<HeadPosition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadPosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PersonId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersonId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DetectedPerson {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDetectedPerson>();
}
unsafe impl windows_core::Interface for DetectedPerson {
    type Vtable = <IDetectedPerson as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDetectedPerson as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DetectedPerson {
    const NAME: &'static str = "Windows.Devices.Sensors.DetectedPerson";
}
unsafe impl Send for DetectedPerson {}
unsafe impl Sync for DetectedPerson {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gyrometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Gyrometer, windows_core::IUnknown, windows_core::IInspectable);
impl Gyrometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<GyrometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IGyrometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IGyrometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Gyrometer> {
        Self::IGyrometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Gyrometer>> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGyrometerStatics<R, F: FnOnce(&IGyrometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gyrometer, IGyrometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGyrometerStatics2<R, F: FnOnce(&IGyrometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gyrometer, IGyrometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Gyrometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometer>();
}
unsafe impl windows_core::Interface for Gyrometer {
    type Vtable = <IGyrometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGyrometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Gyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Gyrometer";
}
unsafe impl Send for Gyrometer {}
unsafe impl Sync for Gyrometer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GyrometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerDataThreshold {
    pub fn XAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for GyrometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerDataThreshold>();
}
unsafe impl windows_core::Interface for GyrometerDataThreshold {
    type Vtable = <IGyrometerDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGyrometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerDataThreshold";
}
unsafe impl Send for GyrometerDataThreshold {}
unsafe impl Sync for GyrometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GyrometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityZ(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GyrometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerReading>();
}
unsafe impl windows_core::Interface for GyrometerReading {
    type Vtable = <IGyrometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGyrometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReading";
}
unsafe impl Send for GyrometerReading {}
unsafe impl Sync for GyrometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GyrometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GyrometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for GyrometerReadingChangedEventArgs {
    type Vtable = <IGyrometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGyrometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReadingChangedEventArgs";
}
unsafe impl Send for GyrometerReadingChangedEventArgs {}
unsafe impl Sync for GyrometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadOrientation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HeadOrientation, windows_core::IUnknown, windows_core::IInspectable);
impl HeadOrientation {
    pub fn RollInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PitchInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HeadOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadOrientation>();
}
unsafe impl windows_core::Interface for HeadOrientation {
    type Vtable = <IHeadOrientation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHeadOrientation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HeadOrientation {
    const NAME: &'static str = "Windows.Devices.Sensors.HeadOrientation";
}
unsafe impl Send for HeadOrientation {}
unsafe impl Sync for HeadOrientation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadPosition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HeadPosition, windows_core::IUnknown, windows_core::IInspectable);
impl HeadPosition {
    pub fn AzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HeadPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadPosition>();
}
unsafe impl windows_core::Interface for HeadPosition {
    type Vtable = <IHeadPosition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHeadPosition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HeadPosition {
    const NAME: &'static str = "Windows.Devices.Sensors.HeadPosition";
}
unsafe impl Send for HeadPosition {}
unsafe impl Sync for HeadPosition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HingeAngleReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleReading, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngleInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngleInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HingeAngleReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleReading>();
}
unsafe impl windows_core::Interface for HingeAngleReading {
    type Vtable = <IHingeAngleReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHingeAngleReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleReading";
}
unsafe impl Send for HingeAngleReading {}
unsafe impl Sync for HingeAngleReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HingeAngleSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleSensor, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleSensor {
    pub fn GetCurrentReadingAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<HingeAngleReading>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MinReportThresholdInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinReportThresholdInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThresholdInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThresholdInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportThresholdInDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportThresholdInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetRelatedToAdjacentPanelsAsync(firstpanelid: &windows_core::HSTRING, secondpanelid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRelatedToAdjacentPanelsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(firstpanelid), core::mem::transmute_copy(secondpanelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IHingeAngleSensorStatics<R, F: FnOnce(&IHingeAngleSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HingeAngleSensor, IHingeAngleSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HingeAngleSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleSensor>();
}
unsafe impl windows_core::Interface for HingeAngleSensor {
    type Vtable = <IHingeAngleSensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHingeAngleSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensor";
}
unsafe impl Send for HingeAngleSensor {}
unsafe impl Sync for HingeAngleSensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HingeAngleSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<HingeAngleReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for HingeAngleSensorReadingChangedEventArgs {
    type Vtable = <IHingeAngleSensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHingeAngleSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs";
}
unsafe impl Send for HingeAngleSensorReadingChangedEventArgs {}
unsafe impl Sync for HingeAngleSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HumanEngagement(pub i32);
impl HumanEngagement {
    pub const Unknown: Self = Self(0i32);
    pub const Engaged: Self = Self(1i32);
    pub const Unengaged: Self = Self(2i32);
}
impl windows_core::TypeKind for HumanEngagement {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HumanEngagement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanEngagement;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HumanPresence(pub i32);
impl HumanPresence {
    pub const Unknown: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
    pub const NotPresent: Self = Self(2i32);
}
impl windows_core::TypeKind for HumanPresence {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HumanPresence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanPresence;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceFeatures(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceFeatures, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceFeatures {
    pub fn SensorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SupportedWakeOrLockDistancesInMillimeters(&self) -> windows_core::Result<windows_collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedWakeOrLockDistancesInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWakeOnApproachSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeOnApproachSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLockOnLeaveSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLockOnLeaveSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAttentionAwareDimmingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAttentionAwareDimmingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAdaptiveDimmingSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceFeatures2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdaptiveDimmingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOnlookerDetectionSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceFeatures3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnlookerDetectionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceFeatures>();
}
unsafe impl windows_core::Interface for HumanPresenceFeatures {
    type Vtable = <IHumanPresenceFeatures as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceFeatures as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceFeatures {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceFeatures";
}
unsafe impl Send for HumanPresenceFeatures {}
unsafe impl Sync for HumanPresenceFeatures {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensor, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensor {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MaxDetectableDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDetectableDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HumanPresenceSensor, HumanPresenceSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsPresenceSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPresenceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEngagementSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEngagementSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxDetectablePersons(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectablePersons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinDetectableAzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableAzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDetectableAzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableAzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDetectableAltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableAltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDetectableAltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableAltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(sensorid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(sensorid: &windows_core::HSTRING) -> windows_core::Result<HumanPresenceSensor> {
        Self::IHumanPresenceSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<HumanPresenceSensor> {
        Self::IHumanPresenceSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IHumanPresenceSensorStatics<R, F: FnOnce(&IHumanPresenceSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensor, IHumanPresenceSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHumanPresenceSensorStatics2<R, F: FnOnce(&IHumanPresenceSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensor, IHumanPresenceSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HumanPresenceSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensor>();
}
unsafe impl windows_core::Interface for HumanPresenceSensor {
    type Vtable = <IHumanPresenceSensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensor";
}
unsafe impl Send for HumanPresenceSensor {}
unsafe impl Sync for HumanPresenceSensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Presence(&self) -> windows_core::Result<HumanPresence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Presence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Engagement(&self) -> windows_core::Result<HumanEngagement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnlookerPresence(&self) -> windows_core::Result<HumanPresence> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlookerPresence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DetectedPersons(&self) -> windows_core::Result<windows_collections::IVectorView<DetectedPerson>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetectedPersons)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReading>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReading {
    type Vtable = <IHumanPresenceSensorReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReading";
}
unsafe impl Send for HumanPresenceSensorReading {}
unsafe impl Sync for HumanPresenceSensorReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReadingChangedEventArgs {
    type Vtable = <IHumanPresenceSensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReadingChangedEventArgs";
}
unsafe impl Send for HumanPresenceSensorReadingChangedEventArgs {}
unsafe impl Sync for HumanPresenceSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceSensorReadingUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReadingUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReadingUpdate {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensorReadingUpdate, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTimestamp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTimestamp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Presence(&self) -> windows_core::Result<super::super::Foundation::IReference<HumanPresence>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Presence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPresence<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<HumanPresence>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPresence)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Engagement(&self) -> windows_core::Result<super::super::Foundation::IReference<HumanEngagement>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEngagement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<HumanEngagement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEngagement)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OnlookerPresence(&self) -> windows_core::Result<super::super::Foundation::IReference<HumanPresence>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReadingUpdate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlookerPresence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOnlookerPresence<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<HumanPresence>>,
    {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReadingUpdate2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOnlookerPresence)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReadingUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReadingUpdate>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReadingUpdate {
    type Vtable = <IHumanPresenceSensorReadingUpdate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceSensorReadingUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReadingUpdate {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReadingUpdate";
}
unsafe impl Send for HumanPresenceSensorReadingUpdate {}
unsafe impl Sync for HumanPresenceSensorReadingUpdate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanPresenceSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSettings, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSettings {
    pub fn SensorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSensorId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSensorId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsWakeOnApproachEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeOnApproachEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsWakeOnApproachEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsWakeOnApproachEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WakeOnApproachDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WakeOnApproachDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWakeOnApproachDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWakeOnApproachDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsLockOnLeaveEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLockOnLeaveEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLockOnLeaveEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsLockOnLeaveEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LockOnLeaveDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOnLeaveDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLockOnLeaveDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLockOnLeaveDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LockOnLeaveTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOnLeaveTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLockOnLeaveTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLockOnLeaveTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAttentionAwareDimmingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAttentionAwareDimmingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAttentionAwareDimmingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAttentionAwareDimmingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAdaptiveDimmingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdaptiveDimmingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAdaptiveDimmingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAdaptiveDimmingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WakeOptions(&self) -> windows_core::Result<WakeOnApproachOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WakeOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DimmingOptions(&self) -> windows_core::Result<AdaptiveDimmingOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DimmingOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LockOptions(&self) -> windows_core::Result<LockOnLeaveOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsOnlookerDetectionEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnlookerDetectionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOnlookerDetectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsOnlookerDetectionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OnlookerDetectionOptions(&self) -> windows_core::Result<OnlookerDetectionOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlookerDetectionOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentSettingsAsync() -> windows_core::Result<windows_future::IAsyncOperation<HumanPresenceSettings>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettingsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentSettings() -> windows_core::Result<HumanPresenceSettings> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UpdateSettingsAsync<P0>(settings: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<HumanPresenceSettings>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateSettingsAsync)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UpdateSettings<P0>(settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HumanPresenceSettings>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (windows_core::Interface::vtable(this).UpdateSettings)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() })
    }
    pub fn GetSupportedFeaturesForSensorIdAsync(sensorid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<HumanPresenceFeatures>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedFeaturesForSensorIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSupportedFeaturesForSensorId(sensorid: &windows_core::HSTRING) -> windows_core::Result<HumanPresenceFeatures> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedFeaturesForSensorId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSupportedLockOnLeaveTimeouts() -> windows_core::Result<windows_collections::IVectorView<super::super::Foundation::TimeSpan>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedLockOnLeaveTimeouts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SettingsChanged<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveSettingsChanged(token: i64) -> windows_core::Result<()> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSettingsChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    fn IHumanPresenceSettingsStatics<R, F: FnOnce(&IHumanPresenceSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSettings, IHumanPresenceSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HumanPresenceSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSettings>();
}
unsafe impl windows_core::Interface for HumanPresenceSettings {
    type Vtable = <IHumanPresenceSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHumanPresenceSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSettings {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSettings";
}
unsafe impl Send for HumanPresenceSettings {}
unsafe impl Sync for HumanPresenceSettings {}
windows_core::imp::define_interface!(IAccelerometer, IAccelerometer_Vtbl, 0xdce3c591_9754_5774_bd8e_897b301b39ab);
impl windows_core::RuntimeType for IAccelerometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Shaken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveShaken: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer2, IAccelerometer2_Vtbl, 0xee1e6682_9d19_554e_8813_af2eb2dc0d57);
impl windows_core::RuntimeType for IAccelerometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IAccelerometer3, IAccelerometer3_Vtbl, 0x87e0022a_ed80_49eb_bf8a_a4ea31e5cd84);
impl windows_core::RuntimeType for IAccelerometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer4, IAccelerometer4_Vtbl, 0x1d373c4f_42d3_45b2_8144_ab7fb665eb59);
impl windows_core::RuntimeType for IAccelerometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AccelerometerReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer5, IAccelerometer5_Vtbl, 0x64bade5d_55ac_5999_9f71_47e63f6693e1);
impl windows_core::RuntimeType for IAccelerometer5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerDataThreshold, IAccelerometerDataThreshold_Vtbl, 0xff398f92_aacb_516e_995b_f9a38e9d37a6);
impl windows_core::RuntimeType for IAccelerometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetXAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub YAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetYAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ZAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetZAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerDeviceId, IAccelerometerDeviceId_Vtbl, 0x20157bf7_411d_57d3_b65d_daafdd9f6db1);
impl windows_core::RuntimeType for IAccelerometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerReading, IAccelerometerReading_Vtbl, 0xf77990eb_0e82_5a44_a4a8_72ae2cf5bd2d);
impl windows_core::RuntimeType for IAccelerometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AccelerationX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AccelerationY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AccelerationZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerReading2, IAccelerometerReading2_Vtbl, 0x6d7158e6_1362_5f85_9af5_9447bfa24e76);
impl windows_core::RuntimeType for IAccelerometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerReadingChangedEventArgs, IAccelerometerReadingChangedEventArgs_Vtbl, 0xf967a438_98db_58e6_93da_c92b753874a6);
impl windows_core::RuntimeType for IAccelerometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerShakenEventArgs, IAccelerometerShakenEventArgs_Vtbl, 0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
impl windows_core::RuntimeType for IAccelerometerShakenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerShakenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics, IAccelerometerStatics_Vtbl, 0x3d7458ab_a2ac_523c_85cd_b8d59bf55720);
impl windows_core::RuntimeType for IAccelerometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics2, IAccelerometerStatics2_Vtbl, 0xc4c4842f_d86b_4685_b2d7_3396f798d57b);
impl windows_core::RuntimeType for IAccelerometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, AccelerometerReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics3, IAccelerometerStatics3_Vtbl, 0x48648a9a_0ba1_52fe_a4d4_84929c327ae4);
impl windows_core::RuntimeType for IAccelerometerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, AccelerometerReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensor, IActivitySensor_Vtbl, 0x5b6d1d4e_b2f7_5b1e_ab5a_b22d3da69ab1);
impl windows_core::RuntimeType for IActivitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReadingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubscribedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReading, IActivitySensorReading_Vtbl, 0x85125a96_1472_40a2_b2ae_e1ef29226c78);
impl windows_core::RuntimeType for IActivitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Activity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivityType) -> windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivitySensorReadingConfidence) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReadingChangeReport, IActivitySensorReadingChangeReport_Vtbl, 0xd53f4481_454a_591b_853b_057b806052ee);
impl windows_core::RuntimeType for IActivitySensorReadingChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangeReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReadingChangedEventArgs, IActivitySensorReadingChangedEventArgs_Vtbl, 0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
impl windows_core::RuntimeType for IActivitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorStatics, IActivitySensorStatics_Vtbl, 0x74ed0bd5_a77b_51f7_bafc_10b6c71f780f);
impl windows_core::RuntimeType for IActivitySensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemHistoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorTriggerDetails, IActivitySensorTriggerDetails_Vtbl, 0x2c9e6612_b9ca_4677_b263_243297f79d3a);
impl windows_core::RuntimeType for IActivitySensorTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveDimmingOptions, IAdaptiveDimmingOptions_Vtbl, 0xce4b33d2_2169_5285_86e4_7eb7cc9c326d);
impl windows_core::RuntimeType for IAdaptiveDimmingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveDimmingOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeter, IAltimeter_Vtbl, 0xd4108b03_f6bc_5bb6_a6be_52e446ff7e48);
impl windows_core::RuntimeType for IAltimeter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeter2, IAltimeter2_Vtbl, 0xc9471bf9_2add_48f5_9f08_3d0c7660d938);
impl windows_core::RuntimeType for IAltimeter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterReading, IAltimeterReading_Vtbl, 0xa5a9faa1_62a9_5e3a_b72f_aa9924b0a037);
impl windows_core::RuntimeType for IAltimeterReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AltitudeChangeInMeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterReading2, IAltimeterReading2_Vtbl, 0x27d086fa_e0b9_583e_b5eb_8fb6288b7ef6);
impl windows_core::RuntimeType for IAltimeterReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterReadingChangedEventArgs, IAltimeterReadingChangedEventArgs_Vtbl, 0x7069d077_446d_47f7_998c_ebc23b45e4a2);
impl windows_core::RuntimeType for IAltimeterReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterStatics, IAltimeterStatics_Vtbl, 0x73f52468_2232_5996_a4f1_958f54ddd75e);
impl windows_core::RuntimeType for IAltimeterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer, IBarometer_Vtbl, 0x934475a8_78bf_452f_b017_f0209ce6dab4);
impl windows_core::RuntimeType for IBarometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer2, IBarometer2_Vtbl, 0x51389488_63ba_50b3_9b6a_e504b14999eb);
impl windows_core::RuntimeType for IBarometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer3, IBarometer3_Vtbl, 0xb115ceb8_2fa4_50a0_9f37_1702a1ce5ca2);
impl windows_core::RuntimeType for IBarometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerDataThreshold, IBarometerDataThreshold_Vtbl, 0x35a1a1e6_e349_507f_82b5_667af1ad4b3b);
impl windows_core::RuntimeType for IBarometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerReading, IBarometerReading_Vtbl, 0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
impl windows_core::RuntimeType for IBarometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub StationPressureInHectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerReading2, IBarometerReading2_Vtbl, 0x3d5ea1bb_9c94_576a_afd2_38f494973db5);
impl windows_core::RuntimeType for IBarometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerReadingChangedEventArgs, IBarometerReadingChangedEventArgs_Vtbl, 0xf13f9f38_3dbd_509b_9e3d_835bcb4f8657);
impl windows_core::RuntimeType for IBarometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerStatics, IBarometerStatics_Vtbl, 0xb6db52ba_fd4d_54ac_960b_47cb996a49d5);
impl windows_core::RuntimeType for IBarometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerStatics2, IBarometerStatics2_Vtbl, 0x831e5c34_f5f9_5efa_bd91_d2496bd7535c);
impl windows_core::RuntimeType for IBarometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass, ICompass_Vtbl, 0x2b7571cb_d8fd_5d55_a924_5b601ef68169);
impl windows_core::RuntimeType for ICompass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass2, ICompass2_Vtbl, 0x36f26d09_c7d7_434f_b461_979ddfc2322f);
impl windows_core::RuntimeType for ICompass2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(ICompass3, ICompass3_Vtbl, 0xb51147a5_7945_50e4_bad9_85bfe97e6cc6);
impl windows_core::RuntimeType for ICompass3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass4, ICompass4_Vtbl, 0xeda2cb8d_aaed_5e09_b93c_a261ee46c273);
impl windows_core::RuntimeType for ICompass4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassDataThreshold, ICompassDataThreshold_Vtbl, 0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
impl windows_core::RuntimeType for ICompassDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Degrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassDeviceId, ICompassDeviceId_Vtbl, 0xd181ca29_b085_4b1d_870a_4ff57ba74fd4);
impl windows_core::RuntimeType for ICompassDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReading, ICompassReading_Vtbl, 0x1e12c77f_5283_58e0_9e59_ca3de86f8dd9);
impl windows_core::RuntimeType for ICompassReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub HeadingMagneticNorth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HeadingTrueNorth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReading2, ICompassReading2_Vtbl, 0xc8b5de10_be39_53ef_8cfe_247aa80e9bb1);
impl windows_core::RuntimeType for ICompassReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReadingChangedEventArgs, ICompassReadingChangedEventArgs_Vtbl, 0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
impl windows_core::RuntimeType for ICompassReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReadingHeadingAccuracy, ICompassReadingHeadingAccuracy_Vtbl, 0xe761354e_8911_40f7_9e16_6ecc7daec5de);
impl windows_core::RuntimeType for ICompassReadingHeadingAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingHeadingAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeadingAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassStatics, ICompassStatics_Vtbl, 0x0221c2b4_a713_59bc_accf_9ef6a8937ddd);
impl windows_core::RuntimeType for ICompassStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassStatics2, ICompassStatics2_Vtbl, 0x1a80153d_70d8_555a_bed4_7f5e047fcc78);
impl windows_core::RuntimeType for ICompassStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDetectedPerson, IDetectedPerson_Vtbl, 0x168cc0d9_3f05_5029_a0bf_cdcab4be3f9e);
impl windows_core::RuntimeType for IDetectedPerson {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectedPerson_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanEngagement) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeadOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeadPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PersonId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer, IGyrometer_Vtbl, 0xf5e3b06f_95e1_500b_a07f_fefd7bd8724e);
impl windows_core::RuntimeType for IGyrometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer2, IGyrometer2_Vtbl, 0x8424a8c1_bf1b_5b55_84cc_8cfdd5b14577);
impl windows_core::RuntimeType for IGyrometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IGyrometer3, IGyrometer3_Vtbl, 0x5d6f88d5_8fbc_4484_914b_528adfd947b1);
impl windows_core::RuntimeType for IGyrometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer4, IGyrometer4_Vtbl, 0x7f8461c4_7a4a_5149_bc6d_e4786a8facf4);
impl windows_core::RuntimeType for IGyrometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerDataThreshold, IGyrometerDataThreshold_Vtbl, 0x2b5c473e_5cf9_5099_95e7_bbfd5fc3dc5a);
impl windows_core::RuntimeType for IGyrometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetXAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub YAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetYAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ZAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetZAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerDeviceId, IGyrometerDeviceId_Vtbl, 0xcac87d07_3381_5310_9a8f_5616a9b6ccbd);
impl windows_core::RuntimeType for IGyrometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerReading, IGyrometerReading_Vtbl, 0x3b559aa8_91c3_5874_bb02_bda7edde8438);
impl windows_core::RuntimeType for IGyrometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AngularVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AngularVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AngularVelocityZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerReading2, IGyrometerReading2_Vtbl, 0x16afe13c_2b89_44bb_822b_d1e1556ff09b);
impl windows_core::RuntimeType for IGyrometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerReadingChangedEventArgs, IGyrometerReadingChangedEventArgs_Vtbl, 0xf8890bd1_0981_5a4a_84c4_1e6eb6be622b);
impl windows_core::RuntimeType for IGyrometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerStatics, IGyrometerStatics_Vtbl, 0xd79a64f2_c113_5dd0_9d15_66a06e9db802);
impl windows_core::RuntimeType for IGyrometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerStatics2, IGyrometerStatics2_Vtbl, 0xef83f7a1_d700_4204_9613_79c6b161df4e);
impl windows_core::RuntimeType for IGyrometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHeadOrientation, IHeadOrientation_Vtbl, 0x519f54a9_513e_55e8_9c35_3e8da21dee69);
impl windows_core::RuntimeType for IHeadOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadOrientation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHeadPosition, IHeadPosition_Vtbl, 0x585aeb65_cf35_5e6d_a76a_37db131e17de);
impl windows_core::RuntimeType for IHeadPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadPosition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleReading, IHingeAngleReading_Vtbl, 0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
impl windows_core::RuntimeType for IHingeAngleReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AngleInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleSensor, IHingeAngleSensor_Vtbl, 0x48820bab_0ca9_564f_81eb_53d4313443aa);
impl windows_core::RuntimeType for IHingeAngleSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReadingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleSensorReadingChangedEventArgs, IHingeAngleSensorReadingChangedEventArgs_Vtbl, 0x24d9558b_fad0_42b8_a854_78923049a1ba);
impl windows_core::RuntimeType for IHingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleSensorStatics, IHingeAngleSensorStatics_Vtbl, 0x72acb57d_7286_5ce7_9deb_8d19814b7fcc);
impl windows_core::RuntimeType for IHingeAngleSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelatedToAdjacentPanelsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceFeatures, IHumanPresenceFeatures_Vtbl, 0xbdb09fda_3244_557a_bd29_8b004f59f2cc);
impl windows_core::RuntimeType for IHumanPresenceFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceFeatures_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedWakeOrLockDistancesInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsWakeOnApproachSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLockOnLeaveSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAttentionAwareDimmingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceFeatures2, IHumanPresenceFeatures2_Vtbl, 0x08a9cdda_d929_5ec2_81e2_940bafa089cf);
impl windows_core::RuntimeType for IHumanPresenceFeatures2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceFeatures2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAdaptiveDimmingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceFeatures3, IHumanPresenceFeatures3_Vtbl, 0xf8e0973a_ff2e_5c6d_b858_235e6674e50c);
impl windows_core::RuntimeType for IHumanPresenceFeatures3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceFeatures3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOnlookerDetectionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor, IHumanPresenceSensor_Vtbl, 0x6fb295d3_af6f_583a_89dd_f20d60c05417);
impl windows_core::RuntimeType for IHumanPresenceSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDetectableDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDetectableDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor2, IHumanPresenceSensor2_Vtbl, 0x43e195a2_c018_52b7_b06e_5e8320629453);
impl windows_core::RuntimeType for IHumanPresenceSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPresenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEngagementSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor3, IHumanPresenceSensor3_Vtbl, 0xd0b42aba_0f51_59c0_88aa_8ea3dd4721c8);
impl windows_core::RuntimeType for IHumanPresenceSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxDetectablePersons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinDetectableAzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDetectableAzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDetectableAltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDetectableAltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorExtension, IHumanPresenceSensorExtension_Vtbl, 0x4f2bbcef_a03f_5dc1_a981_cbc2a2fcdd3b);
impl windows_core::RuntimeType for IHumanPresenceSensorExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IHumanPresenceSensorExtension, windows_core::IUnknown, windows_core::IInspectable);
impl IHumanPresenceSensorExtension {
    pub fn Initialize(&self, deviceinterface: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceinterface)).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ProcessReading<P0>(&self, reading: P0) -> windows_core::Result<HumanPresenceSensorReadingUpdate>
    where
        P0: windows_core::Param<HumanPresenceSensorReading>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessReading)(windows_core::Interface::as_raw(this), reading.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessReadingTimeoutExpired<P0>(&self, reading: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HumanPresenceSensorReading>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessReadingTimeoutExpired)(windows_core::Interface::as_raw(this), reading.param().abi()).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uninitialize(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Uninitialize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IHumanPresenceSensorExtension {
    const NAME: &'static str = "Windows.Devices.Sensors.IHumanPresenceSensorExtension";
}
pub trait IHumanPresenceSensorExtension_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, deviceInterface: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn ProcessReading(&self, reading: windows_core::Ref<HumanPresenceSensorReading>) -> windows_core::Result<HumanPresenceSensorReadingUpdate>;
    fn ProcessReadingTimeoutExpired(&self, reading: windows_core::Ref<HumanPresenceSensorReading>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IHumanPresenceSensorExtension_Vtbl {
    pub const fn new<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::Initialize(this, core::mem::transmute(&deviceinterface)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn ProcessReading<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reading: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHumanPresenceSensorExtension_Impl::ProcessReading(this, core::mem::transmute_copy(&reading)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessReadingTimeoutExpired<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reading: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::ProcessReadingTimeoutExpired(this, core::mem::transmute_copy(&reading)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::Uninitialize(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHumanPresenceSensorExtension_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHumanPresenceSensorExtension, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            ProcessReading: ProcessReading::<Identity, OFFSET>,
            ProcessReadingTimeoutExpired: ProcessReadingTimeoutExpired::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHumanPresenceSensorExtension as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorExtension_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessReadingTimeoutExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading, IHumanPresenceSensorReading_Vtbl, 0xb1ce6dd4_1e0c_5e94_98a4_0223ccf33f4d);
impl windows_core::RuntimeType for IHumanPresenceSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Presence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanPresence) -> windows_core::HRESULT,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanEngagement) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading2, IHumanPresenceSensorReading2_Vtbl, 0xe900fb5f_f9bb_5372_bc82_b9a3042c9c4b);
impl windows_core::RuntimeType for IHumanPresenceSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading3, IHumanPresenceSensorReading3_Vtbl, 0x85a563d5_c14e_51dc_9236_94de6b76627f);
impl windows_core::RuntimeType for IHumanPresenceSensorReading3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReading3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnlookerPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanPresence) -> windows_core::HRESULT,
    pub DetectedPersons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReadingChangedEventArgs, IHumanPresenceSensorReadingChangedEventArgs_Vtbl, 0xa9dc4583_fd69_5c5e_ab1f_942204eae2db);
impl windows_core::RuntimeType for IHumanPresenceSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReadingUpdate, IHumanPresenceSensorReadingUpdate_Vtbl, 0x9a4512b6_cde6_592f_af55_4d8e4d07bc7e);
impl windows_core::RuntimeType for IHumanPresenceSensorReadingUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReadingUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Presence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEngagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReadingUpdate2, IHumanPresenceSensorReadingUpdate2_Vtbl, 0xaff658d0_578d_54c1_8132_7eb81a16f2f9);
impl windows_core::RuntimeType for IHumanPresenceSensorReadingUpdate2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorReadingUpdate2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnlookerPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOnlookerPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorStatics, IHumanPresenceSensorStatics_Vtbl, 0xec81489e_5c0f_55dd_b1e6_5f6a26db6d9b);
impl windows_core::RuntimeType for IHumanPresenceSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorStatics2, IHumanPresenceSensorStatics2_Vtbl, 0x8440c729_f075_55b4_b893_42444f4d5e87);
impl windows_core::RuntimeType for IHumanPresenceSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettings, IHumanPresenceSettings_Vtbl, 0xc7042591_0afd_501e_8276_fa2453de27f0);
impl windows_core::RuntimeType for IHumanPresenceSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsWakeOnApproachEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsWakeOnApproachEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLockOnLeaveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLockOnLeaveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub LockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockOnLeaveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetLockOnLeaveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub IsAttentionAwareDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAttentionAwareDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettings2, IHumanPresenceSettings2_Vtbl, 0xa7fe1369_152a_5d5b_9b65_a05d68929936);
impl windows_core::RuntimeType for IHumanPresenceSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAdaptiveDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAdaptiveDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WakeOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DimmingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettings3, IHumanPresenceSettings3_Vtbl, 0x8b3d9c30_ad20_507e_b6b2_928b7630f424);
impl windows_core::RuntimeType for IHumanPresenceSettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOnlookerDetectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOnlookerDetectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OnlookerDetectionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettingsStatics, IHumanPresenceSettingsStatics_Vtbl, 0xfddc8c82_0073_58bd_8b6b_3ae52f490c1b);
impl windows_core::RuntimeType for IHumanPresenceSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHumanPresenceSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFeaturesForSensorIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFeaturesForSensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedLockOnLeaveTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettingsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSettingsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer, IInclinometer_Vtbl, 0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
impl windows_core::RuntimeType for IInclinometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer2, IInclinometer2_Vtbl, 0x45e1484d_f85c_5a04_b57e_1d0037686ee9);
impl windows_core::RuntimeType for IInclinometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer3, IInclinometer3_Vtbl, 0x310ac60c_9de7_5c53_b693_104ef3e948a7);
impl windows_core::RuntimeType for IInclinometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer4, IInclinometer4_Vtbl, 0x43852618_8fca_548e_bbf5_5c50412b6aa4);
impl windows_core::RuntimeType for IInclinometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerDataThreshold, IInclinometerDataThreshold_Vtbl, 0x0f32030d_d8c9_5db3_8163_c14305a32d7d);
impl windows_core::RuntimeType for IInclinometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetYawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerDeviceId, IInclinometerDeviceId_Vtbl, 0xe1af2dcd_4d0a_5fb6_8549_5be66665d905);
impl windows_core::RuntimeType for IInclinometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReading, IInclinometerReading_Vtbl, 0x9f44f055_b6f6_497f_b127_1a775e501458);
impl windows_core::RuntimeType for IInclinometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PitchDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RollDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YawDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReading2, IInclinometerReading2_Vtbl, 0x4f164781_e90b_4658_8915_0103e08a805a);
impl windows_core::RuntimeType for IInclinometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReadingChangedEventArgs, IInclinometerReadingChangedEventArgs_Vtbl, 0xba6877fe_5132_5dd6_811e_e3c4cc9fbe63);
impl windows_core::RuntimeType for IInclinometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReadingYawAccuracy, IInclinometerReadingYawAccuracy_Vtbl, 0xb453e880_1fe3_4986_a257_e6ece2723949);
impl windows_core::RuntimeType for IInclinometerReadingYawAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingYawAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics, IInclinometerStatics_Vtbl, 0x68e125bc_c102_554c_9624_798e24a9e548);
impl windows_core::RuntimeType for IInclinometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics2, IInclinometerStatics2_Vtbl, 0x8743af99_c404_5cf8_84f7_f6579b4c7282);
impl windows_core::RuntimeType for IInclinometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics3, IInclinometerStatics3_Vtbl, 0xbd9a4280_b91a_4829_9392_abc0b6bdf2b4);
impl windows_core::RuntimeType for IInclinometerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics4, IInclinometerStatics4_Vtbl, 0xbd6a2339_6b6a_5e8a_97eb_4a022f765d97);
impl windows_core::RuntimeType for IInclinometerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor, ILightSensor_Vtbl, 0xf84c0718_0c54_47ae_922e_789f57fb03a0);
impl windows_core::RuntimeType for ILightSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor2, ILightSensor2_Vtbl, 0x148cfaf5_ddd7_5441_91e2_5db6c6c1816d);
impl windows_core::RuntimeType for ILightSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor3, ILightSensor3_Vtbl, 0x802f03c8_708e_5b50_962c_7dc846b569fa);
impl windows_core::RuntimeType for ILightSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor4, ILightSensor4_Vtbl, 0x714494ab_5743_5c7c_bec1_48a46b33aca6);
impl windows_core::RuntimeType for ILightSensor4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsChromaticitySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorDataThreshold, ILightSensorDataThreshold_Vtbl, 0xf6a8a1f6_fb72_5055_a253_2dea598e05ab);
impl windows_core::RuntimeType for ILightSensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LuxPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorDataThreshold2, ILightSensorDataThreshold2_Vtbl, 0xdbc0285d_6a37_55e8_9d9f_cb5c0f0c7625);
impl windows_core::RuntimeType for ILightSensorDataThreshold2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDataThreshold2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Chromaticity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LightSensorChromaticity) -> windows_core::HRESULT,
    pub SetChromaticity: unsafe extern "system" fn(*mut core::ffi::c_void, LightSensorChromaticity) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorDeviceId, ILightSensorDeviceId_Vtbl, 0x7fee49f8_0afb_4f51_87f0_6c26375ce94f);
impl windows_core::RuntimeType for ILightSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReading, ILightSensorReading_Vtbl, 0xeb88190e_082a_5d6c_94f9_2a42efd54ff5);
impl windows_core::RuntimeType for ILightSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IlluminanceInLux: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReading2, ILightSensorReading2_Vtbl, 0x0aeef3fe_1eac_5c03_b1b1_b67170783471);
impl windows_core::RuntimeType for ILightSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReading3, ILightSensorReading3_Vtbl, 0x3c69e49d_4c87_5007_a51b_6956f046cfb6);
impl windows_core::RuntimeType for ILightSensorReading3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Chromaticity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LightSensorChromaticity) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReadingChangedEventArgs, ILightSensorReadingChangedEventArgs_Vtbl, 0x625fe3ef_1c01_57ed_b8ff_4733e9cea7af);
impl windows_core::RuntimeType for ILightSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorStatics, ILightSensorStatics_Vtbl, 0x2f1ee485_976e_5656_b490_3eeb20ef2f84);
impl windows_core::RuntimeType for ILightSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorStatics2, ILightSensorStatics2_Vtbl, 0xd6923325_bc1a_5f44_a06c_fa9c5985cc93);
impl windows_core::RuntimeType for ILightSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockOnLeaveOptions, ILockOnLeaveOptions_Vtbl, 0x3c6bf8bd_04c1_5829_8d4e_70521755b8be);
impl windows_core::RuntimeType for ILockOnLeaveOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockOnLeaveOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer, IMagnetometer_Vtbl, 0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
impl windows_core::RuntimeType for IMagnetometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer2, IMagnetometer2_Vtbl, 0xb4656c85_26f6_444b_a9e2_a23f966cd368);
impl windows_core::RuntimeType for IMagnetometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IMagnetometer3, IMagnetometer3_Vtbl, 0xbe93db7c_a625_48ef_acf7_fac104832671);
impl windows_core::RuntimeType for IMagnetometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer4, IMagnetometer4_Vtbl, 0xc886ef98_f910_59f6_83ed_e01344b259f7);
impl windows_core::RuntimeType for IMagnetometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerDataThreshold, IMagnetometerDataThreshold_Vtbl, 0x769cbc5c_329e_5d0a_83b1_602cce06b9d1);
impl windows_core::RuntimeType for IMagnetometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetXAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub YAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetYAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ZAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetZAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerDeviceId, IMagnetometerDeviceId_Vtbl, 0x58b498c2_7e4b_404c_9fc5_5de8b40ebae3);
impl windows_core::RuntimeType for IMagnetometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerReading, IMagnetometerReading_Vtbl, 0xc7db9f6a_b9e8_5e26_95ad_3c24c859fc94);
impl windows_core::RuntimeType for IMagnetometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub MagneticFieldX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MagneticFieldY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MagneticFieldZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub DirectionalAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerReading2, IMagnetometerReading2_Vtbl, 0xd4c95c61_61d9_404b_a328_066f177a1409);
impl windows_core::RuntimeType for IMagnetometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerReadingChangedEventArgs, IMagnetometerReadingChangedEventArgs_Vtbl, 0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
impl windows_core::RuntimeType for IMagnetometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerStatics, IMagnetometerStatics_Vtbl, 0x08c92f73_f2ee_5107_a9ff_02563d61ae3c);
impl windows_core::RuntimeType for IMagnetometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerStatics2, IMagnetometerStatics2_Vtbl, 0x2c0819f0_ffc6_4f89_a06f_18fa10792933);
impl windows_core::RuntimeType for IMagnetometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlookerDetectionOptions, IOnlookerDetectionOptions_Vtbl, 0x927f5227_00e1_509c_bed5_bf41b3a54081);
impl windows_core::RuntimeType for IOnlookerDetectionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlookerDetectionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OnlookerDetectionAction) -> windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(*mut core::ffi::c_void, OnlookerDetectionAction) -> windows_core::HRESULT,
    pub BackOnMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OnlookerDetectionBackOnMode) -> windows_core::HRESULT,
    pub SetBackOnMode: unsafe extern "system" fn(*mut core::ffi::c_void, OnlookerDetectionBackOnMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor, IOrientationSensor_Vtbl, 0xb28f0d5d_e318_5787_b71e_1ea5dcf1b45c);
impl windows_core::RuntimeType for IOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor2, IOrientationSensor2_Vtbl, 0x6df73363_eba4_5f1b_9ea2_8864c7c213af);
impl windows_core::RuntimeType for IOrientationSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor3, IOrientationSensor3_Vtbl, 0xd85ac7b7_be90_5f92_ba73_6308683b09d2);
impl windows_core::RuntimeType for IOrientationSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorDeviceId, IOrientationSensorDeviceId_Vtbl, 0xd403b566_b3d2_5c79_8565_c7a35492da52);
impl windows_core::RuntimeType for IOrientationSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReading, IOrientationSensorReading_Vtbl, 0x4756c993_6595_4897_bcc6_d537ee757564);
impl windows_core::RuntimeType for IOrientationSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RotationMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Quaternion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReading2, IOrientationSensorReading2_Vtbl, 0x00576e5f_49f8_4c05_9e07_24fac79408c3);
impl windows_core::RuntimeType for IOrientationSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReadingChangedEventArgs, IOrientationSensorReadingChangedEventArgs_Vtbl, 0x38cab896_0b47_51a7_973f_7810bd3cb794);
impl windows_core::RuntimeType for IOrientationSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReadingYawAccuracy, IOrientationSensorReadingYawAccuracy_Vtbl, 0x3edfd1de_6be9_50dc_83b3_8e5cb2483eab);
impl windows_core::RuntimeType for IOrientationSensorReadingYawAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingYawAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics, IOrientationSensorStatics_Vtbl, 0x10ef8712_fb4c_428a_898b_2765e409e669);
impl windows_core::RuntimeType for IOrientationSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics2, IOrientationSensorStatics2_Vtbl, 0x1c17498a_575d_58dc_afde_7035d4fbd5e9);
impl windows_core::RuntimeType for IOrientationSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics3, IOrientationSensorStatics3_Vtbl, 0xd82ce920_2777_40ff_9f59_d654b085f12f);
impl windows_core::RuntimeType for IOrientationSensorStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, SensorOptimizationGoal, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics4, IOrientationSensorStatics4_Vtbl, 0xb364dd2f_d8a1_5b3c_bd80_823def26309e);
impl windows_core::RuntimeType for IOrientationSensorStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, SensorOptimizationGoal, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometer, IPedometer_Vtbl, 0x4e6e6449_4134_5243_be0c_fe5d00e56190);
impl windows_core::RuntimeType for IPedometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometer2, IPedometer2_Vtbl, 0x172e7cb5_b712_541c_8a6c_4abe97d8ac3b);
impl windows_core::RuntimeType for IPedometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerDataThresholdFactory, IPedometerDataThresholdFactory_Vtbl, 0xcbad8f50_7a54_466b_9010_77a162fca5d7);
impl windows_core::RuntimeType for IPedometerDataThresholdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerDataThresholdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerReading, IPedometerReading_Vtbl, 0xc2c2ba4b_09b5_57d4_96c7_a16001c54421);
impl windows_core::RuntimeType for IPedometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StepKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PedometerStepKind) -> windows_core::HRESULT,
    pub CumulativeSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub CumulativeStepsDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerReadingChangedEventArgs, IPedometerReadingChangedEventArgs_Vtbl, 0xc0daab67_36d8_57b1_9e80_9a61df8123a8);
impl windows_core::RuntimeType for IPedometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerStatics, IPedometerStatics_Vtbl, 0x82980a2f_4083_4dfb_b411_938ea0f4b946);
impl windows_core::RuntimeType for IPedometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemHistoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerStatics2, IPedometerStatics2_Vtbl, 0x0611c744_b49c_5ebe_ac26_e1ac5e2ce263);
impl windows_core::RuntimeType for IPedometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensor, IProximitySensor_Vtbl, 0x54c076b8_ecfb_4944_b928_74fc504d47ee);
impl windows_core::RuntimeType for IProximitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CreateDisplayOnOffController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorDataThresholdFactory, IProximitySensorDataThresholdFactory_Vtbl, 0x6a5b9137_2faf_5759_8035_e6516d16f569);
impl windows_core::RuntimeType for IProximitySensorDataThresholdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorDataThresholdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorReading, IProximitySensorReading_Vtbl, 0x871491b8_6816_5b88_8b50_6bd64825ba35);
impl windows_core::RuntimeType for IProximitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IsDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorReadingChangedEventArgs, IProximitySensorReadingChangedEventArgs_Vtbl, 0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
impl windows_core::RuntimeType for IProximitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorStatics, IProximitySensorStatics_Vtbl, 0x5660ef26_fea5_5b1d_9329_a241327ad527);
impl windows_core::RuntimeType for IProximitySensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorStatics2, IProximitySensorStatics2_Vtbl, 0xc5b994a0_4bdb_554c_9ab2_8c8f26302704);
impl windows_core::RuntimeType for IProximitySensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorDataThreshold, ISensorDataThreshold_Vtbl, 0xcb1ce9cf_bb08_5235_8899_1e76e80b41a7);
impl windows_core::RuntimeType for ISensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
pub trait ISensorDataThreshold_Impl: windows_core::IUnknownImpl {}
impl ISensorDataThreshold_Vtbl {
    pub const fn new<Identity: ISensorDataThreshold_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISensorDataThreshold, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorDataThreshold as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISensorDataThresholdTriggerDetails, ISensorDataThresholdTriggerDetails_Vtbl, 0x1ad91060_f38e_54da_98d8_90520b637efe);
impl windows_core::RuntimeType for ISensorDataThresholdTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SensorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorQuaternion, ISensorQuaternion_Vtbl, 0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
impl windows_core::RuntimeType for ISensorQuaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorQuaternion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub W: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub X: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Z: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorRotationMatrix, ISensorRotationMatrix_Vtbl, 0x00eac29e_aa94_55c9_a269_bf9cab40879d);
impl windows_core::RuntimeType for ISensorRotationMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorRotationMatrix_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub M11: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M13: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M21: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M22: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M23: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M31: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M33: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensor, ISimpleOrientationSensor_Vtbl, 0x15d2d400_ff90_5bab_b2e0_dadd0504a737);
impl windows_core::RuntimeType for ISimpleOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimpleOrientation) -> windows_core::HRESULT,
    pub OrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensor2, ISimpleOrientationSensor2_Vtbl, 0xe1ff6658_7004_5416_88ee_6e057d87530e);
impl windows_core::RuntimeType for ISimpleOrientationSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorDeviceId, ISimpleOrientationSensorDeviceId_Vtbl, 0xb8b8c89c_6561_535e_94bf_beff369f3b6f);
impl windows_core::RuntimeType for ISimpleOrientationSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorOrientationChangedEventArgs, ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl, 0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
impl windows_core::RuntimeType for ISimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimpleOrientation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorStatics, ISimpleOrientationSensorStatics_Vtbl, 0x72ed066f_70aa_40c6_9b1b_3433f7459b4e);
impl windows_core::RuntimeType for ISimpleOrientationSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorStatics2, ISimpleOrientationSensorStatics2_Vtbl, 0x848f9c7f_b138_4e11_8910_a2a2a3b56d83);
impl windows_core::RuntimeType for ISimpleOrientationSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWakeOnApproachOptions, IWakeOnApproachOptions_Vtbl, 0xe5d6cd62_1362_50a5_a586_b702785d8eb9);
impl windows_core::RuntimeType for IWakeOnApproachOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWakeOnApproachOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DisableWhenBatterySaverOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDisableWhenBatterySaverOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inclinometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Inclinometer, windows_core::IUnknown, windows_core::IInspectable);
impl Inclinometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<SensorReadingType> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<InclinometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IInclinometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IInclinometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultForRelativeReadings() -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(windows_core::Interface::as_raw(this), sensorreadingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Inclinometer>> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInclinometerStatics<R, F: FnOnce(&IInclinometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInclinometerStatics2<R, F: FnOnce(&IInclinometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInclinometerStatics3<R, F: FnOnce(&IInclinometerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInclinometerStatics4<R, F: FnOnce(&IInclinometerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Inclinometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometer>();
}
unsafe impl windows_core::Interface for Inclinometer {
    type Vtable = <IInclinometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInclinometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Inclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Inclinometer";
}
unsafe impl Send for Inclinometer {}
unsafe impl Sync for Inclinometer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InclinometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerDataThreshold {
    pub fn PitchInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPitchInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPitchInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RollInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRollInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRollInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YawInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYawInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYawInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InclinometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerDataThreshold>();
}
unsafe impl windows_core::Interface for InclinometerDataThreshold {
    type Vtable = <IInclinometerDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInclinometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerDataThreshold";
}
unsafe impl Send for InclinometerDataThreshold {}
unsafe impl Sync for InclinometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InclinometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PitchDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RollDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn YawDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<IInclinometerReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InclinometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerReading>();
}
unsafe impl windows_core::Interface for InclinometerReading {
    type Vtable = <IInclinometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInclinometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReading";
}
unsafe impl Send for InclinometerReading {}
unsafe impl Sync for InclinometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InclinometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InclinometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for InclinometerReadingChangedEventArgs {
    type Vtable = <IInclinometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInclinometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReadingChangedEventArgs";
}
unsafe impl Send for InclinometerReadingChangedEventArgs {}
unsafe impl Sync for InclinometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LightSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensor, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensor {
    pub fn GetCurrentReading(&self) -> windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<LightSensorDataThreshold> {
        let this = &windows_core::Interface::cast::<ILightSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsChromaticitySupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILightSensor4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChromaticitySupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILightSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<LightSensor> {
        Self::ILightSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<LightSensor>> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ILightSensorStatics<R, F: FnOnce(&ILightSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LightSensor, ILightSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ILightSensorStatics2<R, F: FnOnce(&ILightSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LightSensor, ILightSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LightSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensor>();
}
unsafe impl windows_core::Interface for LightSensor {
    type Vtable = <ILightSensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILightSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensor";
}
unsafe impl Send for LightSensor {}
unsafe impl Sync for LightSensor {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LightSensorChromaticity {
    pub X: f64,
    pub Y: f64,
}
impl windows_core::TypeKind for LightSensorChromaticity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LightSensorChromaticity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Sensors.LightSensorChromaticity;f8;f8)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LightSensorDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorDataThreshold {
    pub fn LuxPercentage(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LuxPercentage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLuxPercentage(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLuxPercentage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AbsoluteLux(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteLux)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAbsoluteLux(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAbsoluteLux)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Chromaticity(&self) -> windows_core::Result<LightSensorChromaticity> {
        let this = &windows_core::Interface::cast::<ILightSensorDataThreshold2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Chromaticity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetChromaticity(&self, value: LightSensorChromaticity) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILightSensorDataThreshold2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChromaticity)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for LightSensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorDataThreshold>();
}
unsafe impl windows_core::Interface for LightSensorDataThreshold {
    type Vtable = <ILightSensorDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILightSensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorDataThreshold";
}
unsafe impl Send for LightSensorDataThreshold {}
unsafe impl Sync for LightSensorDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LightSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IlluminanceInLux(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IlluminanceInLux)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Chromaticity(&self) -> windows_core::Result<LightSensorChromaticity> {
        let this = &windows_core::Interface::cast::<ILightSensorReading3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Chromaticity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LightSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorReading>();
}
unsafe impl windows_core::Interface for LightSensorReading {
    type Vtable = <ILightSensorReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILightSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReading";
}
unsafe impl Send for LightSensorReading {}
unsafe impl Sync for LightSensorReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LightSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LightSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for LightSensorReadingChangedEventArgs {
    type Vtable = <ILightSensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILightSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReadingChangedEventArgs";
}
unsafe impl Send for LightSensorReadingChangedEventArgs {}
unsafe impl Sync for LightSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockOnLeaveOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockOnLeaveOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LockOnLeaveOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for LockOnLeaveOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockOnLeaveOptions>();
}
unsafe impl windows_core::Interface for LockOnLeaveOptions {
    type Vtable = <ILockOnLeaveOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILockOnLeaveOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockOnLeaveOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.LockOnLeaveOptions";
}
unsafe impl Send for LockOnLeaveOptions {}
unsafe impl Sync for LockOnLeaveOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Magnetometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Magnetometer, windows_core::IUnknown, windows_core::IInspectable);
impl Magnetometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<MagnetometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IMagnetometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMagnetometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Magnetometer> {
        Self::IMagnetometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Magnetometer>> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IMagnetometerStatics<R, F: FnOnce(&IMagnetometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Magnetometer, IMagnetometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IMagnetometerStatics2<R, F: FnOnce(&IMagnetometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Magnetometer, IMagnetometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Magnetometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometer>();
}
unsafe impl windows_core::Interface for Magnetometer {
    type Vtable = <IMagnetometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMagnetometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Magnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Magnetometer";
}
unsafe impl Send for Magnetometer {}
unsafe impl Sync for Magnetometer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl windows_core::TypeKind for MagnetometerAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MagnetometerAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.MagnetometerAccuracy;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MagnetometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerDataThreshold {
    pub fn XAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MagnetometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerDataThreshold>();
}
unsafe impl windows_core::Interface for MagnetometerDataThreshold {
    type Vtable = <IMagnetometerDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMagnetometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerDataThreshold";
}
unsafe impl Send for MagnetometerDataThreshold {}
unsafe impl Sync for MagnetometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MagnetometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldZ(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DirectionalAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DirectionalAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagnetometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerReading>();
}
unsafe impl windows_core::Interface for MagnetometerReading {
    type Vtable = <IMagnetometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMagnetometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReading";
}
unsafe impl Send for MagnetometerReading {}
unsafe impl Sync for MagnetometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MagnetometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagnetometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for MagnetometerReadingChangedEventArgs {
    type Vtable = <IMagnetometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMagnetometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs";
}
unsafe impl Send for MagnetometerReadingChangedEventArgs {}
unsafe impl Sync for MagnetometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OnlookerDetectionAction(pub i32);
impl OnlookerDetectionAction {
    pub const Dim: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const DimAndNotify: Self = Self(2i32);
}
impl windows_core::TypeKind for OnlookerDetectionAction {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for OnlookerDetectionAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.OnlookerDetectionAction;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OnlookerDetectionBackOnMode(pub i32);
impl OnlookerDetectionBackOnMode {
    pub const Manually: Self = Self(0i32);
    pub const OneHour: Self = Self(1i32);
    pub const FourHours: Self = Self(2i32);
    pub const OneDay: Self = Self(3i32);
}
impl windows_core::TypeKind for OnlookerDetectionBackOnMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for OnlookerDetectionBackOnMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.OnlookerDetectionBackOnMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnlookerDetectionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlookerDetectionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl OnlookerDetectionOptions {
    pub fn Action(&self) -> windows_core::Result<OnlookerDetectionAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAction(&self, value: OnlookerDetectionAction) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackOnMode(&self) -> windows_core::Result<OnlookerDetectionBackOnMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackOnMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackOnMode(&self, value: OnlookerDetectionBackOnMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackOnMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for OnlookerDetectionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlookerDetectionOptions>();
}
unsafe impl windows_core::Interface for OnlookerDetectionOptions {
    type Vtable = <IOnlookerDetectionOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOnlookerDetectionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlookerDetectionOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.OnlookerDetectionOptions";
}
unsafe impl Send for OnlookerDetectionOptions {}
unsafe impl Sync for OnlookerDetectionOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrientationSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensor, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensor {
    pub fn GetCurrentReading(&self) -> windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<SensorReadingType> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultForRelativeReadings() -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(windows_core::Interface::as_raw(this), sensorreadingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal)(windows_core::Interface::as_raw(this), sensorreadingtype, optimizationgoal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> windows_core::Result<windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal)(windows_core::Interface::as_raw(this), readingtype, optimizationgoal, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<OrientationSensor>> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IOrientationSensorStatics<R, F: FnOnce(&IOrientationSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IOrientationSensorStatics2<R, F: FnOnce(&IOrientationSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IOrientationSensorStatics3<R, F: FnOnce(&IOrientationSensorStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IOrientationSensorStatics4<R, F: FnOnce(&IOrientationSensorStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensor>();
}
unsafe impl windows_core::Interface for OrientationSensor {
    type Vtable = <IOrientationSensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOrientationSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensor";
}
unsafe impl Send for OrientationSensor {}
unsafe impl Sync for OrientationSensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrientationSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RotationMatrix(&self) -> windows_core::Result<SensorRotationMatrix> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationMatrix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Quaternion(&self) -> windows_core::Result<SensorQuaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Quaternion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OrientationSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensorReading>();
}
unsafe impl windows_core::Interface for OrientationSensorReading {
    type Vtable = <IOrientationSensorReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOrientationSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReading";
}
unsafe impl Send for OrientationSensorReading {}
unsafe impl Sync for OrientationSensorReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrientationSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OrientationSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for OrientationSensorReadingChangedEventArgs {
    type Vtable = <IOrientationSensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOrientationSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs";
}
unsafe impl Send for OrientationSensorReadingChangedEventArgs {}
unsafe impl Sync for OrientationSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pedometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Pedometer, windows_core::IUnknown, windows_core::IInspectable);
impl Pedometer {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PowerInMilliwatts(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerInMilliwatts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetCurrentReadings(&self) -> windows_core::Result<windows_collections::IMapView<PedometerStepKind, PedometerReading>> {
        let this = &windows_core::Interface::cast::<IPedometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryAsync)(windows_core::Interface::as_raw(this), fromtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(windows_core::Interface::as_raw(this), fromtime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetReadingsFromTriggerDetails<P0>(triggerdetails: P0) -> windows_core::Result<windows_collections::IVectorView<PedometerReading>>
    where
        P0: windows_core::Param<SensorDataThresholdTriggerDetails>,
    {
        Self::IPedometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(windows_core::Interface::as_raw(this), triggerdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPedometerStatics<R, F: FnOnce(&IPedometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pedometer, IPedometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPedometerStatics2<R, F: FnOnce(&IPedometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pedometer, IPedometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Pedometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometer>();
}
unsafe impl windows_core::Interface for Pedometer {
    type Vtable = <IPedometer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPedometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Pedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Pedometer";
}
unsafe impl Send for Pedometer {}
unsafe impl Sync for Pedometer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PedometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable, ISensorDataThreshold);
impl PedometerDataThreshold {
    pub fn Create<P0>(sensor: P0, stepgoal: i32) -> windows_core::Result<PedometerDataThreshold>
    where
        P0: windows_core::Param<Pedometer>,
    {
        Self::IPedometerDataThresholdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), sensor.param().abi(), stepgoal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPedometerDataThresholdFactory<R, F: FnOnce(&IPedometerDataThresholdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PedometerDataThreshold, IPedometerDataThresholdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PedometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThreshold>();
}
unsafe impl windows_core::Interface for PedometerDataThreshold {
    type Vtable = <ISensorDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerDataThreshold";
}
unsafe impl Send for PedometerDataThreshold {}
unsafe impl Sync for PedometerDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PedometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl PedometerReading {
    pub fn StepKind(&self) -> windows_core::Result<PedometerStepKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StepKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CumulativeSteps(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CumulativeSteps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CumulativeStepsDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CumulativeStepsDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PedometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometerReading>();
}
unsafe impl windows_core::Interface for PedometerReading {
    type Vtable = <IPedometerReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPedometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReading";
}
unsafe impl Send for PedometerReading {}
unsafe impl Sync for PedometerReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PedometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PedometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<PedometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PedometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for PedometerReadingChangedEventArgs {
    type Vtable = <IPedometerReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPedometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReadingChangedEventArgs";
}
unsafe impl Send for PedometerReadingChangedEventArgs {}
unsafe impl Sync for PedometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl windows_core::TypeKind for PedometerStepKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PedometerStepKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.PedometerStepKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProximitySensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensor, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensor {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MaxDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateDisplayOnOffController(&self) -> windows_core::Result<ProximitySensorDisplayOnOffController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDisplayOnOffController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromId(sensorid: &windows_core::HSTRING) -> windows_core::Result<ProximitySensor> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetReadingsFromTriggerDetails<P0>(triggerdetails: P0) -> windows_core::Result<windows_collections::IVectorView<ProximitySensorReading>>
    where
        P0: windows_core::Param<SensorDataThresholdTriggerDetails>,
    {
        Self::IProximitySensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(windows_core::Interface::as_raw(this), triggerdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IProximitySensorStatics<R, F: FnOnce(&IProximitySensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensor, IProximitySensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IProximitySensorStatics2<R, F: FnOnce(&IProximitySensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensor, IProximitySensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProximitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensor>();
}
unsafe impl windows_core::Interface for ProximitySensor {
    type Vtable = <IProximitySensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProximitySensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensor";
}
unsafe impl Send for ProximitySensor {}
unsafe impl Sync for ProximitySensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProximitySensorDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable, ISensorDataThreshold);
impl ProximitySensorDataThreshold {
    pub fn Create<P0>(sensor: P0) -> windows_core::Result<ProximitySensorDataThreshold>
    where
        P0: windows_core::Param<ProximitySensor>,
    {
        Self::IProximitySensorDataThresholdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), sensor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IProximitySensorDataThresholdFactory<R, F: FnOnce(&IProximitySensorDataThresholdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensorDataThreshold, IProximitySensorDataThresholdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProximitySensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThreshold>();
}
unsafe impl windows_core::Interface for ProximitySensorDataThreshold {
    type Vtable = <ISensorDataThreshold as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDataThreshold";
}
unsafe impl Send for ProximitySensorDataThreshold {}
unsafe impl Sync for ProximitySensorDataThreshold {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProximitySensorDisplayOnOffController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorDisplayOnOffController, windows_core::IUnknown, windows_core::IInspectable, super::super::Foundation::IClosable);
impl ProximitySensorDisplayOnOffController {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ProximitySensorDisplayOnOffController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::IClosable>();
}
unsafe impl windows_core::Interface for ProximitySensorDisplayOnOffController {
    type Vtable = <super::super::Foundation::IClosable as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::Foundation::IClosable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorDisplayOnOffController {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDisplayOnOffController";
}
unsafe impl Send for ProximitySensorDisplayOnOffController {}
unsafe impl Sync for ProximitySensorDisplayOnOffController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProximitySensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDetected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDetected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProximitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensorReading>();
}
unsafe impl windows_core::Interface for ProximitySensorReading {
    type Vtable = <IProximitySensorReading as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProximitySensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReading";
}
unsafe impl Send for ProximitySensorReading {}
unsafe impl Sync for ProximitySensorReading {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProximitySensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProximitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for ProximitySensorReadingChangedEventArgs {
    type Vtable = <IProximitySensorReadingChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProximitySensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs";
}
unsafe impl Send for ProximitySensorReadingChangedEventArgs {}
unsafe impl Sync for ProximitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensorDataThresholdTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorDataThresholdTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SensorDataThresholdTriggerDetails {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SensorType(&self) -> windows_core::Result<SensorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorDataThresholdTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThresholdTriggerDetails>();
}
unsafe impl windows_core::Interface for SensorDataThresholdTriggerDetails {
    type Vtable = <ISensorDataThresholdTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISensorDataThresholdTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorDataThresholdTriggerDetails";
}
unsafe impl Send for SensorDataThresholdTriggerDetails {}
unsafe impl Sync for SensorDataThresholdTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl windows_core::TypeKind for SensorOptimizationGoal {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SensorOptimizationGoal {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorOptimizationGoal;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensorQuaternion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorQuaternion, windows_core::IUnknown, windows_core::IInspectable);
impl SensorQuaternion {
    pub fn W(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).W)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn X(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).X)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Y(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Y)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Z(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Z)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorQuaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorQuaternion>();
}
unsafe impl windows_core::Interface for SensorQuaternion {
    type Vtable = <ISensorQuaternion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISensorQuaternion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorQuaternion";
}
unsafe impl Send for SensorQuaternion {}
unsafe impl Sync for SensorQuaternion {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl windows_core::TypeKind for SensorReadingType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SensorReadingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorReadingType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensorRotationMatrix(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorRotationMatrix, windows_core::IUnknown, windows_core::IInspectable);
impl SensorRotationMatrix {
    pub fn M11(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M11)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M12(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M12)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M13(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M13)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M21(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M21)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M22(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M22)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M23(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M23)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M31(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M31)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M32(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M33(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M33)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorRotationMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorRotationMatrix>();
}
unsafe impl windows_core::Interface for SensorRotationMatrix {
    type Vtable = <ISensorRotationMatrix as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISensorRotationMatrix as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorRotationMatrix";
}
unsafe impl Send for SensorRotationMatrix {}
unsafe impl Sync for SensorRotationMatrix {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for SensorType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SensorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl windows_core::TypeKind for SimpleOrientation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SimpleOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SimpleOrientation;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleOrientationSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleOrientationSensor, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleOrientationSensor {
    pub fn GetCurrentOrientation(&self) -> windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientationChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOrientationChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOrientationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<SimpleOrientationSensor> {
        Self::ISimpleOrientationSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<SimpleOrientationSensor>> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISimpleOrientationSensorStatics<R, F: FnOnce(&ISimpleOrientationSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISimpleOrientationSensorStatics2<R, F: FnOnce(&ISimpleOrientationSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SimpleOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleOrientationSensor>();
}
unsafe impl windows_core::Interface for SimpleOrientationSensor {
    type Vtable = <ISimpleOrientationSensor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISimpleOrientationSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensor";
}
unsafe impl Send for SimpleOrientationSensor {}
unsafe impl Sync for SimpleOrientationSensor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleOrientationSensorOrientationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleOrientationSensorOrientationChangedEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleOrientationSensorOrientationChangedEventArgs>();
}
unsafe impl windows_core::Interface for SimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = <ISimpleOrientationSensorOrientationChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISimpleOrientationSensorOrientationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs";
}
unsafe impl Send for SimpleOrientationSensorOrientationChangedEventArgs {}
unsafe impl Sync for SimpleOrientationSensorOrientationChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WakeOnApproachOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WakeOnApproachOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WakeOnApproachOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableWhenBatterySaverOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableWhenBatterySaverOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisableWhenBatterySaverOn(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisableWhenBatterySaverOn)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for WakeOnApproachOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWakeOnApproachOptions>();
}
unsafe impl windows_core::Interface for WakeOnApproachOptions {
    type Vtable = <IWakeOnApproachOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWakeOnApproachOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WakeOnApproachOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.WakeOnApproachOptions";
}
unsafe impl Send for WakeOnApproachOptions {}
unsafe impl Sync for WakeOnApproachOptions {}
