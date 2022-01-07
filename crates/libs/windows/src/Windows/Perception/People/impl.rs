#[cfg(feature = "implement_exclusive")]
pub trait IEyesPoseImpl: Sized {
    fn IsCalibrationValid(&self) -> ::windows::core::Result<bool>;
    fn Gaze(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>>;
    fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEyesPose {
    const NAME: &'static str = "Windows.Perception.People.IEyesPose";
}
#[cfg(feature = "implement_exclusive")]
impl IEyesPoseVtbl {
    pub const fn new<Impl: IEyesPoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEyesPoseVtbl {
        unsafe extern "system" fn IsCalibrationValid<Impl: IEyesPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCalibrationValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gaze<Impl: IEyesPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gaze() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTimestamp<Impl: IEyesPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEyesPose>, base.5, IsCalibrationValid::<Impl, OFFSET>, Gaze::<Impl, OFFSET>, UpdateTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEyesPoseStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEyesPoseStatics {
    const NAME: &'static str = "Windows.Perception.People.IEyesPoseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEyesPoseStaticsVtbl {
    pub const fn new<Impl: IEyesPoseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEyesPoseStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IEyesPoseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IEyesPoseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEyesPoseStatics>, base.5, IsSupported::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandMeshObserverImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::UI::Input::Spatial::SpatialInteractionSource>;
    fn TriangleIndexCount(&self) -> ::windows::core::Result<u32>;
    fn VertexCount(&self) -> ::windows::core::Result<u32>;
    fn GetTriangleIndices(&self, indices: &mut [<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetVertexStateForPose(&self, handpose: &::core::option::Option<HandPose>) -> ::windows::core::Result<HandMeshVertexState>;
    fn NeutralPose(&self) -> ::windows::core::Result<HandPose>;
    fn NeutralPoseVersion(&self) -> ::windows::core::Result<i32>;
    fn ModelId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHandMeshObserver {
    const NAME: &'static str = "Windows.Perception.People.IHandMeshObserver";
}
#[cfg(feature = "implement_exclusive")]
impl IHandMeshObserverVtbl {
    pub const fn new<Impl: IHandMeshObserverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHandMeshObserverVtbl {
        unsafe extern "system" fn Source<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriangleIndexCount<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriangleIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VertexCount<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VertexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriangleIndices<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indices_array_size: u32, indices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTriangleIndices(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&indices), indices_array_size as _)).into()
        }
        unsafe extern "system" fn GetVertexStateForPose<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handpose: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVertexStateForPose(&*(&handpose as *const <HandPose as ::windows::core::Abi>::Abi as *const <HandPose as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeutralPose<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NeutralPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeutralPoseVersion<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NeutralPoseVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelId<Impl: IHandMeshObserverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHandMeshObserver>, base.5, Source::<Impl, OFFSET>, TriangleIndexCount::<Impl, OFFSET>, VertexCount::<Impl, OFFSET>, GetTriangleIndices::<Impl, OFFSET>, GetVertexStateForPose::<Impl, OFFSET>, NeutralPose::<Impl, OFFSET>, NeutralPoseVersion::<Impl, OFFSET>, ModelId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandMeshVertexStateImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::Spatial::SpatialCoordinateSystem>;
    fn GetVertices(&self, vertices: &mut [<HandMeshVertex as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHandMeshVertexState {
    const NAME: &'static str = "Windows.Perception.People.IHandMeshVertexState";
}
#[cfg(feature = "implement_exclusive")]
impl IHandMeshVertexStateVtbl {
    pub const fn new<Impl: IHandMeshVertexStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHandMeshVertexStateVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: IHandMeshVertexStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVertices<Impl: IHandMeshVertexStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertices_array_size: u32, vertices: *mut HandMeshVertex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetVertices(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&vertices), vertices_array_size as _)).into()
        }
        unsafe extern "system" fn UpdateTimestamp<Impl: IHandMeshVertexStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHandMeshVertexState>, base.5, CoordinateSystem::<Impl, OFFSET>, GetVertices::<Impl, OFFSET>, UpdateTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandPoseImpl: Sized {
    fn TryGetJoint(&self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joint: HandJointKind, jointpose: &mut JointPose) -> ::windows::core::Result<bool>;
    fn TryGetJoints(&self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool>;
    fn GetRelativeJoint(&self, joint: HandJointKind, referencejoint: HandJointKind) -> ::windows::core::Result<JointPose>;
    fn GetRelativeJoints(&self, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], referencejoints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHandPose {
    const NAME: &'static str = "Windows.Perception.People.IHandPose";
}
#[cfg(feature = "implement_exclusive")]
impl IHandPoseVtbl {
    pub const fn new<Impl: IHandPoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHandPoseVtbl {
        unsafe extern "system" fn TryGetJoint<Impl: IHandPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, joint: HandJointKind, jointpose: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetJoint(&*(&coordinatesystem as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), joint, ::core::mem::transmute_copy(&jointpose)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetJoints<Impl: IHandPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, joints_array_size: u32, joints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetJoints(&*(&coordinatesystem as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&joints), joints_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&jointposes), jointPoses_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeJoint<Impl: IHandPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, joint: HandJointKind, referencejoint: HandJointKind, result__: *mut JointPose) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelativeJoint(joint, referencejoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeJoints<Impl: IHandPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, joints_array_size: u32, joints: *const HandJointKind, referenceJoints_array_size: u32, referencejoints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetRelativeJoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&joints), joints_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&referencejoints), referenceJoints_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&jointposes), jointPoses_array_size as _)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHandPose>, base.5, TryGetJoint::<Impl, OFFSET>, TryGetJoints::<Impl, OFFSET>, GetRelativeJoint::<Impl, OFFSET>, GetRelativeJoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHeadPoseImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHeadPose {
    const NAME: &'static str = "Windows.Perception.People.IHeadPose";
}
#[cfg(feature = "implement_exclusive")]
impl IHeadPoseVtbl {
    pub const fn new<Impl: IHeadPoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHeadPoseVtbl {
        unsafe extern "system" fn Position<Impl: IHeadPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForwardDirection<Impl: IHeadPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpDirection<Impl: IHeadPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHeadPose>, base.5, Position::<Impl, OFFSET>, ForwardDirection::<Impl, OFFSET>, UpDirection::<Impl, OFFSET>)
    }
}
