#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControl_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ControlDescription(&mut self) -> ::windows::core::Result<HidBooleanControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHidBooleanControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidBooleanControl_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsActive<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsActive<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsActive(value).into()
        }
        unsafe extern "system" fn ControlDescription<Impl: IHidBooleanControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidBooleanControl, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            SetIsActive: SetIsActive::<Impl, IMPL_OFFSET>,
            ControlDescription: ControlDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidBooleanControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHidBooleanControlDescription_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn ReportId(&mut self) -> ::windows::core::Result<u16>;
    fn ReportType(&mut self) -> ::windows::core::Result<HidReportType>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn ParentCollections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHidBooleanControlDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControlDescription_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidBooleanControlDescription_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportId<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportType<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParentCollections<Impl: IHidBooleanControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidBooleanControlDescription, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            ReportId: ReportId::<Impl, IMPL_OFFSET>,
            ReportType: ReportType::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            ParentCollections: ParentCollections::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidBooleanControlDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidBooleanControlDescription2_Impl: Sized {
    fn IsAbsolute(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidBooleanControlDescription2 {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidBooleanControlDescription2";
}
#[cfg(feature = "implement_exclusive")]
impl IHidBooleanControlDescription2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidBooleanControlDescription2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidBooleanControlDescription2_Vtbl {
        unsafe extern "system" fn IsAbsolute<Impl: IHidBooleanControlDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidBooleanControlDescription2, BASE_OFFSET>(),
            IsAbsolute: IsAbsolute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidBooleanControlDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidCollection_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Type(&mut self) -> ::windows::core::Result<HidCollectionType>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u32>;
    fn UsageId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHidCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidCollection_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IHidCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidCollectionType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidCollection, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHidDevice_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn VendorId(&mut self) -> ::windows::core::Result<u16>;
    fn ProductId(&mut self) -> ::windows::core::Result<u16>;
    fn Version(&mut self) -> ::windows::core::Result<u16>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn GetInputReportAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetInputReportByIdAsync(&mut self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>>;
    fn GetFeatureReportAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn GetFeatureReportByIdAsync(&mut self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>>;
    fn CreateOutputReport(&mut self) -> ::windows::core::Result<HidOutputReport>;
    fn CreateOutputReportById(&mut self, reportid: u16) -> ::windows::core::Result<HidOutputReport>;
    fn CreateFeatureReport(&mut self) -> ::windows::core::Result<HidFeatureReport>;
    fn CreateFeatureReportById(&mut self, reportid: u16) -> ::windows::core::Result<HidFeatureReport>;
    fn SendOutputReportAsync(&mut self, outputreport: &::core::option::Option<HidOutputReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendFeatureReportAsync(&mut self, featurereport: &::core::option::Option<HidFeatureReport>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetBooleanControlDescriptions(&mut self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>;
    fn GetNumericControlDescriptions(&mut self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>;
    fn InputReportReceived(&mut self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputReportReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidDevice";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHidDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidDevice_Vtbl {
        unsafe extern "system" fn VendorId<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProductId<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Version<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInputReportAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInputReportByIdAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFeatureReportAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFeatureReportByIdAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateOutputReport<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateOutputReportById<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFeatureReport<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFeatureReportById<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendOutputReportAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputreport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendFeatureReportAsync<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, featurereport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanControlDescriptions<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControlDescriptions<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputReportReceived<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveInputReportReceived<Impl: IHidDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputReportReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidDevice, BASE_OFFSET>(),
            VendorId: VendorId::<Impl, IMPL_OFFSET>,
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            GetInputReportAsync: GetInputReportAsync::<Impl, IMPL_OFFSET>,
            GetInputReportByIdAsync: GetInputReportByIdAsync::<Impl, IMPL_OFFSET>,
            GetFeatureReportAsync: GetFeatureReportAsync::<Impl, IMPL_OFFSET>,
            GetFeatureReportByIdAsync: GetFeatureReportByIdAsync::<Impl, IMPL_OFFSET>,
            CreateOutputReport: CreateOutputReport::<Impl, IMPL_OFFSET>,
            CreateOutputReportById: CreateOutputReportById::<Impl, IMPL_OFFSET>,
            CreateFeatureReport: CreateFeatureReport::<Impl, IMPL_OFFSET>,
            CreateFeatureReportById: CreateFeatureReportById::<Impl, IMPL_OFFSET>,
            SendOutputReportAsync: SendOutputReportAsync::<Impl, IMPL_OFFSET>,
            SendFeatureReportAsync: SendFeatureReportAsync::<Impl, IMPL_OFFSET>,
            GetBooleanControlDescriptions: GetBooleanControlDescriptions::<Impl, IMPL_OFFSET>,
            GetNumericControlDescriptions: GetNumericControlDescriptions::<Impl, IMPL_OFFSET>,
            InputReportReceived: InputReportReceived::<Impl, IMPL_OFFSET>,
            RemoveInputReportReceived: RemoveInputReportReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IHidDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorVidPid(&mut self, usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING, accessmode: super::super::Storage::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidDeviceStatics {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IHidDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IHidDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorVidPid<Impl: IHidDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IHidDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorVidPid: GetDeviceSelectorVidPid::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHidFeatureReport_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u16>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&mut self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&mut self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidFeatureReport";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHidFeatureReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidFeatureReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidFeatureReport_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Data<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBooleanControl<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControl<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidFeatureReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidFeatureReport, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetBooleanControl: GetBooleanControl::<Impl, IMPL_OFFSET>,
            GetBooleanControlByDescription: GetBooleanControlByDescription::<Impl, IMPL_OFFSET>,
            GetNumericControl: GetNumericControl::<Impl, IMPL_OFFSET>,
            GetNumericControlByDescription: GetNumericControlByDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidFeatureReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHidInputReport_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u16>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ActivatedBooleanControls(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn TransitionedBooleanControls(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>;
    fn GetBooleanControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&mut self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&mut self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidInputReport";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHidInputReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidInputReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidInputReport_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Data<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivatedBooleanControls<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransitionedBooleanControls<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanControl<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControl<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidInputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidInputReport, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            ActivatedBooleanControls: ActivatedBooleanControls::<Impl, IMPL_OFFSET>,
            TransitionedBooleanControls: TransitionedBooleanControls::<Impl, IMPL_OFFSET>,
            GetBooleanControl: GetBooleanControl::<Impl, IMPL_OFFSET>,
            GetBooleanControlByDescription: GetBooleanControlByDescription::<Impl, IMPL_OFFSET>,
            GetNumericControl: GetNumericControl::<Impl, IMPL_OFFSET>,
            GetNumericControlByDescription: GetNumericControlByDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidInputReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidInputReportReceivedEventArgs_Impl: Sized {
    fn Report(&mut self) -> ::windows::core::Result<HidInputReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidInputReportReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHidInputReportReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidInputReportReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidInputReportReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Report<Impl: IHidInputReportReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHidInputReportReceivedEventArgs, BASE_OFFSET>(), Report: Report::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidInputReportReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidNumericControl_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn IsGrouped(&mut self) -> ::windows::core::Result<bool>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn Value(&mut self) -> ::windows::core::Result<i64>;
    fn SetValue(&mut self, value: i64) -> ::windows::core::Result<()>;
    fn ScaledValue(&mut self) -> ::windows::core::Result<i64>;
    fn SetScaledValue(&mut self, value: i64) -> ::windows::core::Result<()>;
    fn ControlDescription(&mut self) -> ::windows::core::Result<HidNumericControlDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidNumericControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHidNumericControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidNumericControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidNumericControl_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGrouped<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn ScaledValue<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaledValue<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaledValue(value).into()
        }
        unsafe extern "system" fn ControlDescription<Impl: IHidNumericControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidNumericControl, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            IsGrouped: IsGrouped::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            ScaledValue: ScaledValue::<Impl, IMPL_OFFSET>,
            SetScaledValue: SetScaledValue::<Impl, IMPL_OFFSET>,
            ControlDescription: ControlDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidNumericControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHidNumericControlDescription_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn ReportId(&mut self) -> ::windows::core::Result<u16>;
    fn ReportType(&mut self) -> ::windows::core::Result<HidReportType>;
    fn ReportSize(&mut self) -> ::windows::core::Result<u32>;
    fn ReportCount(&mut self) -> ::windows::core::Result<u32>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn LogicalMinimum(&mut self) -> ::windows::core::Result<i32>;
    fn LogicalMaximum(&mut self) -> ::windows::core::Result<i32>;
    fn PhysicalMinimum(&mut self) -> ::windows::core::Result<i32>;
    fn PhysicalMaximum(&mut self) -> ::windows::core::Result<i32>;
    fn UnitExponent(&mut self) -> ::windows::core::Result<u32>;
    fn Unit(&mut self) -> ::windows::core::Result<u32>;
    fn IsAbsolute(&mut self) -> ::windows::core::Result<bool>;
    fn HasNull(&mut self) -> ::windows::core::Result<bool>;
    fn ParentCollections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidNumericControlDescription";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHidNumericControlDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidNumericControlDescription_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidNumericControlDescription_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportId<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportType<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportSize<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportCount<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsageId<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogicalMinimum<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogicalMaximum<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhysicalMinimum<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhysicalMaximum<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnitExponent<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Unit<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAbsolute<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasNull<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParentCollections<Impl: IHidNumericControlDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidNumericControlDescription, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            ReportId: ReportId::<Impl, IMPL_OFFSET>,
            ReportType: ReportType::<Impl, IMPL_OFFSET>,
            ReportSize: ReportSize::<Impl, IMPL_OFFSET>,
            ReportCount: ReportCount::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            LogicalMinimum: LogicalMinimum::<Impl, IMPL_OFFSET>,
            LogicalMaximum: LogicalMaximum::<Impl, IMPL_OFFSET>,
            PhysicalMinimum: PhysicalMinimum::<Impl, IMPL_OFFSET>,
            PhysicalMaximum: PhysicalMaximum::<Impl, IMPL_OFFSET>,
            UnitExponent: UnitExponent::<Impl, IMPL_OFFSET>,
            Unit: Unit::<Impl, IMPL_OFFSET>,
            IsAbsolute: IsAbsolute::<Impl, IMPL_OFFSET>,
            HasNull: HasNull::<Impl, IMPL_OFFSET>,
            ParentCollections: ParentCollections::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidNumericControlDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHidOutputReport_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u16>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetData(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetBooleanControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl>;
    fn GetBooleanControlByDescription(&mut self, controldescription: &::core::option::Option<HidBooleanControlDescription>) -> ::windows::core::Result<HidBooleanControl>;
    fn GetNumericControl(&mut self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl>;
    fn GetNumericControlByDescription(&mut self, controldescription: &::core::option::Option<HidNumericControlDescription>) -> ::windows::core::Result<HidNumericControl>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.IHidOutputReport";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHidOutputReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidOutputReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidOutputReport_Vtbl {
        unsafe extern "system" fn Id<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Data<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBooleanControl<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBooleanControlByDescription<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControl<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericControlByDescription<Impl: IHidOutputReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidOutputReport, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetBooleanControl: GetBooleanControl::<Impl, IMPL_OFFSET>,
            GetBooleanControlByDescription: GetBooleanControlByDescription::<Impl, IMPL_OFFSET>,
            GetNumericControl: GetNumericControl::<Impl, IMPL_OFFSET>,
            GetNumericControlByDescription: GetNumericControlByDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidOutputReport as ::windows::core::Interface>::IID
    }
}
