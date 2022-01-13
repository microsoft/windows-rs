#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownCameraIntrinsicsPropertiesStaticsImpl: Sized {
    fn FocalLength(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrincipalPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RadialDistortion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TangentialDistortion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownCameraIntrinsicsPropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownCameraIntrinsicsPropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownCameraIntrinsicsPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownCameraIntrinsicsPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownCameraIntrinsicsPropertiesStaticsVtbl {
        unsafe extern "system" fn FocalLength<Impl: IKnownCameraIntrinsicsPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalPoint<Impl: IKnownCameraIntrinsicsPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrincipalPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadialDistortion<Impl: IKnownCameraIntrinsicsPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadialDistortion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TangentialDistortion<Impl: IKnownCameraIntrinsicsPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TangentialDistortion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownCameraIntrinsicsPropertiesStatics, BASE_OFFSET>(),
            FocalLength: FocalLength::<Impl, IMPL_OFFSET>,
            PrincipalPoint: PrincipalPoint::<Impl, IMPL_OFFSET>,
            RadialDistortion: RadialDistortion::<Impl, IMPL_OFFSET>,
            TangentialDistortion: TangentialDistortion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownCameraIntrinsicsPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionColorFrameSourcePropertiesStaticsImpl: Sized {
    fn Exposure(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoExposureEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExposureCompensation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionColorFrameSourcePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionColorFrameSourcePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionColorFrameSourcePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionColorFrameSourcePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionColorFrameSourcePropertiesStaticsVtbl {
        unsafe extern "system" fn Exposure<Impl: IKnownPerceptionColorFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoExposureEnabled<Impl: IKnownPerceptionColorFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExposureCompensation<Impl: IKnownPerceptionColorFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionColorFrameSourcePropertiesStatics, BASE_OFFSET>(),
            Exposure: Exposure::<Impl, IMPL_OFFSET>,
            AutoExposureEnabled: AutoExposureEnabled::<Impl, IMPL_OFFSET>,
            ExposureCompensation: ExposureCompensation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionColorFrameSourcePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionDepthFrameSourcePropertiesStaticsImpl: Sized {
    fn MinDepth(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxDepth(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionDepthFrameSourcePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionDepthFrameSourcePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionDepthFrameSourcePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionDepthFrameSourcePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionDepthFrameSourcePropertiesStaticsVtbl {
        unsafe extern "system" fn MinDepth<Impl: IKnownPerceptionDepthFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDepth<Impl: IKnownPerceptionDepthFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionDepthFrameSourcePropertiesStatics, BASE_OFFSET>(),
            MinDepth: MinDepth::<Impl, IMPL_OFFSET>,
            MaxDepth: MaxDepth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionDepthFrameSourcePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameSourcePropertiesStaticsImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceIds(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceModelVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnclosureLocation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionFrameSourcePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionFrameSourcePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionFrameSourcePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionFrameSourcePropertiesStaticsVtbl {
        unsafe extern "system" fn Id<Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhysicalDeviceIds<Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameKind<Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceModelVersion<Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceModelVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnclosureLocation<Impl: IKnownPerceptionFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnclosureLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionFrameSourcePropertiesStatics, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            PhysicalDeviceIds: PhysicalDeviceIds::<Impl, IMPL_OFFSET>,
            FrameKind: FrameKind::<Impl, IMPL_OFFSET>,
            DeviceModelVersion: DeviceModelVersion::<Impl, IMPL_OFFSET>,
            EnclosureLocation: EnclosureLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionFrameSourcePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameSourcePropertiesStatics2Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionFrameSourcePropertiesStatics2 {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionFrameSourcePropertiesStatics2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionFrameSourcePropertiesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionFrameSourcePropertiesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionFrameSourcePropertiesStatics2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IKnownPerceptionFrameSourcePropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionFrameSourcePropertiesStatics2, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionFrameSourcePropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl: Sized {
    fn Exposure(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoExposureEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExposureCompensation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActiveIlluminationEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmbientSubtractionEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StructureLightPatternEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InterleavedIlluminationEnabled(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionInfraredFrameSourcePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionInfraredFrameSourcePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionInfraredFrameSourcePropertiesStaticsVtbl {
        unsafe extern "system" fn Exposure<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoExposureEnabled<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExposureCompensation<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveIlluminationEnabled<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveIlluminationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AmbientSubtractionEnabled<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AmbientSubtractionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructureLightPatternEnabled<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StructureLightPatternEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterleavedIlluminationEnabled<Impl: IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterleavedIlluminationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionInfraredFrameSourcePropertiesStatics, BASE_OFFSET>(),
            Exposure: Exposure::<Impl, IMPL_OFFSET>,
            AutoExposureEnabled: AutoExposureEnabled::<Impl, IMPL_OFFSET>,
            ExposureCompensation: ExposureCompensation::<Impl, IMPL_OFFSET>,
            ActiveIlluminationEnabled: ActiveIlluminationEnabled::<Impl, IMPL_OFFSET>,
            AmbientSubtractionEnabled: AmbientSubtractionEnabled::<Impl, IMPL_OFFSET>,
            StructureLightPatternEnabled: StructureLightPatternEnabled::<Impl, IMPL_OFFSET>,
            InterleavedIlluminationEnabled: InterleavedIlluminationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionInfraredFrameSourcePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl: Sized {
    fn VideoProfile(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedVideoProfiles(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AvailableVideoProfiles(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsMirrored(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CameraIntrinsics(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionVideoFrameSourcePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionVideoFrameSourcePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionVideoFrameSourcePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionVideoFrameSourcePropertiesStaticsVtbl {
        unsafe extern "system" fn VideoProfile<Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedVideoProfiles<Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableVideoProfiles<Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMirrored<Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMirrored() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionVideoFrameSourcePropertiesStatics, BASE_OFFSET>(),
            VideoProfile: VideoProfile::<Impl, IMPL_OFFSET>,
            SupportedVideoProfiles: SupportedVideoProfiles::<Impl, IMPL_OFFSET>,
            AvailableVideoProfiles: AvailableVideoProfiles::<Impl, IMPL_OFFSET>,
            IsMirrored: IsMirrored::<Impl, IMPL_OFFSET>,
            CameraIntrinsics: CameraIntrinsics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionVideoFrameSourcePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionVideoProfilePropertiesStaticsImpl: Sized {
    fn BitmapPixelFormat(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BitmapAlphaMode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Height(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameDuration(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownPerceptionVideoProfilePropertiesStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IKnownPerceptionVideoProfilePropertiesStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IKnownPerceptionVideoProfilePropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownPerceptionVideoProfilePropertiesStaticsVtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Height<Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameDuration<Impl: IKnownPerceptionVideoProfilePropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownPerceptionVideoProfilePropertiesStatics, BASE_OFFSET>(),
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            FrameDuration: FrameDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownPerceptionVideoProfilePropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&mut self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrame {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrame";
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameVtbl {
        unsafe extern "system" fn VideoFrame<Impl: IPerceptionColorFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrame, BASE_OFFSET>(), VideoFrame: VideoFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&mut self) -> ::windows::core::Result<PerceptionColorFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameArrivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameArrivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameArrivedEventArgsVtbl {
        unsafe extern "system" fn RelativeTime<Impl: IPerceptionColorFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryOpenFrame<Impl: IPerceptionColorFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryOpenFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameArrivedEventArgs, BASE_OFFSET>(),
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            TryOpenFrame: TryOpenFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<PerceptionColorFrameSource>;
    fn IsPaused(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&mut self) -> ::windows::core::Result<PerceptionColorFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaused<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPaused<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPaused(value).into()
        }
        unsafe extern "system" fn TryReadLatestFrame<Impl: IPerceptionColorFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReadLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameReader, BASE_OFFSET>(),
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            IsPaused: IsPaused::<Impl, IMPL_OFFSET>,
            SetIsPaused: SetIsPaused::<Impl, IMPL_OFFSET>,
            TryReadLatestFrame: TryReadLatestFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceImpl: Sized {
    fn AvailableChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&mut self) -> ::windows::core::Result<bool>;
    fn Active(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlled(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&mut self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&mut self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&mut self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&mut self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&mut self, correlateddepthframesource: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&mut self, targetsourceid: &::windows::core::HSTRING, correlateddepthframesource: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&mut self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&mut self) -> ::windows::core::Result<PerceptionColorFrameReader>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSourceVtbl {
        unsafe extern "system" fn AvailableChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailableChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActiveChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActiveChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertiesChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertiesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProfileChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoProfileChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraIntrinsicsChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsicsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraIntrinsicsChanged<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraIntrinsicsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceKind<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Active<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Active() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlled<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedVideoProfiles<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableVideoProfiles<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProfile<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireControlSession<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireControlSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanControlIndependentlyFrom<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanControlIndependentlyFrom(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCorrelatedWith<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCorrelatedWith(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetTransformTo<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetTransformTo(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCameraIntrinsicsAsync<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlateddepthframesource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCameraIntrinsicsAsync(&*(&correlateddepthframesource as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCoordinateMapperAsync<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetsourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlateddepthframesource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCoordinateMapperAsync(&*(&targetsourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlateddepthframesource as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetVideoProfileAsync<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlsession: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetVideoProfileAsync(&*(&controlsession as *const <PerceptionControlSession as ::windows::core::Abi>::Abi as *const <PerceptionControlSession as ::windows::core::DefaultType>::DefaultType), &*(&profile as *const <PerceptionVideoProfile as ::windows::core::Abi>::Abi as *const <PerceptionVideoProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenReader<Impl: IPerceptionColorFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSource, BASE_OFFSET>(),
            AvailableChanged: AvailableChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailableChanged: RemoveAvailableChanged::<Impl, IMPL_OFFSET>,
            ActiveChanged: ActiveChanged::<Impl, IMPL_OFFSET>,
            RemoveActiveChanged: RemoveActiveChanged::<Impl, IMPL_OFFSET>,
            PropertiesChanged: PropertiesChanged::<Impl, IMPL_OFFSET>,
            RemovePropertiesChanged: RemovePropertiesChanged::<Impl, IMPL_OFFSET>,
            VideoProfileChanged: VideoProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoProfileChanged: RemoveVideoProfileChanged::<Impl, IMPL_OFFSET>,
            CameraIntrinsicsChanged: CameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            RemoveCameraIntrinsicsChanged: RemoveCameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            DeviceKind: DeviceKind::<Impl, IMPL_OFFSET>,
            Available: Available::<Impl, IMPL_OFFSET>,
            Active: Active::<Impl, IMPL_OFFSET>,
            IsControlled: IsControlled::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SupportedVideoProfiles: SupportedVideoProfiles::<Impl, IMPL_OFFSET>,
            AvailableVideoProfiles: AvailableVideoProfiles::<Impl, IMPL_OFFSET>,
            VideoProfile: VideoProfile::<Impl, IMPL_OFFSET>,
            CameraIntrinsics: CameraIntrinsics::<Impl, IMPL_OFFSET>,
            AcquireControlSession: AcquireControlSession::<Impl, IMPL_OFFSET>,
            CanControlIndependentlyFrom: CanControlIndependentlyFrom::<Impl, IMPL_OFFSET>,
            IsCorrelatedWith: IsCorrelatedWith::<Impl, IMPL_OFFSET>,
            TryGetTransformTo: TryGetTransformTo::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCameraIntrinsicsAsync: TryGetDepthCorrelatedCameraIntrinsicsAsync::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCoordinateMapperAsync: TryGetDepthCorrelatedCoordinateMapperAsync::<Impl, IMPL_OFFSET>,
            TrySetVideoProfileAsync: TrySetVideoProfileAsync::<Impl, IMPL_OFFSET>,
            OpenReader: OpenReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSource2Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSource2 {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSource2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSource2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IPerceptionColorFrameSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSource2, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionColorFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSourceAddedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSourceAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSourceAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSourceAddedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionColorFrameSourceAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSourceAddedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSourceAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionColorFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSourceRemovedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSourceRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSourceRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSourceRemovedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionColorFrameSourceRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSourceRemovedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<PerceptionColorFrameSourceWatcher>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionColorFrameSource>>>;
    fn FromIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionColorFrameSource>>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSourceStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSourceStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IPerceptionColorFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IPerceptionColorFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPerceptionColorFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IPerceptionColorFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSourceStatics, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionColorFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionColorFrameSourceWatcher";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionColorFrameSourceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionColorFrameSourceWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionColorFrameSourceWatcherVtbl {
        unsafe extern "system" fn SourceAdded<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceAdded<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceRemoved<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceRemoved<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionColorFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionColorFrameSourceWatcher, BASE_OFFSET>(),
            SourceAdded: SourceAdded::<Impl, IMPL_OFFSET>,
            RemoveSourceAdded: RemoveSourceAdded::<Impl, IMPL_OFFSET>,
            SourceRemoved: SourceRemoved::<Impl, IMPL_OFFSET>,
            RemoveSourceRemoved: RemoveSourceRemoved::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionColorFrameSourceWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlSessionImpl: Sized + IClosableImpl {
    fn ControlLost(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionControlSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlLost(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TrySetPropertyAsync(&mut self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionControlSession {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionControlSession";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionControlSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionControlSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionControlSessionVtbl {
        unsafe extern "system" fn ControlLost<Impl: IPerceptionControlSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionControlSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionControlSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveControlLost<Impl: IPerceptionControlSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveControlLost(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrySetPropertyAsync<Impl: IPerceptionControlSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetPropertyAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionControlSession, BASE_OFFSET>(),
            ControlLost: ControlLost::<Impl, IMPL_OFFSET>,
            RemoveControlLost: RemoveControlLost::<Impl, IMPL_OFFSET>,
            TrySetPropertyAsync: TrySetPropertyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionControlSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthCorrelatedCameraIntrinsicsImpl: Sized {
    fn UnprojectPixelAtCorrelatedDepth(&mut self, pixelcoordinate: &super::super::Foundation::Point, depthframe: &::core::option::Option<PerceptionDepthFrame>) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn UnprojectPixelsAtCorrelatedDepth(&mut self, sourcecoordinates: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UnprojectRegionPixelsAtCorrelatedDepthAsync(&mut self, region: &super::super::Foundation::Rect, depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UnprojectAllPixelsAtCorrelatedDepthAsync(&mut self, depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthCorrelatedCameraIntrinsics {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthCorrelatedCameraIntrinsics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthCorrelatedCameraIntrinsicsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthCorrelatedCameraIntrinsicsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthCorrelatedCameraIntrinsicsVtbl {
        unsafe extern "system" fn UnprojectPixelAtCorrelatedDepth<Impl: IPerceptionDepthCorrelatedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::Foundation::Point, depthframe: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprojectPixelAtCorrelatedDepth(&*(&pixelcoordinate as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprojectPixelsAtCorrelatedDepth<Impl: IPerceptionDepthCorrelatedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnprojectPixelsAtCorrelatedDepth(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sourcecoordinates), sourceCoordinates_array_size as _), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        unsafe extern "system" fn UnprojectRegionPixelsAtCorrelatedDepthAsync<Impl: IPerceptionDepthCorrelatedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: super::super::Foundation::Rect, depthframe: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprojectRegionPixelsAtCorrelatedDepthAsync(&*(&region as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprojectAllPixelsAtCorrelatedDepthAsync<Impl: IPerceptionDepthCorrelatedCameraIntrinsicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depthframe: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnprojectAllPixelsAtCorrelatedDepthAsync(&*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthCorrelatedCameraIntrinsics, BASE_OFFSET>(),
            UnprojectPixelAtCorrelatedDepth: UnprojectPixelAtCorrelatedDepth::<Impl, IMPL_OFFSET>,
            UnprojectPixelsAtCorrelatedDepth: UnprojectPixelsAtCorrelatedDepth::<Impl, IMPL_OFFSET>,
            UnprojectRegionPixelsAtCorrelatedDepthAsync: UnprojectRegionPixelsAtCorrelatedDepthAsync::<Impl, IMPL_OFFSET>,
            UnprojectAllPixelsAtCorrelatedDepthAsync: UnprojectAllPixelsAtCorrelatedDepthAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthCorrelatedCameraIntrinsics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthCorrelatedCoordinateMapperImpl: Sized {
    fn MapPixelToTarget(&mut self, sourcepixelcoordinate: &super::super::Foundation::Point, depthframe: &::core::option::Option<PerceptionDepthFrame>) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MapPixelsToTarget(&mut self, sourcecoordinates: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn MapRegionOfPixelsToTargetAsync(&mut self, region: &super::super::Foundation::Rect, depthframe: &::core::option::Option<PerceptionDepthFrame>, targetcoordinates: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MapAllPixelsToTargetAsync(&mut self, depthframe: &::core::option::Option<PerceptionDepthFrame>, targetcoordinates: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthCorrelatedCoordinateMapper";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthCorrelatedCoordinateMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthCorrelatedCoordinateMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthCorrelatedCoordinateMapperVtbl {
        unsafe extern "system" fn MapPixelToTarget<Impl: IPerceptionDepthCorrelatedCoordinateMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcepixelcoordinate: super::super::Foundation::Point, depthframe: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapPixelToTarget(&*(&sourcepixelcoordinate as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPixelsToTarget<Impl: IPerceptionDepthCorrelatedCoordinateMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MapPixelsToTarget(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sourcecoordinates), sourceCoordinates_array_size as _), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        unsafe extern "system" fn MapRegionOfPixelsToTargetAsync<Impl: IPerceptionDepthCorrelatedCoordinateMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, region: super::super::Foundation::Rect, depthframe: ::windows::core::RawPtr, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapRegionOfPixelsToTargetAsync(&*(&region as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&targetcoordinates), targetCoordinates_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapAllPixelsToTargetAsync<Impl: IPerceptionDepthCorrelatedCoordinateMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depthframe: ::windows::core::RawPtr, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapAllPixelsToTargetAsync(&*(&depthframe as *const <PerceptionDepthFrame as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrame as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&targetcoordinates), targetCoordinates_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthCorrelatedCoordinateMapper, BASE_OFFSET>(),
            MapPixelToTarget: MapPixelToTarget::<Impl, IMPL_OFFSET>,
            MapPixelsToTarget: MapPixelsToTarget::<Impl, IMPL_OFFSET>,
            MapRegionOfPixelsToTargetAsync: MapRegionOfPixelsToTargetAsync::<Impl, IMPL_OFFSET>,
            MapAllPixelsToTargetAsync: MapAllPixelsToTargetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthCorrelatedCoordinateMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&mut self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrame {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrame";
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameVtbl {
        unsafe extern "system" fn VideoFrame<Impl: IPerceptionDepthFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrame, BASE_OFFSET>(), VideoFrame: VideoFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&mut self) -> ::windows::core::Result<PerceptionDepthFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameArrivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameArrivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameArrivedEventArgsVtbl {
        unsafe extern "system" fn RelativeTime<Impl: IPerceptionDepthFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryOpenFrame<Impl: IPerceptionDepthFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryOpenFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameArrivedEventArgs, BASE_OFFSET>(),
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            TryOpenFrame: TryOpenFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
    fn IsPaused(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&mut self) -> ::windows::core::Result<PerceptionDepthFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaused<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPaused<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPaused(value).into()
        }
        unsafe extern "system" fn TryReadLatestFrame<Impl: IPerceptionDepthFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReadLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameReader, BASE_OFFSET>(),
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            IsPaused: IsPaused::<Impl, IMPL_OFFSET>,
            SetIsPaused: SetIsPaused::<Impl, IMPL_OFFSET>,
            TryReadLatestFrame: TryReadLatestFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceImpl: Sized {
    fn AvailableChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&mut self) -> ::windows::core::Result<bool>;
    fn Active(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlled(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&mut self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&mut self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&mut self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&mut self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&mut self, target: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&mut self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&mut self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&mut self) -> ::windows::core::Result<PerceptionDepthFrameReader>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSourceVtbl {
        unsafe extern "system" fn AvailableChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailableChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActiveChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActiveChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertiesChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertiesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProfileChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoProfileChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraIntrinsicsChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsicsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraIntrinsicsChanged<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraIntrinsicsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceKind<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Active<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Active() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlled<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedVideoProfiles<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableVideoProfiles<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProfile<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireControlSession<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireControlSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanControlIndependentlyFrom<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanControlIndependentlyFrom(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCorrelatedWith<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCorrelatedWith(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetTransformTo<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetTransformTo(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCameraIntrinsicsAsync<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCameraIntrinsicsAsync(&*(&target as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCoordinateMapperAsync<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, depthframesourcetomapwith: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCoordinateMapperAsync(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&depthframesourcetomapwith as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetVideoProfileAsync<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlsession: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetVideoProfileAsync(&*(&controlsession as *const <PerceptionControlSession as ::windows::core::Abi>::Abi as *const <PerceptionControlSession as ::windows::core::DefaultType>::DefaultType), &*(&profile as *const <PerceptionVideoProfile as ::windows::core::Abi>::Abi as *const <PerceptionVideoProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenReader<Impl: IPerceptionDepthFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSource, BASE_OFFSET>(),
            AvailableChanged: AvailableChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailableChanged: RemoveAvailableChanged::<Impl, IMPL_OFFSET>,
            ActiveChanged: ActiveChanged::<Impl, IMPL_OFFSET>,
            RemoveActiveChanged: RemoveActiveChanged::<Impl, IMPL_OFFSET>,
            PropertiesChanged: PropertiesChanged::<Impl, IMPL_OFFSET>,
            RemovePropertiesChanged: RemovePropertiesChanged::<Impl, IMPL_OFFSET>,
            VideoProfileChanged: VideoProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoProfileChanged: RemoveVideoProfileChanged::<Impl, IMPL_OFFSET>,
            CameraIntrinsicsChanged: CameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            RemoveCameraIntrinsicsChanged: RemoveCameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            DeviceKind: DeviceKind::<Impl, IMPL_OFFSET>,
            Available: Available::<Impl, IMPL_OFFSET>,
            Active: Active::<Impl, IMPL_OFFSET>,
            IsControlled: IsControlled::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SupportedVideoProfiles: SupportedVideoProfiles::<Impl, IMPL_OFFSET>,
            AvailableVideoProfiles: AvailableVideoProfiles::<Impl, IMPL_OFFSET>,
            VideoProfile: VideoProfile::<Impl, IMPL_OFFSET>,
            CameraIntrinsics: CameraIntrinsics::<Impl, IMPL_OFFSET>,
            AcquireControlSession: AcquireControlSession::<Impl, IMPL_OFFSET>,
            CanControlIndependentlyFrom: CanControlIndependentlyFrom::<Impl, IMPL_OFFSET>,
            IsCorrelatedWith: IsCorrelatedWith::<Impl, IMPL_OFFSET>,
            TryGetTransformTo: TryGetTransformTo::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCameraIntrinsicsAsync: TryGetDepthCorrelatedCameraIntrinsicsAsync::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCoordinateMapperAsync: TryGetDepthCorrelatedCoordinateMapperAsync::<Impl, IMPL_OFFSET>,
            TrySetVideoProfileAsync: TrySetVideoProfileAsync::<Impl, IMPL_OFFSET>,
            OpenReader: OpenReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSource2Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSource2 {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSource2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSource2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IPerceptionDepthFrameSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSource2, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSourceAddedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSourceAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSourceAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSourceAddedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionDepthFrameSourceAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSourceAddedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSourceAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSourceRemovedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSourceRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSourceRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSourceRemovedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionDepthFrameSourceRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSourceRemovedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<PerceptionDepthFrameSourceWatcher>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionDepthFrameSource>>>;
    fn FromIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthFrameSource>>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSourceStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSourceStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IPerceptionDepthFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IPerceptionDepthFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPerceptionDepthFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IPerceptionDepthFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSourceStatics, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionDepthFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionDepthFrameSourceWatcher";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionDepthFrameSourceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionDepthFrameSourceWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionDepthFrameSourceWatcherVtbl {
        unsafe extern "system" fn SourceAdded<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceAdded<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceRemoved<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceRemoved<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionDepthFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionDepthFrameSourceWatcher, BASE_OFFSET>(),
            SourceAdded: SourceAdded::<Impl, IMPL_OFFSET>,
            RemoveSourceAdded: RemoveSourceAdded::<Impl, IMPL_OFFSET>,
            SourceRemoved: SourceRemoved::<Impl, IMPL_OFFSET>,
            RemoveSourceRemoved: RemoveSourceRemoved::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionDepthFrameSourceWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameSourcePropertiesChangedEventArgsImpl: Sized {
    fn CollectionChange(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::CollectionChange>;
    fn Key(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameSourcePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionFrameSourcePropertiesChangedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameSourcePropertiesChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameSourcePropertiesChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameSourcePropertiesChangedEventArgsVtbl {
        unsafe extern "system" fn CollectionChange<Impl: IPerceptionFrameSourcePropertiesChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Collections::CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<Impl: IPerceptionFrameSourcePropertiesChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameSourcePropertiesChangedEventArgs, BASE_OFFSET>(),
            CollectionChange: CollectionChange::<Impl, IMPL_OFFSET>,
            Key: Key::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameSourcePropertiesChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameSourcePropertyChangeResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PerceptionFrameSourcePropertyChangeStatus>;
    fn NewValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionFrameSourcePropertyChangeResult {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionFrameSourcePropertyChangeResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionFrameSourcePropertyChangeResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameSourcePropertyChangeResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameSourcePropertyChangeResultVtbl {
        unsafe extern "system" fn Status<Impl: IPerceptionFrameSourcePropertyChangeResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewValue<Impl: IPerceptionFrameSourcePropertyChangeResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameSourcePropertyChangeResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameSourcePropertyChangeResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&mut self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrame {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrame";
}
#[cfg(all(feature = "Foundation", feature = "Media", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameVtbl {
        unsafe extern "system" fn VideoFrame<Impl: IPerceptionInfraredFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrame, BASE_OFFSET>(), VideoFrame: VideoFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&mut self) -> ::windows::core::Result<PerceptionInfraredFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameArrivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameArrivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameArrivedEventArgsVtbl {
        unsafe extern "system" fn RelativeTime<Impl: IPerceptionInfraredFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryOpenFrame<Impl: IPerceptionInfraredFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryOpenFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameArrivedEventArgs, BASE_OFFSET>(),
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            TryOpenFrame: TryOpenFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
    fn IsPaused(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&mut self) -> ::windows::core::Result<PerceptionInfraredFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaused<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPaused<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPaused(value).into()
        }
        unsafe extern "system" fn TryReadLatestFrame<Impl: IPerceptionInfraredFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryReadLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameReader, BASE_OFFSET>(),
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            IsPaused: IsPaused::<Impl, IMPL_OFFSET>,
            SetIsPaused: SetIsPaused::<Impl, IMPL_OFFSET>,
            TryReadLatestFrame: TryReadLatestFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceImpl: Sized {
    fn AvailableChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&mut self) -> ::windows::core::Result<bool>;
    fn Active(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlled(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&mut self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&mut self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&mut self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&mut self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&mut self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&mut self, target: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&mut self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&mut self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&mut self) -> ::windows::core::Result<PerceptionInfraredFrameReader>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Media_Devices_Core", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSourceVtbl {
        unsafe extern "system" fn AvailableChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailableChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActiveChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActiveChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActiveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertiesChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertiesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProfileChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoProfileChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraIntrinsicsChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsicsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraIntrinsicsChanged<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraIntrinsicsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceKind<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Active<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Active() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlled<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedVideoProfiles<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableVideoProfiles<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableVideoProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProfile<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireControlSession<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireControlSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanControlIndependentlyFrom<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanControlIndependentlyFrom(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCorrelatedWith<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCorrelatedWith(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetTransformTo<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetTransformTo(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCameraIntrinsicsAsync<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCameraIntrinsicsAsync(&*(&target as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetDepthCorrelatedCoordinateMapperAsync<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, depthframesourcetomapwith: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetDepthCorrelatedCoordinateMapperAsync(&*(&targetid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&depthframesourcetomapwith as *const <PerceptionDepthFrameSource as ::windows::core::Abi>::Abi as *const <PerceptionDepthFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetVideoProfileAsync<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlsession: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetVideoProfileAsync(&*(&controlsession as *const <PerceptionControlSession as ::windows::core::Abi>::Abi as *const <PerceptionControlSession as ::windows::core::DefaultType>::DefaultType), &*(&profile as *const <PerceptionVideoProfile as ::windows::core::Abi>::Abi as *const <PerceptionVideoProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenReader<Impl: IPerceptionInfraredFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSource, BASE_OFFSET>(),
            AvailableChanged: AvailableChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailableChanged: RemoveAvailableChanged::<Impl, IMPL_OFFSET>,
            ActiveChanged: ActiveChanged::<Impl, IMPL_OFFSET>,
            RemoveActiveChanged: RemoveActiveChanged::<Impl, IMPL_OFFSET>,
            PropertiesChanged: PropertiesChanged::<Impl, IMPL_OFFSET>,
            RemovePropertiesChanged: RemovePropertiesChanged::<Impl, IMPL_OFFSET>,
            VideoProfileChanged: VideoProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoProfileChanged: RemoveVideoProfileChanged::<Impl, IMPL_OFFSET>,
            CameraIntrinsicsChanged: CameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            RemoveCameraIntrinsicsChanged: RemoveCameraIntrinsicsChanged::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            DeviceKind: DeviceKind::<Impl, IMPL_OFFSET>,
            Available: Available::<Impl, IMPL_OFFSET>,
            Active: Active::<Impl, IMPL_OFFSET>,
            IsControlled: IsControlled::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SupportedVideoProfiles: SupportedVideoProfiles::<Impl, IMPL_OFFSET>,
            AvailableVideoProfiles: AvailableVideoProfiles::<Impl, IMPL_OFFSET>,
            VideoProfile: VideoProfile::<Impl, IMPL_OFFSET>,
            CameraIntrinsics: CameraIntrinsics::<Impl, IMPL_OFFSET>,
            AcquireControlSession: AcquireControlSession::<Impl, IMPL_OFFSET>,
            CanControlIndependentlyFrom: CanControlIndependentlyFrom::<Impl, IMPL_OFFSET>,
            IsCorrelatedWith: IsCorrelatedWith::<Impl, IMPL_OFFSET>,
            TryGetTransformTo: TryGetTransformTo::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCameraIntrinsicsAsync: TryGetDepthCorrelatedCameraIntrinsicsAsync::<Impl, IMPL_OFFSET>,
            TryGetDepthCorrelatedCoordinateMapperAsync: TryGetDepthCorrelatedCoordinateMapperAsync::<Impl, IMPL_OFFSET>,
            TrySetVideoProfileAsync: TrySetVideoProfileAsync::<Impl, IMPL_OFFSET>,
            OpenReader: OpenReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSource2Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSource2 {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSource2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSource2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IPerceptionInfraredFrameSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSource2, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSourceAddedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSourceAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSourceAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSourceAddedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionInfraredFrameSourceAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSourceAddedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSourceAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&mut self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSourceRemovedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSourceRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSourceRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSourceRemovedEventArgsVtbl {
        unsafe extern "system" fn FrameSource<Impl: IPerceptionInfraredFrameSourceRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSourceRemovedEventArgs, BASE_OFFSET>(),
            FrameSource: FrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<PerceptionInfraredFrameSourceWatcher>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>>;
    fn FromIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionInfraredFrameSource>>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSourceStatics {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSourceStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IPerceptionInfraredFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IPerceptionInfraredFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPerceptionInfraredFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IPerceptionInfraredFrameSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSourceStatics, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionInfraredFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionInfraredFrameSourceWatcher";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionInfraredFrameSourceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionInfraredFrameSourceWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionInfraredFrameSourceWatcherVtbl {
        unsafe extern "system" fn SourceAdded<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceAdded<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceRemoved<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceRemoved<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionInfraredFrameSourceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionInfraredFrameSourceWatcher, BASE_OFFSET>(),
            SourceAdded: SourceAdded::<Impl, IMPL_OFFSET>,
            RemoveSourceAdded: RemoveSourceAdded::<Impl, IMPL_OFFSET>,
            SourceRemoved: SourceRemoved::<Impl, IMPL_OFFSET>,
            RemoveSourceRemoved: RemoveSourceRemoved::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionInfraredFrameSourceWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoProfileImpl: Sized {
    fn BitmapPixelFormat(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn BitmapAlphaMode(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn FrameDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsEqual(&mut self, other: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPerceptionVideoProfile {
    const NAME: &'static str = "Windows.Devices.Perception.IPerceptionVideoProfile";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl IPerceptionVideoProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionVideoProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionVideoProfileVtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Height<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameDuration<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IPerceptionVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, other: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&other as *const <PerceptionVideoProfile as ::windows::core::Abi>::Abi as *const <PerceptionVideoProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionVideoProfile, BASE_OFFSET>(),
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            FrameDuration: FrameDuration::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionVideoProfile as ::windows::core::Interface>::IID
    }
}
