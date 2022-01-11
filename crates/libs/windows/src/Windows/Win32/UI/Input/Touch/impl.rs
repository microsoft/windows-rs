#[cfg(feature = "Win32_Foundation")]
pub trait IInertiaProcessorImpl: Sized {
    fn InitialOriginX();
    fn SetInitialOriginX();
    fn InitialOriginY();
    fn SetInitialOriginY();
    fn InitialVelocityX();
    fn SetInitialVelocityX();
    fn InitialVelocityY();
    fn SetInitialVelocityY();
    fn InitialAngularVelocity();
    fn SetInitialAngularVelocity();
    fn InitialExpansionVelocity();
    fn SetInitialExpansionVelocity();
    fn InitialRadius();
    fn SetInitialRadius();
    fn BoundaryLeft();
    fn SetBoundaryLeft();
    fn BoundaryTop();
    fn SetBoundaryTop();
    fn BoundaryRight();
    fn SetBoundaryRight();
    fn BoundaryBottom();
    fn SetBoundaryBottom();
    fn ElasticMarginLeft();
    fn SetElasticMarginLeft();
    fn ElasticMarginTop();
    fn SetElasticMarginTop();
    fn ElasticMarginRight();
    fn SetElasticMarginRight();
    fn ElasticMarginBottom();
    fn SetElasticMarginBottom();
    fn DesiredDisplacement();
    fn SetDesiredDisplacement();
    fn DesiredRotation();
    fn SetDesiredRotation();
    fn DesiredExpansion();
    fn SetDesiredExpansion();
    fn DesiredDeceleration();
    fn SetDesiredDeceleration();
    fn DesiredAngularDeceleration();
    fn SetDesiredAngularDeceleration();
    fn DesiredExpansionDeceleration();
    fn SetDesiredExpansionDeceleration();
    fn InitialTimestamp();
    fn SetInitialTimestamp();
    fn Reset();
    fn Process();
    fn ProcessTime();
    fn Complete();
    fn CompleteTime();
}
#[cfg(feature = "Win32_Foundation")]
impl IInertiaProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaProcessorVtbl {
        unsafe extern "system" fn InitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Process<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Complete<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompleteTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitialOriginX: InitialOriginX::<Impl, IMPL_OFFSET>,
            SetInitialOriginX: SetInitialOriginX::<Impl, IMPL_OFFSET>,
            InitialOriginY: InitialOriginY::<Impl, IMPL_OFFSET>,
            SetInitialOriginY: SetInitialOriginY::<Impl, IMPL_OFFSET>,
            InitialVelocityX: InitialVelocityX::<Impl, IMPL_OFFSET>,
            SetInitialVelocityX: SetInitialVelocityX::<Impl, IMPL_OFFSET>,
            InitialVelocityY: InitialVelocityY::<Impl, IMPL_OFFSET>,
            SetInitialVelocityY: SetInitialVelocityY::<Impl, IMPL_OFFSET>,
            InitialAngularVelocity: InitialAngularVelocity::<Impl, IMPL_OFFSET>,
            SetInitialAngularVelocity: SetInitialAngularVelocity::<Impl, IMPL_OFFSET>,
            InitialExpansionVelocity: InitialExpansionVelocity::<Impl, IMPL_OFFSET>,
            SetInitialExpansionVelocity: SetInitialExpansionVelocity::<Impl, IMPL_OFFSET>,
            InitialRadius: InitialRadius::<Impl, IMPL_OFFSET>,
            SetInitialRadius: SetInitialRadius::<Impl, IMPL_OFFSET>,
            BoundaryLeft: BoundaryLeft::<Impl, IMPL_OFFSET>,
            SetBoundaryLeft: SetBoundaryLeft::<Impl, IMPL_OFFSET>,
            BoundaryTop: BoundaryTop::<Impl, IMPL_OFFSET>,
            SetBoundaryTop: SetBoundaryTop::<Impl, IMPL_OFFSET>,
            BoundaryRight: BoundaryRight::<Impl, IMPL_OFFSET>,
            SetBoundaryRight: SetBoundaryRight::<Impl, IMPL_OFFSET>,
            BoundaryBottom: BoundaryBottom::<Impl, IMPL_OFFSET>,
            SetBoundaryBottom: SetBoundaryBottom::<Impl, IMPL_OFFSET>,
            ElasticMarginLeft: ElasticMarginLeft::<Impl, IMPL_OFFSET>,
            SetElasticMarginLeft: SetElasticMarginLeft::<Impl, IMPL_OFFSET>,
            ElasticMarginTop: ElasticMarginTop::<Impl, IMPL_OFFSET>,
            SetElasticMarginTop: SetElasticMarginTop::<Impl, IMPL_OFFSET>,
            ElasticMarginRight: ElasticMarginRight::<Impl, IMPL_OFFSET>,
            SetElasticMarginRight: SetElasticMarginRight::<Impl, IMPL_OFFSET>,
            ElasticMarginBottom: ElasticMarginBottom::<Impl, IMPL_OFFSET>,
            SetElasticMarginBottom: SetElasticMarginBottom::<Impl, IMPL_OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Impl, IMPL_OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Impl, IMPL_OFFSET>,
            DesiredRotation: DesiredRotation::<Impl, IMPL_OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Impl, IMPL_OFFSET>,
            DesiredExpansion: DesiredExpansion::<Impl, IMPL_OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Impl, IMPL_OFFSET>,
            DesiredDeceleration: DesiredDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Impl, IMPL_OFFSET>,
            DesiredAngularDeceleration: DesiredAngularDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredAngularDeceleration: SetDesiredAngularDeceleration::<Impl, IMPL_OFFSET>,
            DesiredExpansionDeceleration: DesiredExpansionDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredExpansionDeceleration: SetDesiredExpansionDeceleration::<Impl, IMPL_OFFSET>,
            InitialTimestamp: InitialTimestamp::<Impl, IMPL_OFFSET>,
            SetInitialTimestamp: SetInitialTimestamp::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Process: Process::<Impl, IMPL_OFFSET>,
            ProcessTime: ProcessTime::<Impl, IMPL_OFFSET>,
            Complete: Complete::<Impl, IMPL_OFFSET>,
            CompleteTime: CompleteTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInertiaProcessor as ::windows::core::Interface>::IID
    }
}
pub trait IManipulationProcessorImpl: Sized {
    fn SupportedManipulations();
    fn SetSupportedManipulations();
    fn PivotPointX();
    fn SetPivotPointX();
    fn PivotPointY();
    fn SetPivotPointY();
    fn PivotRadius();
    fn SetPivotRadius();
    fn CompleteManipulation();
    fn ProcessDown();
    fn ProcessMove();
    fn ProcessUp();
    fn ProcessDownWithTime();
    fn ProcessMoveWithTime();
    fn ProcessUpWithTime();
    fn GetVelocityX();
    fn GetVelocityY();
    fn GetExpansionVelocity();
    fn GetAngularVelocity();
    fn MinimumScaleRotateRadius();
    fn SetMinimumScaleRotateRadius();
}
impl IManipulationProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationProcessorVtbl {
        unsafe extern "system" fn SupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompleteManipulation<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessDown<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessMove<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessUp<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessDownWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessMoveWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessUpWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVelocityX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVelocityY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExpansionVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAngularVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SupportedManipulations: SupportedManipulations::<Impl, IMPL_OFFSET>,
            SetSupportedManipulations: SetSupportedManipulations::<Impl, IMPL_OFFSET>,
            PivotPointX: PivotPointX::<Impl, IMPL_OFFSET>,
            SetPivotPointX: SetPivotPointX::<Impl, IMPL_OFFSET>,
            PivotPointY: PivotPointY::<Impl, IMPL_OFFSET>,
            SetPivotPointY: SetPivotPointY::<Impl, IMPL_OFFSET>,
            PivotRadius: PivotRadius::<Impl, IMPL_OFFSET>,
            SetPivotRadius: SetPivotRadius::<Impl, IMPL_OFFSET>,
            CompleteManipulation: CompleteManipulation::<Impl, IMPL_OFFSET>,
            ProcessDown: ProcessDown::<Impl, IMPL_OFFSET>,
            ProcessMove: ProcessMove::<Impl, IMPL_OFFSET>,
            ProcessUp: ProcessUp::<Impl, IMPL_OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Impl, IMPL_OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Impl, IMPL_OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Impl, IMPL_OFFSET>,
            GetVelocityX: GetVelocityX::<Impl, IMPL_OFFSET>,
            GetVelocityY: GetVelocityY::<Impl, IMPL_OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Impl, IMPL_OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Impl, IMPL_OFFSET>,
            MinimumScaleRotateRadius: MinimumScaleRotateRadius::<Impl, IMPL_OFFSET>,
            SetMinimumScaleRotateRadius: SetMinimumScaleRotateRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationProcessor as ::windows::core::Interface>::IID
    }
}
pub trait _IManipulationEventsImpl: Sized {
    fn ManipulationStarted();
    fn ManipulationDelta();
    fn ManipulationCompleted();
}
impl _IManipulationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IManipulationEventsVtbl {
        unsafe extern "system" fn ManipulationStarted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ManipulationDelta<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Impl, IMPL_OFFSET>,
            ManipulationDelta: ManipulationDelta::<Impl, IMPL_OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IManipulationEvents as ::windows::core::Interface>::IID
    }
}
