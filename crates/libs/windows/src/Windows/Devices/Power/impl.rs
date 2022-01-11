#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBatteryImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetReport(&self) -> ::windows::core::Result<BatteryReport>;
    fn ReportUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReportUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBattery {
    const NAME: &'static str = "Windows.Devices.Power.IBattery";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBatteryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBatteryVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetReport<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportUpdated<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReportUpdated<Impl: IBatteryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReportUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBattery>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, GetReport::<Impl, IMPL_OFFSET>, ReportUpdated::<Impl, IMPL_OFFSET>, RemoveReportUpdated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBattery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System_Power", feature = "implement_exclusive"))]
pub trait IBatteryReportImpl: Sized {
    fn ChargeRateInMilliwatts(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn DesignCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn FullChargeCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn RemainingCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Status(&self) -> ::windows::core::Result<super::super::System::Power::BatteryStatus>;
}
#[cfg(all(feature = "Foundation", feature = "System_Power", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBatteryReport {
    const NAME: &'static str = "Windows.Devices.Power.IBatteryReport";
}
#[cfg(all(feature = "Foundation", feature = "System_Power", feature = "implement_exclusive"))]
impl IBatteryReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBatteryReportVtbl {
        unsafe extern "system" fn ChargeRateInMilliwatts<Impl: IBatteryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChargeRateInMilliwatts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesignCapacityInMilliwattHours<Impl: IBatteryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesignCapacityInMilliwattHours() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullChargeCapacityInMilliwattHours<Impl: IBatteryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullChargeCapacityInMilliwattHours() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingCapacityInMilliwattHours<Impl: IBatteryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingCapacityInMilliwattHours() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IBatteryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::Power::BatteryStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
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
            ::windows::core::GetRuntimeClassName::<IBatteryReport>,
            ::windows::core::GetTrustLevel,
            ChargeRateInMilliwatts::<Impl, IMPL_OFFSET>,
            DesignCapacityInMilliwattHours::<Impl, IMPL_OFFSET>,
            FullChargeCapacityInMilliwattHours::<Impl, IMPL_OFFSET>,
            RemainingCapacityInMilliwattHours::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBatteryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBatteryStaticsImpl: Sized {
    fn AggregateBattery(&self) -> ::windows::core::Result<Battery>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Battery>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBatteryStatics {
    const NAME: &'static str = "Windows.Devices.Power.IBatteryStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBatteryStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBatteryStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBatteryStaticsVtbl {
        unsafe extern "system" fn AggregateBattery<Impl: IBatteryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateBattery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IBatteryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IBatteryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBatteryStatics>, ::windows::core::GetTrustLevel, AggregateBattery::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBatteryStatics as ::windows::core::Interface>::IID
    }
}
