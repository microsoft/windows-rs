#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialAnchor_Impl: Sized {
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystem(&mut self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystemAdjusted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawCoordinateSystemAdjusted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialAnchor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchor_Vtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawCoordinateSystem<Impl: ISpatialAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawCoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawCoordinateSystemAdjusted<Impl: ISpatialAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawCoordinateSystemAdjusted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRawCoordinateSystemAdjusted<Impl: ISpatialAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRawCoordinateSystemAdjusted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchor, BASE_OFFSET>(),
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
            RawCoordinateSystem: RawCoordinateSystem::<Impl, IMPL_OFFSET>,
            RawCoordinateSystemAdjusted: RawCoordinateSystemAdjusted::<Impl, IMPL_OFFSET>,
            RemoveRawCoordinateSystemAdjusted: RemoveRawCoordinateSystemAdjusted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchor2_Impl: Sized {
    fn RemovedByUser(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchor2 {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchor2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchor2_Vtbl {
        unsafe extern "system" fn RemovedByUser<Impl: ISpatialAnchor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchor2, BASE_OFFSET>(), RemovedByUser: RemovedByUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExportSufficiency_Impl: Sized {
    fn IsMinimallySufficient(&mut self) -> ::windows::core::Result<bool>;
    fn SufficiencyLevel(&mut self) -> ::windows::core::Result<f64>;
    fn RecommendedSufficiencyLevel(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExportSufficiency";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorExportSufficiency_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorExportSufficiency_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorExportSufficiency_Vtbl {
        unsafe extern "system" fn IsMinimallySufficient<Impl: ISpatialAnchorExportSufficiency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMinimallySufficient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SufficiencyLevel<Impl: ISpatialAnchorExportSufficiency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SufficiencyLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedSufficiencyLevel<Impl: ISpatialAnchorExportSufficiency_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendedSufficiencyLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorExportSufficiency, BASE_OFFSET>(),
            IsMinimallySufficient: IsMinimallySufficient::<Impl, IMPL_OFFSET>,
            SufficiencyLevel: SufficiencyLevel::<Impl, IMPL_OFFSET>,
            RecommendedSufficiencyLevel: RecommendedSufficiencyLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorExportSufficiency as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpatialAnchorExporter_Impl: Sized {
    fn GetAnchorExportSufficiencyAsync(&mut self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>;
    fn TryExportAnchorAsync(&mut self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExporter";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpatialAnchorExporter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorExporter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorExporter_Vtbl {
        unsafe extern "system" fn GetAnchorExportSufficiencyAsync<Impl: ISpatialAnchorExporter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnchorExportSufficiencyAsync(&*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), purpose) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryExportAnchorAsync<Impl: ISpatialAnchorExporter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryExportAnchorAsync(&*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), purpose, &*(&stream as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorExporter, BASE_OFFSET>(),
            GetAnchorExportSufficiencyAsync: GetAnchorExportSufficiencyAsync::<Impl, IMPL_OFFSET>,
            TryExportAnchorAsync: TryExportAnchorAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorExporter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialAnchorExporterStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<SpatialAnchorExporter>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorExporterStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExporterStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialAnchorExporterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorExporterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorExporterStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ISpatialAnchorExporterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: ISpatialAnchorExporterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorExporterStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorExporterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialAnchorManagerStatics_Impl: Sized {
    fn RequestStoreAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorManagerStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialAnchorManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorManagerStatics_Vtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: ISpatialAnchorManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorManagerStatics, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Impl: Sized {
    fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
        unsafe extern "system" fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform<Impl: ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorRawCoordinateSystemAdjustedEventArgs, BASE_OFFSET>(),
            OldRawCoordinateSystemToNewRawCoordinateSystemTransform: OldRawCoordinateSystemToNewRawCoordinateSystemTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorRawCoordinateSystemAdjustedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialAnchorStatics_Impl: Sized {
    fn TryCreateRelativeTo(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionRelativeTo(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionAndOrientationRelativeTo(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialAnchor>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorStatics";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialAnchorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorStatics_Vtbl {
        unsafe extern "system" fn TryCreateRelativeTo<Impl: ISpatialAnchorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateRelativeTo(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWithPositionRelativeTo<Impl: ISpatialAnchorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateWithPositionRelativeTo(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWithPositionAndOrientationRelativeTo<Impl: ISpatialAnchorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateWithPositionAndOrientationRelativeTo(
                &*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorStatics, BASE_OFFSET>(),
            TryCreateRelativeTo: TryCreateRelativeTo::<Impl, IMPL_OFFSET>,
            TryCreateWithPositionRelativeTo: TryCreateWithPositionRelativeTo::<Impl, IMPL_OFFSET>,
            TryCreateWithPositionAndOrientationRelativeTo: TryCreateWithPositionAndOrientationRelativeTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISpatialAnchorStore_Impl: Sized {
    fn GetAllSavedAnchors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>;
    fn TrySave(&mut self, id: &::windows::core::HSTRING, anchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<bool>;
    fn Remove(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorStore";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISpatialAnchorStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorStore_Vtbl {
        unsafe extern "system" fn GetAllSavedAnchors<Impl: ISpatialAnchorStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllSavedAnchors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySave<Impl: ISpatialAnchorStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, anchor: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySave(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISpatialAnchorStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISpatialAnchorStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorStore, BASE_OFFSET>(),
            GetAllSavedAnchors: GetAllSavedAnchors::<Impl, IMPL_OFFSET>,
            TrySave: TrySave::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISpatialAnchorTransferManagerStatics_Impl: Sized {
    fn TryImportAnchorsAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>>;
    fn TryExportAnchorsAsync(&mut self, anchors: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>>>, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorTransferManagerStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISpatialAnchorTransferManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAnchorTransferManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAnchorTransferManagerStatics_Vtbl {
        unsafe extern "system" fn TryImportAnchorsAsync<Impl: ISpatialAnchorTransferManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryImportAnchorsAsync(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryExportAnchorsAsync<Impl: ISpatialAnchorTransferManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchors: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryExportAnchorsAsync(
                &*(&anchors as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>> as ::windows::core::DefaultType>::DefaultType),
                &*(&stream as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: ISpatialAnchorTransferManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialAnchorTransferManagerStatics, BASE_OFFSET>(),
            TryImportAnchorsAsync: TryImportAnchorsAsync::<Impl, IMPL_OFFSET>,
            TryExportAnchorsAsync: TryExportAnchorsAsync::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAnchorTransferManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialBoundingVolume_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialBoundingVolume";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialBoundingVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialBoundingVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialBoundingVolume_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialBoundingVolume, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialBoundingVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialBoundingVolumeStatics_Impl: Sized {
    fn FromBox(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromOrientedBox(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingOrientedBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromSphere(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, sphere: &SpatialBoundingSphere) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromFrustum(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, frustum: &SpatialBoundingFrustum) -> ::windows::core::Result<SpatialBoundingVolume>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialBoundingVolumeStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialBoundingVolumeStatics";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialBoundingVolumeStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialBoundingVolumeStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialBoundingVolumeStatics_Vtbl {
        unsafe extern "system" fn FromBox<Impl: ISpatialBoundingVolumeStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromBox(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&r#box as *const <SpatialBoundingBox as ::windows::core::Abi>::Abi as *const <SpatialBoundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromOrientedBox<Impl: ISpatialBoundingVolumeStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingOrientedBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromOrientedBox(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&r#box as *const <SpatialBoundingOrientedBox as ::windows::core::Abi>::Abi as *const <SpatialBoundingOrientedBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromSphere<Impl: ISpatialBoundingVolumeStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, sphere: SpatialBoundingSphere, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromSphere(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&sphere as *const <SpatialBoundingSphere as ::windows::core::Abi>::Abi as *const <SpatialBoundingSphere as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromFrustum<Impl: ISpatialBoundingVolumeStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, frustum: SpatialBoundingFrustum, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromFrustum(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&frustum as *const <SpatialBoundingFrustum as ::windows::core::Abi>::Abi as *const <SpatialBoundingFrustum as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialBoundingVolumeStatics, BASE_OFFSET>(),
            FromBox: FromBox::<Impl, IMPL_OFFSET>,
            FromOrientedBox: FromOrientedBox::<Impl, IMPL_OFFSET>,
            FromSphere: FromSphere::<Impl, IMPL_OFFSET>,
            FromFrustum: FromFrustum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialBoundingVolumeStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialCoordinateSystem_Impl: Sized {
    fn TryGetTransformTo(&mut self, target: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialCoordinateSystem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialCoordinateSystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialCoordinateSystem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialCoordinateSystem_Vtbl {
        unsafe extern "system" fn TryGetTransformTo<Impl: ISpatialCoordinateSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetTransformTo(&*(&target as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialCoordinateSystem, BASE_OFFSET>(),
            TryGetTransformTo: TryGetTransformTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialCoordinateSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISpatialEntity_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Anchor(&mut self) -> ::windows::core::Result<SpatialAnchor>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntity";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISpatialEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntity_Vtbl {
        unsafe extern "system" fn Id<Impl: ISpatialEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Anchor<Impl: ISpatialEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Anchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpatialEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntity, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Anchor: Anchor::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityAddedEventArgs_Impl: Sized {
    fn Entity(&mut self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityAddedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityAddedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityAddedEventArgs_Vtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityAddedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityAddedEventArgs, BASE_OFFSET>(), Entity: Entity::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISpatialEntityFactory_Impl: Sized {
    fn CreateWithSpatialAnchor(&mut self, spatialanchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<SpatialEntity>;
    fn CreateWithSpatialAnchorAndProperties(&mut self, spatialanchor: &::core::option::Option<SpatialAnchor>, propertyset: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialEntityFactory {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISpatialEntityFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityFactory_Vtbl {
        unsafe extern "system" fn CreateWithSpatialAnchor<Impl: ISpatialEntityFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSpatialAnchor(&*(&spatialanchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSpatialAnchorAndProperties<Impl: ISpatialEntityFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSpatialAnchorAndProperties(&*(&spatialanchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), &*(&propertyset as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityFactory, BASE_OFFSET>(),
            CreateWithSpatialAnchor: CreateWithSpatialAnchor::<Impl, IMPL_OFFSET>,
            CreateWithSpatialAnchorAndProperties: CreateWithSpatialAnchorAndProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityRemovedEventArgs_Impl: Sized {
    fn Entity(&mut self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityRemovedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityRemovedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityRemovedEventArgs_Vtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityRemovedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityRemovedEventArgs, BASE_OFFSET>(), Entity: Entity::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialEntityStore_Impl: Sized {
    fn SaveAsync(&mut self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAsync(&mut self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateEntityWatcher(&mut self) -> ::windows::core::Result<SpatialEntityWatcher>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityStore";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialEntityStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityStore_Vtbl {
        unsafe extern "system" fn SaveAsync<Impl: ISpatialEntityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync(&*(&entity as *const <SpatialEntity as ::windows::core::Abi>::Abi as *const <SpatialEntity as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAsync<Impl: ISpatialEntityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAsync(&*(&entity as *const <SpatialEntity as ::windows::core::Abi>::Abi as *const <SpatialEntity as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEntityWatcher<Impl: ISpatialEntityStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEntityWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityStore, BASE_OFFSET>(),
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            RemoveAsync: RemoveAsync::<Impl, IMPL_OFFSET>,
            CreateEntityWatcher: CreateEntityWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System_RemoteSystems", feature = "implement_exclusive"))]
pub trait ISpatialEntityStoreStatics_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn TryGetForRemoteSystemSession(&mut self, session: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemSession>) -> ::windows::core::Result<SpatialEntityStore>;
}
#[cfg(all(feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialEntityStoreStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityStoreStatics";
}
#[cfg(all(feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ISpatialEntityStoreStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityStoreStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityStoreStatics_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ISpatialEntityStoreStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetForRemoteSystemSession<Impl: ISpatialEntityStoreStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetForRemoteSystemSession(&*(&session as *const <super::super::System::RemoteSystems::RemoteSystemSession as ::windows::core::Abi>::Abi as *const <super::super::System::RemoteSystems::RemoteSystemSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityStoreStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            TryGetForRemoteSystemSession: TryGetForRemoteSystemSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityStoreStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityUpdatedEventArgs_Impl: Sized {
    fn Entity(&mut self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityUpdatedEventArgs, BASE_OFFSET>(), Entity: Entity::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialEntityWatcher_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SpatialEntityWatcherStatus>;
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialEntityWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialEntityWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialEntityWatcher_Vtbl {
        unsafe extern "system" fn Status<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialEntityWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Added<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ISpatialEntityWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialEntityWatcher, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialEntityWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialLocation_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteLinearVelocity(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteLinearAcceleration(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularVelocity(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteAngularAcceleration(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocation_Vtbl {
        unsafe extern "system" fn Position<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AbsoluteLinearVelocity<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteLinearVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteLinearAcceleration<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteLinearAcceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularVelocity<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularAcceleration<Impl: ISpatialLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularAcceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocation, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            AbsoluteLinearVelocity: AbsoluteLinearVelocity::<Impl, IMPL_OFFSET>,
            AbsoluteLinearAcceleration: AbsoluteLinearAcceleration::<Impl, IMPL_OFFSET>,
            AbsoluteAngularVelocity: AbsoluteAngularVelocity::<Impl, IMPL_OFFSET>,
            AbsoluteAngularAcceleration: AbsoluteAngularAcceleration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialLocation2_Impl: Sized {
    fn AbsoluteAngularVelocityAxisAngle(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularAccelerationAxisAngle(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialLocation2 {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocation2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialLocation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocation2_Vtbl {
        unsafe extern "system" fn AbsoluteAngularVelocityAxisAngle<Impl: ISpatialLocation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularVelocityAxisAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularAccelerationAxisAngle<Impl: ISpatialLocation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularAccelerationAxisAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocation2, BASE_OFFSET>(),
            AbsoluteAngularVelocityAxisAngle: AbsoluteAngularVelocityAxisAngle::<Impl, IMPL_OFFSET>,
            AbsoluteAngularAccelerationAxisAngle: AbsoluteAngularAccelerationAxisAngle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialLocator_Impl: Sized {
    fn Locatability(&mut self) -> ::windows::core::Result<SpatialLocatability>;
    fn LocatabilityChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLocatabilityChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionalTrackingDeactivating(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionalTrackingDeactivating(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryLocateAtTimestamp(&mut self, timestamp: &::core::option::Option<super::PerceptionTimestamp>, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialLocation>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&mut self) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&mut self) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&mut self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocator_Vtbl {
        unsafe extern "system" fn Locatability<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialLocatability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locatability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocatabilityChanged<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocatabilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLocatabilityChanged<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLocatabilityChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionalTrackingDeactivating<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionalTrackingDeactivating(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionalTrackingDeactivating<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionalTrackingDeactivating(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryLocateAtTimestamp<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryLocateAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeading<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType), relativeheadinginradians) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocation<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<Impl: ISpatialLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType), relativeheadinginradians) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocator, BASE_OFFSET>(),
            Locatability: Locatability::<Impl, IMPL_OFFSET>,
            LocatabilityChanged: LocatabilityChanged::<Impl, IMPL_OFFSET>,
            RemoveLocatabilityChanged: RemoveLocatabilityChanged::<Impl, IMPL_OFFSET>,
            PositionalTrackingDeactivating: PositionalTrackingDeactivating::<Impl, IMPL_OFFSET>,
            RemovePositionalTrackingDeactivating: RemovePositionalTrackingDeactivating::<Impl, IMPL_OFFSET>,
            TryLocateAtTimestamp: TryLocateAtTimestamp::<Impl, IMPL_OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeading: CreateAttachedFrameOfReferenceAtCurrentHeading::<Impl, IMPL_OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition::<Impl, IMPL_OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation::<Impl, IMPL_OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading::<Impl, IMPL_OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocation: CreateStationaryFrameOfReferenceAtCurrentLocation::<Impl, IMPL_OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition::<Impl, IMPL_OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation::<Impl, IMPL_OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialLocatorAttachedFrameOfReference_Impl: Sized {
    fn RelativePosition(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativePosition(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeOrientation(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetRelativeOrientation(&mut self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn AdjustHeading(&mut self, headingoffsetinradians: f64) -> ::windows::core::Result<()>;
    fn GetStationaryCoordinateSystemAtTimestamp(&mut self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetRelativeHeadingAtTimestamp(&mut self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialLocatorAttachedFrameOfReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocatorAttachedFrameOfReference_Vtbl {
        unsafe extern "system" fn RelativePosition<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativePosition<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativePosition(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeOrientation<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeOrientation<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdjustHeading<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headingoffsetinradians: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustHeading(headingoffsetinradians).into()
        }
        unsafe extern "system" fn GetStationaryCoordinateSystemAtTimestamp<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStationaryCoordinateSystemAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetRelativeHeadingAtTimestamp<Impl: ISpatialLocatorAttachedFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetRelativeHeadingAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocatorAttachedFrameOfReference, BASE_OFFSET>(),
            RelativePosition: RelativePosition::<Impl, IMPL_OFFSET>,
            SetRelativePosition: SetRelativePosition::<Impl, IMPL_OFFSET>,
            RelativeOrientation: RelativeOrientation::<Impl, IMPL_OFFSET>,
            SetRelativeOrientation: SetRelativeOrientation::<Impl, IMPL_OFFSET>,
            AdjustHeading: AdjustHeading::<Impl, IMPL_OFFSET>,
            GetStationaryCoordinateSystemAtTimestamp: GetStationaryCoordinateSystemAtTimestamp::<Impl, IMPL_OFFSET>,
            TryGetRelativeHeadingAtTimestamp: TryGetRelativeHeadingAtTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocatorAttachedFrameOfReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Impl: Sized {
    fn Canceled(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanceled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
        unsafe extern "system" fn Canceled<Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanceled<Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanceled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocatorPositionalTrackingDeactivatingEventArgs, BASE_OFFSET>(),
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
            SetCanceled: SetCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocatorPositionalTrackingDeactivatingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocatorStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialLocatorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialLocatorStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ISpatialLocatorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialLocatorStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialLocatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialStageFrameOfReference_Impl: Sized {
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn MovementRange(&mut self) -> ::windows::core::Result<SpatialMovementRange>;
    fn LookDirectionRange(&mut self) -> ::windows::core::Result<SpatialLookDirectionRange>;
    fn GetCoordinateSystemAtCurrentLocation(&mut self, locator: &::core::option::Option<SpatialLocator>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetMovementBounds(&mut self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStageFrameOfReference";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialStageFrameOfReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialStageFrameOfReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialStageFrameOfReference_Vtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialStageFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovementRange<Impl: ISpatialStageFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialMovementRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovementRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookDirectionRange<Impl: ISpatialStageFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialLookDirectionRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookDirectionRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoordinateSystemAtCurrentLocation<Impl: ISpatialStageFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locator: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCoordinateSystemAtCurrentLocation(&*(&locator as *const <SpatialLocator as ::windows::core::Abi>::Abi as *const <SpatialLocator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMovementBounds<Impl: ISpatialStageFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetMovementBounds(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialStageFrameOfReference, BASE_OFFSET>(),
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
            MovementRange: MovementRange::<Impl, IMPL_OFFSET>,
            LookDirectionRange: LookDirectionRange::<Impl, IMPL_OFFSET>,
            GetCoordinateSystemAtCurrentLocation: GetCoordinateSystemAtCurrentLocation::<Impl, IMPL_OFFSET>,
            TryGetMovementBounds: TryGetMovementBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialStageFrameOfReference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpatialStageFrameOfReferenceStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<SpatialStageFrameOfReference>;
    fn CurrentChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestNewStageAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialStageFrameOfReferenceStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpatialStageFrameOfReferenceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialStageFrameOfReferenceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialStageFrameOfReferenceStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: ISpatialStageFrameOfReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Impl: ISpatialStageFrameOfReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Impl: ISpatialStageFrameOfReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestNewStageAsync<Impl: ISpatialStageFrameOfReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewStageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialStageFrameOfReferenceStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            CurrentChanged: CurrentChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentChanged: RemoveCurrentChanged::<Impl, IMPL_OFFSET>,
            RequestNewStageAsync: RequestNewStageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialStageFrameOfReferenceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStationaryFrameOfReference_Impl: Sized {
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<SpatialCoordinateSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStationaryFrameOfReference";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialStationaryFrameOfReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialStationaryFrameOfReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialStationaryFrameOfReference_Vtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialStationaryFrameOfReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialStationaryFrameOfReference, BASE_OFFSET>(),
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialStationaryFrameOfReference as ::windows::core::Interface>::IID
    }
}
