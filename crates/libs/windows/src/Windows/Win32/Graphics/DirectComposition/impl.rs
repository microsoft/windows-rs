#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetInterpolationMode();
    fn SetBorderMode();
    fn SetTransformMatrix();
    fn SetTransformMatrixElement();
    fn SetTransformMatrixElement();
    fn SetSharpness();
    fn SetSharpness();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionAffineTransform2DEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAffineTransform2DEffectVtbl {
        unsafe extern "system" fn SetInterpolationMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetInterpolationMode: SetInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Impl, IMPL_OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Impl, IMPL_OFFSET>,
            SetSharpness: SetSharpness::<Impl, IMPL_OFFSET>,
            SetSharpness: SetSharpness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionAffineTransform2DEffect as ::windows::core::Interface>::IID
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
impl IDCompositionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAnimationVtbl {
        unsafe extern "system" fn Reset<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCubic<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRepeat<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Impl, IMPL_OFFSET>,
            AddCubic: AddCubic::<Impl, IMPL_OFFSET>,
            AddSinusoidal: AddSinusoidal::<Impl, IMPL_OFFSET>,
            AddRepeat: AddRepeat::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionArithmeticCompositeEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionArithmeticCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionArithmeticCompositeEffectVtbl {
        unsafe extern "system" fn SetCoefficients<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetCoefficients: SetCoefficients::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
            SetCoefficient1: SetCoefficient1::<Impl, IMPL_OFFSET>,
            SetCoefficient1: SetCoefficient1::<Impl, IMPL_OFFSET>,
            SetCoefficient2: SetCoefficient2::<Impl, IMPL_OFFSET>,
            SetCoefficient2: SetCoefficient2::<Impl, IMPL_OFFSET>,
            SetCoefficient3: SetCoefficient3::<Impl, IMPL_OFFSET>,
            SetCoefficient3: SetCoefficient3::<Impl, IMPL_OFFSET>,
            SetCoefficient4: SetCoefficient4::<Impl, IMPL_OFFSET>,
            SetCoefficient4: SetCoefficient4::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionArithmeticCompositeEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetMode();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBlendEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionBlendEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
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
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBrightnessEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBrightnessEffectVtbl {
        unsafe extern "system" fn SetWhitePoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlackPoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetWhitePoint: SetWhitePoint::<Impl, IMPL_OFFSET>,
            SetBlackPoint: SetBlackPoint::<Impl, IMPL_OFFSET>,
            SetWhitePointX: SetWhitePointX::<Impl, IMPL_OFFSET>,
            SetWhitePointX: SetWhitePointX::<Impl, IMPL_OFFSET>,
            SetWhitePointY: SetWhitePointY::<Impl, IMPL_OFFSET>,
            SetWhitePointY: SetWhitePointY::<Impl, IMPL_OFFSET>,
            SetBlackPointX: SetBlackPointX::<Impl, IMPL_OFFSET>,
            SetBlackPointX: SetBlackPointX::<Impl, IMPL_OFFSET>,
            SetBlackPointY: SetBlackPointY::<Impl, IMPL_OFFSET>,
            SetBlackPointY: SetBlackPointY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBrightnessEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionClipImpl: Sized {}
impl IDCompositionClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionClipVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionColorMatrixEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
    fn SetAlphaMode();
    fn SetClampOutput();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionColorMatrixEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionColorMatrixEffectVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetAlphaMode: SetAlphaMode::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionColorMatrixEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetMode();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionCompositeEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrailImpl: Sized {
    fn AddTrailPoints();
    fn AddTrailPointsWithPrediction();
    fn RemoveTrailPoints();
    fn StartNewTrail();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDelegatedInkTrailVtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddTrailPoints: AddTrailPoints::<Impl, IMPL_OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Impl, IMPL_OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Impl, IMPL_OFFSET>,
            StartNewTrail: StartNewTrail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDelegatedInkTrail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionDesktopDeviceImpl: Sized + IDCompositionDevice2Impl {
    fn CreateTargetForHwnd();
    fn CreateSurfaceFromHandle();
    fn CreateSurfaceFromHwnd();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDesktopDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDesktopDeviceVtbl {
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateTargetForHwnd: CreateTargetForHwnd::<Impl, IMPL_OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Impl, IMPL_OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDesktopDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDeviceVtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Impl, IMPL_OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Impl, IMPL_OFFSET>,
            CreateTargetForHwnd: CreateTargetForHwnd::<Impl, IMPL_OFFSET>,
            CreateVisual: CreateVisual::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Impl, IMPL_OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Impl, IMPL_OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Impl, IMPL_OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Impl, IMPL_OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Impl, IMPL_OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Impl, IMPL_OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Impl, IMPL_OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Impl, IMPL_OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Impl, IMPL_OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Impl, IMPL_OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Impl, IMPL_OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Impl, IMPL_OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Impl, IMPL_OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Impl, IMPL_OFFSET>,
            CreateAnimation: CreateAnimation::<Impl, IMPL_OFFSET>,
            CheckDeviceState: CheckDeviceState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice2Vtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurfaceFactory<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Impl, IMPL_OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Impl, IMPL_OFFSET>,
            CreateVisual: CreateVisual::<Impl, IMPL_OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Impl, IMPL_OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Impl, IMPL_OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Impl, IMPL_OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Impl, IMPL_OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Impl, IMPL_OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Impl, IMPL_OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Impl, IMPL_OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Impl, IMPL_OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Impl, IMPL_OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Impl, IMPL_OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Impl, IMPL_OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Impl, IMPL_OFFSET>,
            CreateAnimation: CreateAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice3Vtbl {
        unsafe extern "system" fn CreateGaussianBlurEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBrightnessEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateShadowEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateHueRotationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSaturationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTableTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateGaussianBlurEffect: CreateGaussianBlurEffect::<Impl, IMPL_OFFSET>,
            CreateBrightnessEffect: CreateBrightnessEffect::<Impl, IMPL_OFFSET>,
            CreateColorMatrixEffect: CreateColorMatrixEffect::<Impl, IMPL_OFFSET>,
            CreateShadowEffect: CreateShadowEffect::<Impl, IMPL_OFFSET>,
            CreateHueRotationEffect: CreateHueRotationEffect::<Impl, IMPL_OFFSET>,
            CreateSaturationEffect: CreateSaturationEffect::<Impl, IMPL_OFFSET>,
            CreateTurbulenceEffect: CreateTurbulenceEffect::<Impl, IMPL_OFFSET>,
            CreateLinearTransferEffect: CreateLinearTransferEffect::<Impl, IMPL_OFFSET>,
            CreateTableTransferEffect: CreateTableTransferEffect::<Impl, IMPL_OFFSET>,
            CreateCompositeEffect: CreateCompositeEffect::<Impl, IMPL_OFFSET>,
            CreateBlendEffect: CreateBlendEffect::<Impl, IMPL_OFFSET>,
            CreateArithmeticCompositeEffect: CreateArithmeticCompositeEffect::<Impl, IMPL_OFFSET>,
            CreateAffineTransform2DEffect: CreateAffineTransform2DEffect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionDeviceDebugImpl: Sized {
    fn EnableDebugCounters();
    fn DisableDebugCounters();
}
impl IDCompositionDeviceDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDeviceDebugVtbl {
        unsafe extern "system" fn EnableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableDebugCounters: EnableDebugCounters::<Impl, IMPL_OFFSET>,
            DisableDebugCounters: DisableDebugCounters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDeviceDebug as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionEffectImpl: Sized {}
impl IDCompositionEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionEffectVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionEffectGroupImpl: Sized + IDCompositionEffectImpl {
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform3D();
}
impl IDCompositionEffectGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionEffectGroupVtbl {
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform3D<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform3D: SetTransform3D::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffectGroup as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionFilterEffectImpl: Sized + IDCompositionEffectImpl {
    fn SetInput();
}
impl IDCompositionFilterEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionFilterEffectVtbl {
        unsafe extern "system" fn SetInput<Impl: IDCompositionFilterEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDCompositionEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetInput: SetInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetStandardDeviation();
    fn SetStandardDeviation();
    fn SetBorderMode();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionGaussianBlurEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionGaussianBlurEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionHueRotationEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetAngle();
    fn SetAngle();
}
impl IDCompositionHueRotationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionHueRotationEffectVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionHueRotationEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionInkTrailDeviceImpl: Sized {
    fn CreateDelegatedInkTrail();
    fn CreateDelegatedInkTrailForSwapChain();
}
impl IDCompositionInkTrailDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionInkTrailDeviceVtbl {
        unsafe extern "system" fn CreateDelegatedInkTrail<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDelegatedInkTrail: CreateDelegatedInkTrail::<Impl, IMPL_OFFSET>,
            CreateDelegatedInkTrailForSwapChain: CreateDelegatedInkTrailForSwapChain::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionInkTrailDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionLinearTransferEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
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
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionLinearTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionLinearTransferEffectVtbl {
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRedYIntercept: SetRedYIntercept::<Impl, IMPL_OFFSET>,
            SetRedYIntercept: SetRedYIntercept::<Impl, IMPL_OFFSET>,
            SetRedSlope: SetRedSlope::<Impl, IMPL_OFFSET>,
            SetRedSlope: SetRedSlope::<Impl, IMPL_OFFSET>,
            SetRedDisable: SetRedDisable::<Impl, IMPL_OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Impl, IMPL_OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Impl, IMPL_OFFSET>,
            SetGreenSlope: SetGreenSlope::<Impl, IMPL_OFFSET>,
            SetGreenSlope: SetGreenSlope::<Impl, IMPL_OFFSET>,
            SetGreenDisable: SetGreenDisable::<Impl, IMPL_OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Impl, IMPL_OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Impl, IMPL_OFFSET>,
            SetBlueSlope: SetBlueSlope::<Impl, IMPL_OFFSET>,
            SetBlueSlope: SetBlueSlope::<Impl, IMPL_OFFSET>,
            SetBlueDisable: SetBlueDisable::<Impl, IMPL_OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Impl, IMPL_OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Impl, IMPL_OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Impl, IMPL_OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Impl, IMPL_OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionLinearTransferEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl + IDCompositionTransformImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransformVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransformVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDCompositionMatrixTransform3DImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDCompositionMatrixTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransform3DVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransform3DVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform3D as ::windows::core::Interface>::IID
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
impl IDCompositionRectangleClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRectangleClipVtbl {
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionClipVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetLeft: SetLeft::<Impl, IMPL_OFFSET>,
            SetLeft: SetLeft::<Impl, IMPL_OFFSET>,
            SetTop: SetTop::<Impl, IMPL_OFFSET>,
            SetTop: SetTop::<Impl, IMPL_OFFSET>,
            SetRight: SetRight::<Impl, IMPL_OFFSET>,
            SetRight: SetRight::<Impl, IMPL_OFFSET>,
            SetBottom: SetBottom::<Impl, IMPL_OFFSET>,
            SetBottom: SetBottom::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRectangleClip as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl + IDCompositionTransformImpl {
    fn SetAngle();
    fn SetAngle();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl IDCompositionRotateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransformVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransformVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform3DImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl {
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
impl IDCompositionRotateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransform3DVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransform3DVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAxisX: SetAxisX::<Impl, IMPL_OFFSET>,
            SetAxisX: SetAxisX::<Impl, IMPL_OFFSET>,
            SetAxisY: SetAxisY::<Impl, IMPL_OFFSET>,
            SetAxisY: SetAxisY::<Impl, IMPL_OFFSET>,
            SetAxisZ: SetAxisZ::<Impl, IMPL_OFFSET>,
            SetAxisZ: SetAxisZ::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSaturationEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetSaturation();
    fn SetSaturation();
}
impl IDCompositionSaturationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSaturationEffectVtbl {
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSaturation: SetSaturation::<Impl, IMPL_OFFSET>,
            SetSaturation: SetSaturation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSaturationEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl + IDCompositionTransformImpl {
    fn SetScaleX();
    fn SetScaleX();
    fn SetScaleY();
    fn SetScaleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl IDCompositionScaleTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransformVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransformVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform3DImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl {
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
impl IDCompositionScaleTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransform3DVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransform3DVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetScaleZ: SetScaleZ::<Impl, IMPL_OFFSET>,
            SetScaleZ: SetScaleZ::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionShadowEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
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
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionShadowEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionShadowEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColor<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            SetRed: SetRed::<Impl, IMPL_OFFSET>,
            SetRed: SetRed::<Impl, IMPL_OFFSET>,
            SetGreen: SetGreen::<Impl, IMPL_OFFSET>,
            SetGreen: SetGreen::<Impl, IMPL_OFFSET>,
            SetBlue: SetBlue::<Impl, IMPL_OFFSET>,
            SetBlue: SetBlue::<Impl, IMPL_OFFSET>,
            SetAlpha: SetAlpha::<Impl, IMPL_OFFSET>,
            SetAlpha: SetAlpha::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionShadowEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSkewTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl + IDCompositionTransformImpl {
    fn SetAngleX();
    fn SetAngleX();
    fn SetAngleY();
    fn SetAngleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
impl IDCompositionSkewTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSkewTransformVtbl {
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransformVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngleX: SetAngleX::<Impl, IMPL_OFFSET>,
            SetAngleX: SetAngleX::<Impl, IMPL_OFFSET>,
            SetAngleY: SetAngleY::<Impl, IMPL_OFFSET>,
            SetAngleY: SetAngleY::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSkewTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionSurfaceImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
    fn Scroll();
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurfaceVtbl {
        unsafe extern "system" fn BeginDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuspendDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResumeDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scroll<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginDraw: BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw: EndDraw::<Impl, IMPL_OFFSET>,
            SuspendDraw: SuspendDraw::<Impl, IMPL_OFFSET>,
            ResumeDraw: ResumeDraw::<Impl, IMPL_OFFSET>,
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionSurfaceFactoryImpl: Sized {
    fn CreateSurface();
    fn CreateVirtualSurface();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurfaceFactoryVtbl {
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionTableTransferEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
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
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionTableTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTableTransferEffectVtbl {
        unsafe extern "system" fn SetRedTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRedTable: SetRedTable::<Impl, IMPL_OFFSET>,
            SetGreenTable: SetGreenTable::<Impl, IMPL_OFFSET>,
            SetBlueTable: SetBlueTable::<Impl, IMPL_OFFSET>,
            SetAlphaTable: SetAlphaTable::<Impl, IMPL_OFFSET>,
            SetRedDisable: SetRedDisable::<Impl, IMPL_OFFSET>,
            SetGreenDisable: SetGreenDisable::<Impl, IMPL_OFFSET>,
            SetBlueDisable: SetBlueDisable::<Impl, IMPL_OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
            SetRedTableValue: SetRedTableValue::<Impl, IMPL_OFFSET>,
            SetRedTableValue: SetRedTableValue::<Impl, IMPL_OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Impl, IMPL_OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Impl, IMPL_OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Impl, IMPL_OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Impl, IMPL_OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Impl, IMPL_OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTableTransferEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTargetImpl: Sized {
    fn SetRoot();
}
impl IDCompositionTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTargetVtbl {
        unsafe extern "system" fn SetRoot<Impl: IDCompositionTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetRoot: SetRoot::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTarget as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl {}
impl IDCompositionTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTransformVtbl {
        Self { base: IDCompositionTransform3DVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransform3DImpl: Sized + IDCompositionEffectImpl {}
impl IDCompositionTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTransform3DVtbl {
        Self { base: IDCompositionEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransformImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl + IDCompositionTransformImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
}
impl IDCompositionTranslateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransformVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransformVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform3DImpl: Sized + IDCompositionEffectImpl + IDCompositionTransform3DImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
    fn SetOffsetZ();
    fn SetOffsetZ();
}
impl IDCompositionTranslateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransform3DVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionTransform3DVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionTurbulenceEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetOffset();
    fn SetBaseFrequency();
    fn SetSize();
    fn SetNumOctaves();
    fn SetSeed();
    fn SetNoise();
    fn SetStitchable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionTurbulenceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTurbulenceEffectVtbl {
        unsafe extern "system" fn SetOffset<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBaseFrequency<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNumOctaves<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeed<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNoise<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStitchable<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            SetBaseFrequency: SetBaseFrequency::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            SetNumOctaves: SetNumOctaves::<Impl, IMPL_OFFSET>,
            SetSeed: SetSeed::<Impl, IMPL_OFFSET>,
            SetNoise: SetNoise::<Impl, IMPL_OFFSET>,
            SetStitchable: SetStitchable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTurbulenceEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionVirtualSurfaceImpl: Sized + IDCompositionSurfaceImpl {
    fn Resize();
    fn Trim();
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionVirtualSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVirtualSurfaceVtbl {
        unsafe extern "system" fn Resize<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trim<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionSurfaceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Trim: Trim::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVirtualSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisualVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformParent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffect<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllVisuals<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetTransformParent: SetTransformParent::<Impl, IMPL_OFFSET>,
            SetEffect: SetEffect::<Impl, IMPL_OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
            SetClip: SetClip::<Impl, IMPL_OFFSET>,
            SetClip: SetClip::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            AddVisual: AddVisual::<Impl, IMPL_OFFSET>,
            RemoveVisual: RemoveVisual::<Impl, IMPL_OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Impl, IMPL_OFFSET>,
            SetCompositeMode: SetCompositeMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual2Impl: Sized + IDCompositionVisualImpl {
    fn SetOpacityMode();
    fn SetBackFaceVisibility();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual2Vtbl {
        unsafe extern "system" fn SetOpacityMode<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackFaceVisibility<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionVisualVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOpacityMode: SetOpacityMode::<Impl, IMPL_OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual3Impl: Sized + IDCompositionVisualImpl + IDCompositionVisual2Impl + IDCompositionVisualDebugImpl {
    fn SetDepthMode();
    fn SetOffsetZ();
    fn SetOffsetZ();
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform();
    fn SetTransform();
    fn SetVisible();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual3Vtbl {
        unsafe extern "system" fn SetDepthMode<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVisible<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionVisualDebugVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDepthMode: SetDepthMode::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisualDebugImpl: Sized + IDCompositionVisualImpl + IDCompositionVisual2Impl {
    fn EnableHeatMap();
    fn DisableHeatMap();
    fn EnableRedrawRegions();
    fn DisableRedrawRegions();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisualDebugVtbl {
        unsafe extern "system" fn EnableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDCompositionVisual2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnableHeatMap: EnableHeatMap::<Impl, IMPL_OFFSET>,
            DisableHeatMap: DisableHeatMap::<Impl, IMPL_OFFSET>,
            EnableRedrawRegions: EnableRedrawRegions::<Impl, IMPL_OFFSET>,
            DisableRedrawRegions: DisableRedrawRegions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisualDebug as ::windows::core::Interface>::IID
    }
}
