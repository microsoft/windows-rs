#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<AccelerometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Shaken(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShaken(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Shaken<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShaken<Impl: IAccelerometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShaken(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccelerometer>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
            Shaken::<Impl, IMPL_OFFSET>,
            RemoveShaken::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IAccelerometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IAccelerometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IAccelerometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IAccelerometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometer2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IAccelerometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IAccelerometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IAccelerometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometer3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer4Impl: Sized {
    fn ReadingType(&self) -> ::windows::core::Result<AccelerometerReadingType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer4Vtbl {
        unsafe extern "system" fn ReadingType<Impl: IAccelerometer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccelerometerReadingType) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometer4>, ::windows::core::GetTrustLevel, ReadingType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer5Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<AccelerometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometer5 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometer5";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometer5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometer5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometer5Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IAccelerometer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometer5>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometer5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDataThresholdImpl: Sized {
    fn XAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetXAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetYAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetZAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerDataThresholdVtbl {
        unsafe extern "system" fn XAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetXAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisInGForce(value).into()
        }
        unsafe extern "system" fn YAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetYAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisInGForce(value).into()
        }
        unsafe extern "system" fn ZAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZAxisInGForce<Impl: IAccelerometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisInGForce(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccelerometerDataThreshold>,
            ::windows::core::GetTrustLevel,
            XAxisInGForce::<Impl, IMPL_OFFSET>,
            SetXAxisInGForce::<Impl, IMPL_OFFSET>,
            YAxisInGForce::<Impl, IMPL_OFFSET>,
            SetYAxisInGForce::<Impl, IMPL_OFFSET>,
            ZAxisInGForce::<Impl, IMPL_OFFSET>,
            SetZAxisInGForce::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: IAccelerometerDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AccelerationX(&self) -> ::windows::core::Result<f64>;
    fn AccelerationY(&self) -> ::windows::core::Result<f64>;
    fn AccelerationZ(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IAccelerometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccelerationX<Impl: IAccelerometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccelerationY<Impl: IAccelerometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccelerationZ<Impl: IAccelerometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, AccelerationX::<Impl, IMPL_OFFSET>, AccelerationY::<Impl, IMPL_OFFSET>, AccelerationZ::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAccelerometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAccelerometerReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IAccelerometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IAccelerometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<AccelerometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IAccelerometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerShakenEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerShakenEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerShakenEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerShakenEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerShakenEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: IAccelerometerShakenEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerShakenEventArgs>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerShakenEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAccelerometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStatics2Impl: Sized {
    fn GetDefaultWithAccelerometerReadingType(&self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccelerometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccelerometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStatics2Vtbl {
        unsafe extern "system" fn GetDefaultWithAccelerometerReadingType<Impl: IAccelerometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerStatics2>, ::windows::core::GetTrustLevel, GetDefaultWithAccelerometerReadingType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccelerometerStatics3Impl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>>;
    fn GetDeviceSelector(&self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccelerometerStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAccelerometerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccelerometerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccelerometerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccelerometerStatics3Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IAccelerometerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IAccelerometerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccelerometerStatics3>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccelerometerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivitySensorImpl: Sized {
    fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>;
    fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ActivityType>>;
    fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensor";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivitySensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorVtbl {
        unsafe extern "system" fn GetCurrentReadingAsync<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SubscribedActivities<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PowerInMilliwatts<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedActivities<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IActivitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivitySensor>,
            ::windows::core::GetTrustLevel,
            GetCurrentReadingAsync::<Impl, IMPL_OFFSET>,
            SubscribedActivities::<Impl, IMPL_OFFSET>,
            PowerInMilliwatts::<Impl, IMPL_OFFSET>,
            DeviceId::<Impl, IMPL_OFFSET>,
            SupportedActivities::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IActivitySensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Activity(&self) -> ::windows::core::Result<ActivityType>;
    fn Confidence(&self) -> ::windows::core::Result<ActivitySensorReadingConfidence>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivitySensorReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IActivitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Activity<Impl: IActivitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivityType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Confidence<Impl: IActivitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivitySensorReadingConfidence) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivitySensorReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, Activity::<Impl, IMPL_OFFSET>, Confidence::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangeReportImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReadingChangeReport";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorReadingChangeReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReadingChangeReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReadingChangeReportVtbl {
        unsafe extern "system" fn Reading<Impl: IActivitySensorReadingChangeReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivitySensorReadingChangeReport>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReadingChangeReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IActivitySensorReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivitySensorReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IActivitySensorStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetSystemHistoryAsync(&self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
    fn GetSystemHistoryWithDurationAsync(&self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivitySensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IActivitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IActivitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IActivitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSystemHistoryAsync<Impl: IActivitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSystemHistoryWithDurationAsync<Impl: IActivitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivitySensorStatics>,
            ::windows::core::GetTrustLevel,
            GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryWithDurationAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivitySensorTriggerDetailsImpl: Sized {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.IActivitySensorTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivitySensorTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorTriggerDetailsVtbl {
        unsafe extern "system" fn ReadReports<Impl: IActivitySensorTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivitySensorTriggerDetails>, ::windows::core::GetTrustLevel, ReadReports::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAltimeterImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<AltimeterReading>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAltimeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IAltimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAltimeter>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            DeviceId::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeter2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeter2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeter2";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeter2Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IAltimeter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IAltimeter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IAltimeter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAltimeter2>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAltimeterReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AltitudeChangeInMeters(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAltimeterReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IAltimeterReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AltitudeChangeInMeters<Impl: IAltimeterReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAltimeterReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, AltitudeChangeInMeters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAltimeterReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAltimeterReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAltimeterReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IAltimeterReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IAltimeterReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAltimeterReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<AltimeterReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeterReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IAltimeterReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAltimeterReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Altimeter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAltimeterStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IAltimeterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAltimeterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAltimeterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAltimeterStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAltimeterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAltimeterStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAltimeterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<BarometerReading>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IBarometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBarometer>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            DeviceId::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometer2Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IBarometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IBarometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IBarometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometer2>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer3Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<BarometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometer3Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IBarometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometer3>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerDataThresholdImpl: Sized {
    fn Hectopascals(&self) -> ::windows::core::Result<f64>;
    fn SetHectopascals(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerDataThresholdVtbl {
        unsafe extern "system" fn Hectopascals<Impl: IBarometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHectopascals<Impl: IBarometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHectopascals(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerDataThreshold>, ::windows::core::GetTrustLevel, Hectopascals::<Impl, IMPL_OFFSET>, SetHectopascals::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn StationPressureInHectopascals(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IBarometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StationPressureInHectopascals<Impl: IBarometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, StationPressureInHectopascals::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBarometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBarometerReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IBarometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IBarometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<BarometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IBarometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Barometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBarometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IBarometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarometerStatics2Impl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Barometer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IBarometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarometerStatics2Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBarometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IBarometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarometerStatics2>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompassImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<CompassReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompass {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: ICompassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICompass>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait ICompass2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompass2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ICompass2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: ICompass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: ICompass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompass2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompass3 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompass3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: ICompass3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: ICompass3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: ICompass3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompass3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<CompassDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompass4 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompass4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompass4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompass4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompass4Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: ICompass4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompass4>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompass4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDataThresholdImpl: Sized {
    fn Degrees(&self) -> ::windows::core::Result<f64>;
    fn SetDegrees(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassDataThresholdVtbl {
        unsafe extern "system" fn Degrees<Impl: ICompassDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDegrees<Impl: ICompassDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDegrees(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassDataThreshold>, ::windows::core::GetTrustLevel, Degrees::<Impl, IMPL_OFFSET>, SetDegrees::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: ICompassDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompassReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn HeadingMagneticNorth(&self) -> ::windows::core::Result<f64>;
    fn HeadingTrueNorth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompassReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: ICompassReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeadingMagneticNorth<Impl: ICompassReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeadingTrueNorth<Impl: ICompassReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, HeadingMagneticNorth::<Impl, IMPL_OFFSET>, HeadingTrueNorth::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompassReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompassReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: ICompassReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: ICompassReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<CompassReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: ICompassReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingHeadingAccuracyImpl: Sized {
    fn HeadingAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassReadingHeadingAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassReadingHeadingAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassReadingHeadingAccuracyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassReadingHeadingAccuracyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassReadingHeadingAccuracyVtbl {
        unsafe extern "system" fn HeadingAccuracy<Impl: ICompassReadingHeadingAccuracyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassReadingHeadingAccuracy>, ::windows::core::GetTrustLevel, HeadingAccuracy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassReadingHeadingAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Compass>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompassStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompassStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ICompassStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompassStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Compass>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompassStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ICompassStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompassStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompassStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompassStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ICompassStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ICompassStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompassStatics2>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompassStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<GyrometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IGyrometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGyrometer>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IGyrometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IGyrometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IGyrometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IGyrometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometer2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IGyrometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IGyrometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IGyrometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometer3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<GyrometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometer4Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IGyrometer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometer4>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDataThresholdImpl: Sized {
    fn XAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetXAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetYAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetZAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerDataThresholdVtbl {
        unsafe extern "system" fn XAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetXAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisInDegreesPerSecond(value).into()
        }
        unsafe extern "system" fn YAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetYAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisInDegreesPerSecond(value).into()
        }
        unsafe extern "system" fn ZAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZAxisInDegreesPerSecond<Impl: IGyrometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisInDegreesPerSecond(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGyrometerDataThreshold>,
            ::windows::core::GetTrustLevel,
            XAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetXAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            YAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetYAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            ZAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
            SetZAxisInDegreesPerSecond::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: IGyrometerDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngularVelocityX(&self) -> ::windows::core::Result<f64>;
    fn AngularVelocityY(&self) -> ::windows::core::Result<f64>;
    fn AngularVelocityZ(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IGyrometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AngularVelocityX<Impl: IGyrometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AngularVelocityY<Impl: IGyrometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AngularVelocityZ<Impl: IGyrometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, AngularVelocityX::<Impl, IMPL_OFFSET>, AngularVelocityY::<Impl, IMPL_OFFSET>, AngularVelocityZ::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGyrometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGyrometerReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IGyrometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IGyrometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<GyrometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IGyrometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Gyrometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGyrometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGyrometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGyrometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGyrometerStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGyrometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IGyrometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGyrometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGyrometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGyrometerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IGyrometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IGyrometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGyrometerStatics2>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGyrometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHingeAngleReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngleInDegrees(&self) -> ::windows::core::Result<f64>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleReading";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHingeAngleReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IHingeAngleReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AngleInDegrees<Impl: IHingeAngleReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IHingeAngleReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHingeAngleReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, AngleInDegrees::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHingeAngleSensorImpl: Sized {
    fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinReportThresholdInDegrees(&self) -> ::windows::core::Result<f64>;
    fn ReportThresholdInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetReportThresholdInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHingeAngleSensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensorVtbl {
        unsafe extern "system" fn GetCurrentReadingAsync<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinReportThresholdInDegrees<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportThresholdInDegrees<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportThresholdInDegrees<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportThresholdInDegrees(value).into()
        }
        unsafe extern "system" fn ReadingChanged<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IHingeAngleSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHingeAngleSensor>,
            ::windows::core::GetTrustLevel,
            GetCurrentReadingAsync::<Impl, IMPL_OFFSET>,
            DeviceId::<Impl, IMPL_OFFSET>,
            MinReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            ReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            SetReportThresholdInDegrees::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<HingeAngleReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHingeAngleSensorReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensorReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensorReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IHingeAngleSensorReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHingeAngleSensorReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHingeAngleSensorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn GetRelatedToAdjacentPanelsAsync(&self, firstpanelid: &::windows::core::HSTRING, secondpanelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHingeAngleSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IHingeAngleSensorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHingeAngleSensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHingeAngleSensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHingeAngleSensorStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IHingeAngleSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: IHingeAngleSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRelatedToAdjacentPanelsAsync<Impl: IHingeAngleSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, secondpanelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IHingeAngleSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHingeAngleSensorStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, GetDefaultAsync::<Impl, IMPL_OFFSET>, GetRelatedToAdjacentPanelsAsync::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHingeAngleSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<InclinometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IInclinometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInclinometer>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IInclinometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IInclinometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IInclinometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IInclinometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingType<Impl: IInclinometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometer2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>, ReadingType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IInclinometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IInclinometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IInclinometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometer3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<InclinometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometer4Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IInclinometer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometer4>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDataThresholdImpl: Sized {
    fn PitchInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetPitchInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RollInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRollInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn YawInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetYawInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerDataThresholdVtbl {
        unsafe extern "system" fn PitchInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPitchInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPitchInDegrees(value).into()
        }
        unsafe extern "system" fn RollInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRollInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRollInDegrees(value).into()
        }
        unsafe extern "system" fn YawInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetYawInDegrees<Impl: IInclinometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYawInDegrees(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInclinometerDataThreshold>,
            ::windows::core::GetTrustLevel,
            PitchInDegrees::<Impl, IMPL_OFFSET>,
            SetPitchInDegrees::<Impl, IMPL_OFFSET>,
            RollInDegrees::<Impl, IMPL_OFFSET>,
            SetRollInDegrees::<Impl, IMPL_OFFSET>,
            YawInDegrees::<Impl, IMPL_OFFSET>,
            SetYawInDegrees::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: IInclinometerDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PitchDegrees(&self) -> ::windows::core::Result<f32>;
    fn RollDegrees(&self) -> ::windows::core::Result<f32>;
    fn YawDegrees(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IInclinometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PitchDegrees<Impl: IInclinometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RollDegrees<Impl: IInclinometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn YawDegrees<Impl: IInclinometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, PitchDegrees::<Impl, IMPL_OFFSET>, RollDegrees::<Impl, IMPL_OFFSET>, YawDegrees::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInclinometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInclinometerReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IInclinometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IInclinometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<InclinometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IInclinometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingYawAccuracyImpl: Sized {
    fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerReadingYawAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerReadingYawAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerReadingYawAccuracyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerReadingYawAccuracyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerReadingYawAccuracyVtbl {
        unsafe extern "system" fn YawAccuracy<Impl: IInclinometerReadingYawAccuracyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerReadingYawAccuracy>, ::windows::core::GetTrustLevel, YawAccuracy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerReadingYawAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IInclinometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics2Impl: Sized {
    fn GetDefaultForRelativeReadings(&self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics2Vtbl {
        unsafe extern "system" fn GetDefaultForRelativeReadings<Impl: IInclinometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerStatics2>, ::windows::core::GetTrustLevel, GetDefaultForRelativeReadings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics3Impl: Sized {
    fn GetDefaultWithSensorReadingType(&self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInclinometerStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IInclinometerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics3Vtbl {
        unsafe extern "system" fn GetDefaultWithSensorReadingType<Impl: IInclinometerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerStatics3>, ::windows::core::GetTrustLevel, GetDefaultWithSensorReadingType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInclinometerStatics4Impl: Sized {
    fn GetDeviceSelector(&self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInclinometerStatics4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IInclinometerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInclinometerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInclinometerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInclinometerStatics4Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IInclinometerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IInclinometerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInclinometerStatics4>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInclinometerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensorImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<LightSensorReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: ILightSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILightSensor>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor2";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensor2Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: ILightSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: ILightSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: ILightSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensor2>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor3Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<LightSensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensor3 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensor3";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensor3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensor3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensor3Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: ILightSensor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensor3>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDataThresholdImpl: Sized {
    fn LuxPercentage(&self) -> ::windows::core::Result<f32>;
    fn SetLuxPercentage(&self, value: f32) -> ::windows::core::Result<()>;
    fn AbsoluteLux(&self) -> ::windows::core::Result<f32>;
    fn SetAbsoluteLux(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorDataThresholdVtbl {
        unsafe extern "system" fn LuxPercentage<Impl: ILightSensorDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLuxPercentage<Impl: ILightSensorDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLuxPercentage(value).into()
        }
        unsafe extern "system" fn AbsoluteLux<Impl: ILightSensorDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAbsoluteLux<Impl: ILightSensorDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbsoluteLux(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorDataThreshold>, ::windows::core::GetTrustLevel, LuxPercentage::<Impl, IMPL_OFFSET>, SetLuxPercentage::<Impl, IMPL_OFFSET>, AbsoluteLux::<Impl, IMPL_OFFSET>, SetAbsoluteLux::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: ILightSensorDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IlluminanceInLux(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensorReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: ILightSensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IlluminanceInLux<Impl: ILightSensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, IlluminanceInLux::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILightSensorReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILightSensorReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: ILightSensorReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: ILightSensorReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<LightSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: ILightSensorReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<LightSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILightSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILightSensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ILightSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILightSensorStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LightSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILightSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ILightSensorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILightSensorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILightSensorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILightSensorStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ILightSensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ILightSensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILightSensorStatics2>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILightSensorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<MagnetometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IMagnetometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMagnetometer>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IMagnetometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IMagnetometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IMagnetometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IMagnetometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometer2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometer3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer3";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IMagnetometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IMagnetometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IMagnetometer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometer3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<MagnetometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometer4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometer4";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometer4Vtbl {
        unsafe extern "system" fn ReportThreshold<Impl: IMagnetometer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometer4>, ::windows::core::GetTrustLevel, ReportThreshold::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDataThresholdImpl: Sized {
    fn XAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetXAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
    fn YAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetYAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
    fn ZAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetZAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerDataThreshold";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerDataThresholdVtbl {
        unsafe extern "system" fn XAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetXAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXAxisMicroteslas(value).into()
        }
        unsafe extern "system" fn YAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetYAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisMicroteslas(value).into()
        }
        unsafe extern "system" fn ZAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetZAxisMicroteslas<Impl: IMagnetometerDataThresholdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZAxisMicroteslas(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMagnetometerDataThreshold>,
            ::windows::core::GetTrustLevel,
            XAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetXAxisMicroteslas::<Impl, IMPL_OFFSET>,
            YAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetYAxisMicroteslas::<Impl, IMPL_OFFSET>,
            ZAxisMicroteslas::<Impl, IMPL_OFFSET>,
            SetZAxisMicroteslas::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMagnetometerDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn MagneticFieldX(&self) -> ::windows::core::Result<f32>;
    fn MagneticFieldY(&self) -> ::windows::core::Result<f32>;
    fn MagneticFieldZ(&self) -> ::windows::core::Result<f32>;
    fn DirectionalAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IMagnetometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MagneticFieldX<Impl: IMagnetometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MagneticFieldY<Impl: IMagnetometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MagneticFieldZ<Impl: IMagnetometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DirectionalAccuracy<Impl: IMagnetometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, MagneticFieldX::<Impl, IMPL_OFFSET>, MagneticFieldY::<Impl, IMPL_OFFSET>, MagneticFieldZ::<Impl, IMPL_OFFSET>, DirectionalAccuracy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMagnetometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMagnetometerReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IMagnetometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IMagnetometerReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<MagnetometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IMagnetometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Magnetometer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagnetometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagnetometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IMagnetometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagnetometerStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagnetometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IMagnetometerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagnetometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagnetometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagnetometerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IMagnetometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IMagnetometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMagnetometerStatics2>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagnetometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensorImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<OrientationSensorReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOrientationSensor>,
            ::windows::core::GetTrustLevel,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait IOrientationSensor2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl IOrientationSensor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensor2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: IOrientationSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: IOrientationSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingType<Impl: IOrientationSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensor2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>, ReadingType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensor3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensor3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensor3";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensor3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensor3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensor3Vtbl {
        unsafe extern "system" fn SetReportLatency<Impl: IOrientationSensor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportLatency(value).into()
        }
        unsafe extern "system" fn ReportLatency<Impl: IOrientationSensor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBatchSize<Impl: IOrientationSensor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensor3>, ::windows::core::GetTrustLevel, SetReportLatency::<Impl, IMPL_OFFSET>, ReportLatency::<Impl, IMPL_OFFSET>, MaxBatchSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: IOrientationSensorDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RotationMatrix(&self) -> ::windows::core::Result<SensorRotationMatrix>;
    fn Quaternion(&self) -> ::windows::core::Result<SensorQuaternion>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensorReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IOrientationSensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RotationMatrix<Impl: IOrientationSensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Quaternion<Impl: IOrientationSensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, RotationMatrix::<Impl, IMPL_OFFSET>, Quaternion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOrientationSensorReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorReading2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReading2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOrientationSensorReading2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReading2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReading2Vtbl {
        unsafe extern "system" fn PerformanceCount<Impl: IOrientationSensorReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IOrientationSensorReading2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorReading2>, ::windows::core::GetTrustLevel, PerformanceCount::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReading2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<OrientationSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IOrientationSensorReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingYawAccuracyImpl: Sized {
    fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorReadingYawAccuracy {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorReadingYawAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorReadingYawAccuracyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorReadingYawAccuracyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorReadingYawAccuracyVtbl {
        unsafe extern "system" fn YawAccuracy<Impl: IOrientationSensorReadingYawAccuracyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorReadingYawAccuracy>, ::windows::core::GetTrustLevel, YawAccuracy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorReadingYawAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IOrientationSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics2Impl: Sized {
    fn GetDefaultForRelativeReadings(&self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics2Vtbl {
        unsafe extern "system" fn GetDefaultForRelativeReadings<Impl: IOrientationSensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorStatics2>, ::windows::core::GetTrustLevel, GetDefaultForRelativeReadings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics3Impl: Sized {
    fn GetDefaultWithSensorReadingType(&self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<OrientationSensor>;
    fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(&self, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientationSensorStatics3 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientationSensorStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics3Vtbl {
        unsafe extern "system" fn GetDefaultWithSensorReadingType<Impl: IOrientationSensorStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal<Impl: IOrientationSensorStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorStatics3>, ::windows::core::GetTrustLevel, GetDefaultWithSensorReadingType::<Impl, IMPL_OFFSET>, GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOrientationSensorStatics4Impl: Sized {
    fn GetDeviceSelector(&self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(&self, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientationSensorStatics4 {
    const NAME: &'static str = "Windows.Devices.Sensors.IOrientationSensorStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientationSensorStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientationSensorStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientationSensorStatics4Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IOrientationSensorStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal<Impl: IOrientationSensorStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IOrientationSensorStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOrientationSensorStatics4>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientationSensorStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PowerInMilliwatts<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimumReportInterval<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IPedometerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPedometer>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, IMPL_OFFSET>,
            PowerInMilliwatts::<Impl, IMPL_OFFSET>,
            MinimumReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval::<Impl, IMPL_OFFSET>,
            ReportInterval::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPedometer2Impl: Sized {
    fn GetCurrentReadings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometer2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometer2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPedometer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometer2Vtbl {
        unsafe extern "system" fn GetCurrentReadings<Impl: IPedometer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPedometer2>, ::windows::core::GetTrustLevel, GetCurrentReadings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerDataThresholdFactoryImpl: Sized {
    fn Create(&self, sensor: &::core::option::Option<Pedometer>, stepgoal: i32) -> ::windows::core::Result<PedometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPedometerDataThresholdFactory {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerDataThresholdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPedometerDataThresholdFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerDataThresholdFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerDataThresholdFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPedometerDataThresholdFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensor: ::windows::core::RawPtr, stepgoal: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPedometerDataThresholdFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerDataThresholdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometerReadingImpl: Sized {
    fn StepKind(&self) -> ::windows::core::Result<PedometerStepKind>;
    fn CumulativeSteps(&self) -> ::windows::core::Result<i32>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CumulativeStepsDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometerReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerReadingVtbl {
        unsafe extern "system" fn StepKind<Impl: IPedometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PedometerStepKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CumulativeSteps<Impl: IPedometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: IPedometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CumulativeStepsDuration<Impl: IPedometerReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPedometerReading>, ::windows::core::GetTrustLevel, StepKind::<Impl, IMPL_OFFSET>, CumulativeSteps::<Impl, IMPL_OFFSET>, Timestamp::<Impl, IMPL_OFFSET>, CumulativeStepsDuration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<PedometerReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPedometerReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IPedometerReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPedometerReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPedometerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSystemHistoryAsync(&self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
    fn GetSystemHistoryWithDurationAsync(&self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPedometerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPedometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: IPedometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IPedometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSystemHistoryAsync<Impl: IPedometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSystemHistoryWithDurationAsync<Impl: IPedometerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPedometerStatics>,
            ::windows::core::GetTrustLevel,
            FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetSystemHistoryAsync::<Impl, IMPL_OFFSET>,
            GetSystemHistoryWithDurationAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPedometerStatics2Impl: Sized {
    fn GetReadingsFromTriggerDetails(&self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPedometerStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IPedometerStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPedometerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPedometerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPedometerStatics2Vtbl {
        unsafe extern "system" fn GetReadingsFromTriggerDetails<Impl: IPedometerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPedometerStatics2>, ::windows::core::GetTrustLevel, GetReadingsFromTriggerDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPedometerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProximitySensorImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MinDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<ProximitySensorReading>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateDisplayOnOffController(&self) -> ::windows::core::Result<ProximitySensorDisplayOnOffController>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProximitySensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorVtbl {
        unsafe extern "system" fn DeviceId<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxDistanceInMillimeters<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinDistanceInMillimeters<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentReading<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingChanged<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveReadingChanged<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateDisplayOnOffController<Impl: IProximitySensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProximitySensor>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, IMPL_OFFSET>,
            MaxDistanceInMillimeters::<Impl, IMPL_OFFSET>,
            MinDistanceInMillimeters::<Impl, IMPL_OFFSET>,
            GetCurrentReading::<Impl, IMPL_OFFSET>,
            ReadingChanged::<Impl, IMPL_OFFSET>,
            RemoveReadingChanged::<Impl, IMPL_OFFSET>,
            CreateDisplayOnOffController::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorDataThresholdFactoryImpl: Sized {
    fn Create(&self, sensor: &::core::option::Option<ProximitySensor>) -> ::windows::core::Result<ProximitySensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorDataThresholdFactory {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorDataThresholdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorDataThresholdFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorDataThresholdFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorDataThresholdFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IProximitySensorDataThresholdFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProximitySensorDataThresholdFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorDataThresholdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProximitySensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsDetected(&self) -> ::windows::core::Result<bool>;
    fn DistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorReading";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProximitySensorReadingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorReadingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorReadingVtbl {
        unsafe extern "system" fn Timestamp<Impl: IProximitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDetected<Impl: IProximitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DistanceInMillimeters<Impl: IProximitySensorReadingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProximitySensorReading>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, IsDetected::<Impl, IMPL_OFFSET>, DistanceInMillimeters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorReading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ProximitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorReadingChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorReadingChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorReadingChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorReadingChangedEventArgsVtbl {
        unsafe extern "system" fn Reading<Impl: IProximitySensorReadingChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProximitySensorReadingChangedEventArgs>, ::windows::core::GetTrustLevel, Reading::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorReadingChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&self, sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximitySensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximitySensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProximitySensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IProximitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromId<Impl: IProximitySensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProximitySensorStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProximitySensorStatics2Impl: Sized {
    fn GetReadingsFromTriggerDetails(&self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximitySensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.IProximitySensorStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProximitySensorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximitySensorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximitySensorStatics2Vtbl {
        unsafe extern "system" fn GetReadingsFromTriggerDetails<Impl: IProximitySensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProximitySensorStatics2>, ::windows::core::GetTrustLevel, GetReadingsFromTriggerDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximitySensorStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait ISensorDataThresholdImpl: Sized {}
impl ::windows::core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
impl ISensorDataThresholdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThresholdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThresholdVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorDataThreshold>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThreshold as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SensorType(&self) -> ::windows::core::Result<SensorType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThresholdTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorDataThresholdTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThresholdTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThresholdTriggerDetailsVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISensorDataThresholdTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SensorType<Impl: ISensorDataThresholdTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SensorType) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorDataThresholdTriggerDetails>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, SensorType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThresholdTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorQuaternionImpl: Sized {
    fn W(&self) -> ::windows::core::Result<f32>;
    fn X(&self) -> ::windows::core::Result<f32>;
    fn Y(&self) -> ::windows::core::Result<f32>;
    fn Z(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorQuaternion";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorQuaternionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorQuaternionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorQuaternionVtbl {
        unsafe extern "system" fn W<Impl: ISensorQuaternionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn X<Impl: ISensorQuaternionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Y<Impl: ISensorQuaternionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Z<Impl: ISensorQuaternionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorQuaternion>, ::windows::core::GetTrustLevel, W::<Impl, IMPL_OFFSET>, X::<Impl, IMPL_OFFSET>, Y::<Impl, IMPL_OFFSET>, Z::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorQuaternion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorRotationMatrixImpl: Sized {
    fn M11(&self) -> ::windows::core::Result<f32>;
    fn M12(&self) -> ::windows::core::Result<f32>;
    fn M13(&self) -> ::windows::core::Result<f32>;
    fn M21(&self) -> ::windows::core::Result<f32>;
    fn M22(&self) -> ::windows::core::Result<f32>;
    fn M23(&self) -> ::windows::core::Result<f32>;
    fn M31(&self) -> ::windows::core::Result<f32>;
    fn M32(&self) -> ::windows::core::Result<f32>;
    fn M33(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorRotationMatrix";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorRotationMatrixVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorRotationMatrixImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorRotationMatrixVtbl {
        unsafe extern "system" fn M11<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M12<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M13<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M21<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M22<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M23<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M31<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M32<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn M33<Impl: ISensorRotationMatrixImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISensorRotationMatrix>,
            ::windows::core::GetTrustLevel,
            M11::<Impl, IMPL_OFFSET>,
            M12::<Impl, IMPL_OFFSET>,
            M13::<Impl, IMPL_OFFSET>,
            M21::<Impl, IMPL_OFFSET>,
            M22::<Impl, IMPL_OFFSET>,
            M23::<Impl, IMPL_OFFSET>,
            M31::<Impl, IMPL_OFFSET>,
            M32::<Impl, IMPL_OFFSET>,
            M33::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorRotationMatrix as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensorImpl: Sized {
    fn GetCurrentOrientation(&self) -> ::windows::core::Result<SimpleOrientation>;
    fn OrientationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorVtbl {
        unsafe extern "system" fn GetCurrentOrientation<Impl: ISimpleOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OrientationChanged<Impl: ISimpleOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOrientationChanged<Impl: ISimpleOrientationSensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOrientationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensor>, ::windows::core::GetTrustLevel, GetCurrentOrientation::<Impl, IMPL_OFFSET>, OrientationChanged::<Impl, IMPL_OFFSET>, RemoveOrientationChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensor2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensor2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensor2";
}
#[cfg(all(feature = "Graphics_Display", feature = "implement_exclusive"))]
impl ISimpleOrientationSensor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensor2Vtbl {
        unsafe extern "system" fn SetReadingTransform<Impl: ISimpleOrientationSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadingTransform(value).into()
        }
        unsafe extern "system" fn ReadingTransform<Impl: ISimpleOrientationSensor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensor2>, ::windows::core::GetTrustLevel, SetReadingTransform::<Impl, IMPL_OFFSET>, ReadingTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorDeviceId {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleOrientationSensorDeviceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorDeviceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorDeviceIdVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISimpleOrientationSensorDeviceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensorDeviceId>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensorOrientationChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Orientation(&self) -> ::windows::core::Result<SimpleOrientation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorOrientationChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensorOrientationChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorOrientationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorOrientationChangedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: ISimpleOrientationSensorOrientationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Orientation<Impl: ISimpleOrientationSensorOrientationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensorOrientationChangedEventArgs>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, Orientation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorOrientationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SimpleOrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorStatics {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleOrientationSensorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISimpleOrientationSensorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensorStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleOrientationSensorStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleOrientationSensorStatics2 {
    const NAME: &'static str = "Windows.Devices.Sensors.ISimpleOrientationSensorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleOrientationSensorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleOrientationSensorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleOrientationSensorStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISimpleOrientationSensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ISimpleOrientationSensorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleOrientationSensorStatics2>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleOrientationSensorStatics2 as ::windows::core::Interface>::IID
    }
}
