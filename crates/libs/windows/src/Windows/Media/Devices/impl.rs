#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoCaptureSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn SetMode(&self, value: AdvancedPhotoMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedPhotoCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedPhotoCaptureSettingsVtbl {
    pub const fn new<Impl: IAdvancedPhotoCaptureSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedPhotoCaptureSettingsVtbl {
        unsafe extern "system" fn Mode<Impl: IAdvancedPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IAdvancedPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedPhotoCaptureSettings>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>>;
    fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn Configure(&self, settings: &::core::option::Option<AdvancedPhotoCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedPhotoControl";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedPhotoControlVtbl {
    pub const fn new<Impl: IAdvancedPhotoControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedPhotoControlVtbl {
        unsafe extern "system" fn Supported<Impl: IAdvancedPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IAdvancedPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IAdvancedPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IAdvancedPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <AdvancedPhotoCaptureSettings as ::windows::core::Abi>::Abi as *const <AdvancedPhotoCaptureSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedPhotoControl>, base.5, Supported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, Mode::<Impl, OFFSET>, Configure::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceControllerImpl: Sized {
    fn SetDeviceProperty(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetDeviceProperty(&self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceControllerVtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceControllerVtbl {
        unsafe extern "system" fn SetDeviceProperty<Impl: IAdvancedVideoCaptureDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeviceProperty(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeviceProperty<Impl: IAdvancedVideoCaptureDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceProperty(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController>, base.5, SetDeviceProperty::<Impl, OFFSET>, GetDeviceProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController10Impl: Sized {
    fn CameraOcclusionInfo(&self) -> ::windows::core::Result<CameraOcclusionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController10 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController10Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController10Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController10Vtbl {
        unsafe extern "system" fn CameraOcclusionInfo<Impl: IAdvancedVideoCaptureDeviceController10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraOcclusionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController10>, base.5, CameraOcclusionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController2Impl: Sized {
    fn LowLagPhotoSequence(&self) -> ::windows::core::Result<LowLagPhotoSequenceControl>;
    fn LowLagPhoto(&self) -> ::windows::core::Result<LowLagPhotoControl>;
    fn SceneModeControl(&self) -> ::windows::core::Result<SceneModeControl>;
    fn TorchControl(&self) -> ::windows::core::Result<TorchControl>;
    fn FlashControl(&self) -> ::windows::core::Result<FlashControl>;
    fn WhiteBalanceControl(&self) -> ::windows::core::Result<WhiteBalanceControl>;
    fn ExposureControl(&self) -> ::windows::core::Result<ExposureControl>;
    fn FocusControl(&self) -> ::windows::core::Result<FocusControl>;
    fn ExposureCompensationControl(&self) -> ::windows::core::Result<ExposureCompensationControl>;
    fn IsoSpeedControl(&self) -> ::windows::core::Result<IsoSpeedControl>;
    fn RegionsOfInterestControl(&self) -> ::windows::core::Result<RegionsOfInterestControl>;
    fn PrimaryUse(&self) -> ::windows::core::Result<CaptureUse>;
    fn SetPrimaryUse(&self, value: CaptureUse) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController2 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController2Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController2Vtbl {
        unsafe extern "system" fn LowLagPhotoSequence<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LowLagPhotoSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowLagPhoto<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LowLagPhoto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneModeControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SceneModeControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TorchControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TorchControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlashControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlashControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalanceControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhiteBalanceControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensationControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureCompensationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeedControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoSpeedControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegionsOfInterestControl<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegionsOfInterestControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryUse<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CaptureUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUse<Impl: IAdvancedVideoCaptureDeviceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CaptureUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrimaryUse(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController2>,
            base.5,
            LowLagPhotoSequence::<Impl, OFFSET>,
            LowLagPhoto::<Impl, OFFSET>,
            SceneModeControl::<Impl, OFFSET>,
            TorchControl::<Impl, OFFSET>,
            FlashControl::<Impl, OFFSET>,
            WhiteBalanceControl::<Impl, OFFSET>,
            ExposureControl::<Impl, OFFSET>,
            FocusControl::<Impl, OFFSET>,
            ExposureCompensationControl::<Impl, OFFSET>,
            IsoSpeedControl::<Impl, OFFSET>,
            RegionsOfInterestControl::<Impl, OFFSET>,
            PrimaryUse::<Impl, OFFSET>,
            SetPrimaryUse::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController3Impl: Sized {
    fn VariablePhotoSequenceController(&self) -> ::windows::core::Result<Core::VariablePhotoSequenceController>;
    fn PhotoConfirmationControl(&self) -> ::windows::core::Result<PhotoConfirmationControl>;
    fn ZoomControl(&self) -> ::windows::core::Result<ZoomControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController3 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController3Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController3Vtbl {
        unsafe extern "system" fn VariablePhotoSequenceController<Impl: IAdvancedVideoCaptureDeviceController3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VariablePhotoSequenceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoConfirmationControl<Impl: IAdvancedVideoCaptureDeviceController3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomControl<Impl: IAdvancedVideoCaptureDeviceController3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController3>, base.5, VariablePhotoSequenceController::<Impl, OFFSET>, PhotoConfirmationControl::<Impl, OFFSET>, ZoomControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController4Impl: Sized {
    fn ExposurePriorityVideoControl(&self) -> ::windows::core::Result<ExposurePriorityVideoControl>;
    fn DesiredOptimization(&self) -> ::windows::core::Result<MediaCaptureOptimization>;
    fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> ::windows::core::Result<()>;
    fn HdrVideoControl(&self) -> ::windows::core::Result<HdrVideoControl>;
    fn OpticalImageStabilizationControl(&self) -> ::windows::core::Result<OpticalImageStabilizationControl>;
    fn AdvancedPhotoControl(&self) -> ::windows::core::Result<AdvancedPhotoControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController4 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController4Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController4Vtbl {
        unsafe extern "system" fn ExposurePriorityVideoControl<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposurePriorityVideoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredOptimization<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredOptimization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredOptimization<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaCaptureOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredOptimization(value).into()
        }
        unsafe extern "system" fn HdrVideoControl<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HdrVideoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalImageStabilizationControl<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpticalImageStabilizationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvancedPhotoControl<Impl: IAdvancedVideoCaptureDeviceController4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvancedPhotoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController4>, base.5, ExposurePriorityVideoControl::<Impl, OFFSET>, DesiredOptimization::<Impl, OFFSET>, SetDesiredOptimization::<Impl, OFFSET>, HdrVideoControl::<Impl, OFFSET>, OpticalImageStabilizationControl::<Impl, OFFSET>, AdvancedPhotoControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController5Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDevicePropertyById(&self, propertyid: &::windows::core::HSTRING, maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyById(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
    fn GetDevicePropertyByExtendedId(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController5 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController5Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController5Vtbl {
        unsafe extern "system" fn Id<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePropertyById<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevicePropertyById(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&maxpropertyvaluesize as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDevicePropertyById<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDevicePropertyById(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePropertyByExtendedId<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevicePropertyByExtendedId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), &*(&maxpropertyvaluesize as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDevicePropertyByExtendedId<Impl: IAdvancedVideoCaptureDeviceController5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDevicePropertyByExtendedId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&propertyvalue), propertyValue_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController5>, base.5, Id::<Impl, OFFSET>, GetDevicePropertyById::<Impl, OFFSET>, SetDevicePropertyById::<Impl, OFFSET>, GetDevicePropertyByExtendedId::<Impl, OFFSET>, SetDevicePropertyByExtendedId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController6Impl: Sized {
    fn VideoTemporalDenoisingControl(&self) -> ::windows::core::Result<VideoTemporalDenoisingControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController6 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController6Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController6Vtbl {
        unsafe extern "system" fn VideoTemporalDenoisingControl<Impl: IAdvancedVideoCaptureDeviceController6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoTemporalDenoisingControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController6>, base.5, VideoTemporalDenoisingControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController7Impl: Sized {
    fn InfraredTorchControl(&self) -> ::windows::core::Result<InfraredTorchControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController7 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController7Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController7Vtbl {
        unsafe extern "system" fn InfraredTorchControl<Impl: IAdvancedVideoCaptureDeviceController7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InfraredTorchControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController7>, base.5, InfraredTorchControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController8Impl: Sized {
    fn PanelBasedOptimizationControl(&self) -> ::windows::core::Result<PanelBasedOptimizationControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController8 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController8Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController8Vtbl {
        unsafe extern "system" fn PanelBasedOptimizationControl<Impl: IAdvancedVideoCaptureDeviceController8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PanelBasedOptimizationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController8>, base.5, PanelBasedOptimizationControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController9Impl: Sized {
    fn DigitalWindowControl(&self) -> ::windows::core::Result<DigitalWindowControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController9 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController9Vtbl {
    pub const fn new<Impl: IAdvancedVideoCaptureDeviceController9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedVideoCaptureDeviceController9Vtbl {
        unsafe extern "system" fn DigitalWindowControl<Impl: IAdvancedVideoCaptureDeviceController9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DigitalWindowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedVideoCaptureDeviceController9>, base.5, DigitalWindowControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceControllerImpl: Sized + IMediaDeviceControllerImpl {
    fn SetMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn Muted(&self) -> ::windows::core::Result<bool>;
    fn SetVolumePercent(&self, value: f32) -> ::windows::core::Result<()>;
    fn VolumePercent(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceController";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceControllerVtbl {
    pub const fn new<Impl: IAudioDeviceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceControllerVtbl {
        unsafe extern "system" fn SetMuted<Impl: IAudioDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMuted(value).into()
        }
        unsafe extern "system" fn Muted<Impl: IAudioDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Muted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumePercent<Impl: IAudioDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVolumePercent(value).into()
        }
        unsafe extern "system" fn VolumePercent<Impl: IAudioDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VolumePercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceController>, base.5, SetMuted::<Impl, OFFSET>, Muted::<Impl, OFFSET>, SetVolumePercent::<Impl, OFFSET>, VolumePercent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModuleImpl: Sized {
    fn ClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstanceId(&self) -> ::windows::core::Result<u32>;
    fn MajorVersion(&self) -> ::windows::core::Result<u32>;
    fn MinorVersion(&self) -> ::windows::core::Result<u32>;
    fn SendCommandAsync(&self, command: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModule";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceModuleVtbl {
    pub const fn new<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceModuleVtbl {
        unsafe extern "system" fn ClassId<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceId<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommandAsync<Impl: IAudioDeviceModuleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendCommandAsync(&*(&command as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceModule>, base.5, ClassId::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, InstanceId::<Impl, OFFSET>, MajorVersion::<Impl, OFFSET>, MinorVersion::<Impl, OFFSET>, SendCommandAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModuleNotificationEventArgsImpl: Sized {
    fn Module(&self) -> ::windows::core::Result<AudioDeviceModule>;
    fn NotificationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceModuleNotificationEventArgsVtbl {
    pub const fn new<Impl: IAudioDeviceModuleNotificationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceModuleNotificationEventArgsVtbl {
        unsafe extern "system" fn Module<Impl: IAudioDeviceModuleNotificationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Module() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationData<Impl: IAudioDeviceModuleNotificationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceModuleNotificationEventArgs>, base.5, Module::<Impl, OFFSET>, NotificationData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModulesManagerImpl: Sized {
    fn ModuleNotificationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveModuleNotificationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindAllById(&self, moduleid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
    fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModulesManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceModulesManagerVtbl {
    pub const fn new<Impl: IAudioDeviceModulesManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceModulesManagerVtbl {
        unsafe extern "system" fn ModuleNotificationReceived<Impl: IAudioDeviceModulesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModuleNotificationReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveModuleNotificationReceived<Impl: IAudioDeviceModulesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveModuleNotificationReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindAllById<Impl: IAudioDeviceModulesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllById(&*(&moduleid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Impl: IAudioDeviceModulesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceModulesManager>, base.5, ModuleNotificationReceived::<Impl, OFFSET>, RemoveModuleNotificationReceived::<Impl, OFFSET>, FindAllById::<Impl, OFFSET>, FindAll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModulesManagerFactoryImpl: Sized {
    fn Create(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioDeviceModulesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceModulesManagerFactory {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModulesManagerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceModulesManagerFactoryVtbl {
    pub const fn new<Impl: IAudioDeviceModulesManagerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceModulesManagerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioDeviceModulesManagerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceModulesManagerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallControlImpl: Sized {
    fn IndicateNewIncomingCall(&self, enableringer: bool, callerid: &::windows::core::HSTRING) -> ::windows::core::Result<u64>;
    fn IndicateNewOutgoingCall(&self) -> ::windows::core::Result<u64>;
    fn IndicateActiveCall(&self, calltoken: u64) -> ::windows::core::Result<()>;
    fn EndCall(&self, calltoken: u64) -> ::windows::core::Result<()>;
    fn HasRinger(&self) -> ::windows::core::Result<bool>;
    fn AnswerRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnswerRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HangUpRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHangUpRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DialRequested(&self, handler: &::core::option::Option<DialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDialRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RedialRequested(&self, handler: &::core::option::Option<RedialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRedialRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeypadPressed(&self, handler: &::core::option::Option<KeypadPressedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeypadPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioTransferRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTransferRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallControl {
    const NAME: &'static str = "Windows.Media.Devices.ICallControl";
}
#[cfg(feature = "implement_exclusive")]
impl ICallControlVtbl {
    pub const fn new<Impl: ICallControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallControlVtbl {
        unsafe extern "system" fn IndicateNewIncomingCall<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enableringer: bool, callerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicateNewIncomingCall(enableringer, &*(&callerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicateNewOutgoingCall<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicateNewOutgoingCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicateActiveCall<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).IndicateActiveCall(calltoken).into()
        }
        unsafe extern "system" fn EndCall<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EndCall(calltoken).into()
        }
        unsafe extern "system" fn HasRinger<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasRinger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnswerRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAnswerRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAnswerRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HangUpRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HangUpRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHangUpRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHangUpRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DialRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DialRequested(&*(&handler as *const <DialRequestedEventHandler as ::windows::core::Abi>::Abi as *const <DialRequestedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDialRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDialRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RedialRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedialRequested(&*(&handler as *const <RedialRequestedEventHandler as ::windows::core::Abi>::Abi as *const <RedialRequestedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRedialRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRedialRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeypadPressed<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeypadPressed(&*(&handler as *const <KeypadPressedEventHandler as ::windows::core::Abi>::Abi as *const <KeypadPressedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeypadPressed<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKeypadPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioTransferRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioTransferRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioTransferRequested<Impl: ICallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioTransferRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICallControl>,
            base.5,
            IndicateNewIncomingCall::<Impl, OFFSET>,
            IndicateNewOutgoingCall::<Impl, OFFSET>,
            IndicateActiveCall::<Impl, OFFSET>,
            EndCall::<Impl, OFFSET>,
            HasRinger::<Impl, OFFSET>,
            AnswerRequested::<Impl, OFFSET>,
            RemoveAnswerRequested::<Impl, OFFSET>,
            HangUpRequested::<Impl, OFFSET>,
            RemoveHangUpRequested::<Impl, OFFSET>,
            DialRequested::<Impl, OFFSET>,
            RemoveDialRequested::<Impl, OFFSET>,
            RedialRequested::<Impl, OFFSET>,
            RemoveRedialRequested::<Impl, OFFSET>,
            KeypadPressed::<Impl, OFFSET>,
            RemoveKeypadPressed::<Impl, OFFSET>,
            AudioTransferRequested::<Impl, OFFSET>,
            RemoveAudioTransferRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallControlStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<CallControl>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<CallControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallControlStatics {
    const NAME: &'static str = "Windows.Media.Devices.ICallControlStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICallControlStaticsVtbl {
    pub const fn new<Impl: ICallControlStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallControlStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ICallControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromId<Impl: ICallControlStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallControlStatics>, base.5, GetDefault::<Impl, OFFSET>, FromId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionInfoImpl: Sized {
    fn GetState(&self) -> ::windows::core::Result<CameraOcclusionState>;
    fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOcclusionInfoVtbl {
    pub const fn new<Impl: ICameraOcclusionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraOcclusionInfoVtbl {
        unsafe extern "system" fn GetState<Impl: ICameraOcclusionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOcclusionKindSupported<Impl: ICameraOcclusionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOcclusionKindSupported(occlusionkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: ICameraOcclusionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ICameraOcclusionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraOcclusionInfo>, base.5, GetState::<Impl, OFFSET>, IsOcclusionKindSupported::<Impl, OFFSET>, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionStateImpl: Sized {
    fn IsOccluded(&self) -> ::windows::core::Result<bool>;
    fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionState";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOcclusionStateVtbl {
    pub const fn new<Impl: ICameraOcclusionStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraOcclusionStateVtbl {
        unsafe extern "system" fn IsOccluded<Impl: ICameraOcclusionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOccluded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOcclusionKind<Impl: ICameraOcclusionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOcclusionKind(occlusionkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraOcclusionState>, base.5, IsOccluded::<Impl, OFFSET>, IsOcclusionKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<CameraOcclusionState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOcclusionStateChangedEventArgsVtbl {
    pub const fn new<Impl: ICameraOcclusionStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraOcclusionStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: ICameraOcclusionStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraOcclusionStateChangedEventArgs>, base.5, State::<Impl, OFFSET>)
    }
}
pub trait IDefaultAudioDeviceChangedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Role(&self) -> ::windows::core::Result<AudioDeviceRole>;
}
impl ::windows::core::RuntimeName for IDefaultAudioDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
}
impl IDefaultAudioDeviceChangedEventArgsVtbl {
    pub const fn new<Impl: IDefaultAudioDeviceChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDefaultAudioDeviceChangedEventArgsVtbl {
        unsafe extern "system" fn Id<Impl: IDefaultAudioDeviceChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Impl: IDefaultAudioDeviceChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDefaultAudioDeviceChangedEventArgs>, base.5, Id::<Impl, OFFSET>, Role::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<()>;
    fn Contact(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDialRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDialRequestedEventArgsVtbl {
    pub const fn new<Impl: IDialRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDialRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDialRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Handled().into()
        }
        unsafe extern "system" fn Contact<Impl: IDialRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDialRequestedEventArgs>, base.5, Handled::<Impl, OFFSET>, Contact::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowBoundsImpl: Sized {
    fn NormalizedOriginTop(&self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginTop(&self, value: f64) -> ::windows::core::Result<()>;
    fn NormalizedOriginLeft(&self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginLeft(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<f64>;
    fn SetScale(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowBounds";
}
#[cfg(feature = "implement_exclusive")]
impl IDigitalWindowBoundsVtbl {
    pub const fn new<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDigitalWindowBoundsVtbl {
        unsafe extern "system" fn NormalizedOriginTop<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedOriginTop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedOriginTop<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalizedOriginTop(value).into()
        }
        unsafe extern "system" fn NormalizedOriginLeft<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedOriginLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedOriginLeft<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalizedOriginLeft(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IDigitalWindowBoundsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScale(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDigitalWindowBounds>, base.5, NormalizedOriginTop::<Impl, OFFSET>, SetNormalizedOriginTop::<Impl, OFFSET>, NormalizedOriginLeft::<Impl, OFFSET>, SetNormalizedOriginLeft::<Impl, OFFSET>, Scale::<Impl, OFFSET>, SetScale::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowCapabilityImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn MinScaleValue(&self) -> ::windows::core::Result<f64>;
    fn MaxScaleValue(&self) -> ::windows::core::Result<f64>;
    fn MinScaleValueWithoutUpsampling(&self) -> ::windows::core::Result<f64>;
    fn NormalizedFieldOfViewLimit(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowCapability";
}
#[cfg(feature = "implement_exclusive")]
impl IDigitalWindowCapabilityVtbl {
    pub const fn new<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDigitalWindowCapabilityVtbl {
        unsafe extern "system" fn Width<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinScaleValue<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinScaleValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScaleValue<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxScaleValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinScaleValueWithoutUpsampling<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinScaleValueWithoutUpsampling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedFieldOfViewLimit<Impl: IDigitalWindowCapabilityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedFieldOfViewLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDigitalWindowCapability>, base.5, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>, MinScaleValue::<Impl, OFFSET>, MaxScaleValue::<Impl, OFFSET>, MinScaleValueWithoutUpsampling::<Impl, OFFSET>, NormalizedFieldOfViewLimit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<::windows::core::Array<DigitalWindowMode>>;
    fn CurrentMode(&self) -> ::windows::core::Result<DigitalWindowMode>;
    fn GetBounds(&self) -> ::windows::core::Result<DigitalWindowBounds>;
    fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> ::windows::core::Result<()>;
    fn ConfigureWithBounds(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: &::core::option::Option<DigitalWindowBounds>) -> ::windows::core::Result<()>;
    fn SupportedCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>>;
    fn GetCapabilityForSize(&self, width: i32, height: i32) -> ::windows::core::Result<DigitalWindowCapability>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowControl";
}
#[cfg(feature = "implement_exclusive")]
impl IDigitalWindowControlVtbl {
    pub const fn new<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDigitalWindowControlVtbl {
        unsafe extern "system" fn IsSupported<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMode<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBounds<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Configure(digitalwindowmode).into()
        }
        unsafe extern "system" fn ConfigureWithBounds<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureWithBounds(digitalwindowmode, &*(&digitalwindowbounds as *const <DigitalWindowBounds as ::windows::core::Abi>::Abi as *const <DigitalWindowBounds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedCapabilities<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilityForSize<Impl: IDigitalWindowControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCapabilityForSize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDigitalWindowControl>, base.5, IsSupported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, CurrentMode::<Impl, OFFSET>, GetBounds::<Impl, OFFSET>, Configure::<Impl, OFFSET>, ConfigureWithBounds::<Impl, OFFSET>, SupportedCapabilities::<Impl, OFFSET>, GetCapabilityForSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposureCompensationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValueAsync(&self, value: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposureCompensationControl";
}
#[cfg(feature = "implement_exclusive")]
impl IExposureCompensationControlVtbl {
    pub const fn new<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExposureCompensationControlVtbl {
        unsafe extern "system" fn Supported<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: IExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExposureCompensationControl>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposureControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Max(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Step(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetValueAsync(&self, shutterduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposureControl";
}
#[cfg(feature = "implement_exclusive")]
impl IExposureControlVtbl {
    pub const fn new<Impl: IExposureControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExposureControlVtbl {
        unsafe extern "system" fn Supported<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Auto<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoAsync<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: IExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shutterduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(&*(&shutterduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExposureControl>, base.5, Supported::<Impl, OFFSET>, Auto::<Impl, OFFSET>, SetAutoAsync::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposurePriorityVideoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposurePriorityVideoControl";
}
#[cfg(feature = "implement_exclusive")]
impl IExposurePriorityVideoControlVtbl {
    pub const fn new<Impl: IExposurePriorityVideoControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExposurePriorityVideoControlVtbl {
        unsafe extern "system" fn Supported<Impl: IExposurePriorityVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IExposurePriorityVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IExposurePriorityVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExposurePriorityVideoControl>, base.5, Supported::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
    fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn RedEyeReduction(&self) -> ::windows::core::Result<bool>;
    fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.IFlashControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFlashControlVtbl {
    pub const fn new<Impl: IFlashControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlashControlVtbl {
        unsafe extern "system" fn Supported<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupported<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedEyeReductionSupported<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedEyeReductionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Auto<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuto<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuto(value).into()
        }
        unsafe extern "system" fn RedEyeReduction<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedEyeReduction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedEyeReduction<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRedEyeReduction(value).into()
        }
        unsafe extern "system" fn PowerPercent<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPowerPercent<Impl: IFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPowerPercent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlashControl>, base.5, Supported::<Impl, OFFSET>, PowerSupported::<Impl, OFFSET>, RedEyeReductionSupported::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, Auto::<Impl, OFFSET>, SetAuto::<Impl, OFFSET>, RedEyeReduction::<Impl, OFFSET>, SetRedEyeReduction::<Impl, OFFSET>, PowerPercent::<Impl, OFFSET>, SetPowerPercent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControl2Impl: Sized {
    fn AssistantLightSupported(&self) -> ::windows::core::Result<bool>;
    fn AssistantLightEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAssistantLightEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlashControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IFlashControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IFlashControl2Vtbl {
    pub const fn new<Impl: IFlashControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlashControl2Vtbl {
        unsafe extern "system" fn AssistantLightSupported<Impl: IFlashControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssistantLightSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssistantLightEnabled<Impl: IFlashControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssistantLightEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAssistantLightEnabled<Impl: IFlashControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAssistantLightEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlashControl2>, base.5, AssistantLightSupported::<Impl, OFFSET>, AssistantLightEnabled::<Impl, OFFSET>, SetAssistantLightEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>>;
    fn Preset(&self) -> ::windows::core::Result<FocusPreset>;
    fn SetPresetAsync(&self, preset: FocusPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, focus: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FocusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.IFocusControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusControlVtbl {
    pub const fn new<Impl: IFocusControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusControlVtbl {
        unsafe extern "system" fn Supported<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPresets<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedPresets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusPreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preset: FocusPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetWithCompletionOptionAsync<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preset: FocusPreset, completebeforefocus: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPresetWithCompletionOptionAsync(preset, completebeforefocus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focus: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(focus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusAsync<Impl: IFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusControl>, base.5, Supported::<Impl, OFFSET>, SupportedPresets::<Impl, OFFSET>, Preset::<Impl, OFFSET>, SetPresetAsync::<Impl, OFFSET>, SetPresetWithCompletionOptionAsync::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>, FocusAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusControl2Impl: Sized {
    fn FocusChangedSupported(&self) -> ::windows::core::Result<bool>;
    fn WaitForFocusSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedFocusModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>>;
    fn SupportedFocusDistances(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>>;
    fn SupportedFocusRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>>;
    fn Mode(&self) -> ::windows::core::Result<FocusMode>;
    fn FocusState(&self) -> ::windows::core::Result<MediaCaptureFocusState>;
    fn UnlockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn LockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Configure(&self, settings: &::core::option::Option<FocusSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IFocusControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusControl2Vtbl {
    pub const fn new<Impl: IFocusControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusControl2Vtbl {
        unsafe extern "system" fn FocusChangedSupported<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusChangedSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForFocusSupported<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForFocusSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusModes<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedFocusModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusDistances<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedFocusDistances() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusRanges<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedFocusRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusState<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureFocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockAsync<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlockAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockAsync<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IFocusControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <FocusSettings as ::windows::core::Abi>::Abi as *const <FocusSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusControl2>, base.5, FocusChangedSupported::<Impl, OFFSET>, WaitForFocusSupported::<Impl, OFFSET>, SupportedFocusModes::<Impl, OFFSET>, SupportedFocusDistances::<Impl, OFFSET>, SupportedFocusRanges::<Impl, OFFSET>, Mode::<Impl, OFFSET>, FocusState::<Impl, OFFSET>, UnlockAsync::<Impl, OFFSET>, LockAsync::<Impl, OFFSET>, Configure::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<FocusMode>;
    fn SetMode(&self, value: FocusMode) -> ::windows::core::Result<()>;
    fn AutoFocusRange(&self) -> ::windows::core::Result<AutoFocusRange>;
    fn SetAutoFocusRange(&self, value: AutoFocusRange) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Distance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<ManualFocusDistance>>;
    fn SetDistance(&self, value: &::core::option::Option<super::super::Foundation::IReference<ManualFocusDistance>>) -> ::windows::core::Result<()>;
    fn WaitForFocus(&self) -> ::windows::core::Result<bool>;
    fn SetWaitForFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisableDriverFallback(&self) -> ::windows::core::Result<bool>;
    fn SetDisableDriverFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.IFocusSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusSettingsVtbl {
    pub const fn new<Impl: IFocusSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusSettingsVtbl {
        unsafe extern "system" fn Mode<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn AutoFocusRange<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutoFocusRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoFocusRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoFocusRange<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AutoFocusRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoFocusRange(value).into()
        }
        unsafe extern "system" fn Value<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Distance<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Distance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDistance<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDistance(&*(&value as *const <super::super::Foundation::IReference<ManualFocusDistance> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<ManualFocusDistance> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForFocus<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitForFocus<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWaitForFocus(value).into()
        }
        unsafe extern "system" fn DisableDriverFallback<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableDriverFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableDriverFallback<Impl: IFocusSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisableDriverFallback(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusSettings>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, AutoFocusRange::<Impl, OFFSET>, SetAutoFocusRange::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Distance::<Impl, OFFSET>, SetDistance::<Impl, OFFSET>, WaitForFocus::<Impl, OFFSET>, SetWaitForFocus::<Impl, OFFSET>, DisableDriverFallback::<Impl, OFFSET>, SetDisableDriverFallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdrVideoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>>;
    fn Mode(&self) -> ::windows::core::Result<HdrVideoMode>;
    fn SetMode(&self, value: HdrVideoMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.IHdrVideoControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHdrVideoControlVtbl {
    pub const fn new<Impl: IHdrVideoControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHdrVideoControlVtbl {
        unsafe extern "system" fn Supported<Impl: IHdrVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IHdrVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IHdrVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HdrVideoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IHdrVideoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HdrVideoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHdrVideoControl>, base.5, Supported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInfraredTorchControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>>;
    fn CurrentMode(&self) -> ::windows::core::Result<InfraredTorchMode>;
    fn SetCurrentMode(&self, value: InfraredTorchMode) -> ::windows::core::Result<()>;
    fn MinPower(&self) -> ::windows::core::Result<i32>;
    fn MaxPower(&self) -> ::windows::core::Result<i32>;
    fn PowerStep(&self) -> ::windows::core::Result<i32>;
    fn Power(&self) -> ::windows::core::Result<i32>;
    fn SetPower(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.IInfraredTorchControl";
}
#[cfg(feature = "implement_exclusive")]
impl IInfraredTorchControlVtbl {
    pub const fn new<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInfraredTorchControlVtbl {
        unsafe extern "system" fn IsSupported<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMode<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InfraredTorchMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentMode<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InfraredTorchMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurrentMode(value).into()
        }
        unsafe extern "system" fn MinPower<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPower<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerStep<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Power<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Power() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPower<Impl: IInfraredTorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPower(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInfraredTorchControl>, base.5, IsSupported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, CurrentMode::<Impl, OFFSET>, SetCurrentMode::<Impl, OFFSET>, MinPower::<Impl, OFFSET>, MaxPower::<Impl, OFFSET>, PowerStep::<Impl, OFFSET>, Power::<Impl, OFFSET>, SetPower::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsoSpeedControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>>;
    fn Preset(&self) -> ::windows::core::Result<IsoSpeedPreset>;
    fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IIsoSpeedControl";
}
#[cfg(feature = "implement_exclusive")]
impl IIsoSpeedControlVtbl {
    pub const fn new<Impl: IIsoSpeedControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsoSpeedControlVtbl {
        unsafe extern "system" fn Supported<Impl: IIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPresets<Impl: IIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedPresets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsoSpeedPreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preset: IsoSpeedPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsoSpeedControl>, base.5, Supported::<Impl, OFFSET>, SupportedPresets::<Impl, OFFSET>, Preset::<Impl, OFFSET>, SetPresetAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsoSpeedControl2Impl: Sized {
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, isospeed: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsoSpeedControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IIsoSpeedControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IIsoSpeedControl2Vtbl {
    pub const fn new<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsoSpeedControl2Vtbl {
        unsafe extern "system" fn Min<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isospeed: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(isospeed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Auto<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoAsync<Impl: IIsoSpeedControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsoSpeedControl2>, base.5, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>, Auto::<Impl, OFFSET>, SetAutoAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeypadPressedEventArgsImpl: Sized {
    fn TelephonyKey(&self) -> ::windows::core::Result<TelephonyKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IKeypadPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeypadPressedEventArgsVtbl {
    pub const fn new<Impl: IKeypadPressedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeypadPressedEventArgsVtbl {
        unsafe extern "system" fn TelephonyKey<Impl: IKeypadPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TelephonyKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TelephonyKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeypadPressedEventArgs>, base.5, TelephonyKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoControlImpl: Sized {
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.ILowLagPhotoControl";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagPhotoControlVtbl {
    pub const fn new<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagPhotoControlVtbl {
        unsafe extern "system" fn GetHighestConcurrentFrameRate<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHighestConcurrentFrameRate(&*(&captureproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFrameRate<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailEnabled<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbnailEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailEnabled<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnailEnabled(value).into()
        }
        unsafe extern "system" fn ThumbnailFormat<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbnailFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailFormat<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnailFormat(value).into()
        }
        unsafe extern "system" fn DesiredThumbnailSize<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredThumbnailSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredThumbnailSize<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredThumbnailSize(value).into()
        }
        unsafe extern "system" fn HardwareAcceleratedThumbnailSupported<Impl: ILowLagPhotoControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HardwareAcceleratedThumbnailSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagPhotoControl>, base.5, GetHighestConcurrentFrameRate::<Impl, OFFSET>, GetCurrentFrameRate::<Impl, OFFSET>, ThumbnailEnabled::<Impl, OFFSET>, SetThumbnailEnabled::<Impl, OFFSET>, ThumbnailFormat::<Impl, OFFSET>, SetThumbnailFormat::<Impl, OFFSET>, DesiredThumbnailSize::<Impl, OFFSET>, SetDesiredThumbnailSize::<Impl, OFFSET>, HardwareAcceleratedThumbnailSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoSequenceControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn MaxPastPhotos(&self) -> ::windows::core::Result<u32>;
    fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32>;
    fn PastPhotoLimit(&self) -> ::windows::core::Result<u32>;
    fn SetPastPhotoLimit(&self, value: u32) -> ::windows::core::Result<()>;
    fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32>;
    fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.ILowLagPhotoSequenceControl";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagPhotoSequenceControlVtbl {
    pub const fn new<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagPhotoSequenceControlVtbl {
        unsafe extern "system" fn Supported<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPastPhotos<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPastPhotos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPhotosPerSecond<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPhotosPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PastPhotoLimit<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PastPhotoLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPastPhotoLimit<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPastPhotoLimit(value).into()
        }
        unsafe extern "system" fn PhotosPerSecondLimit<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotosPerSecondLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotosPerSecondLimit<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPhotosPerSecondLimit(value).into()
        }
        unsafe extern "system" fn GetHighestConcurrentFrameRate<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHighestConcurrentFrameRate(&*(&captureproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFrameRate<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailEnabled<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbnailEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailEnabled<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnailEnabled(value).into()
        }
        unsafe extern "system" fn ThumbnailFormat<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThumbnailFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailFormat<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnailFormat(value).into()
        }
        unsafe extern "system" fn DesiredThumbnailSize<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredThumbnailSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredThumbnailSize<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredThumbnailSize(value).into()
        }
        unsafe extern "system" fn HardwareAcceleratedThumbnailSupported<Impl: ILowLagPhotoSequenceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HardwareAcceleratedThumbnailSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILowLagPhotoSequenceControl>,
            base.5,
            Supported::<Impl, OFFSET>,
            MaxPastPhotos::<Impl, OFFSET>,
            MaxPhotosPerSecond::<Impl, OFFSET>,
            PastPhotoLimit::<Impl, OFFSET>,
            SetPastPhotoLimit::<Impl, OFFSET>,
            PhotosPerSecondLimit::<Impl, OFFSET>,
            SetPhotosPerSecondLimit::<Impl, OFFSET>,
            GetHighestConcurrentFrameRate::<Impl, OFFSET>,
            GetCurrentFrameRate::<Impl, OFFSET>,
            ThumbnailEnabled::<Impl, OFFSET>,
            SetThumbnailEnabled::<Impl, OFFSET>,
            ThumbnailFormat::<Impl, OFFSET>,
            SetThumbnailFormat::<Impl, OFFSET>,
            DesiredThumbnailSize::<Impl, OFFSET>,
            SetDesiredThumbnailSize::<Impl, OFFSET>,
            HardwareAcceleratedThumbnailSupported::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControlImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<MediaDeviceControlCapabilities>;
    fn TryGetValue(&self, value: &mut f64) -> ::windows::core::Result<bool>;
    fn TrySetValue(&self, value: f64) -> ::windows::core::Result<bool>;
    fn TryGetAuto(&self, value: &mut bool) -> ::windows::core::Result<bool>;
    fn TrySetAuto(&self, value: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceControl";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaDeviceControlVtbl {
    pub const fn new<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaDeviceControlVtbl {
        unsafe extern "system" fn Capabilities<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetValue<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetValue(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetAuto<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetAuto(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetAuto<Impl: IMediaDeviceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetAuto(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaDeviceControl>, base.5, Capabilities::<Impl, OFFSET>, TryGetValue::<Impl, OFFSET>, TrySetValue::<Impl, OFFSET>, TryGetAuto::<Impl, OFFSET>, TrySetAuto::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControlCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f64>;
    fn Max(&self) -> ::windows::core::Result<f64>;
    fn Step(&self) -> ::windows::core::Result<f64>;
    fn Default(&self) -> ::windows::core::Result<f64>;
    fn AutoModeSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceControlCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaDeviceControlCapabilitiesVtbl {
    pub const fn new<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaDeviceControlCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Default<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoModeSupported<Impl: IMediaDeviceControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoModeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaDeviceControlCapabilities>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Default::<Impl, OFFSET>, AutoModeSupported::<Impl, OFFSET>)
    }
}
pub trait IMediaDeviceControllerImpl: Sized {
    fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
impl ::windows::core::RuntimeName for IMediaDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceController";
}
impl IMediaDeviceControllerVtbl {
    pub const fn new<Impl: IMediaDeviceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaDeviceControllerVtbl {
        unsafe extern "system" fn GetAvailableMediaStreamProperties<Impl: IMediaDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaStreamProperties<Impl: IMediaDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaStreamPropertiesAsync<Impl: IMediaDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMediaStreamPropertiesAsync(mediastreamtype, &*(&mediaencodingproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaDeviceController>, base.5, GetAvailableMediaStreamProperties::<Impl, OFFSET>, GetMediaStreamProperties::<Impl, OFFSET>, SetMediaStreamPropertiesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceStaticsImpl: Sized {
    fn GetAudioCaptureSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetVideoCaptureSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioCaptureId(&self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioRenderId(&self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultAudioCaptureDeviceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioCaptureDeviceChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultAudioRenderDeviceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioRenderDeviceChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaDeviceStatics {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaDeviceStaticsVtbl {
    pub const fn new<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaDeviceStaticsVtbl {
        unsafe extern "system" fn GetAudioCaptureSelector<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioRenderSelector<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoCaptureSelector<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoCaptureSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioCaptureId<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioCaptureId(role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioRenderId<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioRenderId(role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAudioCaptureDeviceChanged<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultAudioCaptureDeviceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultAudioCaptureDeviceChanged<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDefaultAudioCaptureDeviceChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultAudioRenderDeviceChanged<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultAudioRenderDeviceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultAudioRenderDeviceChanged<Impl: IMediaDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDefaultAudioRenderDeviceChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaDeviceStatics>,
            base.5,
            GetAudioCaptureSelector::<Impl, OFFSET>,
            GetAudioRenderSelector::<Impl, OFFSET>,
            GetVideoCaptureSelector::<Impl, OFFSET>,
            GetDefaultAudioCaptureId::<Impl, OFFSET>,
            GetDefaultAudioRenderId::<Impl, OFFSET>,
            DefaultAudioCaptureDeviceChanged::<Impl, OFFSET>,
            RemoveDefaultAudioCaptureDeviceChanged::<Impl, OFFSET>,
            DefaultAudioRenderDeviceChanged::<Impl, OFFSET>,
            RemoveDefaultAudioRenderDeviceChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IModuleCommandResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SendCommandStatus>;
    fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.IModuleCommandResult";
}
#[cfg(feature = "implement_exclusive")]
impl IModuleCommandResultVtbl {
    pub const fn new<Impl: IModuleCommandResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IModuleCommandResultVtbl {
        unsafe extern "system" fn Status<Impl: IModuleCommandResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SendCommandStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IModuleCommandResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IModuleCommandResult>, base.5, Status::<Impl, OFFSET>, Result::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOpticalImageStabilizationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>>;
    fn Mode(&self) -> ::windows::core::Result<OpticalImageStabilizationMode>;
    fn SetMode(&self, value: OpticalImageStabilizationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.IOpticalImageStabilizationControl";
}
#[cfg(feature = "implement_exclusive")]
impl IOpticalImageStabilizationControlVtbl {
    pub const fn new<Impl: IOpticalImageStabilizationControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpticalImageStabilizationControlVtbl {
        unsafe extern "system" fn Supported<Impl: IOpticalImageStabilizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IOpticalImageStabilizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IOpticalImageStabilizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut OpticalImageStabilizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IOpticalImageStabilizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: OpticalImageStabilizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpticalImageStabilizationControl>, base.5, Supported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanelBasedOptimizationControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn Panel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.IPanelBasedOptimizationControl";
}
#[cfg(feature = "implement_exclusive")]
impl IPanelBasedOptimizationControlVtbl {
    pub const fn new<Impl: IPanelBasedOptimizationControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPanelBasedOptimizationControlVtbl {
        unsafe extern "system" fn IsSupported<Impl: IPanelBasedOptimizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel<Impl: IPanelBasedOptimizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Panel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPanel<Impl: IPanelBasedOptimizationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPanel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPanelBasedOptimizationControl>, base.5, IsSupported::<Impl, OFFSET>, Panel::<Impl, OFFSET>, SetPanel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoConfirmationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaPixelFormat>;
    fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.IPhotoConfirmationControl";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoConfirmationControlVtbl {
    pub const fn new<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhotoConfirmationControlVtbl {
        unsafe extern "system" fn Supported<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn PixelFormat<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IPhotoConfirmationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPixelFormat(format).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhotoConfirmationControl>, base.5, Supported::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, PixelFormat::<Impl, OFFSET>, SetPixelFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRedialRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IRedialRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRedialRequestedEventArgsVtbl {
    pub const fn new<Impl: IRedialRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRedialRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IRedialRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Handled().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRedialRequestedEventArgs>, base.5, Handled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionOfInterestImpl: Sized {
    fn AutoFocusEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoFocusEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoWhiteBalanceEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoExposureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoExposureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetBounds(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.IRegionOfInterest";
}
#[cfg(feature = "implement_exclusive")]
impl IRegionOfInterestVtbl {
    pub const fn new<Impl: IRegionOfInterestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRegionOfInterestVtbl {
        unsafe extern "system" fn AutoFocusEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoFocusEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoFocusEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoFocusEnabled(value).into()
        }
        unsafe extern "system" fn AutoWhiteBalanceEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoWhiteBalanceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoWhiteBalanceEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoWhiteBalanceEnabled(value).into()
        }
        unsafe extern "system" fn AutoExposureEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoExposureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoExposureEnabled<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoExposureEnabled(value).into()
        }
        unsafe extern "system" fn Bounds<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IRegionOfInterestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRegionOfInterest>, base.5, AutoFocusEnabled::<Impl, OFFSET>, SetAutoFocusEnabled::<Impl, OFFSET>, AutoWhiteBalanceEnabled::<Impl, OFFSET>, SetAutoWhiteBalanceEnabled::<Impl, OFFSET>, AutoExposureEnabled::<Impl, OFFSET>, SetAutoExposureEnabled::<Impl, OFFSET>, Bounds::<Impl, OFFSET>, SetBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionOfInterest2Impl: Sized {
    fn Type(&self) -> ::windows::core::Result<RegionOfInterestType>;
    fn SetType(&self, value: RegionOfInterestType) -> ::windows::core::Result<()>;
    fn BoundsNormalized(&self) -> ::windows::core::Result<bool>;
    fn SetBoundsNormalized(&self, value: bool) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<u32>;
    fn SetWeight(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRegionOfInterest2 {
    const NAME: &'static str = "Windows.Media.Devices.IRegionOfInterest2";
}
#[cfg(feature = "implement_exclusive")]
impl IRegionOfInterest2Vtbl {
    pub const fn new<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRegionOfInterest2Vtbl {
        unsafe extern "system" fn Type<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RegionOfInterestType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RegionOfInterestType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn BoundsNormalized<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BoundsNormalized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundsNormalized<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBoundsNormalized(value).into()
        }
        unsafe extern "system" fn Weight<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: IRegionOfInterest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWeight(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRegionOfInterest2>, base.5, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, BoundsNormalized::<Impl, OFFSET>, SetBoundsNormalized::<Impl, OFFSET>, Weight::<Impl, OFFSET>, SetWeight::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionsOfInterestControlImpl: Sized {
    fn MaxRegions(&self) -> ::windows::core::Result<u32>;
    fn SetRegionsAsync(&self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRegionsWithLockAsync(&self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>, lockvalues: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearRegionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AutoFocusSupported(&self) -> ::windows::core::Result<bool>;
    fn AutoWhiteBalanceSupported(&self) -> ::windows::core::Result<bool>;
    fn AutoExposureSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.IRegionsOfInterestControl";
}
#[cfg(feature = "implement_exclusive")]
impl IRegionsOfInterestControlVtbl {
    pub const fn new<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRegionsOfInterestControlVtbl {
        unsafe extern "system" fn MaxRegions<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegionsAsync<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, regions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRegionsAsync(&*(&regions as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegionsWithLockAsync<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, regions: ::windows::core::RawPtr, lockvalues: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRegionsWithLockAsync(&*(&regions as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::DefaultType>::DefaultType), lockvalues) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRegionsAsync<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearRegionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoFocusSupported<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoFocusSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoWhiteBalanceSupported<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoWhiteBalanceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoExposureSupported<Impl: IRegionsOfInterestControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoExposureSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRegionsOfInterestControl>, base.5, MaxRegions::<Impl, OFFSET>, SetRegionsAsync::<Impl, OFFSET>, SetRegionsWithLockAsync::<Impl, OFFSET>, ClearRegionsAsync::<Impl, OFFSET>, AutoFocusSupported::<Impl, OFFSET>, AutoWhiteBalanceSupported::<Impl, OFFSET>, AutoExposureSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneModeControlImpl: Sized {
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>>;
    fn Value(&self) -> ::windows::core::Result<CaptureSceneMode>;
    fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.ISceneModeControl";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneModeControlVtbl {
    pub const fn new<Impl: ISceneModeControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneModeControlVtbl {
        unsafe extern "system" fn SupportedModes<Impl: ISceneModeControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISceneModeControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CaptureSceneMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: ISceneModeControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenemode: CaptureSceneMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(scenemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneModeControl>, base.5, SupportedModes::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITorchControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITorchControl {
    const NAME: &'static str = "Windows.Media.Devices.ITorchControl";
}
#[cfg(feature = "implement_exclusive")]
impl ITorchControlVtbl {
    pub const fn new<Impl: ITorchControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITorchControlVtbl {
        unsafe extern "system" fn Supported<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupported<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn PowerPercent<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPowerPercent<Impl: ITorchControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPowerPercent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITorchControl>, base.5, Supported::<Impl, OFFSET>, PowerSupported::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, PowerPercent::<Impl, OFFSET>, SetPowerPercent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDeviceControllerImpl: Sized + IMediaDeviceControllerImpl {
    fn Brightness(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Contrast(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Hue(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn WhiteBalance(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn BacklightCompensation(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Pan(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Tilt(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Zoom(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Roll(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Exposure(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Focus(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
    fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IVideoDeviceController";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDeviceControllerVtbl {
    pub const fn new<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoDeviceControllerVtbl {
        unsafe extern "system" fn Brightness<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Brightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hue<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalance<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhiteBalance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BacklightCompensation<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BacklightCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pan<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tilt<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Zoom<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Zoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Roll<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exposure<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exposure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Focus<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Focus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetPowerlineFrequency<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetPowerlineFrequency(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPowerlineFrequency<Impl: IVideoDeviceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPowerlineFrequency(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoDeviceController>, base.5, Brightness::<Impl, OFFSET>, Contrast::<Impl, OFFSET>, Hue::<Impl, OFFSET>, WhiteBalance::<Impl, OFFSET>, BacklightCompensation::<Impl, OFFSET>, Pan::<Impl, OFFSET>, Tilt::<Impl, OFFSET>, Zoom::<Impl, OFFSET>, Roll::<Impl, OFFSET>, Exposure::<Impl, OFFSET>, Focus::<Impl, OFFSET>, TrySetPowerlineFrequency::<Impl, OFFSET>, TryGetPowerlineFrequency::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDeviceControllerGetDevicePropertyResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyStatus>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDeviceControllerGetDevicePropertyResultVtbl {
    pub const fn new<Impl: IVideoDeviceControllerGetDevicePropertyResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoDeviceControllerGetDevicePropertyResultVtbl {
        unsafe extern "system" fn Status<Impl: IVideoDeviceControllerGetDevicePropertyResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IVideoDeviceControllerGetDevicePropertyResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoDeviceControllerGetDevicePropertyResult>, base.5, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTemporalDenoisingControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>>;
    fn Mode(&self) -> ::windows::core::Result<VideoTemporalDenoisingMode>;
    fn SetMode(&self, value: VideoTemporalDenoisingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.IVideoTemporalDenoisingControl";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTemporalDenoisingControlVtbl {
    pub const fn new<Impl: IVideoTemporalDenoisingControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTemporalDenoisingControlVtbl {
        unsafe extern "system" fn Supported<Impl: IVideoTemporalDenoisingControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IVideoTemporalDenoisingControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IVideoTemporalDenoisingControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoTemporalDenoisingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IVideoTemporalDenoisingControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VideoTemporalDenoisingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTemporalDenoisingControl>, base.5, Supported::<Impl, OFFSET>, SupportedModes::<Impl, OFFSET>, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWhiteBalanceControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Preset(&self) -> ::windows::core::Result<ColorTemperaturePreset>;
    fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, temperature: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.IWhiteBalanceControl";
}
#[cfg(feature = "implement_exclusive")]
impl IWhiteBalanceControlVtbl {
    pub const fn new<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWhiteBalanceControlVtbl {
        unsafe extern "system" fn Supported<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ColorTemperaturePreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preset: ColorTemperaturePreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAsync<Impl: IWhiteBalanceControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, temperature: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(temperature) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWhiteBalanceControl>, base.5, Supported::<Impl, OFFSET>, Preset::<Impl, OFFSET>, SetPresetAsync::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValueAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValue(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.IZoomControl";
}
#[cfg(feature = "implement_exclusive")]
impl IZoomControlVtbl {
    pub const fn new<Impl: IZoomControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IZoomControlVtbl {
        unsafe extern "system" fn Supported<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IZoomControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IZoomControl>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomControl2Impl: Sized {
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>>;
    fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn Configure(&self, settings: &::core::option::Option<ZoomSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IZoomControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IZoomControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IZoomControl2Vtbl {
    pub const fn new<Impl: IZoomControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IZoomControl2Vtbl {
        unsafe extern "system" fn SupportedModes<Impl: IZoomControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IZoomControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IZoomControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <ZoomSettings as ::windows::core::Abi>::Abi as *const <ZoomSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IZoomControl2>, base.5, SupportedModes::<Impl, OFFSET>, Mode::<Impl, OFFSET>, Configure::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn SetMode(&self, value: ZoomTransitionMode) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValue(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.IZoomSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IZoomSettingsVtbl {
    pub const fn new<Impl: IZoomSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IZoomSettingsVtbl {
        unsafe extern "system" fn Mode<Impl: IZoomSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IZoomSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Value<Impl: IZoomSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IZoomSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IZoomSettings>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
