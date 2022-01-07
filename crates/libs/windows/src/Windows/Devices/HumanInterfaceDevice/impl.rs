#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()>;
    fn ControlDescription(&self) -> ::windows::core::Result<HidBooleanControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHidBooleanControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControlImpl, const OFFSET: isize>() -> IHidBooleanControlVtbl {
        unsafe extern "system" fn Id<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActive<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsActive(value).into()
        }
        unsafe extern "system" fn ControlDescription<Impl: IHidBooleanControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidBooleanControl>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, UsagePage::<Impl, OFFSET>, UsageId::<Impl, OFFSET>, IsActive::<Impl, OFFSET>, SetIsActive::<Impl, OFFSET>, ControlDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn ReportId(&self) -> ::windows::core::Result<u16>;
    fn ReportType(&self) -> ::windows::core::Result<HidReportType>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IHidBooleanControlDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>() -> IHidBooleanControlDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportId<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportType<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentCollections<Impl: IHidBooleanControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentCollections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidBooleanControlDescription>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, ReportId::<Impl, OFFSET>, ReportType::<Impl, OFFSET>, UsagePage::<Impl, OFFSET>, UsageId::<Impl, OFFSET>, ParentCollections::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlDescription2Impl: Sized {
    fn IsAbsolute(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidBooleanControlDescription2 {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2";
}
#[cfg(feature = "implement_exclusive")]
impl IHidBooleanControlDescription2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControlDescription2Impl, const OFFSET: isize>() -> IHidBooleanControlDescription2Vtbl {
        unsafe extern "system" fn IsAbsolute<Impl: IHidBooleanControlDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAbsolute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidBooleanControlDescription2>, ::windows::core::GetTrustLevel, IsAbsolute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Type(&self) -> ::windows::core::Result<HidCollectionType>;
    fn UsagePage(&self) -> ::windows::core::Result<u32>;
    fn UsageId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHidCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidCollectionImpl, const OFFSET: isize>() -> IHidCollectionVtbl {
        unsafe extern "system" fn Id<Impl: IHidCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IHidCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidCollectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidCollection>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Type::<Impl, OFFSET>, UsagePage::<Impl, OFFSET>, UsageId::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHidDeviceImpl: Sized + IClosableImpl {
    fn VendorId(&self) -> ::windows::core::Result<u16>;
    fn ProductId(&self) -> ::windows::core::Result<u16>;
    fn Version(&self) -> ::windows::core::Result<u16>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn GetInputReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetFeatureReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn CreateOutputReport(&self) -> ::windows::core::Result<HidOutputReport>;
    fn CreateOutputReportById(&self, reportid: u16) -> ::windows::core::Result<HidOutputReport>;
    fn CreateFeatureReport(&self) -> ::windows::core::Result<HidFeatureReport>;
    fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::core::Result<HidFeatureReport>;
    fn SendOutputReportAsync(&self, outputreport: &::core::option::Option<HidOutputReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendFeatureReportAsync(&self, featurereport: &::core::option::Option<HidFeatureReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>;
    fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>;
    fn InputReportReceived(&self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputReportReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHidDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidDeviceImpl, const OFFSET: isize>() -> IHidDeviceVtbl {
        unsafe extern "system" fn VendorId<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputReportAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputReportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputReportByIdAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputReportByIdAsync(reportid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureReportAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureReportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureReportByIdAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureReportByIdAsync(reportid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOutputReport<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOutputReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOutputReportById<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOutputReportById(reportid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeatureReport<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFeatureReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeatureReportById<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFeatureReportById(reportid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOutputReportAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputreport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOutputReportAsync(&*(&outputreport as *const <HidOutputReport as ::windows::core::Abi>::Abi as *const <HidOutputReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendFeatureReportAsync<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, featurereport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendFeatureReportAsync(&*(&featurereport as *const <HidFeatureReport as ::windows::core::Abi>::Abi as *const <HidFeatureReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanControlDescriptions<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControlDescriptions(reporttype, usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControlDescriptions<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControlDescriptions(reporttype, usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputReportReceived<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputReportReceived(&*(&reporthandler as *const <super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputReportReceived<Impl: IHidDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputReportReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidDevice>,
            ::windows::core::GetTrustLevel,
            VendorId::<Impl, OFFSET>,
            ProductId::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            UsagePage::<Impl, OFFSET>,
            UsageId::<Impl, OFFSET>,
            GetInputReportAsync::<Impl, OFFSET>,
            GetInputReportByIdAsync::<Impl, OFFSET>,
            GetFeatureReportAsync::<Impl, OFFSET>,
            GetFeatureReportByIdAsync::<Impl, OFFSET>,
            CreateOutputReport::<Impl, OFFSET>,
            CreateOutputReportById::<Impl, OFFSET>,
            CreateFeatureReport::<Impl, OFFSET>,
            CreateFeatureReportById::<Impl, OFFSET>,
            SendOutputReportAsync::<Impl, OFFSET>,
            SendFeatureReportAsync::<Impl, OFFSET>,
            GetBooleanControlDescriptions::<Impl, OFFSET>,
            GetNumericControlDescriptions::<Impl, OFFSET>,
            InputReportReceived::<Impl, OFFSET>,
            RemoveInputReportReceived::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorVidPid(&self, usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, accessmode: super::super::Storage::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidDeviceStatics {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHidDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidDeviceStaticsImpl, const OFFSET: isize>() -> IHidDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IHidDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorVidPid<Impl: IHidDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorVidPid(usagepage, usageid, vendorid, productid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IHidDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidDeviceStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorVidPid::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidFeatureReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidFeatureReport";
}
#[cfg(feature = "implement_exclusive")]
impl IHidFeatureReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidFeatureReportImpl, const OFFSET: isize>() -> IHidFeatureReportVtbl {
        unsafe extern "system" fn Id<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBooleanControl<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControlByDescription(&*(&controldescription as *const <HidBooleanControlDescription as ::windows::core::Abi>::Abi as *const <HidBooleanControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControl<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidFeatureReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControlByDescription(&*(&controldescription as *const <HidNumericControlDescription as ::windows::core::Abi>::Abi as *const <HidNumericControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidFeatureReport>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            GetBooleanControl::<Impl, OFFSET>,
            GetBooleanControlByDescription::<Impl, OFFSET>,
            GetNumericControl::<Impl, OFFSET>,
            GetNumericControlByDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidInputReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ActivatedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn TransitionedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidInputReport";
}
#[cfg(feature = "implement_exclusive")]
impl IHidInputReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidInputReportImpl, const OFFSET: isize>() -> IHidInputReportVtbl {
        unsafe extern "system" fn Id<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivatedBooleanControls<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatedBooleanControls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitionedBooleanControls<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionedBooleanControls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanControl<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControlByDescription(&*(&controldescription as *const <HidBooleanControlDescription as ::windows::core::Abi>::Abi as *const <HidBooleanControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControl<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidInputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControlByDescription(&*(&controldescription as *const <HidNumericControlDescription as ::windows::core::Abi>::Abi as *const <HidNumericControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidInputReport>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            ActivatedBooleanControls::<Impl, OFFSET>,
            TransitionedBooleanControls::<Impl, OFFSET>,
            GetBooleanControl::<Impl, OFFSET>,
            GetBooleanControlByDescription::<Impl, OFFSET>,
            GetNumericControl::<Impl, OFFSET>,
            GetNumericControlByDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidInputReportReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<HidInputReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHidInputReportReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidInputReportReceivedEventArgsImpl, const OFFSET: isize>() -> IHidInputReportReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IHidInputReportReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidInputReportReceivedEventArgs>, ::windows::core::GetTrustLevel, Report::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidNumericControlImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn IsGrouped(&self) -> ::windows::core::Result<bool>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn Value(&self) -> ::windows::core::Result<i64>;
    fn SetValue(&self, value: i64) -> ::windows::core::Result<()>;
    fn ScaledValue(&self) -> ::windows::core::Result<i64>;
    fn SetScaledValue(&self, value: i64) -> ::windows::core::Result<()>;
    fn ControlDescription(&self) -> ::windows::core::Result<HidNumericControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidNumericControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHidNumericControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidNumericControlImpl, const OFFSET: isize>() -> IHidNumericControlVtbl {
        unsafe extern "system" fn Id<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrouped<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrouped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn ScaledValue<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaledValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaledValue<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaledValue(value).into()
        }
        unsafe extern "system" fn ControlDescription<Impl: IHidNumericControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidNumericControl>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            IsGrouped::<Impl, OFFSET>,
            UsagePage::<Impl, OFFSET>,
            UsageId::<Impl, OFFSET>,
            Value::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            ScaledValue::<Impl, OFFSET>,
            SetScaledValue::<Impl, OFFSET>,
            ControlDescription::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidNumericControlDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn ReportId(&self) -> ::windows::core::Result<u16>;
    fn ReportType(&self) -> ::windows::core::Result<HidReportType>;
    fn ReportSize(&self) -> ::windows::core::Result<u32>;
    fn ReportCount(&self) -> ::windows::core::Result<u32>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn LogicalMinimum(&self) -> ::windows::core::Result<i32>;
    fn LogicalMaximum(&self) -> ::windows::core::Result<i32>;
    fn PhysicalMinimum(&self) -> ::windows::core::Result<i32>;
    fn PhysicalMaximum(&self) -> ::windows::core::Result<i32>;
    fn UnitExponent(&self) -> ::windows::core::Result<u32>;
    fn Unit(&self) -> ::windows::core::Result<u32>;
    fn IsAbsolute(&self) -> ::windows::core::Result<bool>;
    fn HasNull(&self) -> ::windows::core::Result<bool>;
    fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IHidNumericControlDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>() -> IHidNumericControlDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportId<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportType<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSize<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCount<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageId<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalMinimum<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogicalMaximum<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogicalMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalMinimum<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalMaximum<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitExponent<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnitExponent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unit<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAbsolute<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAbsolute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNull<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNull() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentCollections<Impl: IHidNumericControlDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentCollections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidNumericControlDescription>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            ReportId::<Impl, OFFSET>,
            ReportType::<Impl, OFFSET>,
            ReportSize::<Impl, OFFSET>,
            ReportCount::<Impl, OFFSET>,
            UsagePage::<Impl, OFFSET>,
            UsageId::<Impl, OFFSET>,
            LogicalMinimum::<Impl, OFFSET>,
            LogicalMaximum::<Impl, OFFSET>,
            PhysicalMinimum::<Impl, OFFSET>,
            PhysicalMaximum::<Impl, OFFSET>,
            UnitExponent::<Impl, OFFSET>,
            Unit::<Impl, OFFSET>,
            IsAbsolute::<Impl, OFFSET>,
            HasNull::<Impl, OFFSET>,
            ParentCollections::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidOutputReportImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u16>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidOutputReport";
}
#[cfg(feature = "implement_exclusive")]
impl IHidOutputReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidOutputReportImpl, const OFFSET: isize>() -> IHidOutputReportVtbl {
        unsafe extern "system" fn Id<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBooleanControl<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanControlByDescription(&*(&controldescription as *const <HidBooleanControlDescription as ::windows::core::Abi>::Abi as *const <HidBooleanControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControl<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControl(usagepage, usageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidOutputReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericControlByDescription(&*(&controldescription as *const <HidNumericControlDescription as ::windows::core::Abi>::Abi as *const <HidNumericControlDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHidOutputReport>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            GetBooleanControl::<Impl, OFFSET>,
            GetBooleanControlByDescription::<Impl, OFFSET>,
            GetNumericControl::<Impl, OFFSET>,
            GetNumericControlByDescription::<Impl, OFFSET>,
        )
    }
}
