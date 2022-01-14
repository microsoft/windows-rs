#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IEyesPose_Impl: Sized {
    fn IsCalibrationValid(&mut self) -> ::windows::core::Result<bool>;
    fn Gaze(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>>;
    fn UpdateTimestamp(&mut self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEyesPose {
    const NAME: &'static str = "Windows.Perception.People.IEyesPose";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IEyesPose_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEyesPose_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEyesPose_Vtbl {
        unsafe extern "system" fn IsCalibrationValid<Impl: IEyesPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCalibrationValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gaze<Impl: IEyesPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gaze() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTimestamp<Impl: IEyesPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEyesPose, BASE_OFFSET>(),
            IsCalibrationValid: IsCalibrationValid::<Impl, IMPL_OFFSET>,
            Gaze: Gaze::<Impl, IMPL_OFFSET>,
            UpdateTimestamp: UpdateTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEyesPose as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IEyesPoseStatics_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEyesPoseStatics {
    const NAME: &'static str = "Windows.Perception.People.IEyesPoseStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IEyesPoseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEyesPoseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEyesPoseStatics_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IEyesPoseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IEyesPoseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEyesPoseStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEyesPoseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Input_Spatial", feature = "implement_exclusive"))]
pub trait IHandMeshObserver_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<super::super::UI::Input::Spatial::SpatialInteractionSource>;
    fn TriangleIndexCount(&mut self) -> ::windows::core::Result<u32>;
    fn VertexCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetTriangleIndices(&mut self, indices: &mut [<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetVertexStateForPose(&mut self, handpose: &::core::option::Option<HandPose>) -> ::windows::core::Result<HandMeshVertexState>;
    fn NeutralPose(&mut self) -> ::windows::core::Result<HandPose>;
    fn NeutralPoseVersion(&mut self) -> ::windows::core::Result<i32>;
    fn ModelId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "UI_Input_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHandMeshObserver {
    const NAME: &'static str = "Windows.Perception.People.IHandMeshObserver";
}
#[cfg(all(feature = "UI_Input_Spatial", feature = "implement_exclusive"))]
impl IHandMeshObserver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandMeshObserver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHandMeshObserver_Vtbl {
        unsafe extern "system" fn Source<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TriangleIndexCount<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriangleIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VertexCount<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VertexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriangleIndices<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indices_array_size: u32, indices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTriangleIndices(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&indices), indices_array_size as _)).into()
        }
        unsafe extern "system" fn GetVertexStateForPose<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handpose: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVertexStateForPose(&*(&handpose as *const <HandPose as ::windows::core::Abi>::Abi as *const <HandPose as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeutralPose<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeutralPose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeutralPoseVersion<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeutralPoseVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelId<Impl: IHandMeshObserver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHandMeshObserver, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            TriangleIndexCount: TriangleIndexCount::<Impl, IMPL_OFFSET>,
            VertexCount: VertexCount::<Impl, IMPL_OFFSET>,
            GetTriangleIndices: GetTriangleIndices::<Impl, IMPL_OFFSET>,
            GetVertexStateForPose: GetVertexStateForPose::<Impl, IMPL_OFFSET>,
            NeutralPose: NeutralPose::<Impl, IMPL_OFFSET>,
            NeutralPoseVersion: NeutralPoseVersion::<Impl, IMPL_OFFSET>,
            ModelId: ModelId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHandMeshObserver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHandMeshVertexState_Impl: Sized {
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<super::Spatial::SpatialCoordinateSystem>;
    fn GetVertices(&mut self, vertices: &mut [<HandMeshVertex as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateTimestamp(&mut self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHandMeshVertexState {
    const NAME: &'static str = "Windows.Perception.People.IHandMeshVertexState";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHandMeshVertexState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandMeshVertexState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHandMeshVertexState_Vtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: IHandMeshVertexState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVertices<Impl: IHandMeshVertexState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertices_array_size: u32, vertices: *mut HandMeshVertex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVertices(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&vertices), vertices_array_size as _)).into()
        }
        unsafe extern "system" fn UpdateTimestamp<Impl: IHandMeshVertexState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHandMeshVertexState, BASE_OFFSET>(),
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
            GetVertices: GetVertices::<Impl, IMPL_OFFSET>,
            UpdateTimestamp: UpdateTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHandMeshVertexState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHandPose_Impl: Sized {
    fn TryGetJoint(&mut self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joint: HandJointKind, jointpose: &mut JointPose) -> ::windows::core::Result<bool>;
    fn TryGetJoints(&mut self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool>;
    fn GetRelativeJoint(&mut self, joint: HandJointKind, referencejoint: HandJointKind) -> ::windows::core::Result<JointPose>;
    fn GetRelativeJoints(&mut self, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], referencejoints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHandPose {
    const NAME: &'static str = "Windows.Perception.People.IHandPose";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHandPose_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandPose_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHandPose_Vtbl {
        unsafe extern "system" fn TryGetJoint<Impl: IHandPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, joint: HandJointKind, jointpose: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetJoint(&*(&coordinatesystem as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), joint, ::core::mem::transmute_copy(&jointpose)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetJoints<Impl: IHandPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, joints_array_size: u32, joints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetJoints(&*(&coordinatesystem as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&joints), joints_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&jointposes), jointPoses_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeJoint<Impl: IHandPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, joint: HandJointKind, referencejoint: HandJointKind, result__: *mut JointPose) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelativeJoint(joint, referencejoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeJoints<Impl: IHandPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, joints_array_size: u32, joints: *const HandJointKind, referenceJoints_array_size: u32, referencejoints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRelativeJoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&joints), joints_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&referencejoints), referenceJoints_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&jointposes), jointPoses_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHandPose, BASE_OFFSET>(),
            TryGetJoint: TryGetJoint::<Impl, IMPL_OFFSET>,
            TryGetJoints: TryGetJoints::<Impl, IMPL_OFFSET>,
            GetRelativeJoint: GetRelativeJoint::<Impl, IMPL_OFFSET>,
            GetRelativeJoints: GetRelativeJoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHandPose as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IHeadPose_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHeadPose {
    const NAME: &'static str = "Windows.Perception.People.IHeadPose";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IHeadPose_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeadPose_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeadPose_Vtbl {
        unsafe extern "system" fn Position<Impl: IHeadPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForwardDirection<Impl: IHeadPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpDirection<Impl: IHeadPose_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHeadPose, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            ForwardDirection: ForwardDirection::<Impl, IMPL_OFFSET>,
            UpDirection: UpDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeadPose as ::windows::core::Interface>::IID
    }
}
