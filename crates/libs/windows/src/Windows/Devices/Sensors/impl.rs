#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometer_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<AccelerometerReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Shaken(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShaken(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Shaken<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shaken(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShaken<Impl: IAccelerometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShaken(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometer, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
            Shaken: Shaken::<Impl, IMPL_OFFSET>,
            RemoveShaken: RemoveShaken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IAccelerometer2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IAccelerometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IAccelerometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IAccelerometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometer2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IAccelerometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IAccelerometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IAccelerometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometer3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer4_Impl: Sized {
    fn ReadingType(&mut self) -> ::windows::core::Result<AccelerometerReadingType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer4_Vtbl {
        unsafe extern "system" fn ReadingType<Impl: IAccelerometer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccelerometerReadingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometer4, BASE_OFFSET>(), ReadingType: ReadingType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer5_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<AccelerometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer5 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer5";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer5_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IAccelerometer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometer5, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDataThreshold_Impl: Sized {
    fn XAxisInGForce(&mut self) -> ::windows::core::Result<f64>;
    fn SetXAxisInGForce(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInGForce(&mut self) -> ::windows::core::Result<f64>;
    fn SetYAxisInGForce(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInGForce(&mut self) -> ::windows::core::Result<f64>;
    fn SetZAxisInGForce(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerDataThreshold_Vtbl {
        unsafe extern "system" fn XAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XAxisInGForce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisInGForce(value).into()
        }
        unsafe extern "system" fn YAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YAxisInGForce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisInGForce(value).into()
        }
        unsafe extern "system" fn ZAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZAxisInGForce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZAxisInGForce<Impl: IAccelerometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisInGForce(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerDataThreshold, BASE_OFFSET>(),
            XAxisInGForce: XAxisInGForce::<Impl, IMPL_OFFSET>,
            SetXAxisInGForce: SetXAxisInGForce::<Impl, IMPL_OFFSET>,
            YAxisInGForce: YAxisInGForce::<Impl, IMPL_OFFSET>,
            SetYAxisInGForce: SetYAxisInGForce::<Impl, IMPL_OFFSET>,
            ZAxisInGForce: ZAxisInGForce::<Impl, IMPL_OFFSET>,
            SetZAxisInGForce: SetZAxisInGForce::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IAccelerometerDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AccelerationX(&mut self) -> ::windows::core::Result<f64>;
    fn AccelerationY(&mut self) -> ::windows::core::Result<f64>;
    fn AccelerationZ(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IAccelerometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccelerationX<Impl: IAccelerometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccelerationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccelerationY<Impl: IAccelerometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccelerationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccelerationZ<Impl: IAccelerometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccelerationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            AccelerationX: AccelerationX::<Impl, IMPL_OFFSET>,
            AccelerationY: AccelerationY::<Impl, IMPL_OFFSET>,
            AccelerationZ: AccelerationZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAccelerometerReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAccelerometerReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IAccelerometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IAccelerometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<AccelerometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IAccelerometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerShakenEventArgs_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerShakenEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerShakenEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerShakenEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerShakenEventArgs_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IAccelerometerShakenEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerShakenEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerShakenEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAccelerometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStatics2_Impl: Sized {
    fn GetDefaultWithAccelerometerReadingType(&mut self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStatics2_Vtbl {
        unsafe extern "system" fn GetDefaultWithAccelerometerReadingType<Impl: IAccelerometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultWithAccelerometerReadingType(readingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerStatics2, BASE_OFFSET>(),
            GetDefaultWithAccelerometerReadingType: GetDefaultWithAccelerometerReadingType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerStatics3_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>>;
    fn GetDeviceSelector(&mut self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStatics3_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IAccelerometerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IAccelerometerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(readingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccelerometerStatics3, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivitySensor_Impl: Sized {
    fn GetCurrentReadingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>;
    fn SubscribedActivities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ActivityType>>;
    fn PowerInMilliwatts(&mut self) -> ::windows::core::Result<f64>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedActivities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensor";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivitySensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensor_Vtbl {
        unsafe extern "system" fn GetCurrentReadingAsync<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReadingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribedActivities<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerInMilliwatts<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerInMilliwatts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedActivities<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IActivitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensor, BASE_OFFSET>(),
            GetCurrentReadingAsync: GetCurrentReadingAsync::<Impl, IMPL_OFFSET>,
            SubscribedActivities: SubscribedActivities::<Impl, IMPL_OFFSET>,
            PowerInMilliwatts: PowerInMilliwatts::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            SupportedActivities: SupportedActivities::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IActivitySensorReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Activity(&mut self) -> ::windows::core::Result<ActivityType>;
    fn Confidence(&mut self) -> ::windows::core::Result<ActivitySensorReadingConfidence>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivitySensorReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IActivitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activity<Impl: IActivitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivityType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: IActivitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivitySensorReadingConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Activity: Activity::<Impl, IMPL_OFFSET>,
            Confidence: Confidence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangeReport_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReadingChangeReport";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorReadingChangeReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReadingChangeReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReadingChangeReport_Vtbl {
        unsafe extern "system" fn Reading<Impl: IActivitySensorReadingChangeReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorReadingChangeReport, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReadingChangeReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IActivitySensorReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IActivitySensorStatics_Impl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetSystemHistoryAsync(&mut self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
    fn GetSystemHistoryWithDurationAsync(&mut self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivitySensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorStatics_Vtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IActivitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IActivitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IActivitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemHistoryAsync<Impl: IActivitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemHistoryAsync(&*(&fromtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemHistoryWithDurationAsync<Impl: IActivitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemHistoryWithDurationAsync(&*(&fromtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryAsync: GetSystemHistoryAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryWithDurationAsync: GetSystemHistoryWithDurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivitySensorTriggerDetails_Impl: Sized {
    fn ReadReports(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivitySensorTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorTriggerDetails_Vtbl {
        unsafe extern "system" fn ReadReports<Impl: IActivitySensorTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorTriggerDetails, BASE_OFFSET>(),
            ReadReports: ReadReports::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAltimeter_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<AltimeterReading>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAltimeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeter_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IAltimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeter, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeter2_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeter2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeter2";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeter2_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IAltimeter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IAltimeter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IAltimeter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeter2, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAltimeterReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AltitudeChangeInMeters(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAltimeterReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IAltimeterReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeChangeInMeters<Impl: IAltimeterReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeChangeInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeterReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            AltitudeChangeInMeters: AltitudeChangeInMeters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAltimeterReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeterReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAltimeterReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IAltimeterReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IAltimeterReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeterReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<AltimeterReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeterReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IAltimeterReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeterReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Altimeter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeterStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAltimeterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAltimeterStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometer_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<BarometerReading>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometer_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IBarometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometer, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer2_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometer2_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IBarometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IBarometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IBarometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometer2, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer3_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<BarometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometer3_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IBarometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometer3, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerDataThreshold_Impl: Sized {
    fn Hectopascals(&mut self) -> ::windows::core::Result<f64>;
    fn SetHectopascals(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerDataThreshold_Vtbl {
        unsafe extern "system" fn Hectopascals<Impl: IBarometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hectopascals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHectopascals<Impl: IBarometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHectopascals(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerDataThreshold, BASE_OFFSET>(),
            Hectopascals: Hectopascals::<Impl, IMPL_OFFSET>,
            SetHectopascals: SetHectopascals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometerReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn StationPressureInHectopascals(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IBarometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StationPressureInHectopascals<Impl: IBarometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StationPressureInHectopascals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            StationPressureInHectopascals: StationPressureInHectopascals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBarometerReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBarometerReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IBarometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IBarometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<BarometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IBarometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Barometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IBarometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometerStatics2_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Barometer>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerStatics2_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBarometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IBarometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarometerStatics2, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompass_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<CompassReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompass {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: ICompass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompass, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait ICompass2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompass2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ICompass2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: ICompass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: ICompass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompass2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompass3 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompass3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: ICompass3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: ICompass3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: ICompass3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompass3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass4_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<CompassDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompass4 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompass4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass4_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: ICompass4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompass4, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDataThreshold_Impl: Sized {
    fn Degrees(&mut self) -> ::windows::core::Result<f64>;
    fn SetDegrees(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassDataThreshold_Vtbl {
        unsafe extern "system" fn Degrees<Impl: ICompassDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Degrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDegrees<Impl: ICompassDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDegrees(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassDataThreshold, BASE_OFFSET>(),
            Degrees: Degrees::<Impl, IMPL_OFFSET>,
            SetDegrees: SetDegrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ICompassDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompassReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn HeadingMagneticNorth(&mut self) -> ::windows::core::Result<f64>;
    fn HeadingTrueNorth(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompassReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: ICompassReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingMagneticNorth<Impl: ICompassReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingMagneticNorth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeadingTrueNorth<Impl: ICompassReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingTrueNorth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            HeadingMagneticNorth: HeadingMagneticNorth::<Impl, IMPL_OFFSET>,
            HeadingTrueNorth: HeadingTrueNorth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompassReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompassReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: ICompassReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ICompassReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<CompassReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: ICompassReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassReadingChangedEventArgs, BASE_OFFSET>(), Reading: Reading::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingHeadingAccuracy_Impl: Sized {
    fn HeadingAccuracy(&mut self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassReadingHeadingAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReadingHeadingAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassReadingHeadingAccuracy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReadingHeadingAccuracy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReadingHeadingAccuracy_Vtbl {
        unsafe extern "system" fn HeadingAccuracy<Impl: ICompassReadingHeadingAccuracy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadingAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassReadingHeadingAccuracy, BASE_OFFSET>(),
            HeadingAccuracy: HeadingAccuracy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReadingHeadingAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Compass>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ICompassStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompassStatics2_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Compass>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompassStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ICompassStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICompassStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompassStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometer_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<GyrometerReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IGyrometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometer, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IGyrometer2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IGyrometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IGyrometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IGyrometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometer2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IGyrometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IGyrometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IGyrometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometer3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer4_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<GyrometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer4_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IGyrometer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometer4, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDataThreshold_Impl: Sized {
    fn XAxisInDegreesPerSecond(&mut self) -> ::windows::core::Result<f64>;
    fn SetXAxisInDegreesPerSecond(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInDegreesPerSecond(&mut self) -> ::windows::core::Result<f64>;
    fn SetYAxisInDegreesPerSecond(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInDegreesPerSecond(&mut self) -> ::windows::core::Result<f64>;
    fn SetZAxisInDegreesPerSecond(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerDataThreshold_Vtbl {
        unsafe extern "system" fn XAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XAxisInDegreesPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisInDegreesPerSecond(value).into()
        }
        unsafe extern "system" fn YAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YAxisInDegreesPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisInDegreesPerSecond(value).into()
        }
        unsafe extern "system" fn ZAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZAxisInDegreesPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZAxisInDegreesPerSecond<Impl: IGyrometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisInDegreesPerSecond(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerDataThreshold, BASE_OFFSET>(),
            XAxisInDegreesPerSecond: XAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetXAxisInDegreesPerSecond: SetXAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            YAxisInDegreesPerSecond: YAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetYAxisInDegreesPerSecond: SetYAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            ZAxisInDegreesPerSecond: ZAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetZAxisInDegreesPerSecond: SetZAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IGyrometerDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometerReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngularVelocityX(&mut self) -> ::windows::core::Result<f64>;
    fn AngularVelocityY(&mut self) -> ::windows::core::Result<f64>;
    fn AngularVelocityZ(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IGyrometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngularVelocityX<Impl: IGyrometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngularVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngularVelocityY<Impl: IGyrometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngularVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngularVelocityZ<Impl: IGyrometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngularVelocityZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            AngularVelocityX: AngularVelocityX::<Impl, IMPL_OFFSET>,
            AngularVelocityY: AngularVelocityY::<Impl, IMPL_OFFSET>,
            AngularVelocityZ: AngularVelocityZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGyrometerReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGyrometerReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IGyrometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IGyrometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<GyrometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IGyrometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Gyrometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IGyrometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometerStatics2_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IGyrometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IGyrometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGyrometerStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHingeAngleReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngleInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleReading";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHingeAngleReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IHingeAngleReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AngleInDegrees<Impl: IHingeAngleReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IHingeAngleReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHingeAngleReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            AngleInDegrees: AngleInDegrees::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHingeAngleSensor_Impl: Sized {
    fn GetCurrentReadingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinReportThresholdInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn ReportThresholdInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn SetReportThresholdInDegrees(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHingeAngleSensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensor_Vtbl {
        unsafe extern "system" fn GetCurrentReadingAsync<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReadingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinReportThresholdInDegrees<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinReportThresholdInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportThresholdInDegrees<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThresholdInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportThresholdInDegrees<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportThresholdInDegrees(value).into()
        }
        unsafe extern "system" fn ReadingChanged<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IHingeAngleSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHingeAngleSensor, BASE_OFFSET>(),
            GetCurrentReadingAsync: GetCurrentReadingAsync::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            MinReportThresholdInDegrees: MinReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            ReportThresholdInDegrees: ReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            SetReportThresholdInDegrees: SetReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleSensorReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<HingeAngleReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHingeAngleSensorReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensorReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensorReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IHingeAngleSensorReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHingeAngleSensorReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHingeAngleSensorStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn GetRelatedToAdjacentPanelsAsync(&mut self, firstpanelid: &::windows::core::HSTRING, secondpanelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHingeAngleSensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensorStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IHingeAngleSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: IHingeAngleSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelatedToAdjacentPanelsAsync<Impl: IHingeAngleSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, secondpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelatedToAdjacentPanelsAsync(&*(&firstpanelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&secondpanelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IHingeAngleSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHingeAngleSensorStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetRelatedToAdjacentPanelsAsync: GetRelatedToAdjacentPanelsAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometer_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<InclinometerReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IInclinometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometer, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IInclinometer2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&mut self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IInclinometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IInclinometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IInclinometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingType<Impl: IInclinometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometer2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingType: ReadingType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IInclinometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IInclinometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IInclinometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometer3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer4_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<InclinometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer4_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IInclinometer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometer4, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDataThreshold_Impl: Sized {
    fn PitchInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetPitchInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RollInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRollInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn YawInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetYawInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerDataThreshold_Vtbl {
        unsafe extern "system" fn PitchInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPitchInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitchInDegrees(value).into()
        }
        unsafe extern "system" fn RollInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RollInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRollInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRollInDegrees(value).into()
        }
        unsafe extern "system" fn YawInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YawInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYawInDegrees<Impl: IInclinometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYawInDegrees(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerDataThreshold, BASE_OFFSET>(),
            PitchInDegrees: PitchInDegrees::<Impl, IMPL_OFFSET>,
            SetPitchInDegrees: SetPitchInDegrees::<Impl, IMPL_OFFSET>,
            RollInDegrees: RollInDegrees::<Impl, IMPL_OFFSET>,
            SetRollInDegrees: SetRollInDegrees::<Impl, IMPL_OFFSET>,
            YawInDegrees: YawInDegrees::<Impl, IMPL_OFFSET>,
            SetYawInDegrees: SetYawInDegrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IInclinometerDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometerReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PitchDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn RollDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn YawDegrees(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IInclinometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchDegrees<Impl: IInclinometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RollDegrees<Impl: IInclinometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RollDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YawDegrees<Impl: IInclinometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YawDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            PitchDegrees: PitchDegrees::<Impl, IMPL_OFFSET>,
            RollDegrees: RollDegrees::<Impl, IMPL_OFFSET>,
            YawDegrees: YawDegrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInclinometerReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInclinometerReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IInclinometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IInclinometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<InclinometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IInclinometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingYawAccuracy_Impl: Sized {
    fn YawAccuracy(&mut self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerReadingYawAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReadingYawAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerReadingYawAccuracy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReadingYawAccuracy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReadingYawAccuracy_Vtbl {
        unsafe extern "system" fn YawAccuracy<Impl: IInclinometerReadingYawAccuracy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YawAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerReadingYawAccuracy, BASE_OFFSET>(),
            YawAccuracy: YawAccuracy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReadingYawAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IInclinometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics2_Impl: Sized {
    fn GetDefaultForRelativeReadings(&mut self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics2_Vtbl {
        unsafe extern "system" fn GetDefaultForRelativeReadings<Impl: IInclinometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultForRelativeReadings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerStatics2, BASE_OFFSET>(),
            GetDefaultForRelativeReadings: GetDefaultForRelativeReadings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics3_Impl: Sized {
    fn GetDefaultWithSensorReadingType(&mut self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics3_Vtbl {
        unsafe extern "system" fn GetDefaultWithSensorReadingType<Impl: IInclinometerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultWithSensorReadingType(sensorreadingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerStatics3, BASE_OFFSET>(),
            GetDefaultWithSensorReadingType: GetDefaultWithSensorReadingType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometerStatics4_Impl: Sized {
    fn GetDeviceSelector(&mut self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerStatics4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics4_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IInclinometerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(readingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IInclinometerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInclinometerStatics4, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensor_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<LightSensorReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensor_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: ILightSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensor, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor2_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor2";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensor2_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: ILightSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: ILightSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: ILightSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensor2, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor3_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<LightSensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensor3 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor3";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensor3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensor3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensor3_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: ILightSensor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensor3, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDataThreshold_Impl: Sized {
    fn LuxPercentage(&mut self) -> ::windows::core::Result<f32>;
    fn SetLuxPercentage(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn AbsoluteLux(&mut self) -> ::windows::core::Result<f32>;
    fn SetAbsoluteLux(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorDataThreshold_Vtbl {
        unsafe extern "system" fn LuxPercentage<Impl: ILightSensorDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LuxPercentage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLuxPercentage<Impl: ILightSensorDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLuxPercentage(value).into()
        }
        unsafe extern "system" fn AbsoluteLux<Impl: ILightSensorDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteLux() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbsoluteLux<Impl: ILightSensorDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbsoluteLux(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorDataThreshold, BASE_OFFSET>(),
            LuxPercentage: LuxPercentage::<Impl, IMPL_OFFSET>,
            SetLuxPercentage: SetLuxPercentage::<Impl, IMPL_OFFSET>,
            AbsoluteLux: AbsoluteLux::<Impl, IMPL_OFFSET>,
            SetAbsoluteLux: SetAbsoluteLux::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ILightSensorDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensorReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IlluminanceInLux(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensorReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: ILightSensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IlluminanceInLux<Impl: ILightSensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IlluminanceInLux() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            IlluminanceInLux: IlluminanceInLux::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILightSensorReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILightSensorReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: ILightSensorReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ILightSensorReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<LightSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: ILightSensorReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<LightSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ILightSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensorStatics2_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LightSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ILightSensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ILightSensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILightSensorStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometer_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<MagnetometerReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IMagnetometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometer, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IMagnetometer2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IMagnetometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IMagnetometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IMagnetometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometer2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IMagnetometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IMagnetometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IMagnetometer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometer3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer4_Impl: Sized {
    fn ReportThreshold(&mut self) -> ::windows::core::Result<MagnetometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer4_Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IMagnetometer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometer4, BASE_OFFSET>(), ReportThreshold: ReportThreshold::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDataThreshold_Impl: Sized {
    fn XAxisMicroteslas(&mut self) -> ::windows::core::Result<f32>;
    fn SetXAxisMicroteslas(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn YAxisMicroteslas(&mut self) -> ::windows::core::Result<f32>;
    fn SetYAxisMicroteslas(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn ZAxisMicroteslas(&mut self) -> ::windows::core::Result<f32>;
    fn SetZAxisMicroteslas(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerDataThreshold_Vtbl {
        unsafe extern "system" fn XAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XAxisMicroteslas() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisMicroteslas(value).into()
        }
        unsafe extern "system" fn YAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YAxisMicroteslas() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisMicroteslas(value).into()
        }
        unsafe extern "system" fn ZAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZAxisMicroteslas() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZAxisMicroteslas<Impl: IMagnetometerDataThreshold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisMicroteslas(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerDataThreshold, BASE_OFFSET>(),
            XAxisMicroteslas: XAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetXAxisMicroteslas: SetXAxisMicroteslas::<Impl, IMPL_OFFSET>,
            YAxisMicroteslas: YAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetYAxisMicroteslas: SetYAxisMicroteslas::<Impl, IMPL_OFFSET>,
            ZAxisMicroteslas: ZAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetZAxisMicroteslas: SetZAxisMicroteslas::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IMagnetometerDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometerReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn MagneticFieldX(&mut self) -> ::windows::core::Result<f32>;
    fn MagneticFieldY(&mut self) -> ::windows::core::Result<f32>;
    fn MagneticFieldZ(&mut self) -> ::windows::core::Result<f32>;
    fn DirectionalAccuracy(&mut self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IMagnetometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MagneticFieldX<Impl: IMagnetometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MagneticFieldX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MagneticFieldY<Impl: IMagnetometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MagneticFieldY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MagneticFieldZ<Impl: IMagnetometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MagneticFieldZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionalAccuracy<Impl: IMagnetometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectionalAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            MagneticFieldX: MagneticFieldX::<Impl, IMPL_OFFSET>,
            MagneticFieldY: MagneticFieldY::<Impl, IMPL_OFFSET>,
            MagneticFieldZ: MagneticFieldZ::<Impl, IMPL_OFFSET>,
            DirectionalAccuracy: DirectionalAccuracy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMagnetometerReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMagnetometerReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IMagnetometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMagnetometerReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<MagnetometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IMagnetometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<Magnetometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IMagnetometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometerStatics2_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IMagnetometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IMagnetometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagnetometerStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensor_Impl: Sized {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<OrientationSensorReading>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensor_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensor, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IOrientationSensor2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&mut self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IOrientationSensor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensor2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IOrientationSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IOrientationSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingType<Impl: IOrientationSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensor2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingType: ReadingType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensor3_Impl: Sized {
    fn SetReportLatency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&mut self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensor3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor3";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensor3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensor3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensor3_Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IOrientationSensor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IOrientationSensor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IOrientationSensor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensor3, BASE_OFFSET>(),
            SetReportLatency: SetReportLatency::<Impl, IMPL_OFFSET>,
            ReportLatency: ReportLatency::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IOrientationSensorDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorDeviceId, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensorReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RotationMatrix(&mut self) -> ::windows::core::Result<SensorRotationMatrix>;
    fn Quaternion(&mut self) -> ::windows::core::Result<SensorQuaternion>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensorReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IOrientationSensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationMatrix<Impl: IOrientationSensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quaternion<Impl: IOrientationSensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quaternion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            RotationMatrix: RotationMatrix::<Impl, IMPL_OFFSET>,
            Quaternion: Quaternion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOrientationSensorReading2_Impl: Sized {
    fn PerformanceCount(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOrientationSensorReading2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReading2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReading2_Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IOrientationSensorReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IOrientationSensorReading2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorReading2, BASE_OFFSET>(),
            PerformanceCount: PerformanceCount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<OrientationSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IOrientationSensorReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingYawAccuracy_Impl: Sized {
    fn YawAccuracy(&mut self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorReadingYawAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReadingYawAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorReadingYawAccuracy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReadingYawAccuracy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReadingYawAccuracy_Vtbl {
        unsafe extern "system" fn YawAccuracy<Impl: IOrientationSensorReadingYawAccuracy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YawAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorReadingYawAccuracy, BASE_OFFSET>(),
            YawAccuracy: YawAccuracy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReadingYawAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IOrientationSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics2_Impl: Sized {
    fn GetDefaultForRelativeReadings(&mut self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics2_Vtbl {
        unsafe extern "system" fn GetDefaultForRelativeReadings<Impl: IOrientationSensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultForRelativeReadings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorStatics2, BASE_OFFSET>(),
            GetDefaultForRelativeReadings: GetDefaultForRelativeReadings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics3_Impl: Sized {
    fn GetDefaultWithSensorReadingType(&mut self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<OrientationSensor>;
    fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(&mut self, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics3_Vtbl {
        unsafe extern "system" fn GetDefaultWithSensorReadingType<Impl: IOrientationSensorStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultWithSensorReadingType(sensorreadingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal<Impl: IOrientationSensorStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype, optimizationgoal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorStatics3, BASE_OFFSET>(),
            GetDefaultWithSensorReadingType: GetDefaultWithSensorReadingType::<Impl, IMPL_OFFSET>,
            GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensorStatics4_Impl: Sized {
    fn GetDeviceSelector(&mut self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(&mut self, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorStatics4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensorStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics4_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IOrientationSensorStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(readingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal<Impl: IOrientationSensorStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype, optimizationgoal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IOrientationSensorStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientationSensorStatics4, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometer_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PowerInMilliwatts(&mut self) -> ::windows::core::Result<f64>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometer_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerInMilliwatts<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerInMilliwatts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IPedometer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometer, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            PowerInMilliwatts: PowerInMilliwatts::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPedometer2_Impl: Sized {
    fn GetCurrentReadings(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometer2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPedometer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometer2_Vtbl {
        unsafe extern "system" fn GetCurrentReadings<Impl: IPedometer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReadings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometer2, BASE_OFFSET>(),
            GetCurrentReadings: GetCurrentReadings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerDataThresholdFactory_Impl: Sized {
    fn Create(&mut self, sensor: &::core::option::Option<Pedometer>, stepgoal: i32) -> ::windows::core::Result<PedometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPedometerDataThresholdFactory {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerDataThresholdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPedometerDataThresholdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerDataThresholdFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerDataThresholdFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPedometerDataThresholdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensor: ::windows::core::RawPtr, stepgoal: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&sensor as *const <Pedometer as ::windows::core::Abi>::Abi as *const <Pedometer as ::windows::core::DefaultType>::DefaultType), stepgoal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometerDataThresholdFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerDataThresholdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometerReading_Impl: Sized {
    fn StepKind(&mut self) -> ::windows::core::Result<PedometerStepKind>;
    fn CumulativeSteps(&mut self) -> ::windows::core::Result<i32>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CumulativeStepsDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometerReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerReading_Vtbl {
        unsafe extern "system" fn StepKind<Impl: IPedometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PedometerStepKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StepKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CumulativeSteps<Impl: IPedometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CumulativeSteps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IPedometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CumulativeStepsDuration<Impl: IPedometerReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CumulativeStepsDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometerReading, BASE_OFFSET>(),
            StepKind: StepKind::<Impl, IMPL_OFFSET>,
            CumulativeSteps: CumulativeSteps::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            CumulativeStepsDuration: CumulativeStepsDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<PedometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPedometerReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IPedometerReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometerReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometerStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSystemHistoryAsync(&mut self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
    fn GetSystemHistoryWithDurationAsync(&mut self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPedometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: IPedometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IPedometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemHistoryAsync<Impl: IPedometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemHistoryAsync(&*(&fromtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemHistoryWithDurationAsync<Impl: IPedometerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemHistoryWithDurationAsync(&*(&fromtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometerStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetSystemHistoryAsync: GetSystemHistoryAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryWithDurationAsync: GetSystemHistoryWithDurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPedometerStatics2_Impl: Sized {
    fn GetReadingsFromTriggerDetails(&mut self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPedometerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerStatics2_Vtbl {
        unsafe extern "system" fn GetReadingsFromTriggerDetails<Impl: IPedometerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadingsFromTriggerDetails(&*(&triggerdetails as *const <SensorDataThresholdTriggerDetails as ::windows::core::Abi>::Abi as *const <SensorDataThresholdTriggerDetails as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPedometerStatics2, BASE_OFFSET>(),
            GetReadingsFromTriggerDetails: GetReadingsFromTriggerDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProximitySensor_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxDistanceInMillimeters(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MinDistanceInMillimeters(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<ProximitySensorReading>;
    fn ReadingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateDisplayOnOffController(&mut self) -> ::windows::core::Result<ProximitySensorDisplayOnOffController>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProximitySensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensor_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDistanceInMillimeters<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDistanceInMillimeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDistanceInMillimeters<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinDistanceInMillimeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingChanged<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadingChanged<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateDisplayOnOffController<Impl: IProximitySensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDisplayOnOffController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensor, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            MaxDistanceInMillimeters: MaxDistanceInMillimeters::<Impl, IMPL_OFFSET>,
            MinDistanceInMillimeters: MinDistanceInMillimeters::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            ReadingChanged: ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged: RemoveReadingChanged::<Impl, IMPL_OFFSET>,
            CreateDisplayOnOffController: CreateDisplayOnOffController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorDataThresholdFactory_Impl: Sized {
    fn Create(&mut self, sensor: &::core::option::Option<ProximitySensor>) -> ::windows::core::Result<ProximitySensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorDataThresholdFactory {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorDataThresholdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorDataThresholdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorDataThresholdFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorDataThresholdFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IProximitySensorDataThresholdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&sensor as *const <ProximitySensor as ::windows::core::Abi>::Abi as *const <ProximitySensor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensorDataThresholdFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorDataThresholdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProximitySensorReading_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsDetected(&mut self) -> ::windows::core::Result<bool>;
    fn DistanceInMillimeters(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProximitySensorReading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorReading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorReading_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IProximitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDetected<Impl: IProximitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDetected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistanceInMillimeters<Impl: IProximitySensorReading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistanceInMillimeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensorReading, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            IsDetected: IsDetected::<Impl, IMPL_OFFSET>,
            DistanceInMillimeters: DistanceInMillimeters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorReadingChangedEventArgs_Impl: Sized {
    fn Reading(&mut self) -> ::windows::core::Result<ProximitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorReadingChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorReadingChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorReadingChangedEventArgs_Vtbl {
        unsafe extern "system" fn Reading<Impl: IProximitySensorReadingChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensorReadingChangedEventArgs, BASE_OFFSET>(),
            Reading: Reading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&mut self, sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximitySensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IProximitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromId<Impl: IProximitySensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&sensorid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensorStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromId: FromId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProximitySensorStatics2_Impl: Sized {
    fn GetReadingsFromTriggerDetails(&mut self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProximitySensorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorStatics2_Vtbl {
        unsafe extern "system" fn GetReadingsFromTriggerDetails<Impl: IProximitySensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadingsFromTriggerDetails(&*(&triggerdetails as *const <SensorDataThresholdTriggerDetails as ::windows::core::Abi>::Abi as *const <SensorDataThresholdTriggerDetails as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximitySensorStatics2, BASE_OFFSET>(),
            GetReadingsFromTriggerDetails: GetReadingsFromTriggerDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait ISensorDataThreshold_Impl: Sized {}
impl ::windows::core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
impl ISensorDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThreshold_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThreshold_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorDataThreshold, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerDetails_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SensorType(&mut self) -> ::windows::core::Result<SensorType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThresholdTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorDataThresholdTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThresholdTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThresholdTriggerDetails_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ISensorDataThresholdTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SensorType<Impl: ISensorDataThresholdTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SensorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorDataThresholdTriggerDetails, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            SensorType: SensorType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThresholdTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorQuaternion_Impl: Sized {
    fn W(&mut self) -> ::windows::core::Result<f32>;
    fn X(&mut self) -> ::windows::core::Result<f32>;
    fn Y(&mut self) -> ::windows::core::Result<f32>;
    fn Z(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorQuaternion";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorQuaternion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorQuaternion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorQuaternion_Vtbl {
        unsafe extern "system" fn W<Impl: ISensorQuaternion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).W() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X<Impl: ISensorQuaternion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Y<Impl: ISensorQuaternion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Y() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Z<Impl: ISensorQuaternion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Z() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorQuaternion, BASE_OFFSET>(),
            W: W::<Impl, IMPL_OFFSET>,
            X: X::<Impl, IMPL_OFFSET>,
            Y: Y::<Impl, IMPL_OFFSET>,
            Z: Z::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorQuaternion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorRotationMatrix_Impl: Sized {
    fn M11(&mut self) -> ::windows::core::Result<f32>;
    fn M12(&mut self) -> ::windows::core::Result<f32>;
    fn M13(&mut self) -> ::windows::core::Result<f32>;
    fn M21(&mut self) -> ::windows::core::Result<f32>;
    fn M22(&mut self) -> ::windows::core::Result<f32>;
    fn M23(&mut self) -> ::windows::core::Result<f32>;
    fn M31(&mut self) -> ::windows::core::Result<f32>;
    fn M32(&mut self) -> ::windows::core::Result<f32>;
    fn M33(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorRotationMatrix";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorRotationMatrix_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorRotationMatrix_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorRotationMatrix_Vtbl {
        unsafe extern "system" fn M11<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M11() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M12<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M13<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M13() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M21<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M21() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M22<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M22() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M23<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M23() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M31<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M31() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M32<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn M33<Impl: ISensorRotationMatrix_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).M33() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorRotationMatrix, BASE_OFFSET>(),
            M11: M11::<Impl, IMPL_OFFSET>,
            M12: M12::<Impl, IMPL_OFFSET>,
            M13: M13::<Impl, IMPL_OFFSET>,
            M21: M21::<Impl, IMPL_OFFSET>,
            M22: M22::<Impl, IMPL_OFFSET>,
            M23: M23::<Impl, IMPL_OFFSET>,
            M31: M31::<Impl, IMPL_OFFSET>,
            M32: M32::<Impl, IMPL_OFFSET>,
            M33: M33::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorRotationMatrix as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensor_Impl: Sized {
    fn GetCurrentOrientation(&mut self) -> ::windows::core::Result<SimpleOrientation>;
    fn OrientationChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensor_Vtbl {
        unsafe extern "system" fn GetCurrentOrientation<Impl: ISimpleOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientationChanged<Impl: ISimpleOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OrientationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOrientationChanged<Impl: ISimpleOrientationSensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensor, BASE_OFFSET>(),
            GetCurrentOrientation: GetCurrentOrientation::<Impl, IMPL_OFFSET>,
            OrientationChanged: OrientationChanged::<Impl, IMPL_OFFSET>,
            RemoveOrientationChanged: RemoveOrientationChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensor2_Impl: Sized {
    fn SetReadingTransform(&mut self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&mut self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensor2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ISimpleOrientationSensor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensor2_Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: ISimpleOrientationSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: ISimpleOrientationSensor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensor2, BASE_OFFSET>(),
            SetReadingTransform: SetReadingTransform::<Impl, IMPL_OFFSET>,
            ReadingTransform: ReadingTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorDeviceId_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleOrientationSensorDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorDeviceId_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ISimpleOrientationSensorDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensorDeviceId, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensorOrientationChangedEventArgs_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Orientation(&mut self) -> ::windows::core::Result<SimpleOrientation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorOrientationChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorOrientationChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: ISimpleOrientationSensorOrientationChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: ISimpleOrientationSensorOrientationChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensorOrientationChangedEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorOrientationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<SimpleOrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleOrientationSensorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ISimpleOrientationSensorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensorStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensorStatics2_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISimpleOrientationSensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ISimpleOrientationSensorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleOrientationSensorStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorStatics2 as ::windows::core::Interface>::IID
    }
}
