#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransform3DImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterZ(&self) -> ::windows::core::Result<f64>;
    fn SetCenterZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationX(&self) -> ::windows::core::Result<f64>;
    fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationY(&self) -> ::windows::core::Result<f64>;
    fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleZ(&self) -> ::windows::core::Result<f64>;
    fn SetScaleZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateX(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateY(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateZ(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateZ(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.ICompositeTransform3D";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositeTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositeTransform3DVtbl {
        unsafe extern "system" fn CenterX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(value).into()
        }
        unsafe extern "system" fn CenterY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(value).into()
        }
        unsafe extern "system" fn CenterZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ(value).into()
        }
        unsafe extern "system" fn RotationX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationX(value).into()
        }
        unsafe extern "system" fn RotationY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationY(value).into()
        }
        unsafe extern "system" fn RotationZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationZ(value).into()
        }
        unsafe extern "system" fn ScaleX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(value).into()
        }
        unsafe extern "system" fn ScaleY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(value).into()
        }
        unsafe extern "system" fn ScaleZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleZ(value).into()
        }
        unsafe extern "system" fn TranslateX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateX<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslateX(value).into()
        }
        unsafe extern "system" fn TranslateY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateY<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslateY(value).into()
        }
        unsafe extern "system" fn TranslateZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateZ() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslateZ<Impl: ICompositeTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslateZ(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICompositeTransform3D>,
            ::windows::core::GetTrustLevel,
            CenterX::<Impl, IMPL_OFFSET>,
            SetCenterX::<Impl, IMPL_OFFSET>,
            CenterY::<Impl, IMPL_OFFSET>,
            SetCenterY::<Impl, IMPL_OFFSET>,
            CenterZ::<Impl, IMPL_OFFSET>,
            SetCenterZ::<Impl, IMPL_OFFSET>,
            RotationX::<Impl, IMPL_OFFSET>,
            SetRotationX::<Impl, IMPL_OFFSET>,
            RotationY::<Impl, IMPL_OFFSET>,
            SetRotationY::<Impl, IMPL_OFFSET>,
            RotationZ::<Impl, IMPL_OFFSET>,
            SetRotationZ::<Impl, IMPL_OFFSET>,
            ScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX::<Impl, IMPL_OFFSET>,
            ScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY::<Impl, IMPL_OFFSET>,
            ScaleZ::<Impl, IMPL_OFFSET>,
            SetScaleZ::<Impl, IMPL_OFFSET>,
            TranslateX::<Impl, IMPL_OFFSET>,
            SetTranslateX::<Impl, IMPL_OFFSET>,
            TranslateY::<Impl, IMPL_OFFSET>,
            SetTranslateY::<Impl, IMPL_OFFSET>,
            TranslateZ::<Impl, IMPL_OFFSET>,
            SetTranslateZ::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositeTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransform3DStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CenterZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeTransform3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.ICompositeTransform3DStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeTransform3DStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositeTransform3DStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositeTransform3DStaticsVtbl {
        unsafe extern "system" fn CenterXProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterYProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterZProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationXProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationYProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationZProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleXProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleYProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleZProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateXProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateYProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateZProperty<Impl: ICompositeTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateZProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICompositeTransform3DStatics>,
            ::windows::core::GetTrustLevel,
            CenterXProperty::<Impl, IMPL_OFFSET>,
            CenterYProperty::<Impl, IMPL_OFFSET>,
            CenterZProperty::<Impl, IMPL_OFFSET>,
            RotationXProperty::<Impl, IMPL_OFFSET>,
            RotationYProperty::<Impl, IMPL_OFFSET>,
            RotationZProperty::<Impl, IMPL_OFFSET>,
            ScaleXProperty::<Impl, IMPL_OFFSET>,
            ScaleYProperty::<Impl, IMPL_OFFSET>,
            ScaleZProperty::<Impl, IMPL_OFFSET>,
            TranslateXProperty::<Impl, IMPL_OFFSET>,
            TranslateYProperty::<Impl, IMPL_OFFSET>,
            TranslateZProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositeTransform3DStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrix3DHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.IMatrix3DHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrix3DHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrix3DHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrix3DHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMatrix3DHelper>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrix3DHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DHelperStaticsImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<Matrix3D>;
    fn Multiply(&self, matrix1: &Matrix3D, matrix2: &Matrix3D) -> ::windows::core::Result<Matrix3D>;
    fn FromElements(&self, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64) -> ::windows::core::Result<Matrix3D>;
    fn GetHasInverse(&self, target: &Matrix3D) -> ::windows::core::Result<bool>;
    fn GetIsIdentity(&self, target: &Matrix3D) -> ::windows::core::Result<bool>;
    fn Invert(&self, target: &Matrix3D) -> ::windows::core::Result<Matrix3D>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMatrix3DHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.IMatrix3DHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMatrix3DHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMatrix3DHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMatrix3DHelperStaticsVtbl {
        unsafe extern "system" fn Identity<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Multiply<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix1: Matrix3D, matrix2: Matrix3D, result__: *mut Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Multiply(&*(&matrix1 as *const <Matrix3D as ::windows::core::Abi>::Abi as *const <Matrix3D as ::windows::core::DefaultType>::DefaultType), &*(&matrix2 as *const <Matrix3D as ::windows::core::Abi>::Abi as *const <Matrix3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromElements<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64, result__: *mut Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromElements(m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, offsetx, offsety, offsetz, m44) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasInverse<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasInverse(&*(&target as *const <Matrix3D as ::windows::core::Abi>::Abi as *const <Matrix3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsIdentity<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsIdentity(&*(&target as *const <Matrix3D as ::windows::core::Abi>::Abi as *const <Matrix3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invert<Impl: IMatrix3DHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invert(&*(&target as *const <Matrix3D as ::windows::core::Abi>::Abi as *const <Matrix3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMatrix3DHelperStatics>, ::windows::core::GetTrustLevel, Identity::<Impl, IMPL_OFFSET>, Multiply::<Impl, IMPL_OFFSET>, FromElements::<Impl, IMPL_OFFSET>, GetHasInverse::<Impl, IMPL_OFFSET>, GetIsIdentity::<Impl, IMPL_OFFSET>, Invert::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMatrix3DHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerspectiveTransform3DImpl: Sized {
    fn Depth(&self) -> ::windows::core::Result<f64>;
    fn SetDepth(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerspectiveTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.IPerspectiveTransform3D";
}
#[cfg(feature = "implement_exclusive")]
impl IPerspectiveTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerspectiveTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerspectiveTransform3DVtbl {
        unsafe extern "system" fn Depth<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Depth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepth<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepth(value).into()
        }
        unsafe extern "system" fn OffsetX<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetX<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(value).into()
        }
        unsafe extern "system" fn OffsetY<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IPerspectiveTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPerspectiveTransform3D>, ::windows::core::GetTrustLevel, Depth::<Impl, IMPL_OFFSET>, SetDepth::<Impl, IMPL_OFFSET>, OffsetX::<Impl, IMPL_OFFSET>, SetOffsetX::<Impl, IMPL_OFFSET>, OffsetY::<Impl, IMPL_OFFSET>, SetOffsetY::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerspectiveTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerspectiveTransform3DStaticsImpl: Sized {
    fn DepthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPerspectiveTransform3DStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.IPerspectiveTransform3DStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPerspectiveTransform3DStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerspectiveTransform3DStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerspectiveTransform3DStaticsVtbl {
        unsafe extern "system" fn DepthProperty<Impl: IPerspectiveTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetXProperty<Impl: IPerspectiveTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetXProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetYProperty<Impl: IPerspectiveTransform3DStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetYProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPerspectiveTransform3DStatics>, ::windows::core::GetTrustLevel, DepthProperty::<Impl, IMPL_OFFSET>, OffsetXProperty::<Impl, IMPL_OFFSET>, OffsetYProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerspectiveTransform3DStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransform3DImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.ITransform3D";
}
#[cfg(feature = "implement_exclusive")]
impl ITransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransform3DVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransform3D>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransform3DFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Transform3D>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransform3DFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.ITransform3DFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITransform3DFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransform3DFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransform3DFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransform3DFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransform3DFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransform3DFactory as ::windows::core::Interface>::IID
    }
}
