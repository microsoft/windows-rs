#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoCaptureSettings_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn SetMode(&mut self, value: AdvancedPhotoMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedPhotoCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedPhotoCaptureSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedPhotoCaptureSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedPhotoCaptureSettings_Vtbl {
        unsafe extern "system" fn Mode<Impl: IAdvancedPhotoCaptureSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IAdvancedPhotoCaptureSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedPhotoCaptureSettings, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedPhotoCaptureSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAdvancedPhotoControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>>;
    fn Mode(&mut self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn Configure(&mut self, settings: &::core::option::Option<AdvancedPhotoCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedPhotoControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAdvancedPhotoControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedPhotoControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedPhotoControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IAdvancedPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IAdvancedPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IAdvancedPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IAdvancedPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <AdvancedPhotoCaptureSettings as ::windows::core::Abi>::Abi as *const <AdvancedPhotoCaptureSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedPhotoControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedPhotoControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController_Impl: Sized {
    fn SetDeviceProperty(&mut self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetDeviceProperty(&mut self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController_Vtbl {
        unsafe extern "system" fn SetDeviceProperty<Impl: IAdvancedVideoCaptureDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceProperty(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeviceProperty<Impl: IAdvancedVideoCaptureDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceProperty(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController, BASE_OFFSET>(),
            SetDeviceProperty: SetDeviceProperty::<Impl, IMPL_OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController10_Impl: Sized {
    fn CameraOcclusionInfo(&mut self) -> ::windows::core::Result<CameraOcclusionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController10 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController10";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController10_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController10_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController10_Vtbl {
        unsafe extern "system" fn CameraOcclusionInfo<Impl: IAdvancedVideoCaptureDeviceController10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraOcclusionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController10, BASE_OFFSET>(),
            CameraOcclusionInfo: CameraOcclusionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController10 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController2_Impl: Sized {
    fn LowLagPhotoSequence(&mut self) -> ::windows::core::Result<LowLagPhotoSequenceControl>;
    fn LowLagPhoto(&mut self) -> ::windows::core::Result<LowLagPhotoControl>;
    fn SceneModeControl(&mut self) -> ::windows::core::Result<SceneModeControl>;
    fn TorchControl(&mut self) -> ::windows::core::Result<TorchControl>;
    fn FlashControl(&mut self) -> ::windows::core::Result<FlashControl>;
    fn WhiteBalanceControl(&mut self) -> ::windows::core::Result<WhiteBalanceControl>;
    fn ExposureControl(&mut self) -> ::windows::core::Result<ExposureControl>;
    fn FocusControl(&mut self) -> ::windows::core::Result<FocusControl>;
    fn ExposureCompensationControl(&mut self) -> ::windows::core::Result<ExposureCompensationControl>;
    fn IsoSpeedControl(&mut self) -> ::windows::core::Result<IsoSpeedControl>;
    fn RegionsOfInterestControl(&mut self) -> ::windows::core::Result<RegionsOfInterestControl>;
    fn PrimaryUse(&mut self) -> ::windows::core::Result<CaptureUse>;
    fn SetPrimaryUse(&mut self, value: CaptureUse) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController2 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController2_Vtbl {
        unsafe extern "system" fn LowLagPhotoSequence<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowLagPhotoSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowLagPhoto<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowLagPhoto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneModeControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SceneModeControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TorchControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TorchControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlashControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlashControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalanceControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhiteBalanceControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposureControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensationControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposureCompensationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeedControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsoSpeedControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegionsOfInterestControl<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegionsOfInterestControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryUse<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CaptureUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUse<Impl: IAdvancedVideoCaptureDeviceController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CaptureUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimaryUse(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController2, BASE_OFFSET>(),
            LowLagPhotoSequence: LowLagPhotoSequence::<Impl, IMPL_OFFSET>,
            LowLagPhoto: LowLagPhoto::<Impl, IMPL_OFFSET>,
            SceneModeControl: SceneModeControl::<Impl, IMPL_OFFSET>,
            TorchControl: TorchControl::<Impl, IMPL_OFFSET>,
            FlashControl: FlashControl::<Impl, IMPL_OFFSET>,
            WhiteBalanceControl: WhiteBalanceControl::<Impl, IMPL_OFFSET>,
            ExposureControl: ExposureControl::<Impl, IMPL_OFFSET>,
            FocusControl: FocusControl::<Impl, IMPL_OFFSET>,
            ExposureCompensationControl: ExposureCompensationControl::<Impl, IMPL_OFFSET>,
            IsoSpeedControl: IsoSpeedControl::<Impl, IMPL_OFFSET>,
            RegionsOfInterestControl: RegionsOfInterestControl::<Impl, IMPL_OFFSET>,
            PrimaryUse: PrimaryUse::<Impl, IMPL_OFFSET>,
            SetPrimaryUse: SetPrimaryUse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Devices_Core", feature = "implement_exclusive"))]
pub trait IAdvancedVideoCaptureDeviceController3_Impl: Sized {
    fn VariablePhotoSequenceController(&mut self) -> ::windows::core::Result<Core::VariablePhotoSequenceController>;
    fn PhotoConfirmationControl(&mut self) -> ::windows::core::Result<PhotoConfirmationControl>;
    fn ZoomControl(&mut self) -> ::windows::core::Result<ZoomControl>;
}
#[cfg(all(feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController3 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController3";
}
#[cfg(all(feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl IAdvancedVideoCaptureDeviceController3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController3_Vtbl {
        unsafe extern "system" fn VariablePhotoSequenceController<Impl: IAdvancedVideoCaptureDeviceController3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariablePhotoSequenceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoConfirmationControl<Impl: IAdvancedVideoCaptureDeviceController3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomControl<Impl: IAdvancedVideoCaptureDeviceController3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController3, BASE_OFFSET>(),
            VariablePhotoSequenceController: VariablePhotoSequenceController::<Impl, IMPL_OFFSET>,
            PhotoConfirmationControl: PhotoConfirmationControl::<Impl, IMPL_OFFSET>,
            ZoomControl: ZoomControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController4_Impl: Sized {
    fn ExposurePriorityVideoControl(&mut self) -> ::windows::core::Result<ExposurePriorityVideoControl>;
    fn DesiredOptimization(&mut self) -> ::windows::core::Result<MediaCaptureOptimization>;
    fn SetDesiredOptimization(&mut self, value: MediaCaptureOptimization) -> ::windows::core::Result<()>;
    fn HdrVideoControl(&mut self) -> ::windows::core::Result<HdrVideoControl>;
    fn OpticalImageStabilizationControl(&mut self) -> ::windows::core::Result<OpticalImageStabilizationControl>;
    fn AdvancedPhotoControl(&mut self) -> ::windows::core::Result<AdvancedPhotoControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController4 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController4";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController4_Vtbl {
        unsafe extern "system" fn ExposurePriorityVideoControl<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposurePriorityVideoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredOptimization<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredOptimization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredOptimization<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCaptureOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredOptimization(value).into()
        }
        unsafe extern "system" fn HdrVideoControl<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HdrVideoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalImageStabilizationControl<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpticalImageStabilizationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvancedPhotoControl<Impl: IAdvancedVideoCaptureDeviceController4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvancedPhotoControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController4, BASE_OFFSET>(),
            ExposurePriorityVideoControl: ExposurePriorityVideoControl::<Impl, IMPL_OFFSET>,
            DesiredOptimization: DesiredOptimization::<Impl, IMPL_OFFSET>,
            SetDesiredOptimization: SetDesiredOptimization::<Impl, IMPL_OFFSET>,
            HdrVideoControl: HdrVideoControl::<Impl, IMPL_OFFSET>,
            OpticalImageStabilizationControl: OpticalImageStabilizationControl::<Impl, IMPL_OFFSET>,
            AdvancedPhotoControl: AdvancedPhotoControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdvancedVideoCaptureDeviceController5_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDevicePropertyById(&mut self, propertyid: &::windows::core::HSTRING, maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyById(&mut self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
    fn GetDevicePropertyByExtendedId(&mut self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyByExtendedId(&mut self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController5 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdvancedVideoCaptureDeviceController5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController5_Vtbl {
        unsafe extern "system" fn Id<Impl: IAdvancedVideoCaptureDeviceController5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDevicePropertyById<Impl: IAdvancedVideoCaptureDeviceController5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePropertyById(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&maxpropertyvaluesize as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDevicePropertyById<Impl: IAdvancedVideoCaptureDeviceController5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDevicePropertyById(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePropertyByExtendedId<Impl: IAdvancedVideoCaptureDeviceController5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePropertyByExtendedId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), &*(&maxpropertyvaluesize as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDevicePropertyByExtendedId<Impl: IAdvancedVideoCaptureDeviceController5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDevicePropertyByExtendedId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&propertyvalue), propertyValue_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController5, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            GetDevicePropertyById: GetDevicePropertyById::<Impl, IMPL_OFFSET>,
            SetDevicePropertyById: SetDevicePropertyById::<Impl, IMPL_OFFSET>,
            GetDevicePropertyByExtendedId: GetDevicePropertyByExtendedId::<Impl, IMPL_OFFSET>,
            SetDevicePropertyByExtendedId: SetDevicePropertyByExtendedId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController6_Impl: Sized {
    fn VideoTemporalDenoisingControl(&mut self) -> ::windows::core::Result<VideoTemporalDenoisingControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController6 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController6";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController6_Vtbl {
        unsafe extern "system" fn VideoTemporalDenoisingControl<Impl: IAdvancedVideoCaptureDeviceController6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoTemporalDenoisingControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController6, BASE_OFFSET>(),
            VideoTemporalDenoisingControl: VideoTemporalDenoisingControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController7_Impl: Sized {
    fn InfraredTorchControl(&mut self) -> ::windows::core::Result<InfraredTorchControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController7 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController7";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController7_Vtbl {
        unsafe extern "system" fn InfraredTorchControl<Impl: IAdvancedVideoCaptureDeviceController7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfraredTorchControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController7, BASE_OFFSET>(),
            InfraredTorchControl: InfraredTorchControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController8_Impl: Sized {
    fn PanelBasedOptimizationControl(&mut self) -> ::windows::core::Result<PanelBasedOptimizationControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController8 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController8";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController8_Vtbl {
        unsafe extern "system" fn PanelBasedOptimizationControl<Impl: IAdvancedVideoCaptureDeviceController8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PanelBasedOptimizationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController8, BASE_OFFSET>(),
            PanelBasedOptimizationControl: PanelBasedOptimizationControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController9_Impl: Sized {
    fn DigitalWindowControl(&mut self) -> ::windows::core::Result<DigitalWindowControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedVideoCaptureDeviceController9 {
    const NAME: &'static str = "Windows.Media.Devices.IAdvancedVideoCaptureDeviceController9";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedVideoCaptureDeviceController9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedVideoCaptureDeviceController9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedVideoCaptureDeviceController9_Vtbl {
        unsafe extern "system" fn DigitalWindowControl<Impl: IAdvancedVideoCaptureDeviceController9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitalWindowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedVideoCaptureDeviceController9, BASE_OFFSET>(),
            DigitalWindowControl: DigitalWindowControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedVideoCaptureDeviceController9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IAudioDeviceController_Impl: Sized + IMediaDeviceController_Impl {
    fn SetMuted(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Muted(&mut self) -> ::windows::core::Result<bool>;
    fn SetVolumePercent(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn VolumePercent(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IAudioDeviceController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceController_Vtbl {
        unsafe extern "system" fn SetMuted<Impl: IAudioDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuted(value).into()
        }
        unsafe extern "system" fn Muted<Impl: IAudioDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Muted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumePercent<Impl: IAudioDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolumePercent(value).into()
        }
        unsafe extern "system" fn VolumePercent<Impl: IAudioDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumePercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioDeviceController, BASE_OFFSET>(),
            SetMuted: SetMuted::<Impl, IMPL_OFFSET>,
            Muted: Muted::<Impl, IMPL_OFFSET>,
            SetVolumePercent: SetVolumePercent::<Impl, IMPL_OFFSET>,
            VolumePercent: VolumePercent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAudioDeviceModule_Impl: Sized {
    fn ClassId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstanceId(&mut self) -> ::windows::core::Result<u32>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<u32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<u32>;
    fn SendCommandAsync(&mut self, command: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModule";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAudioDeviceModule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModule_Vtbl {
        unsafe extern "system" fn ClassId<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceId<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommandAsync<Impl: IAudioDeviceModule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommandAsync(&*(&command as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioDeviceModule, BASE_OFFSET>(),
            ClassId: ClassId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            SendCommandAsync: SendCommandAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAudioDeviceModuleNotificationEventArgs_Impl: Sized {
    fn Module(&mut self) -> ::windows::core::Result<AudioDeviceModule>;
    fn NotificationData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModuleNotificationEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAudioDeviceModuleNotificationEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModuleNotificationEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModuleNotificationEventArgs_Vtbl {
        unsafe extern "system" fn Module<Impl: IAudioDeviceModuleNotificationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Module() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationData<Impl: IAudioDeviceModuleNotificationEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioDeviceModuleNotificationEventArgs, BASE_OFFSET>(),
            Module: Module::<Impl, IMPL_OFFSET>,
            NotificationData: NotificationData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModuleNotificationEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioDeviceModulesManager_Impl: Sized {
    fn ModuleNotificationReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveModuleNotificationReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindAllById(&mut self, moduleid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
    fn FindAll(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModulesManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioDeviceModulesManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModulesManager_Vtbl {
        unsafe extern "system" fn ModuleNotificationReceived<Impl: IAudioDeviceModulesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleNotificationReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveModuleNotificationReceived<Impl: IAudioDeviceModulesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveModuleNotificationReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindAllById<Impl: IAudioDeviceModulesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllById(&*(&moduleid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Impl: IAudioDeviceModulesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioDeviceModulesManager, BASE_OFFSET>(),
            ModuleNotificationReceived: ModuleNotificationReceived::<Impl, IMPL_OFFSET>,
            RemoveModuleNotificationReceived: RemoveModuleNotificationReceived::<Impl, IMPL_OFFSET>,
            FindAllById: FindAllById::<Impl, IMPL_OFFSET>,
            FindAll: FindAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModulesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModulesManagerFactory_Impl: Sized {
    fn Create(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioDeviceModulesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioDeviceModulesManagerFactory {
    const NAME: &'static str = "Windows.Media.Devices.IAudioDeviceModulesManagerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioDeviceModulesManagerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesManagerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModulesManagerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IAudioDeviceModulesManagerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioDeviceModulesManagerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModulesManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICallControl_Impl: Sized {
    fn IndicateNewIncomingCall(&mut self, enableringer: bool, callerid: &::windows::core::HSTRING) -> ::windows::core::Result<u64>;
    fn IndicateNewOutgoingCall(&mut self) -> ::windows::core::Result<u64>;
    fn IndicateActiveCall(&mut self, calltoken: u64) -> ::windows::core::Result<()>;
    fn EndCall(&mut self, calltoken: u64) -> ::windows::core::Result<()>;
    fn HasRinger(&mut self) -> ::windows::core::Result<bool>;
    fn AnswerRequested(&mut self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnswerRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HangUpRequested(&mut self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHangUpRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DialRequested(&mut self, handler: &::core::option::Option<DialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDialRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RedialRequested(&mut self, handler: &::core::option::Option<RedialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRedialRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeypadPressed(&mut self, handler: &::core::option::Option<KeypadPressedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeypadPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioTransferRequested(&mut self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTransferRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICallControl {
    const NAME: &'static str = "Windows.Media.Devices.ICallControl";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICallControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallControl_Vtbl {
        unsafe extern "system" fn IndicateNewIncomingCall<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enableringer: bool, callerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicateNewIncomingCall(enableringer, &*(&callerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicateNewOutgoingCall<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicateNewOutgoingCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicateActiveCall<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IndicateActiveCall(calltoken).into()
        }
        unsafe extern "system" fn EndCall<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCall(calltoken).into()
        }
        unsafe extern "system" fn HasRinger<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasRinger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAnswerRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAnswerRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HangUpRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HangUpRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHangUpRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHangUpRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DialRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialRequested(&*(&handler as *const <DialRequestedEventHandler as ::windows::core::Abi>::Abi as *const <DialRequestedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDialRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDialRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RedialRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedialRequested(&*(&handler as *const <RedialRequestedEventHandler as ::windows::core::Abi>::Abi as *const <RedialRequestedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRedialRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRedialRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeypadPressed<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeypadPressed(&*(&handler as *const <KeypadPressedEventHandler as ::windows::core::Abi>::Abi as *const <KeypadPressedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeypadPressed<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeypadPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioTransferRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioTransferRequested(&*(&handler as *const <CallControlEventHandler as ::windows::core::Abi>::Abi as *const <CallControlEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioTransferRequested<Impl: ICallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioTransferRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICallControl, BASE_OFFSET>(),
            IndicateNewIncomingCall: IndicateNewIncomingCall::<Impl, IMPL_OFFSET>,
            IndicateNewOutgoingCall: IndicateNewOutgoingCall::<Impl, IMPL_OFFSET>,
            IndicateActiveCall: IndicateActiveCall::<Impl, IMPL_OFFSET>,
            EndCall: EndCall::<Impl, IMPL_OFFSET>,
            HasRinger: HasRinger::<Impl, IMPL_OFFSET>,
            AnswerRequested: AnswerRequested::<Impl, IMPL_OFFSET>,
            RemoveAnswerRequested: RemoveAnswerRequested::<Impl, IMPL_OFFSET>,
            HangUpRequested: HangUpRequested::<Impl, IMPL_OFFSET>,
            RemoveHangUpRequested: RemoveHangUpRequested::<Impl, IMPL_OFFSET>,
            DialRequested: DialRequested::<Impl, IMPL_OFFSET>,
            RemoveDialRequested: RemoveDialRequested::<Impl, IMPL_OFFSET>,
            RedialRequested: RedialRequested::<Impl, IMPL_OFFSET>,
            RemoveRedialRequested: RemoveRedialRequested::<Impl, IMPL_OFFSET>,
            KeypadPressed: KeypadPressed::<Impl, IMPL_OFFSET>,
            RemoveKeypadPressed: RemoveKeypadPressed::<Impl, IMPL_OFFSET>,
            AudioTransferRequested: AudioTransferRequested::<Impl, IMPL_OFFSET>,
            RemoveAudioTransferRequested: RemoveAudioTransferRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallControlStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<CallControl>;
    fn FromId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<CallControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallControlStatics {
    const NAME: &'static str = "Windows.Media.Devices.ICallControlStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICallControlStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallControlStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallControlStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ICallControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromId<Impl: ICallControlStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICallControlStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            FromId: FromId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallControlStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICameraOcclusionInfo_Impl: Sized {
    fn GetState(&mut self) -> ::windows::core::Result<CameraOcclusionState>;
    fn IsOcclusionKindSupported(&mut self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICameraOcclusionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraOcclusionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraOcclusionInfo_Vtbl {
        unsafe extern "system" fn GetState<Impl: ICameraOcclusionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOcclusionKindSupported<Impl: ICameraOcclusionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOcclusionKindSupported(occlusionkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: ICameraOcclusionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ICameraOcclusionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraOcclusionInfo, BASE_OFFSET>(),
            GetState: GetState::<Impl, IMPL_OFFSET>,
            IsOcclusionKindSupported: IsOcclusionKindSupported::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraOcclusionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionState_Impl: Sized {
    fn IsOccluded(&mut self) -> ::windows::core::Result<bool>;
    fn IsOcclusionKind(&mut self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionState";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOcclusionState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraOcclusionState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraOcclusionState_Vtbl {
        unsafe extern "system" fn IsOccluded<Impl: ICameraOcclusionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOccluded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOcclusionKind<Impl: ICameraOcclusionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOcclusionKind(occlusionkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraOcclusionState, BASE_OFFSET>(),
            IsOccluded: IsOccluded::<Impl, IMPL_OFFSET>,
            IsOcclusionKind: IsOcclusionKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraOcclusionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionStateChangedEventArgs_Impl: Sized {
    fn State(&mut self) -> ::windows::core::Result<CameraOcclusionState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.ICameraOcclusionStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOcclusionStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraOcclusionStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraOcclusionStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn State<Impl: ICameraOcclusionStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraOcclusionStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraOcclusionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDefaultAudioDeviceChangedEventArgs_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Role(&mut self) -> ::windows::core::Result<AudioDeviceRole>;
}
impl ::windows::core::RuntimeName for IDefaultAudioDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
}
impl IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultAudioDeviceChangedEventArgs_Vtbl {
        unsafe extern "system" fn Id<Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Role<Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDefaultAudioDeviceChangedEventArgs, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Role: Role::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultAudioDeviceChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialRequestedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<()>;
    fn Contact(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDialRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDialRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDialRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Handled().into()
        }
        unsafe extern "system" fn Contact<Impl: IDialRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDialRequestedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowBounds_Impl: Sized {
    fn NormalizedOriginTop(&mut self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginTop(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn NormalizedOriginLeft(&mut self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginLeft(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<f64>;
    fn SetScale(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowBounds";
}
#[cfg(feature = "implement_exclusive")]
impl IDigitalWindowBounds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDigitalWindowBounds_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDigitalWindowBounds_Vtbl {
        unsafe extern "system" fn NormalizedOriginTop<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedOriginTop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedOriginTop<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedOriginTop(value).into()
        }
        unsafe extern "system" fn NormalizedOriginLeft<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedOriginLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedOriginLeft<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedOriginLeft(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IDigitalWindowBounds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDigitalWindowBounds, BASE_OFFSET>(),
            NormalizedOriginTop: NormalizedOriginTop::<Impl, IMPL_OFFSET>,
            SetNormalizedOriginTop: SetNormalizedOriginTop::<Impl, IMPL_OFFSET>,
            NormalizedOriginLeft: NormalizedOriginLeft::<Impl, IMPL_OFFSET>,
            SetNormalizedOriginLeft: SetNormalizedOriginLeft::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDigitalWindowBounds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDigitalWindowCapability_Impl: Sized {
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn MinScaleValue(&mut self) -> ::windows::core::Result<f64>;
    fn MaxScaleValue(&mut self) -> ::windows::core::Result<f64>;
    fn MinScaleValueWithoutUpsampling(&mut self) -> ::windows::core::Result<f64>;
    fn NormalizedFieldOfViewLimit(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowCapability";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDigitalWindowCapability_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDigitalWindowCapability_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDigitalWindowCapability_Vtbl {
        unsafe extern "system" fn Width<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinScaleValue<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinScaleValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScaleValue<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxScaleValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinScaleValueWithoutUpsampling<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinScaleValueWithoutUpsampling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedFieldOfViewLimit<Impl: IDigitalWindowCapability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedFieldOfViewLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDigitalWindowCapability, BASE_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            MinScaleValue: MinScaleValue::<Impl, IMPL_OFFSET>,
            MaxScaleValue: MaxScaleValue::<Impl, IMPL_OFFSET>,
            MinScaleValueWithoutUpsampling: MinScaleValueWithoutUpsampling::<Impl, IMPL_OFFSET>,
            NormalizedFieldOfViewLimit: NormalizedFieldOfViewLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDigitalWindowCapability as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDigitalWindowControl_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<::windows::core::Array<DigitalWindowMode>>;
    fn CurrentMode(&mut self) -> ::windows::core::Result<DigitalWindowMode>;
    fn GetBounds(&mut self) -> ::windows::core::Result<DigitalWindowBounds>;
    fn Configure(&mut self, digitalwindowmode: DigitalWindowMode) -> ::windows::core::Result<()>;
    fn ConfigureWithBounds(&mut self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: &::core::option::Option<DigitalWindowBounds>) -> ::windows::core::Result<()>;
    fn SupportedCapabilities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>>;
    fn GetCapabilityForSize(&mut self, width: i32, height: i32) -> ::windows::core::Result<DigitalWindowCapability>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.IDigitalWindowControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDigitalWindowControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDigitalWindowControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDigitalWindowControl_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CurrentMode<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBounds<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(digitalwindowmode).into()
        }
        unsafe extern "system" fn ConfigureWithBounds<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureWithBounds(digitalwindowmode, &*(&digitalwindowbounds as *const <DigitalWindowBounds as ::windows::core::Abi>::Abi as *const <DigitalWindowBounds as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedCapabilities<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilityForSize<Impl: IDigitalWindowControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilityForSize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDigitalWindowControl, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            CurrentMode: CurrentMode::<Impl, IMPL_OFFSET>,
            GetBounds: GetBounds::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
            ConfigureWithBounds: ConfigureWithBounds::<Impl, IMPL_OFFSET>,
            SupportedCapabilities: SupportedCapabilities::<Impl, IMPL_OFFSET>,
            GetCapabilityForSize: GetCapabilityForSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDigitalWindowControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExposureCompensationControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Min(&mut self) -> ::windows::core::Result<f32>;
    fn Max(&mut self) -> ::windows::core::Result<f32>;
    fn Step(&mut self) -> ::windows::core::Result<f32>;
    fn Value(&mut self) -> ::windows::core::Result<f32>;
    fn SetValueAsync(&mut self, value: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposureCompensationControl";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IExposureCompensationControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExposureCompensationControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExposureCompensationControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: IExposureCompensationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExposureCompensationControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExposureCompensationControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExposureControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Auto(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&mut self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Max(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Step(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetValueAsync(&mut self, shutterduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposureControl";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IExposureControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExposureControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExposureControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Auto<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoAsync<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: IExposureControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutterduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(&*(&shutterduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExposureControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Auto: Auto::<Impl, IMPL_OFFSET>,
            SetAutoAsync: SetAutoAsync::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExposureControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposurePriorityVideoControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.IExposurePriorityVideoControl";
}
#[cfg(feature = "implement_exclusive")]
impl IExposurePriorityVideoControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExposurePriorityVideoControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExposurePriorityVideoControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IExposurePriorityVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IExposurePriorityVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IExposurePriorityVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExposurePriorityVideoControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExposurePriorityVideoControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&mut self) -> ::windows::core::Result<bool>;
    fn RedEyeReductionSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Auto(&mut self) -> ::windows::core::Result<bool>;
    fn SetAuto(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RedEyeReduction(&mut self) -> ::windows::core::Result<bool>;
    fn SetRedEyeReduction(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&mut self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.IFlashControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFlashControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlashControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlashControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupported<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedEyeReductionSupported<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedEyeReductionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Auto<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuto<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuto(value).into()
        }
        unsafe extern "system" fn RedEyeReduction<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedEyeReduction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedEyeReduction<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedEyeReduction(value).into()
        }
        unsafe extern "system" fn PowerPercent<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPowerPercent<Impl: IFlashControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPowerPercent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlashControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            PowerSupported: PowerSupported::<Impl, IMPL_OFFSET>,
            RedEyeReductionSupported: RedEyeReductionSupported::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Auto: Auto::<Impl, IMPL_OFFSET>,
            SetAuto: SetAuto::<Impl, IMPL_OFFSET>,
            RedEyeReduction: RedEyeReduction::<Impl, IMPL_OFFSET>,
            SetRedEyeReduction: SetRedEyeReduction::<Impl, IMPL_OFFSET>,
            PowerPercent: PowerPercent::<Impl, IMPL_OFFSET>,
            SetPowerPercent: SetPowerPercent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlashControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControl2_Impl: Sized {
    fn AssistantLightSupported(&mut self) -> ::windows::core::Result<bool>;
    fn AssistantLightEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAssistantLightEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlashControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IFlashControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IFlashControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlashControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlashControl2_Vtbl {
        unsafe extern "system" fn AssistantLightSupported<Impl: IFlashControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssistantLightSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssistantLightEnabled<Impl: IFlashControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssistantLightEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAssistantLightEnabled<Impl: IFlashControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAssistantLightEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlashControl2, BASE_OFFSET>(),
            AssistantLightSupported: AssistantLightSupported::<Impl, IMPL_OFFSET>,
            AssistantLightEnabled: AssistantLightEnabled::<Impl, IMPL_OFFSET>,
            SetAssistantLightEnabled: SetAssistantLightEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlashControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFocusControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>>;
    fn Preset(&mut self) -> ::windows::core::Result<FocusPreset>;
    fn SetPresetAsync(&mut self, preset: FocusPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetPresetWithCompletionOptionAsync(&mut self, preset: FocusPreset, completebeforefocus: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&mut self) -> ::windows::core::Result<u32>;
    fn Max(&mut self) -> ::windows::core::Result<u32>;
    fn Step(&mut self) -> ::windows::core::Result<u32>;
    fn Value(&mut self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&mut self, focus: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FocusAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.IFocusControl";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFocusControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPresets<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPresets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusPreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preset: FocusPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetWithCompletionOptionAsync<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preset: FocusPreset, completebeforefocus: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresetWithCompletionOptionAsync(preset, completebeforefocus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focus: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(focus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusAsync<Impl: IFocusControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedPresets: SupportedPresets::<Impl, IMPL_OFFSET>,
            Preset: Preset::<Impl, IMPL_OFFSET>,
            SetPresetAsync: SetPresetAsync::<Impl, IMPL_OFFSET>,
            SetPresetWithCompletionOptionAsync: SetPresetWithCompletionOptionAsync::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
            FocusAsync: FocusAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFocusControl2_Impl: Sized {
    fn FocusChangedSupported(&mut self) -> ::windows::core::Result<bool>;
    fn WaitForFocusSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedFocusModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>>;
    fn SupportedFocusDistances(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>>;
    fn SupportedFocusRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>>;
    fn Mode(&mut self) -> ::windows::core::Result<FocusMode>;
    fn FocusState(&mut self) -> ::windows::core::Result<MediaCaptureFocusState>;
    fn UnlockAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn LockAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Configure(&mut self, settings: &::core::option::Option<FocusSettings>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IFocusControl2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFocusControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusControl2_Vtbl {
        unsafe extern "system" fn FocusChangedSupported<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusChangedSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForFocusSupported<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForFocusSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusModes<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFocusModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusDistances<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFocusDistances() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFocusRanges<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFocusRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusState<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureFocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockAsync<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockAsync<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IFocusControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <FocusSettings as ::windows::core::Abi>::Abi as *const <FocusSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusControl2, BASE_OFFSET>(),
            FocusChangedSupported: FocusChangedSupported::<Impl, IMPL_OFFSET>,
            WaitForFocusSupported: WaitForFocusSupported::<Impl, IMPL_OFFSET>,
            SupportedFocusModes: SupportedFocusModes::<Impl, IMPL_OFFSET>,
            SupportedFocusDistances: SupportedFocusDistances::<Impl, IMPL_OFFSET>,
            SupportedFocusRanges: SupportedFocusRanges::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            FocusState: FocusState::<Impl, IMPL_OFFSET>,
            UnlockAsync: UnlockAsync::<Impl, IMPL_OFFSET>,
            LockAsync: LockAsync::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFocusSettings_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<FocusMode>;
    fn SetMode(&mut self, value: FocusMode) -> ::windows::core::Result<()>;
    fn AutoFocusRange(&mut self) -> ::windows::core::Result<AutoFocusRange>;
    fn SetAutoFocusRange(&mut self, value: AutoFocusRange) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Distance(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<ManualFocusDistance>>;
    fn SetDistance(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<ManualFocusDistance>>) -> ::windows::core::Result<()>;
    fn WaitForFocus(&mut self) -> ::windows::core::Result<bool>;
    fn SetWaitForFocus(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DisableDriverFallback(&mut self) -> ::windows::core::Result<bool>;
    fn SetDisableDriverFallback(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.IFocusSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusSettings_Vtbl {
        unsafe extern "system" fn Mode<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FocusMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn AutoFocusRange<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutoFocusRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoFocusRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoFocusRange<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutoFocusRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoFocusRange(value).into()
        }
        unsafe extern "system" fn Value<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Distance<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Distance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDistance<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDistance(&*(&value as *const <super::super::Foundation::IReference<ManualFocusDistance> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<ManualFocusDistance> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForFocus<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitForFocus<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWaitForFocus(value).into()
        }
        unsafe extern "system" fn DisableDriverFallback<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableDriverFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableDriverFallback<Impl: IFocusSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisableDriverFallback(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusSettings, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            AutoFocusRange: AutoFocusRange::<Impl, IMPL_OFFSET>,
            SetAutoFocusRange: SetAutoFocusRange::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Distance: Distance::<Impl, IMPL_OFFSET>,
            SetDistance: SetDistance::<Impl, IMPL_OFFSET>,
            WaitForFocus: WaitForFocus::<Impl, IMPL_OFFSET>,
            SetWaitForFocus: SetWaitForFocus::<Impl, IMPL_OFFSET>,
            DisableDriverFallback: DisableDriverFallback::<Impl, IMPL_OFFSET>,
            SetDisableDriverFallback: SetDisableDriverFallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHdrVideoControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>>;
    fn Mode(&mut self) -> ::windows::core::Result<HdrVideoMode>;
    fn SetMode(&mut self, value: HdrVideoMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.IHdrVideoControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHdrVideoControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdrVideoControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdrVideoControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IHdrVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IHdrVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IHdrVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HdrVideoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IHdrVideoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HdrVideoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdrVideoControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdrVideoControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInfraredTorchControl_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>>;
    fn CurrentMode(&mut self) -> ::windows::core::Result<InfraredTorchMode>;
    fn SetCurrentMode(&mut self, value: InfraredTorchMode) -> ::windows::core::Result<()>;
    fn MinPower(&mut self) -> ::windows::core::Result<i32>;
    fn MaxPower(&mut self) -> ::windows::core::Result<i32>;
    fn PowerStep(&mut self) -> ::windows::core::Result<i32>;
    fn Power(&mut self) -> ::windows::core::Result<i32>;
    fn SetPower(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.IInfraredTorchControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInfraredTorchControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInfraredTorchControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInfraredTorchControl_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMode<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InfraredTorchMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentMode<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InfraredTorchMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentMode(value).into()
        }
        unsafe extern "system" fn MinPower<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPower<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerStep<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Power<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Power() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPower<Impl: IInfraredTorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPower(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInfraredTorchControl, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            CurrentMode: CurrentMode::<Impl, IMPL_OFFSET>,
            SetCurrentMode: SetCurrentMode::<Impl, IMPL_OFFSET>,
            MinPower: MinPower::<Impl, IMPL_OFFSET>,
            MaxPower: MaxPower::<Impl, IMPL_OFFSET>,
            PowerStep: PowerStep::<Impl, IMPL_OFFSET>,
            Power: Power::<Impl, IMPL_OFFSET>,
            SetPower: SetPower::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInfraredTorchControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IIsoSpeedControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>>;
    fn Preset(&mut self) -> ::windows::core::Result<IsoSpeedPreset>;
    fn SetPresetAsync(&mut self, preset: IsoSpeedPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IIsoSpeedControl";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IIsoSpeedControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoSpeedControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsoSpeedControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IIsoSpeedControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPresets<Impl: IIsoSpeedControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPresets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IIsoSpeedControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IsoSpeedPreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IIsoSpeedControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preset: IsoSpeedPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIsoSpeedControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedPresets: SupportedPresets::<Impl, IMPL_OFFSET>,
            Preset: Preset::<Impl, IMPL_OFFSET>,
            SetPresetAsync: SetPresetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsoSpeedControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IIsoSpeedControl2_Impl: Sized {
    fn Min(&mut self) -> ::windows::core::Result<u32>;
    fn Max(&mut self) -> ::windows::core::Result<u32>;
    fn Step(&mut self) -> ::windows::core::Result<u32>;
    fn Value(&mut self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&mut self, isospeed: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Auto(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIsoSpeedControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IIsoSpeedControl2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IIsoSpeedControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoSpeedControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsoSpeedControl2_Vtbl {
        unsafe extern "system" fn Min<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isospeed: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(isospeed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Auto<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoAsync<Impl: IIsoSpeedControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIsoSpeedControl2, BASE_OFFSET>(),
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
            Auto: Auto::<Impl, IMPL_OFFSET>,
            SetAutoAsync: SetAutoAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsoSpeedControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeypadPressedEventArgs_Impl: Sized {
    fn TelephonyKey(&mut self) -> ::windows::core::Result<TelephonyKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IKeypadPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeypadPressedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeypadPressedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeypadPressedEventArgs_Vtbl {
        unsafe extern "system" fn TelephonyKey<Impl: IKeypadPressedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TelephonyKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephonyKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeypadPressedEventArgs, BASE_OFFSET>(),
            TelephonyKey: TelephonyKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeypadPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait ILowLagPhotoControl_Impl: Sized {
    fn GetHighestConcurrentFrameRate(&mut self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&mut self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.ILowLagPhotoControl";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ILowLagPhotoControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagPhotoControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagPhotoControl_Vtbl {
        unsafe extern "system" fn GetHighestConcurrentFrameRate<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHighestConcurrentFrameRate(&*(&captureproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFrameRate<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailEnabled<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbnailEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailEnabled<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailEnabled(value).into()
        }
        unsafe extern "system" fn ThumbnailFormat<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbnailFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailFormat<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailFormat(value).into()
        }
        unsafe extern "system" fn DesiredThumbnailSize<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredThumbnailSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredThumbnailSize<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredThumbnailSize(value).into()
        }
        unsafe extern "system" fn HardwareAcceleratedThumbnailSupported<Impl: ILowLagPhotoControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareAcceleratedThumbnailSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagPhotoControl, BASE_OFFSET>(),
            GetHighestConcurrentFrameRate: GetHighestConcurrentFrameRate::<Impl, IMPL_OFFSET>,
            GetCurrentFrameRate: GetCurrentFrameRate::<Impl, IMPL_OFFSET>,
            ThumbnailEnabled: ThumbnailEnabled::<Impl, IMPL_OFFSET>,
            SetThumbnailEnabled: SetThumbnailEnabled::<Impl, IMPL_OFFSET>,
            ThumbnailFormat: ThumbnailFormat::<Impl, IMPL_OFFSET>,
            SetThumbnailFormat: SetThumbnailFormat::<Impl, IMPL_OFFSET>,
            DesiredThumbnailSize: DesiredThumbnailSize::<Impl, IMPL_OFFSET>,
            SetDesiredThumbnailSize: SetDesiredThumbnailSize::<Impl, IMPL_OFFSET>,
            HardwareAcceleratedThumbnailSupported: HardwareAcceleratedThumbnailSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagPhotoControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait ILowLagPhotoSequenceControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn MaxPastPhotos(&mut self) -> ::windows::core::Result<u32>;
    fn MaxPhotosPerSecond(&mut self) -> ::windows::core::Result<f32>;
    fn PastPhotoLimit(&mut self) -> ::windows::core::Result<u32>;
    fn SetPastPhotoLimit(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn PhotosPerSecondLimit(&mut self) -> ::windows::core::Result<f32>;
    fn SetPhotosPerSecondLimit(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetHighestConcurrentFrameRate(&mut self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&mut self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.ILowLagPhotoSequenceControl";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ILowLagPhotoSequenceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagPhotoSequenceControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagPhotoSequenceControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPastPhotos<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPastPhotos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPhotosPerSecond<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPhotosPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PastPhotoLimit<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PastPhotoLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPastPhotoLimit<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPastPhotoLimit(value).into()
        }
        unsafe extern "system" fn PhotosPerSecondLimit<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotosPerSecondLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotosPerSecondLimit<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhotosPerSecondLimit(value).into()
        }
        unsafe extern "system" fn GetHighestConcurrentFrameRate<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHighestConcurrentFrameRate(&*(&captureproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFrameRate<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailEnabled<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbnailEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailEnabled<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailEnabled(value).into()
        }
        unsafe extern "system" fn ThumbnailFormat<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbnailFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailFormat<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailFormat(value).into()
        }
        unsafe extern "system" fn DesiredThumbnailSize<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredThumbnailSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredThumbnailSize<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredThumbnailSize(value).into()
        }
        unsafe extern "system" fn HardwareAcceleratedThumbnailSupported<Impl: ILowLagPhotoSequenceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareAcceleratedThumbnailSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagPhotoSequenceControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            MaxPastPhotos: MaxPastPhotos::<Impl, IMPL_OFFSET>,
            MaxPhotosPerSecond: MaxPhotosPerSecond::<Impl, IMPL_OFFSET>,
            PastPhotoLimit: PastPhotoLimit::<Impl, IMPL_OFFSET>,
            SetPastPhotoLimit: SetPastPhotoLimit::<Impl, IMPL_OFFSET>,
            PhotosPerSecondLimit: PhotosPerSecondLimit::<Impl, IMPL_OFFSET>,
            SetPhotosPerSecondLimit: SetPhotosPerSecondLimit::<Impl, IMPL_OFFSET>,
            GetHighestConcurrentFrameRate: GetHighestConcurrentFrameRate::<Impl, IMPL_OFFSET>,
            GetCurrentFrameRate: GetCurrentFrameRate::<Impl, IMPL_OFFSET>,
            ThumbnailEnabled: ThumbnailEnabled::<Impl, IMPL_OFFSET>,
            SetThumbnailEnabled: SetThumbnailEnabled::<Impl, IMPL_OFFSET>,
            ThumbnailFormat: ThumbnailFormat::<Impl, IMPL_OFFSET>,
            SetThumbnailFormat: SetThumbnailFormat::<Impl, IMPL_OFFSET>,
            DesiredThumbnailSize: DesiredThumbnailSize::<Impl, IMPL_OFFSET>,
            SetDesiredThumbnailSize: SetDesiredThumbnailSize::<Impl, IMPL_OFFSET>,
            HardwareAcceleratedThumbnailSupported: HardwareAcceleratedThumbnailSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagPhotoSequenceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControl_Impl: Sized {
    fn Capabilities(&mut self) -> ::windows::core::Result<MediaDeviceControlCapabilities>;
    fn TryGetValue(&mut self, value: &mut f64) -> ::windows::core::Result<bool>;
    fn TrySetValue(&mut self, value: f64) -> ::windows::core::Result<bool>;
    fn TryGetAuto(&mut self, value: &mut bool) -> ::windows::core::Result<bool>;
    fn TrySetAuto(&mut self, value: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceControl";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaDeviceControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaDeviceControl_Vtbl {
        unsafe extern "system" fn Capabilities<Impl: IMediaDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetValue<Impl: IMediaDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Impl: IMediaDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetValue(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetAuto<Impl: IMediaDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetAuto(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetAuto<Impl: IMediaDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetAuto(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaDeviceControl, BASE_OFFSET>(),
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            TryGetValue: TryGetValue::<Impl, IMPL_OFFSET>,
            TrySetValue: TrySetValue::<Impl, IMPL_OFFSET>,
            TryGetAuto: TryGetAuto::<Impl, IMPL_OFFSET>,
            TrySetAuto: TrySetAuto::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaDeviceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControlCapabilities_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Min(&mut self) -> ::windows::core::Result<f64>;
    fn Max(&mut self) -> ::windows::core::Result<f64>;
    fn Step(&mut self) -> ::windows::core::Result<f64>;
    fn Default(&mut self) -> ::windows::core::Result<f64>;
    fn AutoModeSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceControlCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaDeviceControlCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaDeviceControlCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaDeviceControlCapabilities_Vtbl {
        unsafe extern "system" fn Supported<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Default<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoModeSupported<Impl: IMediaDeviceControlCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoModeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaDeviceControlCapabilities, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Default: Default::<Impl, IMPL_OFFSET>,
            AutoModeSupported: AutoModeSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaDeviceControlCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
pub trait IMediaDeviceController_Impl: Sized {
    fn GetAvailableMediaStreamProperties(&mut self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(&mut self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(&mut self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IMediaDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl IMediaDeviceController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaDeviceController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaDeviceController_Vtbl {
        unsafe extern "system" fn GetAvailableMediaStreamProperties<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaStreamProperties<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaStreamPropertiesAsync<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMediaStreamPropertiesAsync(mediastreamtype, &*(&mediaencodingproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaDeviceController, BASE_OFFSET>(),
            GetAvailableMediaStreamProperties: GetAvailableMediaStreamProperties::<Impl, IMPL_OFFSET>,
            GetMediaStreamProperties: GetMediaStreamProperties::<Impl, IMPL_OFFSET>,
            SetMediaStreamPropertiesAsync: SetMediaStreamPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaDeviceController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaDeviceStatics_Impl: Sized {
    fn GetAudioCaptureSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetVideoCaptureSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioCaptureId(&mut self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioRenderId(&mut self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultAudioCaptureDeviceChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioCaptureDeviceChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultAudioRenderDeviceChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioRenderDeviceChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaDeviceStatics {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaDeviceStatics_Vtbl {
        unsafe extern "system" fn GetAudioCaptureSelector<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioRenderSelector<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoCaptureSelector<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoCaptureSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioCaptureId<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioCaptureId(role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioRenderId<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioRenderId(role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAudioCaptureDeviceChanged<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultAudioCaptureDeviceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultAudioCaptureDeviceChanged<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefaultAudioCaptureDeviceChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultAudioRenderDeviceChanged<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultAudioRenderDeviceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultAudioRenderDeviceChanged<Impl: IMediaDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefaultAudioRenderDeviceChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaDeviceStatics, BASE_OFFSET>(),
            GetAudioCaptureSelector: GetAudioCaptureSelector::<Impl, IMPL_OFFSET>,
            GetAudioRenderSelector: GetAudioRenderSelector::<Impl, IMPL_OFFSET>,
            GetVideoCaptureSelector: GetVideoCaptureSelector::<Impl, IMPL_OFFSET>,
            GetDefaultAudioCaptureId: GetDefaultAudioCaptureId::<Impl, IMPL_OFFSET>,
            GetDefaultAudioRenderId: GetDefaultAudioRenderId::<Impl, IMPL_OFFSET>,
            DefaultAudioCaptureDeviceChanged: DefaultAudioCaptureDeviceChanged::<Impl, IMPL_OFFSET>,
            RemoveDefaultAudioCaptureDeviceChanged: RemoveDefaultAudioCaptureDeviceChanged::<Impl, IMPL_OFFSET>,
            DefaultAudioRenderDeviceChanged: DefaultAudioRenderDeviceChanged::<Impl, IMPL_OFFSET>,
            RemoveDefaultAudioRenderDeviceChanged: RemoveDefaultAudioRenderDeviceChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IModuleCommandResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SendCommandStatus>;
    fn Result(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.IModuleCommandResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IModuleCommandResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IModuleCommandResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IModuleCommandResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IModuleCommandResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SendCommandStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Result<Impl: IModuleCommandResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IModuleCommandResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IModuleCommandResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOpticalImageStabilizationControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>>;
    fn Mode(&mut self) -> ::windows::core::Result<OpticalImageStabilizationMode>;
    fn SetMode(&mut self, value: OpticalImageStabilizationMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.IOpticalImageStabilizationControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOpticalImageStabilizationControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpticalImageStabilizationControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpticalImageStabilizationControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IOpticalImageStabilizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IOpticalImageStabilizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IOpticalImageStabilizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut OpticalImageStabilizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IOpticalImageStabilizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: OpticalImageStabilizationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOpticalImageStabilizationControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpticalImageStabilizationControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
pub trait IPanelBasedOptimizationControl_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Panel(&mut self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn SetPanel(&mut self, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.IPanelBasedOptimizationControl";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl IPanelBasedOptimizationControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPanelBasedOptimizationControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPanelBasedOptimizationControl_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPanelBasedOptimizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel<Impl: IPanelBasedOptimizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPanel<Impl: IPanelBasedOptimizationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPanel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPanelBasedOptimizationControl, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            Panel: Panel::<Impl, IMPL_OFFSET>,
            SetPanel: SetPanel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPanelBasedOptimizationControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IPhotoConfirmationControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PixelFormat(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaPixelFormat>;
    fn SetPixelFormat(&mut self, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.IPhotoConfirmationControl";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IPhotoConfirmationControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoConfirmationControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoConfirmationControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IPhotoConfirmationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IPhotoConfirmationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IPhotoConfirmationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn PixelFormat<Impl: IPhotoConfirmationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IPhotoConfirmationControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelFormat(format).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoConfirmationControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            PixelFormat: PixelFormat::<Impl, IMPL_OFFSET>,
            SetPixelFormat: SetPixelFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoConfirmationControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRedialRequestedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IRedialRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRedialRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedialRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRedialRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IRedialRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Handled().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRedialRequestedEventArgs, BASE_OFFSET>(), Handled: Handled::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRedialRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRegionOfInterest_Impl: Sized {
    fn AutoFocusEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoFocusEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoWhiteBalanceEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoWhiteBalanceEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoExposureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoExposureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Bounds(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetBounds(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.IRegionOfInterest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRegionOfInterest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegionOfInterest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegionOfInterest_Vtbl {
        unsafe extern "system" fn AutoFocusEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoFocusEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoFocusEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoFocusEnabled(value).into()
        }
        unsafe extern "system" fn AutoWhiteBalanceEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoWhiteBalanceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoWhiteBalanceEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoWhiteBalanceEnabled(value).into()
        }
        unsafe extern "system" fn AutoExposureEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoExposureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoExposureEnabled<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoExposureEnabled(value).into()
        }
        unsafe extern "system" fn Bounds<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IRegionOfInterest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRegionOfInterest, BASE_OFFSET>(),
            AutoFocusEnabled: AutoFocusEnabled::<Impl, IMPL_OFFSET>,
            SetAutoFocusEnabled: SetAutoFocusEnabled::<Impl, IMPL_OFFSET>,
            AutoWhiteBalanceEnabled: AutoWhiteBalanceEnabled::<Impl, IMPL_OFFSET>,
            SetAutoWhiteBalanceEnabled: SetAutoWhiteBalanceEnabled::<Impl, IMPL_OFFSET>,
            AutoExposureEnabled: AutoExposureEnabled::<Impl, IMPL_OFFSET>,
            SetAutoExposureEnabled: SetAutoExposureEnabled::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            SetBounds: SetBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegionOfInterest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionOfInterest2_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<RegionOfInterestType>;
    fn SetType(&mut self, value: RegionOfInterestType) -> ::windows::core::Result<()>;
    fn BoundsNormalized(&mut self) -> ::windows::core::Result<bool>;
    fn SetBoundsNormalized(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Weight(&mut self) -> ::windows::core::Result<u32>;
    fn SetWeight(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRegionOfInterest2 {
    const NAME: &'static str = "Windows.Media.Devices.IRegionOfInterest2";
}
#[cfg(feature = "implement_exclusive")]
impl IRegionOfInterest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegionOfInterest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegionOfInterest2_Vtbl {
        unsafe extern "system" fn Type<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RegionOfInterestType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RegionOfInterestType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn BoundsNormalized<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundsNormalized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundsNormalized<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundsNormalized(value).into()
        }
        unsafe extern "system" fn Weight<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: IRegionOfInterest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeight(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRegionOfInterest2, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            BoundsNormalized: BoundsNormalized::<Impl, IMPL_OFFSET>,
            SetBoundsNormalized: SetBoundsNormalized::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            SetWeight: SetWeight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegionOfInterest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRegionsOfInterestControl_Impl: Sized {
    fn MaxRegions(&mut self) -> ::windows::core::Result<u32>;
    fn SetRegionsAsync(&mut self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRegionsWithLockAsync(&mut self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>, lockvalues: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearRegionsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AutoFocusSupported(&mut self) -> ::windows::core::Result<bool>;
    fn AutoWhiteBalanceSupported(&mut self) -> ::windows::core::Result<bool>;
    fn AutoExposureSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.IRegionsOfInterestControl";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRegionsOfInterestControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegionsOfInterestControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegionsOfInterestControl_Vtbl {
        unsafe extern "system" fn MaxRegions<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegionsAsync<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, regions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRegionsAsync(&*(&regions as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegionsWithLockAsync<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, regions: ::windows::core::RawPtr, lockvalues: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRegionsWithLockAsync(&*(&regions as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RegionOfInterest> as ::windows::core::DefaultType>::DefaultType), lockvalues) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRegionsAsync<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearRegionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoFocusSupported<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoFocusSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoWhiteBalanceSupported<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoWhiteBalanceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoExposureSupported<Impl: IRegionsOfInterestControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoExposureSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRegionsOfInterestControl, BASE_OFFSET>(),
            MaxRegions: MaxRegions::<Impl, IMPL_OFFSET>,
            SetRegionsAsync: SetRegionsAsync::<Impl, IMPL_OFFSET>,
            SetRegionsWithLockAsync: SetRegionsWithLockAsync::<Impl, IMPL_OFFSET>,
            ClearRegionsAsync: ClearRegionsAsync::<Impl, IMPL_OFFSET>,
            AutoFocusSupported: AutoFocusSupported::<Impl, IMPL_OFFSET>,
            AutoWhiteBalanceSupported: AutoWhiteBalanceSupported::<Impl, IMPL_OFFSET>,
            AutoExposureSupported: AutoExposureSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegionsOfInterestControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISceneModeControl_Impl: Sized {
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>>;
    fn Value(&mut self) -> ::windows::core::Result<CaptureSceneMode>;
    fn SetValueAsync(&mut self, scenemode: CaptureSceneMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.ISceneModeControl";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISceneModeControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneModeControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneModeControl_Vtbl {
        unsafe extern "system" fn SupportedModes<Impl: ISceneModeControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISceneModeControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CaptureSceneMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: ISceneModeControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scenemode: CaptureSceneMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(scenemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneModeControl, BASE_OFFSET>(),
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneModeControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITorchControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&mut self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITorchControl {
    const NAME: &'static str = "Windows.Media.Devices.ITorchControl";
}
#[cfg(feature = "implement_exclusive")]
impl ITorchControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITorchControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITorchControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupported<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn PowerPercent<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPowerPercent<Impl: ITorchControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPowerPercent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITorchControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            PowerSupported: PowerSupported::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            PowerPercent: PowerPercent::<Impl, IMPL_OFFSET>,
            SetPowerPercent: SetPowerPercent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITorchControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IVideoDeviceController_Impl: Sized + IMediaDeviceController_Impl {
    fn Brightness(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Contrast(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Hue(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn WhiteBalance(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn BacklightCompensation(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Pan(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Tilt(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Zoom(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Roll(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Exposure(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Focus(&mut self) -> ::windows::core::Result<MediaDeviceControl>;
    fn TrySetPowerlineFrequency(&mut self, value: super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
    fn TryGetPowerlineFrequency(&mut self, value: &mut super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IVideoDeviceController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IVideoDeviceController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDeviceController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDeviceController_Vtbl {
        unsafe extern "system" fn Brightness<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Brightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hue<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalance<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhiteBalance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BacklightCompensation<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BacklightCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pan<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tilt<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tilt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Zoom<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Zoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Roll<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exposure<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exposure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Focus<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Focus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetPowerlineFrequency<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetPowerlineFrequency(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetPowerlineFrequency<Impl: IVideoDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPowerlineFrequency(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoDeviceController, BASE_OFFSET>(),
            Brightness: Brightness::<Impl, IMPL_OFFSET>,
            Contrast: Contrast::<Impl, IMPL_OFFSET>,
            Hue: Hue::<Impl, IMPL_OFFSET>,
            WhiteBalance: WhiteBalance::<Impl, IMPL_OFFSET>,
            BacklightCompensation: BacklightCompensation::<Impl, IMPL_OFFSET>,
            Pan: Pan::<Impl, IMPL_OFFSET>,
            Tilt: Tilt::<Impl, IMPL_OFFSET>,
            Zoom: Zoom::<Impl, IMPL_OFFSET>,
            Roll: Roll::<Impl, IMPL_OFFSET>,
            Exposure: Exposure::<Impl, IMPL_OFFSET>,
            Focus: Focus::<Impl, IMPL_OFFSET>,
            TrySetPowerlineFrequency: TrySetPowerlineFrequency::<Impl, IMPL_OFFSET>,
            TryGetPowerlineFrequency: TryGetPowerlineFrequency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDeviceController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDeviceControllerGetDevicePropertyResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyStatus>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.IVideoDeviceControllerGetDevicePropertyResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDeviceControllerGetDevicePropertyResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDeviceControllerGetDevicePropertyResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDeviceControllerGetDevicePropertyResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IVideoDeviceControllerGetDevicePropertyResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IVideoDeviceControllerGetDevicePropertyResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoDeviceControllerGetDevicePropertyResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDeviceControllerGetDevicePropertyResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoTemporalDenoisingControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>>;
    fn Mode(&mut self) -> ::windows::core::Result<VideoTemporalDenoisingMode>;
    fn SetMode(&mut self, value: VideoTemporalDenoisingMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.IVideoTemporalDenoisingControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoTemporalDenoisingControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTemporalDenoisingControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTemporalDenoisingControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IVideoTemporalDenoisingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModes<Impl: IVideoTemporalDenoisingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IVideoTemporalDenoisingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoTemporalDenoisingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IVideoTemporalDenoisingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VideoTemporalDenoisingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoTemporalDenoisingControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTemporalDenoisingControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWhiteBalanceControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Preset(&mut self) -> ::windows::core::Result<ColorTemperaturePreset>;
    fn SetPresetAsync(&mut self, preset: ColorTemperaturePreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&mut self) -> ::windows::core::Result<u32>;
    fn Max(&mut self) -> ::windows::core::Result<u32>;
    fn Step(&mut self) -> ::windows::core::Result<u32>;
    fn Value(&mut self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&mut self, temperature: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.IWhiteBalanceControl";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWhiteBalanceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWhiteBalanceControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWhiteBalanceControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preset<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ColorTemperaturePreset) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresetAsync<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preset: ColorTemperaturePreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresetAsync(preset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValueAsync<Impl: IWhiteBalanceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temperature: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueAsync(temperature) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWhiteBalanceControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Preset: Preset::<Impl, IMPL_OFFSET>,
            SetPresetAsync: SetPresetAsync::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValueAsync: SetValueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWhiteBalanceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomControl_Impl: Sized {
    fn Supported(&mut self) -> ::windows::core::Result<bool>;
    fn Min(&mut self) -> ::windows::core::Result<f32>;
    fn Max(&mut self) -> ::windows::core::Result<f32>;
    fn Step(&mut self) -> ::windows::core::Result<f32>;
    fn Value(&mut self) -> ::windows::core::Result<f32>;
    fn SetValue(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.IZoomControl";
}
#[cfg(feature = "implement_exclusive")]
impl IZoomControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoomControl_Vtbl {
        unsafe extern "system" fn Supported<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IZoomControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IZoomControl, BASE_OFFSET>(),
            Supported: Supported::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Step: Step::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IZoomControl2_Impl: Sized {
    fn SupportedModes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>>;
    fn Mode(&mut self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn Configure(&mut self, settings: &::core::option::Option<ZoomSettings>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IZoomControl2 {
    const NAME: &'static str = "Windows.Media.Devices.IZoomControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IZoomControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoomControl2_Vtbl {
        unsafe extern "system" fn SupportedModes<Impl: IZoomControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IZoomControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IZoomControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(&*(&settings as *const <ZoomSettings as ::windows::core::Abi>::Abi as *const <ZoomSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IZoomControl2, BASE_OFFSET>(),
            SupportedModes: SupportedModes::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomSettings_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn SetMode(&mut self, value: ZoomTransitionMode) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<f32>;
    fn SetValue(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.IZoomSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IZoomSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoomSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoomSettings_Vtbl {
        unsafe extern "system" fn Mode<Impl: IZoomSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IZoomSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ZoomTransitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Value<Impl: IZoomSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IZoomSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IZoomSettings, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoomSettings as ::windows::core::Interface>::IID
    }
}
