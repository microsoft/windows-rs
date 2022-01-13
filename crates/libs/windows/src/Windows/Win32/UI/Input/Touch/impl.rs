#[cfg(feature = "Win32_Foundation")]
pub trait IInertiaProcessorImpl: Sized {
    fn InitialOriginX(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialOriginX(&mut self, x: f32) -> ::windows::core::Result<()>;
    fn InitialOriginY(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialOriginY(&mut self, y: f32) -> ::windows::core::Result<()>;
    fn InitialVelocityX(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialVelocityX(&mut self, x: f32) -> ::windows::core::Result<()>;
    fn InitialVelocityY(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialVelocityY(&mut self, y: f32) -> ::windows::core::Result<()>;
    fn InitialAngularVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialAngularVelocity(&mut self, velocity: f32) -> ::windows::core::Result<()>;
    fn InitialExpansionVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialExpansionVelocity(&mut self, velocity: f32) -> ::windows::core::Result<()>;
    fn InitialRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialRadius(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn BoundaryLeft(&mut self) -> ::windows::core::Result<f32>;
    fn SetBoundaryLeft(&mut self, left: f32) -> ::windows::core::Result<()>;
    fn BoundaryTop(&mut self) -> ::windows::core::Result<f32>;
    fn SetBoundaryTop(&mut self, top: f32) -> ::windows::core::Result<()>;
    fn BoundaryRight(&mut self) -> ::windows::core::Result<f32>;
    fn SetBoundaryRight(&mut self, right: f32) -> ::windows::core::Result<()>;
    fn BoundaryBottom(&mut self) -> ::windows::core::Result<f32>;
    fn SetBoundaryBottom(&mut self, bottom: f32) -> ::windows::core::Result<()>;
    fn ElasticMarginLeft(&mut self) -> ::windows::core::Result<f32>;
    fn SetElasticMarginLeft(&mut self, left: f32) -> ::windows::core::Result<()>;
    fn ElasticMarginTop(&mut self) -> ::windows::core::Result<f32>;
    fn SetElasticMarginTop(&mut self, top: f32) -> ::windows::core::Result<()>;
    fn ElasticMarginRight(&mut self) -> ::windows::core::Result<f32>;
    fn SetElasticMarginRight(&mut self, right: f32) -> ::windows::core::Result<()>;
    fn ElasticMarginBottom(&mut self) -> ::windows::core::Result<f32>;
    fn SetElasticMarginBottom(&mut self, bottom: f32) -> ::windows::core::Result<()>;
    fn DesiredDisplacement(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredDisplacement(&mut self, displacement: f32) -> ::windows::core::Result<()>;
    fn DesiredRotation(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredRotation(&mut self, rotation: f32) -> ::windows::core::Result<()>;
    fn DesiredExpansion(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredExpansion(&mut self, expansion: f32) -> ::windows::core::Result<()>;
    fn DesiredDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredDeceleration(&mut self, deceleration: f32) -> ::windows::core::Result<()>;
    fn DesiredAngularDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredAngularDeceleration(&mut self, deceleration: f32) -> ::windows::core::Result<()>;
    fn DesiredExpansionDeceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetDesiredExpansionDeceleration(&mut self, deceleration: f32) -> ::windows::core::Result<()>;
    fn InitialTimestamp(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialTimestamp(&mut self, timestamp: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Process(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn ProcessTime(&mut self, timestamp: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
    fn CompleteTime(&mut self, timestamp: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInertiaProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaProcessorVtbl {
        unsafe extern "system" fn InitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialOriginX() {
                ::core::result::Result::Ok(ok__) => {
                    *x = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialOriginX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialOriginY() {
                ::core::result::Result::Ok(ok__) => {
                    *y = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialOriginY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    *x = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocityX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    *y = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocityY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialAngularVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialExpansionVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *radius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialRadius<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialRadius(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn BoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundaryLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn BoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryTop() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundaryTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn BoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryRight() {
                ::core::result::Result::Ok(ok__) => {
                    *right = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundaryRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn BoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundaryBottom() {
                ::core::result::Result::Ok(ok__) => {
                    *bottom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundaryBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn ElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElasticMarginLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn ElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginTop() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElasticMarginTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn ElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginRight() {
                ::core::result::Result::Ok(ok__) => {
                    *right = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElasticMarginRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn ElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElasticMarginBottom() {
                ::core::result::Result::Ok(ok__) => {
                    *bottom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElasticMarginBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *displacement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDisplacement(::core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *rotation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *expansion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredExpansion(::core::mem::transmute_copy(&expansion)).into()
        }
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAngularDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAngularDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredExpansionDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredExpansionDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn InitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *timestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialTimestamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn Reset<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Process<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process() {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessTime(::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        unsafe extern "system" fn CompleteTime<Impl: IInertiaProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteTime(::core::mem::transmute_copy(&timestamp)).into()
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
    fn SupportedManipulations(&mut self) -> ::windows::core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn SetSupportedManipulations(&mut self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::Result<()>;
    fn PivotPointX(&mut self) -> ::windows::core::Result<f32>;
    fn SetPivotPointX(&mut self, pivotpointx: f32) -> ::windows::core::Result<()>;
    fn PivotPointY(&mut self) -> ::windows::core::Result<f32>;
    fn SetPivotPointY(&mut self, pivotpointy: f32) -> ::windows::core::Result<()>;
    fn PivotRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetPivotRadius(&mut self, pivotradius: f32) -> ::windows::core::Result<()>;
    fn CompleteManipulation(&mut self) -> ::windows::core::Result<()>;
    fn ProcessDown(&mut self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ProcessMove(&mut self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ProcessUp(&mut self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ProcessDownWithTime(&mut self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()>;
    fn ProcessMoveWithTime(&mut self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()>;
    fn ProcessUpWithTime(&mut self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()>;
    fn GetVelocityX(&mut self) -> ::windows::core::Result<f32>;
    fn GetVelocityY(&mut self) -> ::windows::core::Result<f32>;
    fn GetExpansionVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn GetAngularVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn MinimumScaleRotateRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetMinimumScaleRotateRadius(&mut self, minradius: f32) -> ::windows::core::Result<()>;
}
impl IManipulationProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationProcessorVtbl {
        unsafe extern "system" fn SupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedManipulations() {
                ::core::result::Result::Ok(ok__) => {
                    *manipulations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportedManipulations(::core::mem::transmute_copy(&manipulations)).into()
        }
        unsafe extern "system" fn PivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotPointX() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotpointx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivotPointX(::core::mem::transmute_copy(&pivotpointx)).into()
        }
        unsafe extern "system" fn PivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotPointY() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotpointy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivotPointY(::core::mem::transmute_copy(&pivotpointy)).into()
        }
        unsafe extern "system" fn PivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PivotRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotradius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivotRadius(::core::mem::transmute_copy(&pivotradius)).into()
        }
        unsafe extern "system" fn CompleteManipulation<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteManipulation().into()
        }
        unsafe extern "system" fn ProcessDown<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessDown(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessMove<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessMove(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessUp<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUp(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessDownWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessDownWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessMoveWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessMoveWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessUpWithTime<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUpWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn GetVelocityX<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    *velocityx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityY<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    *velocityy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *expansionvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *angularvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumScaleRotateRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *minradius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Impl: IManipulationProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumScaleRotateRadius(::core::mem::transmute_copy(&minradius)).into()
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
    fn ManipulationStarted(&mut self, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ManipulationDelta(&mut self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&mut self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()>;
}
impl _IManipulationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IManipulationEventsVtbl {
        unsafe extern "system" fn ManipulationStarted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ManipulationStarted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ManipulationDelta(
                    ::core::mem::transmute_copy(&x),
                    ::core::mem::transmute_copy(&y),
                    ::core::mem::transmute_copy(&translationdeltax),
                    ::core::mem::transmute_copy(&translationdeltay),
                    ::core::mem::transmute_copy(&scaledelta),
                    ::core::mem::transmute_copy(&expansiondelta),
                    ::core::mem::transmute_copy(&rotationdelta),
                    ::core::mem::transmute_copy(&cumulativetranslationx),
                    ::core::mem::transmute_copy(&cumulativetranslationy),
                    ::core::mem::transmute_copy(&cumulativescale),
                    ::core::mem::transmute_copy(&cumulativeexpansion),
                    ::core::mem::transmute_copy(&cumulativerotation),
                )
                .into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: _IManipulationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ManipulationCompleted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cumulativetranslationx), ::core::mem::transmute_copy(&cumulativetranslationy), ::core::mem::transmute_copy(&cumulativescale), ::core::mem::transmute_copy(&cumulativeexpansion), ::core::mem::transmute_copy(&cumulativerotation)).into()
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
