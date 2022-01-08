pub trait IDCompositionAffineTransform2DEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetInterpolationMode();
    fn SetBorderMode();
    fn SetTransformMatrix();
    fn SetTransformMatrixElement();
    fn SetTransformMatrixElement();
    fn SetSharpness();
    fn SetSharpness();
}
impl ::windows::core::RuntimeName for IDCompositionAffineTransform2DEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionAffineTransform2DEffect";
}
impl IDCompositionAffineTransform2DEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>() -> IDCompositionAffineTransform2DEffectVtbl {
        unsafe extern "system" fn SetInterpolationMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterpolationMode(interpolationmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBorderMode(bordermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransformMatrix(&*(&transformmatrix as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransformMatrixElement(row, column, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransformMatrixElement(row, column, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSharpness(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSharpness(sharpness) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionAffineTransform2DEffect>,
            ::windows::core::GetTrustLevel,
            SetInterpolationMode::<Impl, OFFSET>,
            SetBorderMode::<Impl, OFFSET>,
            SetTransformMatrix::<Impl, OFFSET>,
            SetTransformMatrixElement::<Impl, OFFSET>,
            SetTransformMatrixElement::<Impl, OFFSET>,
            SetSharpness::<Impl, OFFSET>,
            SetSharpness::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionAnimationImpl: Sized {
    fn Reset();
    fn SetAbsoluteBeginTime();
    fn AddCubic();
    fn AddSinusoidal();
    fn AddRepeat();
    fn End();
}
impl ::windows::core::RuntimeName for IDCompositionAnimation {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionAnimation";
}
impl IDCompositionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimationImpl, const OFFSET: isize>() -> IDCompositionAnimationVtbl {
        unsafe extern "system" fn Reset<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAbsoluteBeginTime<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAbsoluteBeginTime(begintime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCubic<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddCubic(beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSinusoidal(beginoffset, bias, amplitude, frequency, phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRepeat<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRepeat(beginoffset, durationtorepeat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End(endoffset, endvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionAnimation>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>, SetAbsoluteBeginTime::<Impl, OFFSET>, AddCubic::<Impl, OFFSET>, AddSinusoidal::<Impl, OFFSET>, AddRepeat::<Impl, OFFSET>, End::<Impl, OFFSET>)
    }
}
pub trait IDCompositionArithmeticCompositeEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetCoefficients();
    fn SetClampOutput();
    fn SetCoefficient1();
    fn SetCoefficient1();
    fn SetCoefficient2();
    fn SetCoefficient2();
    fn SetCoefficient3();
    fn SetCoefficient3();
    fn SetCoefficient4();
    fn SetCoefficient4();
}
impl ::windows::core::RuntimeName for IDCompositionArithmeticCompositeEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionArithmeticCompositeEffect";
}
impl IDCompositionArithmeticCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>() -> IDCompositionArithmeticCompositeEffectVtbl {
        unsafe extern "system" fn SetCoefficients<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficients(&*(&coefficients as *const <super::Direct2D::Common::D2D_VECTOR_4F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_4F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClampOutput(&*(&clampoutput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient1(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient1(coeffcient1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient2(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient2(coefficient2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient3(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient3(coefficient3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient4(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoefficient4(coefficient4) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionArithmeticCompositeEffect>,
            ::windows::core::GetTrustLevel,
            SetCoefficients::<Impl, OFFSET>,
            SetClampOutput::<Impl, OFFSET>,
            SetCoefficient1::<Impl, OFFSET>,
            SetCoefficient1::<Impl, OFFSET>,
            SetCoefficient2::<Impl, OFFSET>,
            SetCoefficient2::<Impl, OFFSET>,
            SetCoefficient3::<Impl, OFFSET>,
            SetCoefficient3::<Impl, OFFSET>,
            SetCoefficient4::<Impl, OFFSET>,
            SetCoefficient4::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionBlendEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMode();
}
impl ::windows::core::RuntimeName for IDCompositionBlendEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionBlendEffect";
}
impl IDCompositionBlendEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffectImpl, const OFFSET: isize>() -> IDCompositionBlendEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionBlendEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionBlendEffect>, ::windows::core::GetTrustLevel, SetMode::<Impl, OFFSET>)
    }
}
pub trait IDCompositionBrightnessEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetWhitePoint();
    fn SetBlackPoint();
    fn SetWhitePointX();
    fn SetWhitePointX();
    fn SetWhitePointY();
    fn SetWhitePointY();
    fn SetBlackPointX();
    fn SetBlackPointX();
    fn SetBlackPointY();
    fn SetBlackPointY();
}
impl ::windows::core::RuntimeName for IDCompositionBrightnessEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionBrightnessEffect";
}
impl IDCompositionBrightnessEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>() -> IDCompositionBrightnessEffectVtbl {
        unsafe extern "system" fn SetWhitePoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePoint(&*(&whitepoint as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlackPoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlackPoint(&*(&blackpoint as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointX(whitepointx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointY(whitepointy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlackPointX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlackPointX(blackpointx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlackPointY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlackPointY(blackpointy) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionBrightnessEffect>,
            ::windows::core::GetTrustLevel,
            SetWhitePoint::<Impl, OFFSET>,
            SetBlackPoint::<Impl, OFFSET>,
            SetWhitePointX::<Impl, OFFSET>,
            SetWhitePointX::<Impl, OFFSET>,
            SetWhitePointY::<Impl, OFFSET>,
            SetWhitePointY::<Impl, OFFSET>,
            SetBlackPointX::<Impl, OFFSET>,
            SetBlackPointX::<Impl, OFFSET>,
            SetBlackPointY::<Impl, OFFSET>,
            SetBlackPointY::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionClipImpl: Sized {}
impl ::windows::core::RuntimeName for IDCompositionClip {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionClip";
}
impl IDCompositionClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionClipImpl, const OFFSET: isize>() -> IDCompositionClipVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionClip>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDCompositionColorMatrixEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
    fn SetAlphaMode();
    fn SetClampOutput();
}
impl ::windows::core::RuntimeName for IDCompositionColorMatrixEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionColorMatrixEffect";
}
impl IDCompositionColorMatrixEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>() -> IDCompositionColorMatrixEffectVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrix(&*(&matrix as *const <super::Direct2D::Common::D2D_MATRIX_5X4_F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_MATRIX_5X4_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClampOutput(&*(&clamp as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionColorMatrixEffect>, ::windows::core::GetTrustLevel, SetMatrix::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>, SetAlphaMode::<Impl, OFFSET>, SetClampOutput::<Impl, OFFSET>)
    }
}
pub trait IDCompositionCompositeEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMode();
}
impl ::windows::core::RuntimeName for IDCompositionCompositeEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionCompositeEffect";
}
impl IDCompositionCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffectImpl, const OFFSET: isize>() -> IDCompositionCompositeEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionCompositeEffect>, ::windows::core::GetTrustLevel, SetMode::<Impl, OFFSET>)
    }
}
pub trait IDCompositionDelegatedInkTrailImpl: Sized {
    fn AddTrailPoints();
    fn AddTrailPointsWithPrediction();
    fn RemoveTrailPoints();
    fn StartNewTrail();
}
impl ::windows::core::RuntimeName for IDCompositionDelegatedInkTrail {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDelegatedInkTrail";
}
impl IDCompositionDelegatedInkTrailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>() -> IDCompositionDelegatedInkTrailVtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPoints(&*(&inkpoints as *const <DCompositionInkTrailPoint as ::windows::core::Abi>::Abi as *const <DCompositionInkTrailPoint as ::windows::core::DefaultType>::DefaultType), inkpointscount, ::core::mem::transmute_copy(&generationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPointsWithPrediction(&*(&inkpoints as *const <DCompositionInkTrailPoint as ::windows::core::Abi>::Abi as *const <DCompositionInkTrailPoint as ::windows::core::DefaultType>::DefaultType), inkpointscount, &*(&predictedinkpoints as *const <DCompositionInkTrailPoint as ::windows::core::Abi>::Abi as *const <DCompositionInkTrailPoint as ::windows::core::DefaultType>::DefaultType), predictedinkpointscount, ::core::mem::transmute_copy(&generationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveTrailPoints(generationid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartNewTrail(&*(&color as *const <super::Direct2D::Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionDelegatedInkTrail>, ::windows::core::GetTrustLevel, AddTrailPoints::<Impl, OFFSET>, AddTrailPointsWithPrediction::<Impl, OFFSET>, RemoveTrailPoints::<Impl, OFFSET>, StartNewTrail::<Impl, OFFSET>)
    }
}
pub trait IDCompositionDesktopDeviceImpl: Sized + IDCompositionDevice2Impl {
    fn CreateTargetForHwnd();
    fn CreateSurfaceFromHandle();
    fn CreateSurfaceFromHwnd();
}
impl ::windows::core::RuntimeName for IDCompositionDesktopDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDesktopDevice";
}
impl IDCompositionDesktopDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>() -> IDCompositionDesktopDeviceVtbl {
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&topmost as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(&*(&handle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionDesktopDevice>, ::windows::core::GetTrustLevel, CreateTargetForHwnd::<Impl, OFFSET>, CreateSurfaceFromHandle::<Impl, OFFSET>, CreateSurfaceFromHwnd::<Impl, OFFSET>)
    }
}
pub trait IDCompositionDeviceImpl: Sized {
    fn Commit();
    fn WaitForCommitCompletion();
    fn GetFrameStatistics();
    fn CreateTargetForHwnd();
    fn CreateVisual();
    fn CreateSurface();
    fn CreateVirtualSurface();
    fn CreateSurfaceFromHandle();
    fn CreateSurfaceFromHwnd();
    fn CreateTranslateTransform();
    fn CreateScaleTransform();
    fn CreateRotateTransform();
    fn CreateSkewTransform();
    fn CreateMatrixTransform();
    fn CreateTransformGroup();
    fn CreateTranslateTransform3D();
    fn CreateScaleTransform3D();
    fn CreateRotateTransform3D();
    fn CreateMatrixTransform3D();
    fn CreateTransform3DGroup();
    fn CreateEffectGroup();
    fn CreateRectangleClip();
    fn CreateAnimation();
    fn CheckDeviceState();
}
impl ::windows::core::RuntimeName for IDCompositionDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDevice";
}
impl IDCompositionDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceImpl, const OFFSET: isize>() -> IDCompositionDeviceVtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCommitCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics(::core::mem::transmute_copy(&statistics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&topmost as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual(::core::mem::transmute_copy(&visual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(width, height, pixelformat, alphamode, ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, ::core::mem::transmute_copy(&virtualsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(&*(&handle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform(::core::mem::transmute_copy(&translatetransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform(::core::mem::transmute_copy(&scaletransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform(::core::mem::transmute_copy(&rotatetransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform(::core::mem::transmute_copy(&skewtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform(::core::mem::transmute_copy(&matrixtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(&*(&transforms as *const <IDCompositionTransform as ::windows::core::Abi>::Abi as *const <IDCompositionTransform as ::windows::core::DefaultType>::DefaultType), elements, ::core::mem::transmute_copy(&transformgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D(::core::mem::transmute_copy(&translatetransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D(::core::mem::transmute_copy(&scaletransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D(::core::mem::transmute_copy(&rotatetransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D(::core::mem::transmute_copy(&matrixtransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(&*(&transforms3d as *const <IDCompositionTransform3D as ::windows::core::Abi>::Abi as *const <IDCompositionTransform3D as ::windows::core::DefaultType>::DefaultType), elements, ::core::mem::transmute_copy(&transform3dgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup(::core::mem::transmute_copy(&effectgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip(::core::mem::transmute_copy(&clip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimation(::core::mem::transmute_copy(&animation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckDeviceState(::core::mem::transmute_copy(&pfvalid)) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionDevice>,
            ::windows::core::GetTrustLevel,
            Commit::<Impl, OFFSET>,
            WaitForCommitCompletion::<Impl, OFFSET>,
            GetFrameStatistics::<Impl, OFFSET>,
            CreateTargetForHwnd::<Impl, OFFSET>,
            CreateVisual::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            CreateVirtualSurface::<Impl, OFFSET>,
            CreateSurfaceFromHandle::<Impl, OFFSET>,
            CreateSurfaceFromHwnd::<Impl, OFFSET>,
            CreateTranslateTransform::<Impl, OFFSET>,
            CreateScaleTransform::<Impl, OFFSET>,
            CreateRotateTransform::<Impl, OFFSET>,
            CreateSkewTransform::<Impl, OFFSET>,
            CreateMatrixTransform::<Impl, OFFSET>,
            CreateTransformGroup::<Impl, OFFSET>,
            CreateTranslateTransform3D::<Impl, OFFSET>,
            CreateScaleTransform3D::<Impl, OFFSET>,
            CreateRotateTransform3D::<Impl, OFFSET>,
            CreateMatrixTransform3D::<Impl, OFFSET>,
            CreateTransform3DGroup::<Impl, OFFSET>,
            CreateEffectGroup::<Impl, OFFSET>,
            CreateRectangleClip::<Impl, OFFSET>,
            CreateAnimation::<Impl, OFFSET>,
            CheckDeviceState::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionDevice2Impl: Sized {
    fn Commit();
    fn WaitForCommitCompletion();
    fn GetFrameStatistics();
    fn CreateVisual();
    fn CreateSurfaceFactory();
    fn CreateSurface();
    fn CreateVirtualSurface();
    fn CreateTranslateTransform();
    fn CreateScaleTransform();
    fn CreateRotateTransform();
    fn CreateSkewTransform();
    fn CreateMatrixTransform();
    fn CreateTransformGroup();
    fn CreateTranslateTransform3D();
    fn CreateScaleTransform3D();
    fn CreateRotateTransform3D();
    fn CreateMatrixTransform3D();
    fn CreateTransform3DGroup();
    fn CreateEffectGroup();
    fn CreateRectangleClip();
    fn CreateAnimation();
}
impl ::windows::core::RuntimeName for IDCompositionDevice2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDevice2";
}
impl IDCompositionDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2Impl, const OFFSET: isize>() -> IDCompositionDevice2Vtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCommitCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics(::core::mem::transmute_copy(&statistics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual(::core::mem::transmute_copy(&visual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFactory(&*(&renderingdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surfacefactory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(width, height, pixelformat, alphamode, ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, ::core::mem::transmute_copy(&virtualsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform(::core::mem::transmute_copy(&translatetransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform(::core::mem::transmute_copy(&scaletransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform(::core::mem::transmute_copy(&rotatetransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform(::core::mem::transmute_copy(&skewtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform(::core::mem::transmute_copy(&matrixtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(&*(&transforms as *const <IDCompositionTransform as ::windows::core::Abi>::Abi as *const <IDCompositionTransform as ::windows::core::DefaultType>::DefaultType), elements, ::core::mem::transmute_copy(&transformgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D(::core::mem::transmute_copy(&translatetransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D(::core::mem::transmute_copy(&scaletransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D(::core::mem::transmute_copy(&rotatetransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D(::core::mem::transmute_copy(&matrixtransform3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(&*(&transforms3d as *const <IDCompositionTransform3D as ::windows::core::Abi>::Abi as *const <IDCompositionTransform3D as ::windows::core::DefaultType>::DefaultType), elements, ::core::mem::transmute_copy(&transform3dgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup(::core::mem::transmute_copy(&effectgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip(::core::mem::transmute_copy(&clip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimation(::core::mem::transmute_copy(&animation)) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionDevice2>,
            ::windows::core::GetTrustLevel,
            Commit::<Impl, OFFSET>,
            WaitForCommitCompletion::<Impl, OFFSET>,
            GetFrameStatistics::<Impl, OFFSET>,
            CreateVisual::<Impl, OFFSET>,
            CreateSurfaceFactory::<Impl, OFFSET>,
            CreateSurface::<Impl, OFFSET>,
            CreateVirtualSurface::<Impl, OFFSET>,
            CreateTranslateTransform::<Impl, OFFSET>,
            CreateScaleTransform::<Impl, OFFSET>,
            CreateRotateTransform::<Impl, OFFSET>,
            CreateSkewTransform::<Impl, OFFSET>,
            CreateMatrixTransform::<Impl, OFFSET>,
            CreateTransformGroup::<Impl, OFFSET>,
            CreateTranslateTransform3D::<Impl, OFFSET>,
            CreateScaleTransform3D::<Impl, OFFSET>,
            CreateRotateTransform3D::<Impl, OFFSET>,
            CreateMatrixTransform3D::<Impl, OFFSET>,
            CreateTransform3DGroup::<Impl, OFFSET>,
            CreateEffectGroup::<Impl, OFFSET>,
            CreateRectangleClip::<Impl, OFFSET>,
            CreateAnimation::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionDevice3Impl: Sized + IDCompositionDevice2Impl {
    fn CreateGaussianBlurEffect();
    fn CreateBrightnessEffect();
    fn CreateColorMatrixEffect();
    fn CreateShadowEffect();
    fn CreateHueRotationEffect();
    fn CreateSaturationEffect();
    fn CreateTurbulenceEffect();
    fn CreateLinearTransferEffect();
    fn CreateTableTransferEffect();
    fn CreateCompositeEffect();
    fn CreateBlendEffect();
    fn CreateArithmeticCompositeEffect();
    fn CreateAffineTransform2DEffect();
}
impl ::windows::core::RuntimeName for IDCompositionDevice3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDevice3";
}
impl IDCompositionDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3Impl, const OFFSET: isize>() -> IDCompositionDevice3Vtbl {
        unsafe extern "system" fn CreateGaussianBlurEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGaussianBlurEffect(::core::mem::transmute_copy(&gaussianblureffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBrightnessEffect(::core::mem::transmute_copy(&brightnesseffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorMatrixEffect(::core::mem::transmute_copy(&colormatrixeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShadowEffect(::core::mem::transmute_copy(&shadoweffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHueRotationEffect(::core::mem::transmute_copy(&huerotationeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSaturationEffect(::core::mem::transmute_copy(&saturationeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTurbulenceEffect(::core::mem::transmute_copy(&turbulenceeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransferEffect(::core::mem::transmute_copy(&lineartransfereffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableTransferEffect(::core::mem::transmute_copy(&tabletransfereffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompositeEffect(::core::mem::transmute_copy(&compositeeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendEffect(::core::mem::transmute_copy(&blendeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateArithmeticCompositeEffect(::core::mem::transmute_copy(&arithmeticcompositeeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAffineTransform2DEffect(::core::mem::transmute_copy(&affinetransform2deffect)) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionDevice3>,
            ::windows::core::GetTrustLevel,
            CreateGaussianBlurEffect::<Impl, OFFSET>,
            CreateBrightnessEffect::<Impl, OFFSET>,
            CreateColorMatrixEffect::<Impl, OFFSET>,
            CreateShadowEffect::<Impl, OFFSET>,
            CreateHueRotationEffect::<Impl, OFFSET>,
            CreateSaturationEffect::<Impl, OFFSET>,
            CreateTurbulenceEffect::<Impl, OFFSET>,
            CreateLinearTransferEffect::<Impl, OFFSET>,
            CreateTableTransferEffect::<Impl, OFFSET>,
            CreateCompositeEffect::<Impl, OFFSET>,
            CreateBlendEffect::<Impl, OFFSET>,
            CreateArithmeticCompositeEffect::<Impl, OFFSET>,
            CreateAffineTransform2DEffect::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionDeviceDebugImpl: Sized {
    fn EnableDebugCounters();
    fn DisableDebugCounters();
}
impl ::windows::core::RuntimeName for IDCompositionDeviceDebug {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionDeviceDebug";
}
impl IDCompositionDeviceDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>() -> IDCompositionDeviceDebugVtbl {
        unsafe extern "system" fn EnableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDebugCounters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableDebugCounters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionDeviceDebug>, ::windows::core::GetTrustLevel, EnableDebugCounters::<Impl, OFFSET>, DisableDebugCounters::<Impl, OFFSET>)
    }
}
pub trait IDCompositionEffectImpl: Sized {}
impl ::windows::core::RuntimeName for IDCompositionEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionEffect";
}
impl IDCompositionEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectImpl, const OFFSET: isize>() -> IDCompositionEffectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionEffect>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDCompositionEffectGroupImpl: Sized + IDCompositionEffectImpl {
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform3D();
}
impl ::windows::core::RuntimeName for IDCompositionEffectGroup {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionEffectGroup";
}
impl IDCompositionEffectGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>() -> IDCompositionEffectGroupVtbl {
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpacity(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpacity(opacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform3D<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform3D(&*(&transform3d as *const <IDCompositionTransform3D as ::windows::core::Abi>::Abi as *const <IDCompositionTransform3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionEffectGroup>, ::windows::core::GetTrustLevel, SetOpacity::<Impl, OFFSET>, SetOpacity::<Impl, OFFSET>, SetTransform3D::<Impl, OFFSET>)
    }
}
pub trait IDCompositionFilterEffectImpl: Sized + IDCompositionEffectImpl {
    fn SetInput();
}
impl ::windows::core::RuntimeName for IDCompositionFilterEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionFilterEffect";
}
impl IDCompositionFilterEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffectImpl, const OFFSET: isize>() -> IDCompositionFilterEffectVtbl {
        unsafe extern "system" fn SetInput<Impl: IDCompositionFilterEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInput(index, &*(&input as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionFilterEffect>, ::windows::core::GetTrustLevel, SetInput::<Impl, OFFSET>)
    }
}
pub trait IDCompositionGaussianBlurEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetStandardDeviation();
    fn SetStandardDeviation();
    fn SetBorderMode();
}
impl ::windows::core::RuntimeName for IDCompositionGaussianBlurEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionGaussianBlurEffect";
}
impl IDCompositionGaussianBlurEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>() -> IDCompositionGaussianBlurEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStandardDeviation(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStandardDeviation(amount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBorderMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionGaussianBlurEffect>, ::windows::core::GetTrustLevel, SetStandardDeviation::<Impl, OFFSET>, SetStandardDeviation::<Impl, OFFSET>, SetBorderMode::<Impl, OFFSET>)
    }
}
pub trait IDCompositionHueRotationEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
}
impl ::windows::core::RuntimeName for IDCompositionHueRotationEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionHueRotationEffect";
}
impl IDCompositionHueRotationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>() -> IDCompositionHueRotationEffectVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(amountdegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionHueRotationEffect>, ::windows::core::GetTrustLevel, SetAngle::<Impl, OFFSET>, SetAngle::<Impl, OFFSET>)
    }
}
pub trait IDCompositionInkTrailDeviceImpl: Sized {
    fn CreateDelegatedInkTrail();
    fn CreateDelegatedInkTrailForSwapChain();
}
impl ::windows::core::RuntimeName for IDCompositionInkTrailDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionInkTrailDevice";
}
impl IDCompositionInkTrailDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>() -> IDCompositionInkTrailDeviceVtbl {
        unsafe extern "system" fn CreateDelegatedInkTrail<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDelegatedInkTrail(::core::mem::transmute_copy(&inktrail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDelegatedInkTrailForSwapChain(&*(&swapchain as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inktrail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionInkTrailDevice>, ::windows::core::GetTrustLevel, CreateDelegatedInkTrail::<Impl, OFFSET>, CreateDelegatedInkTrailForSwapChain::<Impl, OFFSET>)
    }
}
pub trait IDCompositionLinearTransferEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetRedYIntercept();
    fn SetRedYIntercept();
    fn SetRedSlope();
    fn SetRedSlope();
    fn SetRedDisable();
    fn SetGreenYIntercept();
    fn SetGreenYIntercept();
    fn SetGreenSlope();
    fn SetGreenSlope();
    fn SetGreenDisable();
    fn SetBlueYIntercept();
    fn SetBlueYIntercept();
    fn SetBlueSlope();
    fn SetBlueSlope();
    fn SetBlueDisable();
    fn SetAlphaYIntercept();
    fn SetAlphaYIntercept();
    fn SetAlphaSlope();
    fn SetAlphaSlope();
    fn SetAlphaDisable();
    fn SetClampOutput();
}
impl ::windows::core::RuntimeName for IDCompositionLinearTransferEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionLinearTransferEffect";
}
impl IDCompositionLinearTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>() -> IDCompositionLinearTransferEffectVtbl {
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedYIntercept(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedYIntercept(redyintercept) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedSlope(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedSlope(redslope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedDisable(&*(&reddisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenYIntercept(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenYIntercept(greenyintercept) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenSlope(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenSlope(greenslope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenDisable(&*(&greendisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueYIntercept(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueYIntercept(blueyintercept) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueSlope(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueSlope(blueslope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueDisable(&*(&bluedisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaYIntercept(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaYIntercept(alphayintercept) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaSlope(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaSlope(alphaslope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaDisable(&*(&alphadisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClampOutput(&*(&clampoutput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionLinearTransferEffect>,
            ::windows::core::GetTrustLevel,
            SetRedYIntercept::<Impl, OFFSET>,
            SetRedYIntercept::<Impl, OFFSET>,
            SetRedSlope::<Impl, OFFSET>,
            SetRedSlope::<Impl, OFFSET>,
            SetRedDisable::<Impl, OFFSET>,
            SetGreenYIntercept::<Impl, OFFSET>,
            SetGreenYIntercept::<Impl, OFFSET>,
            SetGreenSlope::<Impl, OFFSET>,
            SetGreenSlope::<Impl, OFFSET>,
            SetGreenDisable::<Impl, OFFSET>,
            SetBlueYIntercept::<Impl, OFFSET>,
            SetBlueYIntercept::<Impl, OFFSET>,
            SetBlueSlope::<Impl, OFFSET>,
            SetBlueSlope::<Impl, OFFSET>,
            SetBlueDisable::<Impl, OFFSET>,
            SetAlphaYIntercept::<Impl, OFFSET>,
            SetAlphaYIntercept::<Impl, OFFSET>,
            SetAlphaSlope::<Impl, OFFSET>,
            SetAlphaSlope::<Impl, OFFSET>,
            SetAlphaDisable::<Impl, OFFSET>,
            SetClampOutput::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionMatrixTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
impl ::windows::core::RuntimeName for IDCompositionMatrixTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionMatrixTransform";
}
impl IDCompositionMatrixTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>() -> IDCompositionMatrixTransformVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrix(&*(&matrix as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionMatrixTransform>, ::windows::core::GetTrustLevel, SetMatrix::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>)
    }
}
pub trait IDCompositionMatrixTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
impl ::windows::core::RuntimeName for IDCompositionMatrixTransform3D {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionMatrixTransform3D";
}
impl IDCompositionMatrixTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>() -> IDCompositionMatrixTransform3DVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrix(&*(&matrix as *const <super::Direct3D::D3DMATRIX as ::windows::core::Abi>::Abi as *const <super::Direct3D::D3DMATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixElement(row, column, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionMatrixTransform3D>, ::windows::core::GetTrustLevel, SetMatrix::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>, SetMatrixElement::<Impl, OFFSET>)
    }
}
pub trait IDCompositionRectangleClipImpl: Sized + IDCompositionClipImpl {
    fn SetLeft();
    fn SetLeft();
    fn SetTop();
    fn SetTop();
    fn SetRight();
    fn SetRight();
    fn SetBottom();
    fn SetBottom();
    fn SetTopLeftRadiusX();
    fn SetTopLeftRadiusX();
    fn SetTopLeftRadiusY();
    fn SetTopLeftRadiusY();
    fn SetTopRightRadiusX();
    fn SetTopRightRadiusX();
    fn SetTopRightRadiusY();
    fn SetTopRightRadiusY();
    fn SetBottomLeftRadiusX();
    fn SetBottomLeftRadiusX();
    fn SetBottomLeftRadiusY();
    fn SetBottomLeftRadiusY();
    fn SetBottomRightRadiusX();
    fn SetBottomRightRadiusX();
    fn SetBottomRightRadiusY();
    fn SetBottomRightRadiusY();
}
impl ::windows::core::RuntimeName for IDCompositionRectangleClip {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionRectangleClip";
}
impl IDCompositionRectangleClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>() -> IDCompositionRectangleClipVtbl {
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLeft(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLeft(left) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTop(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTop(top) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRight(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRight(right) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottom(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottom(bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopLeftRadiusX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopLeftRadiusX(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopLeftRadiusY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopLeftRadiusY(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopRightRadiusX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopRightRadiusX(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopRightRadiusY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTopRightRadiusY(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomLeftRadiusX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomLeftRadiusX(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomLeftRadiusY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomLeftRadiusY(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomRightRadiusX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomRightRadiusX(radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomRightRadiusY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottomRightRadiusY(radius) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionRectangleClip>,
            ::windows::core::GetTrustLevel,
            SetLeft::<Impl, OFFSET>,
            SetLeft::<Impl, OFFSET>,
            SetTop::<Impl, OFFSET>,
            SetTop::<Impl, OFFSET>,
            SetRight::<Impl, OFFSET>,
            SetRight::<Impl, OFFSET>,
            SetBottom::<Impl, OFFSET>,
            SetBottom::<Impl, OFFSET>,
            SetTopLeftRadiusX::<Impl, OFFSET>,
            SetTopLeftRadiusX::<Impl, OFFSET>,
            SetTopLeftRadiusY::<Impl, OFFSET>,
            SetTopLeftRadiusY::<Impl, OFFSET>,
            SetTopRightRadiusX::<Impl, OFFSET>,
            SetTopRightRadiusX::<Impl, OFFSET>,
            SetTopRightRadiusY::<Impl, OFFSET>,
            SetTopRightRadiusY::<Impl, OFFSET>,
            SetBottomLeftRadiusX::<Impl, OFFSET>,
            SetBottomLeftRadiusX::<Impl, OFFSET>,
            SetBottomLeftRadiusY::<Impl, OFFSET>,
            SetBottomLeftRadiusY::<Impl, OFFSET>,
            SetBottomRightRadiusX::<Impl, OFFSET>,
            SetBottomRightRadiusX::<Impl, OFFSET>,
            SetBottomRightRadiusY::<Impl, OFFSET>,
            SetBottomRightRadiusY::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionRotateTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl ::windows::core::RuntimeName for IDCompositionRotateTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionRotateTransform";
}
impl IDCompositionRotateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>() -> IDCompositionRotateTransformVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(angle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(centerx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(centery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionRotateTransform>, ::windows::core::GetTrustLevel, SetAngle::<Impl, OFFSET>, SetAngle::<Impl, OFFSET>, SetCenterX::<Impl, OFFSET>, SetCenterX::<Impl, OFFSET>, SetCenterY::<Impl, OFFSET>, SetCenterY::<Impl, OFFSET>)
    }
}
pub trait IDCompositionRotateTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
    fn SetAxisX();
    fn SetAxisX();
    fn SetAxisY();
    fn SetAxisY();
    fn SetAxisZ();
    fn SetAxisZ();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
    fn SetCenterZ();
    fn SetCenterZ();
}
impl ::windows::core::RuntimeName for IDCompositionRotateTransform3D {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionRotateTransform3D";
}
impl IDCompositionRotateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>() -> IDCompositionRotateTransform3DVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngle(angle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisX(axisx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisY(axisy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAxisZ(axisz) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(centerx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(centery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterZ(centerz) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionRotateTransform3D>,
            ::windows::core::GetTrustLevel,
            SetAngle::<Impl, OFFSET>,
            SetAngle::<Impl, OFFSET>,
            SetAxisX::<Impl, OFFSET>,
            SetAxisX::<Impl, OFFSET>,
            SetAxisY::<Impl, OFFSET>,
            SetAxisY::<Impl, OFFSET>,
            SetAxisZ::<Impl, OFFSET>,
            SetAxisZ::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterZ::<Impl, OFFSET>,
            SetCenterZ::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionSaturationEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetSaturation();
    fn SetSaturation();
}
impl ::windows::core::RuntimeName for IDCompositionSaturationEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionSaturationEffect";
}
impl IDCompositionSaturationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>() -> IDCompositionSaturationEffectVtbl {
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSaturation(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSaturation(ratio) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionSaturationEffect>, ::windows::core::GetTrustLevel, SetSaturation::<Impl, OFFSET>, SetSaturation::<Impl, OFFSET>)
    }
}
pub trait IDCompositionScaleTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetScaleX();
    fn SetScaleX();
    fn SetScaleY();
    fn SetScaleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl ::windows::core::RuntimeName for IDCompositionScaleTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionScaleTransform";
}
impl IDCompositionScaleTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>() -> IDCompositionScaleTransformVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleX(scalex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleY(scaley) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(centerx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(centery) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionScaleTransform>,
            ::windows::core::GetTrustLevel,
            SetScaleX::<Impl, OFFSET>,
            SetScaleX::<Impl, OFFSET>,
            SetScaleY::<Impl, OFFSET>,
            SetScaleY::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionScaleTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetScaleX();
    fn SetScaleX();
    fn SetScaleY();
    fn SetScaleY();
    fn SetScaleZ();
    fn SetScaleZ();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
    fn SetCenterZ();
    fn SetCenterZ();
}
impl ::windows::core::RuntimeName for IDCompositionScaleTransform3D {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionScaleTransform3D";
}
impl IDCompositionScaleTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>() -> IDCompositionScaleTransform3DVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleX(scalex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleY(scaley) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaleZ(scalez) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(centerx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(centery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterZ(centerz) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionScaleTransform3D>,
            ::windows::core::GetTrustLevel,
            SetScaleX::<Impl, OFFSET>,
            SetScaleX::<Impl, OFFSET>,
            SetScaleY::<Impl, OFFSET>,
            SetScaleY::<Impl, OFFSET>,
            SetScaleZ::<Impl, OFFSET>,
            SetScaleZ::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterZ::<Impl, OFFSET>,
            SetCenterZ::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionShadowEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetStandardDeviation();
    fn SetStandardDeviation();
    fn SetColor();
    fn SetRed();
    fn SetRed();
    fn SetGreen();
    fn SetGreen();
    fn SetBlue();
    fn SetBlue();
    fn SetAlpha();
    fn SetAlpha();
}
impl ::windows::core::RuntimeName for IDCompositionShadowEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionShadowEffect";
}
impl IDCompositionShadowEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>() -> IDCompositionShadowEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStandardDeviation(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStandardDeviation(amount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColor(&*(&color as *const <super::Direct2D::Common::D2D_VECTOR_4F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_4F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRed(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRed(amount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreen(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreen(amount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlue(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlue(amount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlpha(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlpha(amount) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionShadowEffect>,
            ::windows::core::GetTrustLevel,
            SetStandardDeviation::<Impl, OFFSET>,
            SetStandardDeviation::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            SetRed::<Impl, OFFSET>,
            SetRed::<Impl, OFFSET>,
            SetGreen::<Impl, OFFSET>,
            SetGreen::<Impl, OFFSET>,
            SetBlue::<Impl, OFFSET>,
            SetBlue::<Impl, OFFSET>,
            SetAlpha::<Impl, OFFSET>,
            SetAlpha::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionSkewTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngleX();
    fn SetAngleX();
    fn SetAngleY();
    fn SetAngleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl ::windows::core::RuntimeName for IDCompositionSkewTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionSkewTransform";
}
impl IDCompositionSkewTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>() -> IDCompositionSkewTransformVtbl {
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngleX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngleX(anglex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngleY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAngleY(angley) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterX(centerx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCenterY(centery) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionSkewTransform>,
            ::windows::core::GetTrustLevel,
            SetAngleX::<Impl, OFFSET>,
            SetAngleX::<Impl, OFFSET>,
            SetAngleY::<Impl, OFFSET>,
            SetAngleY::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterX::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
            SetCenterY::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionSurfaceImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
    fn Scroll();
}
impl ::windows::core::RuntimeName for IDCompositionSurface {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionSurface";
}
impl IDCompositionSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceImpl, const OFFSET: isize>() -> IDCompositionSurfaceVtbl {
        unsafe extern "system" fn BeginDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: &::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDraw(&*(&updaterect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scroll<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scroll(&*(&scrollrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&cliprect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), offsetx, offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionSurface>, ::windows::core::GetTrustLevel, BeginDraw::<Impl, OFFSET>, EndDraw::<Impl, OFFSET>, SuspendDraw::<Impl, OFFSET>, ResumeDraw::<Impl, OFFSET>, Scroll::<Impl, OFFSET>)
    }
}
pub trait IDCompositionSurfaceFactoryImpl: Sized {
    fn CreateSurface();
    fn CreateVirtualSurface();
}
impl ::windows::core::RuntimeName for IDCompositionSurfaceFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionSurfaceFactory";
}
impl IDCompositionSurfaceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>() -> IDCompositionSurfaceFactoryVtbl {
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(width, height, pixelformat, alphamode, ::core::mem::transmute_copy(&surface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, ::core::mem::transmute_copy(&virtualsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionSurfaceFactory>, ::windows::core::GetTrustLevel, CreateSurface::<Impl, OFFSET>, CreateVirtualSurface::<Impl, OFFSET>)
    }
}
pub trait IDCompositionTableTransferEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetRedTable();
    fn SetGreenTable();
    fn SetBlueTable();
    fn SetAlphaTable();
    fn SetRedDisable();
    fn SetGreenDisable();
    fn SetBlueDisable();
    fn SetAlphaDisable();
    fn SetClampOutput();
    fn SetRedTableValue();
    fn SetRedTableValue();
    fn SetGreenTableValue();
    fn SetGreenTableValue();
    fn SetBlueTableValue();
    fn SetBlueTableValue();
    fn SetAlphaTableValue();
    fn SetAlphaTableValue();
}
impl ::windows::core::RuntimeName for IDCompositionTableTransferEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTableTransferEffect";
}
impl IDCompositionTableTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>() -> IDCompositionTableTransferEffectVtbl {
        unsafe extern "system" fn SetRedTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedTable(tablevalues, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenTable(tablevalues, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueTable(tablevalues, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaTable(tablevalues, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedDisable(&*(&reddisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenDisable(&*(&greendisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueDisable(&*(&bluedisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaDisable(&*(&alphadisable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClampOutput(&*(&clampoutput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedTableValue(index, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRedTableValue(index, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenTableValue(index, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGreenTableValue(index, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueTableValue(index, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlueTableValue(index, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaTableValue(index, &*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlphaTableValue(index, value) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionTableTransferEffect>,
            ::windows::core::GetTrustLevel,
            SetRedTable::<Impl, OFFSET>,
            SetGreenTable::<Impl, OFFSET>,
            SetBlueTable::<Impl, OFFSET>,
            SetAlphaTable::<Impl, OFFSET>,
            SetRedDisable::<Impl, OFFSET>,
            SetGreenDisable::<Impl, OFFSET>,
            SetBlueDisable::<Impl, OFFSET>,
            SetAlphaDisable::<Impl, OFFSET>,
            SetClampOutput::<Impl, OFFSET>,
            SetRedTableValue::<Impl, OFFSET>,
            SetRedTableValue::<Impl, OFFSET>,
            SetGreenTableValue::<Impl, OFFSET>,
            SetGreenTableValue::<Impl, OFFSET>,
            SetBlueTableValue::<Impl, OFFSET>,
            SetBlueTableValue::<Impl, OFFSET>,
            SetAlphaTableValue::<Impl, OFFSET>,
            SetAlphaTableValue::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionTargetImpl: Sized {
    fn SetRoot();
}
impl ::windows::core::RuntimeName for IDCompositionTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTarget";
}
impl IDCompositionTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTargetImpl, const OFFSET: isize>() -> IDCompositionTargetVtbl {
        unsafe extern "system" fn SetRoot<Impl: IDCompositionTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRoot(&*(&visual as *const <IDCompositionVisual as ::windows::core::Abi>::Abi as *const <IDCompositionVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTarget>, ::windows::core::GetTrustLevel, SetRoot::<Impl, OFFSET>)
    }
}
pub trait IDCompositionTransformImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {}
impl ::windows::core::RuntimeName for IDCompositionTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTransform";
}
impl IDCompositionTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransformImpl, const OFFSET: isize>() -> IDCompositionTransformVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTransform>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDCompositionTransform3DImpl: Sized + IDCompositionEffectImpl {}
impl ::windows::core::RuntimeName for IDCompositionTransform3D {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTransform3D";
}
impl IDCompositionTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform3DImpl, const OFFSET: isize>() -> IDCompositionTransform3DVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTransform3D>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDCompositionTranslateTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
}
impl ::windows::core::RuntimeName for IDCompositionTranslateTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTranslateTransform";
}
impl IDCompositionTranslateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>() -> IDCompositionTranslateTransformVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(offsetx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTranslateTransform>, ::windows::core::GetTrustLevel, SetOffsetX::<Impl, OFFSET>, SetOffsetX::<Impl, OFFSET>, SetOffsetY::<Impl, OFFSET>, SetOffsetY::<Impl, OFFSET>)
    }
}
pub trait IDCompositionTranslateTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
    fn SetOffsetZ();
    fn SetOffsetZ();
}
impl ::windows::core::RuntimeName for IDCompositionTranslateTransform3D {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTranslateTransform3D";
}
impl IDCompositionTranslateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>() -> IDCompositionTranslateTransform3DVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(offsetx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetZ(offsetz) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTranslateTransform3D>, ::windows::core::GetTrustLevel, SetOffsetX::<Impl, OFFSET>, SetOffsetX::<Impl, OFFSET>, SetOffsetY::<Impl, OFFSET>, SetOffsetY::<Impl, OFFSET>, SetOffsetZ::<Impl, OFFSET>, SetOffsetZ::<Impl, OFFSET>)
    }
}
pub trait IDCompositionTurbulenceEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetOffset();
    fn SetBaseFrequency();
    fn SetSize();
    fn SetNumOctaves();
    fn SetSeed();
    fn SetNoise();
    fn SetStitchable();
}
impl ::windows::core::RuntimeName for IDCompositionTurbulenceEffect {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionTurbulenceEffect";
}
impl IDCompositionTurbulenceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>() -> IDCompositionTurbulenceEffectVtbl {
        unsafe extern "system" fn SetOffset<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffset(&*(&offset as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseFrequency<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBaseFrequency(&*(&frequency as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(&*(&size as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_VECTOR_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumOctaves<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNumOctaves(numoctaves) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeed<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSeed(seed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoise<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNoise(noise) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStitchable<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStitchable(&*(&stitchable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionTurbulenceEffect>, ::windows::core::GetTrustLevel, SetOffset::<Impl, OFFSET>, SetBaseFrequency::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, SetNumOctaves::<Impl, OFFSET>, SetSeed::<Impl, OFFSET>, SetNoise::<Impl, OFFSET>, SetStitchable::<Impl, OFFSET>)
    }
}
pub trait IDCompositionVirtualSurfaceImpl: Sized + IDCompositionSurfaceImpl {
    fn Resize();
    fn Trim();
}
impl ::windows::core::RuntimeName for IDCompositionVirtualSurface {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionVirtualSurface";
}
impl IDCompositionVirtualSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>() -> IDCompositionVirtualSurfaceVtbl {
        unsafe extern "system" fn Resize<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trim<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trim(&*(&rectangles as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionVirtualSurface>, ::windows::core::GetTrustLevel, Resize::<Impl, OFFSET>, Trim::<Impl, OFFSET>)
    }
}
pub trait IDCompositionVisualImpl: Sized {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
    fn SetTransform();
    fn SetTransform();
    fn SetTransformParent();
    fn SetEffect();
    fn SetBitmapInterpolationMode();
    fn SetBorderMode();
    fn SetClip();
    fn SetClip();
    fn SetContent();
    fn AddVisual();
    fn RemoveVisual();
    fn RemoveAllVisuals();
    fn SetCompositeMode();
}
impl ::windows::core::RuntimeName for IDCompositionVisual {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionVisual";
}
impl IDCompositionVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualImpl, const OFFSET: isize>() -> IDCompositionVisualVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetX(offsetx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetY(offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&transform as *const <IDCompositionTransform as ::windows::core::Abi>::Abi as *const <IDCompositionTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&matrix as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformParent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransformParent(&*(&visual as *const <IDCompositionVisual as ::windows::core::Abi>::Abi as *const <IDCompositionVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffect<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffect(&*(&effect as *const <IDCompositionEffect as ::windows::core::Abi>::Abi as *const <IDCompositionEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBitmapInterpolationMode(interpolationmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBorderMode(bordermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClip(&*(&clip as *const <IDCompositionClip as ::windows::core::Abi>::Abi as *const <IDCompositionClip as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClip(&*(&rect as *const <super::Direct2D::Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&content as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddVisual(
                &*(&visual as *const <IDCompositionVisual as ::windows::core::Abi>::Abi as *const <IDCompositionVisual as ::windows::core::DefaultType>::DefaultType),
                &*(&insertabove as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&referencevisual as *const <IDCompositionVisual as ::windows::core::Abi>::Abi as *const <IDCompositionVisual as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveVisual(&*(&visual as *const <IDCompositionVisual as ::windows::core::Abi>::Abi as *const <IDCompositionVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllVisuals<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAllVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositeMode(compositemode) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionVisual>,
            ::windows::core::GetTrustLevel,
            SetOffsetX::<Impl, OFFSET>,
            SetOffsetX::<Impl, OFFSET>,
            SetOffsetY::<Impl, OFFSET>,
            SetOffsetY::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            SetTransformParent::<Impl, OFFSET>,
            SetEffect::<Impl, OFFSET>,
            SetBitmapInterpolationMode::<Impl, OFFSET>,
            SetBorderMode::<Impl, OFFSET>,
            SetClip::<Impl, OFFSET>,
            SetClip::<Impl, OFFSET>,
            SetContent::<Impl, OFFSET>,
            AddVisual::<Impl, OFFSET>,
            RemoveVisual::<Impl, OFFSET>,
            RemoveAllVisuals::<Impl, OFFSET>,
            SetCompositeMode::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionVisual2Impl: Sized + IDCompositionVisualImpl {
    fn SetOpacityMode();
    fn SetBackFaceVisibility();
}
impl ::windows::core::RuntimeName for IDCompositionVisual2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionVisual2";
}
impl IDCompositionVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2Impl, const OFFSET: isize>() -> IDCompositionVisual2Vtbl {
        unsafe extern "system" fn SetOpacityMode<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpacityMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackFaceVisibility<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackFaceVisibility(visibility) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionVisual2>, ::windows::core::GetTrustLevel, SetOpacityMode::<Impl, OFFSET>, SetBackFaceVisibility::<Impl, OFFSET>)
    }
}
pub trait IDCompositionVisual3Impl: Sized + IDCompositionVisualDebugImpl + IDCompositionVisual2Impl + IDCompositionVisualImpl {
    fn SetDepthMode();
    fn SetOffsetZ();
    fn SetOffsetZ();
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform();
    fn SetTransform();
    fn SetVisible();
}
impl ::windows::core::RuntimeName for IDCompositionVisual3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionVisual3";
}
impl IDCompositionVisual3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3Impl, const OFFSET: isize>() -> IDCompositionVisual3Vtbl {
        unsafe extern "system" fn SetDepthMode<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDepthMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetZ(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOffsetZ(offsetz) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpacity(&*(&animation as *const <IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpacity(opacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&transform as *const <IDCompositionTransform3D as ::windows::core::Abi>::Abi as *const <IDCompositionTransform3D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&matrix as *const <super::Direct2D::Common::D2D_MATRIX_4X4_F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_MATRIX_4X4_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVisible(&*(&visible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IDCompositionVisual3>,
            ::windows::core::GetTrustLevel,
            SetDepthMode::<Impl, OFFSET>,
            SetOffsetZ::<Impl, OFFSET>,
            SetOffsetZ::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            SetVisible::<Impl, OFFSET>,
        )
    }
}
pub trait IDCompositionVisualDebugImpl: Sized + IDCompositionVisual2Impl + IDCompositionVisualImpl {
    fn EnableHeatMap();
    fn DisableHeatMap();
    fn EnableRedrawRegions();
    fn DisableRedrawRegions();
}
impl ::windows::core::RuntimeName for IDCompositionVisualDebug {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectComposition.IDCompositionVisualDebug";
}
impl IDCompositionVisualDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>() -> IDCompositionVisualDebugVtbl {
        unsafe extern "system" fn EnableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableHeatMap(&*(&color as *const <super::Direct2D::Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableHeatMap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableRedrawRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableRedrawRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDCompositionVisualDebug>, ::windows::core::GetTrustLevel, EnableHeatMap::<Impl, OFFSET>, DisableHeatMap::<Impl, OFFSET>, EnableRedrawRegions::<Impl, OFFSET>, DisableRedrawRegions::<Impl, OFFSET>)
    }
}
