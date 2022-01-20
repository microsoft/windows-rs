#[cfg(feature = "Win32_Foundation")]
pub trait IInertiaProcessor_Impl: Sized {
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
impl IInertiaProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>() -> IInertiaProcessor_Vtbl {
        unsafe extern "system" fn InitialOriginX<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialOriginX() {
                ::core::result::Result::Ok(ok__) => {
                    *x = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialOriginX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialOriginY<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialOriginY() {
                ::core::result::Result::Ok(ok__) => {
                    *y = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialOriginY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialVelocityX<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    *x = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVelocityX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialVelocityY<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    *y = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialVelocityY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialAngularVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialAngularVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialExpansionVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *velocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialExpansionVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialRadius<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *radius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialRadius<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialRadius(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn BoundaryLeft<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoundaryLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoundaryLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn BoundaryTop<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoundaryTop() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoundaryTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn BoundaryRight<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoundaryRight() {
                ::core::result::Result::Ok(ok__) => {
                    *right = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoundaryRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn BoundaryBottom<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoundaryBottom() {
                ::core::result::Result::Ok(ok__) => {
                    *bottom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoundaryBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn ElasticMarginLeft<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElasticMarginLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetElasticMarginLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn ElasticMarginTop<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElasticMarginTop() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetElasticMarginTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn ElasticMarginRight<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElasticMarginRight() {
                ::core::result::Result::Ok(ok__) => {
                    *right = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetElasticMarginRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn ElasticMarginBottom<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElasticMarginBottom() {
                ::core::result::Result::Ok(ok__) => {
                    *bottom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetElasticMarginBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *displacement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredDisplacement(::core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn DesiredRotation<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *rotation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn DesiredExpansion<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *expansion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredExpansion(::core::mem::transmute_copy(&expansion)).into()
        }
        unsafe extern "system" fn DesiredDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredAngularDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredAngularDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredExpansionDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *deceleration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredExpansionDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn InitialTimestamp<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitialTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *timestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialTimestamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Process() {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTime<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessTime(::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Complete().into()
        }
        unsafe extern "system" fn CompleteTime<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteTime(::core::mem::transmute_copy(&timestamp)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitialOriginX: InitialOriginX::<Identity, Impl, OFFSET>,
            SetInitialOriginX: SetInitialOriginX::<Identity, Impl, OFFSET>,
            InitialOriginY: InitialOriginY::<Identity, Impl, OFFSET>,
            SetInitialOriginY: SetInitialOriginY::<Identity, Impl, OFFSET>,
            InitialVelocityX: InitialVelocityX::<Identity, Impl, OFFSET>,
            SetInitialVelocityX: SetInitialVelocityX::<Identity, Impl, OFFSET>,
            InitialVelocityY: InitialVelocityY::<Identity, Impl, OFFSET>,
            SetInitialVelocityY: SetInitialVelocityY::<Identity, Impl, OFFSET>,
            InitialAngularVelocity: InitialAngularVelocity::<Identity, Impl, OFFSET>,
            SetInitialAngularVelocity: SetInitialAngularVelocity::<Identity, Impl, OFFSET>,
            InitialExpansionVelocity: InitialExpansionVelocity::<Identity, Impl, OFFSET>,
            SetInitialExpansionVelocity: SetInitialExpansionVelocity::<Identity, Impl, OFFSET>,
            InitialRadius: InitialRadius::<Identity, Impl, OFFSET>,
            SetInitialRadius: SetInitialRadius::<Identity, Impl, OFFSET>,
            BoundaryLeft: BoundaryLeft::<Identity, Impl, OFFSET>,
            SetBoundaryLeft: SetBoundaryLeft::<Identity, Impl, OFFSET>,
            BoundaryTop: BoundaryTop::<Identity, Impl, OFFSET>,
            SetBoundaryTop: SetBoundaryTop::<Identity, Impl, OFFSET>,
            BoundaryRight: BoundaryRight::<Identity, Impl, OFFSET>,
            SetBoundaryRight: SetBoundaryRight::<Identity, Impl, OFFSET>,
            BoundaryBottom: BoundaryBottom::<Identity, Impl, OFFSET>,
            SetBoundaryBottom: SetBoundaryBottom::<Identity, Impl, OFFSET>,
            ElasticMarginLeft: ElasticMarginLeft::<Identity, Impl, OFFSET>,
            SetElasticMarginLeft: SetElasticMarginLeft::<Identity, Impl, OFFSET>,
            ElasticMarginTop: ElasticMarginTop::<Identity, Impl, OFFSET>,
            SetElasticMarginTop: SetElasticMarginTop::<Identity, Impl, OFFSET>,
            ElasticMarginRight: ElasticMarginRight::<Identity, Impl, OFFSET>,
            SetElasticMarginRight: SetElasticMarginRight::<Identity, Impl, OFFSET>,
            ElasticMarginBottom: ElasticMarginBottom::<Identity, Impl, OFFSET>,
            SetElasticMarginBottom: SetElasticMarginBottom::<Identity, Impl, OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Identity, Impl, OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Identity, Impl, OFFSET>,
            DesiredRotation: DesiredRotation::<Identity, Impl, OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Identity, Impl, OFFSET>,
            DesiredExpansion: DesiredExpansion::<Identity, Impl, OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Identity, Impl, OFFSET>,
            DesiredDeceleration: DesiredDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, Impl, OFFSET>,
            DesiredAngularDeceleration: DesiredAngularDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredAngularDeceleration: SetDesiredAngularDeceleration::<Identity, Impl, OFFSET>,
            DesiredExpansionDeceleration: DesiredExpansionDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredExpansionDeceleration: SetDesiredExpansionDeceleration::<Identity, Impl, OFFSET>,
            InitialTimestamp: InitialTimestamp::<Identity, Impl, OFFSET>,
            SetInitialTimestamp: SetInitialTimestamp::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            ProcessTime: ProcessTime::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
            CompleteTime: CompleteTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInertiaProcessor as ::windows::core::Interface>::IID
    }
}
pub trait IManipulationProcessor_Impl: Sized {
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
impl IManipulationProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>() -> IManipulationProcessor_Vtbl {
        unsafe extern "system" fn SupportedManipulations<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedManipulations() {
                ::core::result::Result::Ok(ok__) => {
                    *manipulations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportedManipulations(::core::mem::transmute_copy(&manipulations)).into()
        }
        unsafe extern "system" fn PivotPointX<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PivotPointX() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotpointx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointX<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPivotPointX(::core::mem::transmute_copy(&pivotpointx)).into()
        }
        unsafe extern "system" fn PivotPointY<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PivotPointY() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotpointy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointY<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPivotPointY(::core::mem::transmute_copy(&pivotpointy)).into()
        }
        unsafe extern "system" fn PivotRadius<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PivotRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *pivotradius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPivotRadius(::core::mem::transmute_copy(&pivotradius)).into()
        }
        unsafe extern "system" fn CompleteManipulation<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteManipulation().into()
        }
        unsafe extern "system" fn ProcessDown<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessDown(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessMove<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessMove(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessUp<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessUp(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessDownWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessMoveWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessUpWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn GetVelocityX<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    *velocityx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityY<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    *velocityy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *expansionvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *angularvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinimumScaleRotateRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *minradius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumScaleRotateRadius(::core::mem::transmute_copy(&minradius)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SupportedManipulations: SupportedManipulations::<Identity, Impl, OFFSET>,
            SetSupportedManipulations: SetSupportedManipulations::<Identity, Impl, OFFSET>,
            PivotPointX: PivotPointX::<Identity, Impl, OFFSET>,
            SetPivotPointX: SetPivotPointX::<Identity, Impl, OFFSET>,
            PivotPointY: PivotPointY::<Identity, Impl, OFFSET>,
            SetPivotPointY: SetPivotPointY::<Identity, Impl, OFFSET>,
            PivotRadius: PivotRadius::<Identity, Impl, OFFSET>,
            SetPivotRadius: SetPivotRadius::<Identity, Impl, OFFSET>,
            CompleteManipulation: CompleteManipulation::<Identity, Impl, OFFSET>,
            ProcessDown: ProcessDown::<Identity, Impl, OFFSET>,
            ProcessMove: ProcessMove::<Identity, Impl, OFFSET>,
            ProcessUp: ProcessUp::<Identity, Impl, OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Identity, Impl, OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Identity, Impl, OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Identity, Impl, OFFSET>,
            GetVelocityX: GetVelocityX::<Identity, Impl, OFFSET>,
            GetVelocityY: GetVelocityY::<Identity, Impl, OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Identity, Impl, OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Identity, Impl, OFFSET>,
            MinimumScaleRotateRadius: MinimumScaleRotateRadius::<Identity, Impl, OFFSET>,
            SetMinimumScaleRotateRadius: SetMinimumScaleRotateRadius::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationProcessor as ::windows::core::Interface>::IID
    }
}
pub trait _IManipulationEvents_Impl: Sized {
    fn ManipulationStarted(&mut self, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ManipulationDelta(&mut self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&mut self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()>;
}
impl _IManipulationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEvents_Impl, const OFFSET: isize>() -> _IManipulationEvents_Vtbl {
        unsafe extern "system" fn ManipulationStarted<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ManipulationStarted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn ManipulationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ManipulationCompleted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cumulativetranslationx), ::core::mem::transmute_copy(&cumulativetranslationy), ::core::mem::transmute_copy(&cumulativescale), ::core::mem::transmute_copy(&cumulativeexpansion), ::core::mem::transmute_copy(&cumulativerotation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Identity, Impl, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, Impl, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IManipulationEvents as ::windows::core::Interface>::IID
    }
}
