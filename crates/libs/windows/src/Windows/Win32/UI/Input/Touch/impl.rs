#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInertiaProcessor_Impl: Sized {
    fn InitialOriginX(&self) -> ::windows_core::Result<f32>;
    fn SetInitialOriginX(&self, x: f32) -> ::windows_core::Result<()>;
    fn InitialOriginY(&self) -> ::windows_core::Result<f32>;
    fn SetInitialOriginY(&self, y: f32) -> ::windows_core::Result<()>;
    fn InitialVelocityX(&self) -> ::windows_core::Result<f32>;
    fn SetInitialVelocityX(&self, x: f32) -> ::windows_core::Result<()>;
    fn InitialVelocityY(&self) -> ::windows_core::Result<f32>;
    fn SetInitialVelocityY(&self, y: f32) -> ::windows_core::Result<()>;
    fn InitialAngularVelocity(&self) -> ::windows_core::Result<f32>;
    fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows_core::Result<()>;
    fn InitialExpansionVelocity(&self) -> ::windows_core::Result<f32>;
    fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows_core::Result<()>;
    fn InitialRadius(&self) -> ::windows_core::Result<f32>;
    fn SetInitialRadius(&self, radius: f32) -> ::windows_core::Result<()>;
    fn BoundaryLeft(&self) -> ::windows_core::Result<f32>;
    fn SetBoundaryLeft(&self, left: f32) -> ::windows_core::Result<()>;
    fn BoundaryTop(&self) -> ::windows_core::Result<f32>;
    fn SetBoundaryTop(&self, top: f32) -> ::windows_core::Result<()>;
    fn BoundaryRight(&self) -> ::windows_core::Result<f32>;
    fn SetBoundaryRight(&self, right: f32) -> ::windows_core::Result<()>;
    fn BoundaryBottom(&self) -> ::windows_core::Result<f32>;
    fn SetBoundaryBottom(&self, bottom: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginLeft(&self) -> ::windows_core::Result<f32>;
    fn SetElasticMarginLeft(&self, left: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginTop(&self) -> ::windows_core::Result<f32>;
    fn SetElasticMarginTop(&self, top: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginRight(&self) -> ::windows_core::Result<f32>;
    fn SetElasticMarginRight(&self, right: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginBottom(&self) -> ::windows_core::Result<f32>;
    fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows_core::Result<()>;
    fn DesiredDisplacement(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows_core::Result<()>;
    fn DesiredRotation(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredRotation(&self, rotation: f32) -> ::windows_core::Result<()>;
    fn DesiredExpansion(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredExpansion(&self, expansion: f32) -> ::windows_core::Result<()>;
    fn DesiredDeceleration(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows_core::Result<()>;
    fn DesiredAngularDeceleration(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows_core::Result<()>;
    fn DesiredExpansionDeceleration(&self) -> ::windows_core::Result<f32>;
    fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows_core::Result<()>;
    fn InitialTimestamp(&self) -> ::windows_core::Result<u32>;
    fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Process(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn ProcessTime(&self, timestamp: u32) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn Complete(&self) -> ::windows_core::Result<()>;
    fn CompleteTime(&self, timestamp: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IInertiaProcessor {}
#[cfg(feature = "Win32_Foundation")]
impl IInertiaProcessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>() -> IInertiaProcessor_Vtbl {
        unsafe extern "system" fn InitialOriginX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialOriginX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(x, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialOriginX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialOriginY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialOriginY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(y, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialOriginY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialVelocityX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(x, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialVelocityX(::core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialVelocityY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(y, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialVelocityY(::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialAngularVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialAngularVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialExpansionVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialExpansionVelocity(::core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialRadius() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialRadius(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn BoundaryLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundaryLeft() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoundaryLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn BoundaryTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundaryTop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoundaryTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn BoundaryRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundaryRight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoundaryRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn BoundaryBottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoundaryBottom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBoundaryBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn ElasticMarginLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElasticMarginLeft() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetElasticMarginLeft(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn ElasticMarginTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElasticMarginTop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetElasticMarginTop(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn ElasticMarginRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElasticMarginRight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetElasticMarginRight(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn ElasticMarginBottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElasticMarginBottom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetElasticMarginBottom(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displacement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredDisplacement(::core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn DesiredRotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredRotation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn DesiredExpansion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expansion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredExpansion(::core::mem::transmute_copy(&expansion)).into()
        }
        unsafe extern "system" fn DesiredDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredAngularDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredAngularDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredExpansionDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredExpansionDeceleration(::core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn InitialTimestamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialTimestamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Process<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Process() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessTime(::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Complete().into()
        }
        unsafe extern "system" fn CompleteTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteTime(::core::mem::transmute_copy(&timestamp)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IInertiaProcessor as ::windows_core::ComInterface>::IID
    }
}
pub trait IManipulationProcessor_Impl: Sized {
    fn SupportedManipulations(&self) -> ::windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::Result<()>;
    fn PivotPointX(&self) -> ::windows_core::Result<f32>;
    fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows_core::Result<()>;
    fn PivotPointY(&self) -> ::windows_core::Result<f32>;
    fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows_core::Result<()>;
    fn PivotRadius(&self) -> ::windows_core::Result<f32>;
    fn SetPivotRadius(&self, pivotradius: f32) -> ::windows_core::Result<()>;
    fn CompleteManipulation(&self) -> ::windows_core::Result<()>;
    fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn GetVelocityX(&self) -> ::windows_core::Result<f32>;
    fn GetVelocityY(&self) -> ::windows_core::Result<f32>;
    fn GetExpansionVelocity(&self) -> ::windows_core::Result<f32>;
    fn GetAngularVelocity(&self) -> ::windows_core::Result<f32>;
    fn MinimumScaleRotateRadius(&self) -> ::windows_core::Result<f32>;
    fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IManipulationProcessor {}
impl IManipulationProcessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>() -> IManipulationProcessor_Vtbl {
        unsafe extern "system" fn SupportedManipulations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedManipulations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manipulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportedManipulations(::core::mem::transmute_copy(&manipulations)).into()
        }
        unsafe extern "system" fn PivotPointX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PivotPointX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotpointx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPivotPointX(::core::mem::transmute_copy(&pivotpointx)).into()
        }
        unsafe extern "system" fn PivotPointY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PivotPointY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotpointy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPivotPointY(::core::mem::transmute_copy(&pivotpointy)).into()
        }
        unsafe extern "system" fn PivotRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PivotRadius() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotradius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPivotRadius(::core::mem::transmute_copy(&pivotradius)).into()
        }
        unsafe extern "system" fn CompleteManipulation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteManipulation().into()
        }
        unsafe extern "system" fn ProcessDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessDown(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessMove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessMove(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUp(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessDownWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessMoveWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUpWithTime(::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn GetVelocityX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVelocityX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocityx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityY<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVelocityY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocityy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExpansionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expansionvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAngularVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(angularvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinimumScaleRotateRadius() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minradius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinimumScaleRotateRadius(::core::mem::transmute_copy(&minradius)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IManipulationProcessor as ::windows_core::ComInterface>::IID
    }
}
pub trait _IManipulationEvents_Impl: Sized {
    fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::Result<()>;
    fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for _IManipulationEvents {}
impl _IManipulationEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: isize>() -> _IManipulationEvents_Vtbl {
        unsafe extern "system" fn ManipulationStarted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ManipulationStarted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ManipulationDelta(
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
        unsafe extern "system" fn ManipulationCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ManipulationCompleted(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cumulativetranslationx), ::core::mem::transmute_copy(&cumulativetranslationy), ::core::mem::transmute_copy(&cumulativescale), ::core::mem::transmute_copy(&cumulativeexpansion), ::core::mem::transmute_copy(&cumulativerotation)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Identity, Impl, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, Impl, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <_IManipulationEvents as ::windows_core::ComInterface>::IID
    }
}
