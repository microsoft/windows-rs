#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystemAdjusted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawCoordinateSystemAdjusted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchor";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorVtbl {
    pub const fn new<Impl: ISpatialAnchorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialAnchorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawCoordinateSystem<Impl: ISpatialAnchorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawCoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawCoordinateSystemAdjusted<Impl: ISpatialAnchorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawCoordinateSystemAdjusted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRawCoordinateSystemAdjusted<Impl: ISpatialAnchorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRawCoordinateSystemAdjusted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchor>, base.5, CoordinateSystem::<Impl, OFFSET>, RawCoordinateSystem::<Impl, OFFSET>, RawCoordinateSystemAdjusted::<Impl, OFFSET>, RemoveRawCoordinateSystemAdjusted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchor2Impl: Sized {
    fn RemovedByUser(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchor2 {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchor2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchor2Vtbl {
    pub const fn new<Impl: ISpatialAnchor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchor2Vtbl {
        unsafe extern "system" fn RemovedByUser<Impl: ISpatialAnchor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemovedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchor2>, base.5, RemovedByUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExportSufficiencyImpl: Sized {
    fn IsMinimallySufficient(&self) -> ::windows::core::Result<bool>;
    fn SufficiencyLevel(&self) -> ::windows::core::Result<f64>;
    fn RecommendedSufficiencyLevel(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExportSufficiency";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorExportSufficiencyVtbl {
    pub const fn new<Impl: ISpatialAnchorExportSufficiencyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorExportSufficiencyVtbl {
        unsafe extern "system" fn IsMinimallySufficient<Impl: ISpatialAnchorExportSufficiencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMinimallySufficient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SufficiencyLevel<Impl: ISpatialAnchorExportSufficiencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SufficiencyLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedSufficiencyLevel<Impl: ISpatialAnchorExportSufficiencyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecommendedSufficiencyLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorExportSufficiency>, base.5, IsMinimallySufficient::<Impl, OFFSET>, SufficiencyLevel::<Impl, OFFSET>, RecommendedSufficiencyLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExporterImpl: Sized {
    fn GetAnchorExportSufficiencyAsync(&self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>;
    fn TryExportAnchorAsync(&self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExporter";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorExporterVtbl {
    pub const fn new<Impl: ISpatialAnchorExporterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorExporterVtbl {
        unsafe extern "system" fn GetAnchorExportSufficiencyAsync<Impl: ISpatialAnchorExporterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnchorExportSufficiencyAsync(&*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), purpose) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryExportAnchorAsync<Impl: ISpatialAnchorExporterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryExportAnchorAsync(&*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), purpose, &*(&stream as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorExporter>, base.5, GetAnchorExportSufficiencyAsync::<Impl, OFFSET>, TryExportAnchorAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExporterStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialAnchorExporter>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorExporterStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorExporterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorExporterStaticsVtbl {
    pub const fn new<Impl: ISpatialAnchorExporterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorExporterStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISpatialAnchorExporterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: ISpatialAnchorExporterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorExporterStatics>, base.5, GetDefault::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorManagerStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorManagerStaticsVtbl {
    pub const fn new<Impl: ISpatialAnchorManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorManagerStaticsVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: ISpatialAnchorManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorManagerStatics>, base.5, RequestStoreAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorRawCoordinateSystemAdjustedEventArgsImpl: Sized {
    fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl {
    pub const fn new<Impl: ISpatialAnchorRawCoordinateSystemAdjustedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl {
        unsafe extern "system" fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform<Impl: ISpatialAnchorRawCoordinateSystemAdjustedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorRawCoordinateSystemAdjustedEventArgs>, base.5, OldRawCoordinateSystemToNewRawCoordinateSystemTransform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorStaticsImpl: Sized {
    fn TryCreateRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionAndOrientationRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialAnchor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorStaticsVtbl {
    pub const fn new<Impl: ISpatialAnchorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorStaticsVtbl {
        unsafe extern "system" fn TryCreateRelativeTo<Impl: ISpatialAnchorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateRelativeTo(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWithPositionRelativeTo<Impl: ISpatialAnchorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateWithPositionRelativeTo(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWithPositionAndOrientationRelativeTo<Impl: ISpatialAnchorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorStatics>, base.5, TryCreateRelativeTo::<Impl, OFFSET>, TryCreateWithPositionRelativeTo::<Impl, OFFSET>, TryCreateWithPositionAndOrientationRelativeTo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorStoreImpl: Sized {
    fn GetAllSavedAnchors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>;
    fn TrySave(&self, id: &::windows::core::HSTRING, anchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<bool>;
    fn Remove(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorStore";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAnchorStoreVtbl {
    pub const fn new<Impl: ISpatialAnchorStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorStoreVtbl {
        unsafe extern "system" fn GetAllSavedAnchors<Impl: ISpatialAnchorStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllSavedAnchors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySave<Impl: ISpatialAnchorStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, anchor: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySave(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&anchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISpatialAnchorStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Remove(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISpatialAnchorStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorStore>, base.5, GetAllSavedAnchors::<Impl, OFFSET>, TrySave::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISpatialAnchorTransferManagerStaticsImpl: Sized {
    fn TryImportAnchorsAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>>;
    fn TryExportAnchorsAsync(&self, anchors: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>>>, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialAnchorTransferManagerStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialAnchorTransferManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISpatialAnchorTransferManagerStaticsVtbl {
    pub const fn new<Impl: ISpatialAnchorTransferManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAnchorTransferManagerStaticsVtbl {
        unsafe extern "system" fn TryImportAnchorsAsync<Impl: ISpatialAnchorTransferManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryImportAnchorsAsync(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryExportAnchorsAsync<Impl: ISpatialAnchorTransferManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anchors: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn RequestAccessAsync<Impl: ISpatialAnchorTransferManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAnchorTransferManagerStatics>, base.5, TryImportAnchorsAsync::<Impl, OFFSET>, TryExportAnchorsAsync::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialBoundingVolumeImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialBoundingVolume";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialBoundingVolumeVtbl {
    pub const fn new<Impl: ISpatialBoundingVolumeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialBoundingVolumeVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialBoundingVolume>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialBoundingVolumeStaticsImpl: Sized {
    fn FromBox(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromOrientedBox(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingOrientedBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromSphere(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, sphere: &SpatialBoundingSphere) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromFrustum(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, frustum: &SpatialBoundingFrustum) -> ::windows::core::Result<SpatialBoundingVolume>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialBoundingVolumeStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialBoundingVolumeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialBoundingVolumeStaticsVtbl {
    pub const fn new<Impl: ISpatialBoundingVolumeStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialBoundingVolumeStaticsVtbl {
        unsafe extern "system" fn FromBox<Impl: ISpatialBoundingVolumeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromBox(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&r#box as *const <SpatialBoundingBox as ::windows::core::Abi>::Abi as *const <SpatialBoundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromOrientedBox<Impl: ISpatialBoundingVolumeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingOrientedBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromOrientedBox(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&r#box as *const <SpatialBoundingOrientedBox as ::windows::core::Abi>::Abi as *const <SpatialBoundingOrientedBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromSphere<Impl: ISpatialBoundingVolumeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, sphere: SpatialBoundingSphere, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromSphere(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&sphere as *const <SpatialBoundingSphere as ::windows::core::Abi>::Abi as *const <SpatialBoundingSphere as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromFrustum<Impl: ISpatialBoundingVolumeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, frustum: SpatialBoundingFrustum, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromFrustum(&*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&frustum as *const <SpatialBoundingFrustum as ::windows::core::Abi>::Abi as *const <SpatialBoundingFrustum as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialBoundingVolumeStatics>, base.5, FromBox::<Impl, OFFSET>, FromOrientedBox::<Impl, OFFSET>, FromSphere::<Impl, OFFSET>, FromFrustum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialCoordinateSystemImpl: Sized {
    fn TryGetTransformTo(&self, target: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialCoordinateSystem";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialCoordinateSystemVtbl {
    pub const fn new<Impl: ISpatialCoordinateSystemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialCoordinateSystemVtbl {
        unsafe extern "system" fn TryGetTransformTo<Impl: ISpatialCoordinateSystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetTransformTo(&*(&target as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialCoordinateSystem>, base.5, TryGetTransformTo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Anchor(&self) -> ::windows::core::Result<SpatialAnchor>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntity";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityVtbl {
    pub const fn new<Impl: ISpatialEntityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityVtbl {
        unsafe extern "system" fn Id<Impl: ISpatialEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Anchor<Impl: ISpatialEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Anchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpatialEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntity>, base.5, Id::<Impl, OFFSET>, Anchor::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityAddedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityAddedEventArgsVtbl {
    pub const fn new<Impl: ISpatialEntityAddedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityAddedEventArgsVtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityAddedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityAddedEventArgs>, base.5, Entity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityFactoryImpl: Sized {
    fn CreateWithSpatialAnchor(&self, spatialanchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<SpatialEntity>;
    fn CreateWithSpatialAnchorAndProperties(&self, spatialanchor: &::core::option::Option<SpatialAnchor>, propertyset: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityFactory {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityFactoryVtbl {
    pub const fn new<Impl: ISpatialEntityFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityFactoryVtbl {
        unsafe extern "system" fn CreateWithSpatialAnchor<Impl: ISpatialEntityFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithSpatialAnchor(&*(&spatialanchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSpatialAnchorAndProperties<Impl: ISpatialEntityFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithSpatialAnchorAndProperties(&*(&spatialanchor as *const <SpatialAnchor as ::windows::core::Abi>::Abi as *const <SpatialAnchor as ::windows::core::DefaultType>::DefaultType), &*(&propertyset as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityFactory>, base.5, CreateWithSpatialAnchor::<Impl, OFFSET>, CreateWithSpatialAnchorAndProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityRemovedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityRemovedEventArgsVtbl {
    pub const fn new<Impl: ISpatialEntityRemovedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityRemovedEventArgsVtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityRemovedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityRemovedEventArgs>, base.5, Entity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityStoreImpl: Sized {
    fn SaveAsync(&self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAsync(&self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateEntityWatcher(&self) -> ::windows::core::Result<SpatialEntityWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityStore";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityStoreVtbl {
    pub const fn new<Impl: ISpatialEntityStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityStoreVtbl {
        unsafe extern "system" fn SaveAsync<Impl: ISpatialEntityStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveAsync(&*(&entity as *const <SpatialEntity as ::windows::core::Abi>::Abi as *const <SpatialEntity as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAsync<Impl: ISpatialEntityStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAsync(&*(&entity as *const <SpatialEntity as ::windows::core::Abi>::Abi as *const <SpatialEntity as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEntityWatcher<Impl: ISpatialEntityStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEntityWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityStore>, base.5, SaveAsync::<Impl, OFFSET>, RemoveAsync::<Impl, OFFSET>, CreateEntityWatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityStoreStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn TryGetForRemoteSystemSession(&self, session: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemSession>) -> ::windows::core::Result<SpatialEntityStore>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityStoreStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityStoreStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityStoreStaticsVtbl {
    pub const fn new<Impl: ISpatialEntityStoreStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityStoreStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: ISpatialEntityStoreStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetForRemoteSystemSession<Impl: ISpatialEntityStoreStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetForRemoteSystemSession(&*(&session as *const <super::super::System::RemoteSystems::RemoteSystemSession as ::windows::core::Abi>::Abi as *const <super::super::System::RemoteSystems::RemoteSystemSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityStoreStatics>, base.5, IsSupported::<Impl, OFFSET>, TryGetForRemoteSystemSession::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityUpdatedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityUpdatedEventArgsVtbl {
    pub const fn new<Impl: ISpatialEntityUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityUpdatedEventArgsVtbl {
        unsafe extern "system" fn Entity<Impl: ISpatialEntityUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Entity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityUpdatedEventArgs>, base.5, Entity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpatialEntityWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialEntityWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialEntityWatcherVtbl {
    pub const fn new<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialEntityWatcherVtbl {
        unsafe extern "system" fn Status<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialEntityWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Added<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ISpatialEntityWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialEntityWatcher>, base.5, Status::<Impl, OFFSET>, Added::<Impl, OFFSET>, RemoveAdded::<Impl, OFFSET>, Updated::<Impl, OFFSET>, RemoveUpdated::<Impl, OFFSET>, Removed::<Impl, OFFSET>, RemoveRemoved::<Impl, OFFSET>, EnumerationCompleted::<Impl, OFFSET>, RemoveEnumerationCompleted::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocationImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteLinearVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteLinearAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteAngularAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocation";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocationVtbl {
    pub const fn new<Impl: ISpatialLocationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocationVtbl {
        unsafe extern "system" fn Position<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteLinearVelocity<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteLinearVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteLinearAcceleration<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteLinearAcceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularVelocity<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularAcceleration<Impl: ISpatialLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularAcceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialLocation>, base.5, Position::<Impl, OFFSET>, Orientation::<Impl, OFFSET>, AbsoluteLinearVelocity::<Impl, OFFSET>, AbsoluteLinearAcceleration::<Impl, OFFSET>, AbsoluteAngularVelocity::<Impl, OFFSET>, AbsoluteAngularAcceleration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocation2Impl: Sized {
    fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocation2 {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocation2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocation2Vtbl {
    pub const fn new<Impl: ISpatialLocation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocation2Vtbl {
        unsafe extern "system" fn AbsoluteAngularVelocityAxisAngle<Impl: ISpatialLocation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularVelocityAxisAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbsoluteAngularAccelerationAxisAngle<Impl: ISpatialLocation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteAngularAccelerationAxisAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialLocation2>, base.5, AbsoluteAngularVelocityAxisAngle::<Impl, OFFSET>, AbsoluteAngularAccelerationAxisAngle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorImpl: Sized {
    fn Locatability(&self) -> ::windows::core::Result<SpatialLocatability>;
    fn LocatabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLocatabilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionalTrackingDeactivating(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionalTrackingDeactivating(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryLocateAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialLocation>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocator";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorVtbl {
    pub const fn new<Impl: ISpatialLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocatorVtbl {
        unsafe extern "system" fn Locatability<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialLocatability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locatability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocatabilityChanged<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocatabilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLocatabilityChanged<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLocatabilityChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionalTrackingDeactivating<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionalTrackingDeactivating(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionalTrackingDeactivating<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePositionalTrackingDeactivating(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryLocateAtTimestamp<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryLocateAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystem as *const <SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeading<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType), relativeheadinginradians) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocation<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<Impl: ISpatialLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&*(&relativeposition as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&relativeorientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType), relativeheadinginradians) {
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
            ::windows::core::GetRuntimeClassName::<ISpatialLocator>,
            base.5,
            Locatability::<Impl, OFFSET>,
            LocatabilityChanged::<Impl, OFFSET>,
            RemoveLocatabilityChanged::<Impl, OFFSET>,
            PositionalTrackingDeactivating::<Impl, OFFSET>,
            RemovePositionalTrackingDeactivating::<Impl, OFFSET>,
            TryLocateAtTimestamp::<Impl, OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeading::<Impl, OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition::<Impl, OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation::<Impl, OFFSET>,
            CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading::<Impl, OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocation::<Impl, OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition::<Impl, OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation::<Impl, OFFSET>,
            CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorAttachedFrameOfReferenceImpl: Sized {
    fn RelativePosition(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativePosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetRelativeOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows::core::Result<()>;
    fn GetStationaryCoordinateSystemAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetRelativeHeadingAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorAttachedFrameOfReference";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorAttachedFrameOfReferenceVtbl {
    pub const fn new<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocatorAttachedFrameOfReferenceVtbl {
        unsafe extern "system" fn RelativePosition<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativePosition<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativePosition(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeOrientation<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeOrientation<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativeOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdjustHeading<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, headingoffsetinradians: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AdjustHeading(headingoffsetinradians).into()
        }
        unsafe extern "system" fn GetStationaryCoordinateSystemAtTimestamp<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStationaryCoordinateSystemAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetRelativeHeadingAtTimestamp<Impl: ISpatialLocatorAttachedFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetRelativeHeadingAtTimestamp(&*(&timestamp as *const <super::PerceptionTimestamp as ::windows::core::Abi>::Abi as *const <super::PerceptionTimestamp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialLocatorAttachedFrameOfReference>, base.5, RelativePosition::<Impl, OFFSET>, SetRelativePosition::<Impl, OFFSET>, RelativeOrientation::<Impl, OFFSET>, SetRelativeOrientation::<Impl, OFFSET>, AdjustHeading::<Impl, OFFSET>, GetStationaryCoordinateSystemAtTimestamp::<Impl, OFFSET>, TryGetRelativeHeadingAtTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorPositionalTrackingDeactivatingEventArgsImpl: Sized {
    fn Canceled(&self) -> ::windows::core::Result<bool>;
    fn SetCanceled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl {
    pub const fn new<Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl {
        unsafe extern "system" fn Canceled<Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Canceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanceled<Impl: ISpatialLocatorPositionalTrackingDeactivatingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanceled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialLocatorPositionalTrackingDeactivatingEventArgs>, base.5, Canceled::<Impl, OFFSET>, SetCanceled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialLocatorStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialLocatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialLocatorStaticsVtbl {
    pub const fn new<Impl: ISpatialLocatorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialLocatorStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISpatialLocatorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialLocatorStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStageFrameOfReferenceImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn MovementRange(&self) -> ::windows::core::Result<SpatialMovementRange>;
    fn LookDirectionRange(&self) -> ::windows::core::Result<SpatialLookDirectionRange>;
    fn GetCoordinateSystemAtCurrentLocation(&self, locator: &::core::option::Option<SpatialLocator>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetMovementBounds(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStageFrameOfReference";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialStageFrameOfReferenceVtbl {
    pub const fn new<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialStageFrameOfReferenceVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovementRange<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialMovementRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovementRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookDirectionRange<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialLookDirectionRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LookDirectionRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoordinateSystemAtCurrentLocation<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locator: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCoordinateSystemAtCurrentLocation(&*(&locator as *const <SpatialLocator as ::windows::core::Abi>::Abi as *const <SpatialLocator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMovementBounds<Impl: ISpatialStageFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialStageFrameOfReference>, base.5, CoordinateSystem::<Impl, OFFSET>, MovementRange::<Impl, OFFSET>, LookDirectionRange::<Impl, OFFSET>, GetCoordinateSystemAtCurrentLocation::<Impl, OFFSET>, TryGetMovementBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStageFrameOfReferenceStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<SpatialStageFrameOfReference>;
    fn CurrentChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestNewStageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialStageFrameOfReferenceStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStageFrameOfReferenceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialStageFrameOfReferenceStaticsVtbl {
    pub const fn new<Impl: ISpatialStageFrameOfReferenceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialStageFrameOfReferenceStaticsVtbl {
        unsafe extern "system" fn Current<Impl: ISpatialStageFrameOfReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Impl: ISpatialStageFrameOfReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Impl: ISpatialStageFrameOfReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestNewStageAsync<Impl: ISpatialStageFrameOfReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestNewStageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialStageFrameOfReferenceStatics>, base.5, Current::<Impl, OFFSET>, CurrentChanged::<Impl, OFFSET>, RemoveCurrentChanged::<Impl, OFFSET>, RequestNewStageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStationaryFrameOfReferenceImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.ISpatialStationaryFrameOfReference";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialStationaryFrameOfReferenceVtbl {
    pub const fn new<Impl: ISpatialStationaryFrameOfReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialStationaryFrameOfReferenceVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialStationaryFrameOfReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialStationaryFrameOfReference>, base.5, CoordinateSystem::<Impl, OFFSET>)
    }
}
