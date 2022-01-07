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
impl ::windows::core::RuntimeName for IInertiaProcessor {
    const NAME: &'static str = "Windows.Win32.UI.Input.Touch.IInertiaProcessor";
}
impl IInertiaProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessorImpl, const OFFSET: isize>() -> IInertiaProcessorVtbl {
        unsafe extern "system" fn InitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialOriginX(::core::mem::transmute_copy(&x)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialOriginX(x) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialOriginY(::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialOriginY(y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocityX(::core::mem::transmute_copy(&x)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialVelocityX(x) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocityY(::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialVelocityY(y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialAngularVelocity(::core::mem::transmute_copy(&velocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialAngularVelocity(velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialExpansionVelocity(::core::mem::transmute_copy(&velocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialExpansionVelocity(velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialRadius(::core::mem::transmute_copy(&radius)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialRadius(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryLeft(::core::mem::transmute_copy(&left)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoundaryLeft(left) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryTop(::core::mem::transmute_copy(&top)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoundaryTop(top) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryRight(::core::mem::transmute_copy(&right)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoundaryRight(right) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryBottom(::core::mem::transmute_copy(&bottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoundaryBottom(bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginLeft(::core::mem::transmute_copy(&left)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetElasticMarginLeft(left) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginTop(::core::mem::transmute_copy(&top)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetElasticMarginTop(top) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginRight(::core::mem::transmute_copy(&right)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetElasticMarginRight(right) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginBottom(::core::mem::transmute_copy(&bottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetElasticMarginBottom(bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDisplacement(::core::mem::transmute_copy(&displacement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredDisplacement(displacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredRotation(::core::mem::transmute_copy(&rotation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredRotation(rotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredExpansion(::core::mem::transmute_copy(&expansion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredExpansion(expansion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration(::core::mem::transmute_copy(&deceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredDeceleration(deceleration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAngularDeceleration(::core::mem::transmute_copy(&deceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredAngularDeceleration(deceleration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredExpansionDeceleration(::core::mem::transmute_copy(&deceleration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredExpansionDeceleration(deceleration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialTimestamp(::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialTimestamp(timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process(::core::mem::transmute_copy(&completed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessTime(timestamp, ::core::mem::transmute_copy(&completed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Complete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteTime(timestamp) {
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
            ::windows::core::GetRuntimeClassName::<IInertiaProcessor>,
            ::windows::core::GetTrustLevel,
            InitialOriginX::<Impl, OFFSET>,
            SetInitialOriginX::<Impl, OFFSET>,
            InitialOriginY::<Impl, OFFSET>,
            SetInitialOriginY::<Impl, OFFSET>,
            InitialVelocityX::<Impl, OFFSET>,
            SetInitialVelocityX::<Impl, OFFSET>,
            InitialVelocityY::<Impl, OFFSET>,
            SetInitialVelocityY::<Impl, OFFSET>,
            InitialAngularVelocity::<Impl, OFFSET>,
            SetInitialAngularVelocity::<Impl, OFFSET>,
            InitialExpansionVelocity::<Impl, OFFSET>,
            SetInitialExpansionVelocity::<Impl, OFFSET>,
            InitialRadius::<Impl, OFFSET>,
            SetInitialRadius::<Impl, OFFSET>,
            BoundaryLeft::<Impl, OFFSET>,
            SetBoundaryLeft::<Impl, OFFSET>,
            BoundaryTop::<Impl, OFFSET>,
            SetBoundaryTop::<Impl, OFFSET>,
            BoundaryRight::<Impl, OFFSET>,
            SetBoundaryRight::<Impl, OFFSET>,
            BoundaryBottom::<Impl, OFFSET>,
            SetBoundaryBottom::<Impl, OFFSET>,
            ElasticMarginLeft::<Impl, OFFSET>,
            SetElasticMarginLeft::<Impl, OFFSET>,
            ElasticMarginTop::<Impl, OFFSET>,
            SetElasticMarginTop::<Impl, OFFSET>,
            ElasticMarginRight::<Impl, OFFSET>,
            SetElasticMarginRight::<Impl, OFFSET>,
            ElasticMarginBottom::<Impl, OFFSET>,
            SetElasticMarginBottom::<Impl, OFFSET>,
            DesiredDisplacement::<Impl, OFFSET>,
            SetDesiredDisplacement::<Impl, OFFSET>,
            DesiredRotation::<Impl, OFFSET>,
            SetDesiredRotation::<Impl, OFFSET>,
            DesiredExpansion::<Impl, OFFSET>,
            SetDesiredExpansion::<Impl, OFFSET>,
            DesiredDeceleration::<Impl, OFFSET>,
            SetDesiredDeceleration::<Impl, OFFSET>,
            DesiredAngularDeceleration::<Impl, OFFSET>,
            SetDesiredAngularDeceleration::<Impl, OFFSET>,
            DesiredExpansionDeceleration::<Impl, OFFSET>,
            SetDesiredExpansionDeceleration::<Impl, OFFSET>,
            InitialTimestamp::<Impl, OFFSET>,
            SetInitialTimestamp::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            Process::<Impl, OFFSET>,
            ProcessTime::<Impl, OFFSET>,
            Complete::<Impl, OFFSET>,
            CompleteTime::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IManipulationProcessor {
    const NAME: &'static str = "Windows.Win32.UI.Input.Touch.IManipulationProcessor";
}
impl IManipulationProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessorImpl, const OFFSET: isize>() -> IManipulationProcessorVtbl {
        unsafe extern "system" fn SupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedManipulations(::core::mem::transmute_copy(&manipulations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportedManipulations(manipulations) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotPointX(::core::mem::transmute_copy(&pivotpointx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPivotPointX(pivotpointx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotPointY(::core::mem::transmute_copy(&pivotpointy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPivotPointY(pivotpointy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotRadius(::core::mem::transmute_copy(&pivotradius)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPivotRadius(pivotradius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteManipulation<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteManipulation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessDown<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessDown(manipulatorid, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessMove<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessMove(manipulatorid, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessUp<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUp(manipulatorid, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessDownWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessDownWithTime(manipulatorid, x, y, timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessMoveWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessMoveWithTime(manipulatorid, x, y, timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessUpWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUpWithTime(manipulatorid, x, y, timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocityX(::core::mem::transmute_copy(&velocityx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocityY(::core::mem::transmute_copy(&velocityy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExpansionVelocity(::core::mem::transmute_copy(&expansionvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAngularVelocity(::core::mem::transmute_copy(&angularvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumScaleRotateRadius(::core::mem::transmute_copy(&minradius)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinimumScaleRotateRadius(minradius) {
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
            ::windows::core::GetRuntimeClassName::<IManipulationProcessor>,
            ::windows::core::GetTrustLevel,
            SupportedManipulations::<Impl, OFFSET>,
            SetSupportedManipulations::<Impl, OFFSET>,
            PivotPointX::<Impl, OFFSET>,
            SetPivotPointX::<Impl, OFFSET>,
            PivotPointY::<Impl, OFFSET>,
            SetPivotPointY::<Impl, OFFSET>,
            PivotRadius::<Impl, OFFSET>,
            SetPivotRadius::<Impl, OFFSET>,
            CompleteManipulation::<Impl, OFFSET>,
            ProcessDown::<Impl, OFFSET>,
            ProcessMove::<Impl, OFFSET>,
            ProcessUp::<Impl, OFFSET>,
            ProcessDownWithTime::<Impl, OFFSET>,
            ProcessMoveWithTime::<Impl, OFFSET>,
            ProcessUpWithTime::<Impl, OFFSET>,
            GetVelocityX::<Impl, OFFSET>,
            GetVelocityY::<Impl, OFFSET>,
            GetExpansionVelocity::<Impl, OFFSET>,
            GetAngularVelocity::<Impl, OFFSET>,
            MinimumScaleRotateRadius::<Impl, OFFSET>,
            SetMinimumScaleRotateRadius::<Impl, OFFSET>,
        )
    }
}
pub trait _IManipulationEventsImpl: Sized {
    fn ManipulationStarted();
    fn ManipulationDelta();
    fn ManipulationCompleted();
}
impl ::windows::core::RuntimeName for _IManipulationEvents {
    const NAME: &'static str = "Windows.Win32.UI.Input.Touch._IManipulationEvents";
}
impl _IManipulationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEventsImpl, const OFFSET: isize>() -> _IManipulationEventsVtbl {
        unsafe extern "system" fn ManipulationStarted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationDelta<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationDelta(x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IManipulationEvents>, ::windows::core::GetTrustLevel, ManipulationStarted::<Impl, OFFSET>, ManipulationDelta::<Impl, OFFSET>, ManipulationCompleted::<Impl, OFFSET>)
    }
}
