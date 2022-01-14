#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetInterpolationMode(&mut self, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()>;
    fn SetBorderMode(&mut self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
    fn SetTransformMatrix(&mut self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetSharpness(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSharpness2(&mut self, sharpness: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionAffineTransform2DEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAffineTransform2DEffectVtbl {
        unsafe extern "system" fn SetInterpolationMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(::core::mem::transmute_copy(&transformmatrix)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharpness(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSharpness2<Impl: IDCompositionAffineTransform2DEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharpness2(::core::mem::transmute_copy(&sharpness)).into()
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
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetAbsoluteBeginTime(&mut self, begintime: i64) -> ::windows::core::Result<()>;
    fn AddCubic(&mut self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()>;
    fn AddSinusoidal(&mut self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()>;
    fn AddRepeat(&mut self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()>;
    fn End(&mut self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAnimationVtbl {
        unsafe extern "system" fn Reset<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbsoluteBeginTime(::core::mem::transmute_copy(&begintime)).into()
        }
        unsafe extern "system" fn AddCubic<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCubic(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSinusoidal(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into()
        }
        unsafe extern "system" fn AddRepeat<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRepeat(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&durationtorepeat)).into()
        }
        unsafe extern "system" fn End<Impl: IDCompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End(::core::mem::transmute_copy(&endoffset), ::core::mem::transmute_copy(&endvalue)).into()
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
    fn SetCoefficients(&mut self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()>;
    fn SetClampOutput(&mut self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCoefficient1(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient12(&mut self, coeffcient1: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient2(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient22(&mut self, coefficient2: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient3(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient32(&mut self, coefficient3: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient4(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient42(&mut self, coefficient4: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionArithmeticCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionArithmeticCompositeEffectVtbl {
        unsafe extern "system" fn SetCoefficients<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficients(::core::mem::transmute_copy(&coefficients)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient1(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient12<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient12(::core::mem::transmute_copy(&coeffcient1)).into()
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient2(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient22<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient22(::core::mem::transmute_copy(&coefficient2)).into()
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient3(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient32<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient32(::core::mem::transmute_copy(&coefficient3)).into()
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient4(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient42<Impl: IDCompositionArithmeticCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient42(::core::mem::transmute_copy(&coefficient4)).into()
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
    fn SetMode(&mut self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBlendEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionBlendEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetWhitePoint(&mut self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetBlackPoint(&mut self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetWhitePointX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetWhitePointX2(&mut self, whitepointx: f32) -> ::windows::core::Result<()>;
    fn SetWhitePointY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetWhitePointY2(&mut self, whitepointy: f32) -> ::windows::core::Result<()>;
    fn SetBlackPointX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlackPointX2(&mut self, blackpointx: f32) -> ::windows::core::Result<()>;
    fn SetBlackPointY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlackPointY2(&mut self, blackpointy: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBrightnessEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBrightnessEffectVtbl {
        unsafe extern "system" fn SetWhitePoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn SetBlackPoint<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPoint(::core::mem::transmute_copy(&blackpoint)).into()
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointX2<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointX2(::core::mem::transmute_copy(&whitepointx)).into()
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointY2<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointY2(::core::mem::transmute_copy(&whitepointy)).into()
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointX2<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointX2(::core::mem::transmute_copy(&blackpointx)).into()
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointY2<Impl: IDCompositionBrightnessEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointY2(::core::mem::transmute_copy(&blackpointy)).into()
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
    fn SetMatrix(&mut self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetAlphaMode(&mut self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()>;
    fn SetClampOutput(&mut self, clamp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionColorMatrixEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionColorMatrixEffectVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionColorMatrixEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clamp)).into()
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
    fn SetMode(&mut self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionCompositeEffectVtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionCompositeEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrailImpl: Sized {
    fn AddTrailPoints(&mut self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&mut self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&mut self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&mut self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDelegatedInkTrailVtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPoints(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPointsWithPrediction(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount), ::core::mem::transmute_copy(&predictedinkpoints), ::core::mem::transmute_copy(&predictedinkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrailPoints(::core::mem::transmute_copy(&generationid)).into()
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDCompositionDelegatedInkTrailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNewTrail(::core::mem::transmute_copy(&color)).into()
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
    fn CreateTargetForHwnd(&mut self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows::core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(&mut self, handle: super::super::Foundation::HANDLE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateSurfaceFromHwnd(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDesktopDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDesktopDeviceVtbl {
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDesktopDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn WaitForCommitCompletion(&mut self) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateTargetForHwnd(&mut self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows::core::Result<IDCompositionTarget>;
    fn CreateVisual(&mut self) -> ::windows::core::Result<IDCompositionVisual>;
    fn CreateSurface(&mut self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&mut self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
    fn CreateSurfaceFromHandle(&mut self, handle: super::super::Foundation::HANDLE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateSurfaceFromHwnd(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateTranslateTransform(&mut self) -> ::windows::core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&mut self) -> ::windows::core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&mut self) -> ::windows::core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&mut self) -> ::windows::core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&mut self) -> ::windows::core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&mut self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&mut self) -> ::windows::core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&mut self) -> ::windows::core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&mut self) -> ::windows::core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&mut self) -> ::windows::core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&mut self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&mut self) -> ::windows::core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&mut self) -> ::windows::core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&mut self) -> ::windows::core::Result<IDCompositionAnimation>;
    fn CheckDeviceState(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDeviceVtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *animation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDCompositionDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckDeviceState() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn WaitForCommitCompletion(&mut self) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateVisual(&mut self) -> ::windows::core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(&mut self, renderingdevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(&mut self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&mut self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(&mut self) -> ::windows::core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&mut self) -> ::windows::core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&mut self) -> ::windows::core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&mut self) -> ::windows::core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&mut self) -> ::windows::core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&mut self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&mut self) -> ::windows::core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&mut self) -> ::windows::core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&mut self) -> ::windows::core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&mut self) -> ::windows::core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&mut self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&mut self) -> ::windows::core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&mut self) -> ::windows::core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&mut self) -> ::windows::core::Result<IDCompositionAnimation>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice2Vtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFactory(::core::mem::transmute(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *surfacefactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *animation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn CreateGaussianBlurEffect(&mut self) -> ::windows::core::Result<IDCompositionGaussianBlurEffect>;
    fn CreateBrightnessEffect(&mut self) -> ::windows::core::Result<IDCompositionBrightnessEffect>;
    fn CreateColorMatrixEffect(&mut self) -> ::windows::core::Result<IDCompositionColorMatrixEffect>;
    fn CreateShadowEffect(&mut self) -> ::windows::core::Result<IDCompositionShadowEffect>;
    fn CreateHueRotationEffect(&mut self) -> ::windows::core::Result<IDCompositionHueRotationEffect>;
    fn CreateSaturationEffect(&mut self) -> ::windows::core::Result<IDCompositionSaturationEffect>;
    fn CreateTurbulenceEffect(&mut self) -> ::windows::core::Result<IDCompositionTurbulenceEffect>;
    fn CreateLinearTransferEffect(&mut self) -> ::windows::core::Result<IDCompositionLinearTransferEffect>;
    fn CreateTableTransferEffect(&mut self) -> ::windows::core::Result<IDCompositionTableTransferEffect>;
    fn CreateCompositeEffect(&mut self) -> ::windows::core::Result<IDCompositionCompositeEffect>;
    fn CreateBlendEffect(&mut self) -> ::windows::core::Result<IDCompositionBlendEffect>;
    fn CreateArithmeticCompositeEffect(&mut self) -> ::windows::core::Result<IDCompositionArithmeticCompositeEffect>;
    fn CreateAffineTransform2DEffect(&mut self) -> ::windows::core::Result<IDCompositionAffineTransform2DEffect>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice3Vtbl {
        unsafe extern "system" fn CreateGaussianBlurEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGaussianBlurEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *gaussianblureffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBrightnessEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *brightnesseffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorMatrixEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *colormatrixeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShadowEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *shadoweffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHueRotationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *huerotationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSaturationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *saturationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTurbulenceEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *turbulenceeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *lineartransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *tabletransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *compositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *blendeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateArithmeticCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *arithmeticcompositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Impl: IDCompositionDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAffineTransform2DEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *affinetransform2deffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EnableDebugCounters(&mut self) -> ::windows::core::Result<()>;
    fn DisableDebugCounters(&mut self) -> ::windows::core::Result<()>;
}
impl IDCompositionDeviceDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDeviceDebugVtbl {
        unsafe extern "system" fn EnableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugCounters().into()
        }
        unsafe extern "system" fn DisableDebugCounters<Impl: IDCompositionDeviceDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableDebugCounters().into()
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
    fn SetOpacity(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOpacity2(&mut self, opacity: f32) -> ::windows::core::Result<()>;
    fn SetTransform3D(&mut self, transform3d: ::core::option::Option<IDCompositionTransform3D>) -> ::windows::core::Result<()>;
}
impl IDCompositionEffectGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionEffectGroupVtbl {
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform3D<Impl: IDCompositionEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform3D(::core::mem::transmute(&transform3d)).into()
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
    fn SetInput(&mut self, index: u32, input: ::core::option::Option<::windows::core::IUnknown>, flags: u32) -> ::windows::core::Result<()>;
}
impl IDCompositionFilterEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionFilterEffectVtbl {
        unsafe extern "system" fn SetInput<Impl: IDCompositionFilterEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInput(::core::mem::transmute_copy(&index), ::core::mem::transmute(&input), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: IDCompositionEffectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetInput: SetInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffectImpl: Sized + IDCompositionEffectImpl + IDCompositionFilterEffectImpl {
    fn SetStandardDeviation(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetStandardDeviation2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetBorderMode(&mut self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionGaussianBlurEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionGaussianBlurEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&mode)).into()
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
    fn SetAngle(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&mut self, amountdegrees: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionHueRotationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionHueRotationEffectVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionHueRotationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&amountdegrees)).into()
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
    fn CreateDelegatedInkTrail(&mut self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(&mut self, swapchain: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
}
impl IDCompositionInkTrailDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionInkTrailDeviceVtbl {
        unsafe extern "system" fn CreateDelegatedInkTrail<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDelegatedInkTrail() {
                ::core::result::Result::Ok(ok__) => {
                    *inktrail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Impl: IDCompositionInkTrailDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDelegatedInkTrailForSwapChain(::core::mem::transmute(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *inktrail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn SetRedYIntercept(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedYIntercept2(&mut self, redyintercept: f32) -> ::windows::core::Result<()>;
    fn SetRedSlope(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedSlope2(&mut self, redslope: f32) -> ::windows::core::Result<()>;
    fn SetRedDisable(&mut self, reddisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGreenYIntercept(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenYIntercept2(&mut self, greenyintercept: f32) -> ::windows::core::Result<()>;
    fn SetGreenSlope(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenSlope2(&mut self, greenslope: f32) -> ::windows::core::Result<()>;
    fn SetGreenDisable(&mut self, greendisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBlueYIntercept(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueYIntercept2(&mut self, blueyintercept: f32) -> ::windows::core::Result<()>;
    fn SetBlueSlope(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueSlope2(&mut self, blueslope: f32) -> ::windows::core::Result<()>;
    fn SetBlueDisable(&mut self, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetAlphaYIntercept(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaYIntercept2(&mut self, alphayintercept: f32) -> ::windows::core::Result<()>;
    fn SetAlphaSlope(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaSlope2(&mut self, alphaslope: f32) -> ::windows::core::Result<()>;
    fn SetAlphaDisable(&mut self, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClampOutput(&mut self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionLinearTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionLinearTransferEffectVtbl {
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedYIntercept2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedYIntercept2(::core::mem::transmute_copy(&redyintercept)).into()
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedSlope2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedSlope2(::core::mem::transmute_copy(&redslope)).into()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenYIntercept2(::core::mem::transmute_copy(&greenyintercept)).into()
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenSlope2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenSlope2(::core::mem::transmute_copy(&greenslope)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueYIntercept2(::core::mem::transmute_copy(&blueyintercept)).into()
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueSlope2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueSlope2(::core::mem::transmute_copy(&blueslope)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaYIntercept2(::core::mem::transmute_copy(&alphayintercept)).into()
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaSlope2<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaSlope2(::core::mem::transmute_copy(&alphaslope)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionLinearTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
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
    fn SetMatrix(&mut self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransformVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
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
    fn SetMatrix(&mut self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDCompositionMatrixTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransform3DVtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionMatrixTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
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
    fn SetLeft(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetLeft2(&mut self, left: f32) -> ::windows::core::Result<()>;
    fn SetTop(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTop2(&mut self, top: f32) -> ::windows::core::Result<()>;
    fn SetRight(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRight2(&mut self, right: f32) -> ::windows::core::Result<()>;
    fn SetBottom(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottom2(&mut self, bottom: f32) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusX2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusY2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusX2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusY2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusX2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusY2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusX2(&mut self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusY2(&mut self, radius: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRectangleClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRectangleClipVtbl {
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetLeft2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft2(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTop2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop2(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRight2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight2(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottom2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom2(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Impl: IDCompositionRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
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
    fn SetAngle(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&mut self, angle: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRotateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransformVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionRotateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
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
    fn SetAngle(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&mut self, angle: f32) -> ::windows::core::Result<()>;
    fn SetAxisX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisX2(&mut self, axisx: f32) -> ::windows::core::Result<()>;
    fn SetAxisY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisY2(&mut self, axisy: f32) -> ::windows::core::Result<()>;
    fn SetAxisZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisZ2(&mut self, axisz: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
    fn SetCenterZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterZ2(&mut self, centerz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRotateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransform3DVtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisX2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisX2(::core::mem::transmute_copy(&axisx)).into()
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisY2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisY2(::core::mem::transmute_copy(&axisy)).into()
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisZ2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisZ2(::core::mem::transmute_copy(&axisz)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Impl: IDCompositionRotateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
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
    fn SetSaturation(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSaturation2(&mut self, ratio: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSaturationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSaturationEffectVtbl {
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaturation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSaturation2<Impl: IDCompositionSaturationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaturation2(::core::mem::transmute_copy(&ratio)).into()
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
    fn SetScaleX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleX2(&mut self, scalex: f32) -> ::windows::core::Result<()>;
    fn SetScaleY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleY2(&mut self, scaley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionScaleTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransformVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionScaleTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
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
    fn SetScaleX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleX2(&mut self, scalex: f32) -> ::windows::core::Result<()>;
    fn SetScaleY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleY2(&mut self, scaley: f32) -> ::windows::core::Result<()>;
    fn SetScaleZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleZ2(&mut self, scalez: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
    fn SetCenterZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterZ2(&mut self, centerz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionScaleTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransform3DVtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleZ2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleZ2(::core::mem::transmute_copy(&scalez)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Impl: IDCompositionScaleTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
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
    fn SetStandardDeviation(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetStandardDeviation2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetColor(&mut self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()>;
    fn SetRed(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRed2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetGreen(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreen2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetBlue(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlue2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetAlpha(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlpha2(&mut self, amount: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionShadowEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionShadowEffectVtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRed(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRed2<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRed2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreen(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreen2<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreen2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlue(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlue2<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlue2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlpha(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlpha2<Impl: IDCompositionShadowEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlpha2(::core::mem::transmute_copy(&amount)).into()
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
    fn SetAngleX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleX2(&mut self, anglex: f32) -> ::windows::core::Result<()>;
    fn SetAngleY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleY2(&mut self, angley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSkewTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSkewTransformVtbl {
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleX2<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleX2(::core::mem::transmute_copy(&anglex)).into()
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleY2<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleY2(::core::mem::transmute_copy(&angley)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionSkewTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
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
    fn BeginDraw(&mut self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&mut self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&mut self) -> ::windows::core::Result<()>;
    fn ResumeDraw(&mut self) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurfaceVtbl {
        unsafe extern "system" fn BeginDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeDraw().into()
        }
        unsafe extern "system" fn Scroll<Impl: IDCompositionSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into()
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
    fn CreateSurface(&mut self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&mut self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurfaceFactoryVtbl {
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionSurfaceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn SetRedTable(&mut self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetGreenTable(&mut self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetBlueTable(&mut self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetAlphaTable(&mut self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetRedDisable(&mut self, reddisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGreenDisable(&mut self, greendisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBlueDisable(&mut self, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetAlphaDisable(&mut self, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClampOutput(&mut self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetRedTableValue(&mut self, index: u32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedTableValue2(&mut self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetGreenTableValue(&mut self, index: u32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenTableValue2(&mut self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetBlueTableValue(&mut self, index: u32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueTableValue2(&mut self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetAlphaTableValue(&mut self, index: u32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaTableValue2(&mut self, index: u32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionTableTransferEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTableTransferEffectVtbl {
        unsafe extern "system" fn SetRedTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetGreenTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBlueTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetAlphaTable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedTableValue2<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenTableValue2<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueTableValue2<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue2<Impl: IDCompositionTableTransferEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
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
    fn SetRoot(&mut self, visual: ::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
}
impl IDCompositionTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTargetVtbl {
        unsafe extern "system" fn SetRoot<Impl: IDCompositionTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoot(::core::mem::transmute(&visual)).into()
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
    fn SetOffsetX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&mut self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&mut self, offsety: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransformVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionTranslateTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
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
    fn SetOffsetX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&mut self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&mut self, offsety: f32) -> ::windows::core::Result<()>;
    fn SetOffsetZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetZ2(&mut self, offsetz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransform3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransform3DVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Impl: IDCompositionTranslateTransform3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
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
    fn SetOffset(&mut self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetBaseFrequency(&mut self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetNumOctaves(&mut self, numoctaves: u32) -> ::windows::core::Result<()>;
    fn SetSeed(&mut self, seed: u32) -> ::windows::core::Result<()>;
    fn SetNoise(&mut self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()>;
    fn SetStitchable(&mut self, stitchable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionTurbulenceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTurbulenceEffectVtbl {
        unsafe extern "system" fn SetOffset<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetBaseFrequency<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseFrequency(::core::mem::transmute_copy(&frequency)).into()
        }
        unsafe extern "system" fn SetSize<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SetNumOctaves<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumOctaves(::core::mem::transmute_copy(&numoctaves)).into()
        }
        unsafe extern "system" fn SetSeed<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeed(::core::mem::transmute_copy(&seed)).into()
        }
        unsafe extern "system" fn SetNoise<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoise(::core::mem::transmute_copy(&noise)).into()
        }
        unsafe extern "system" fn SetStitchable<Impl: IDCompositionTurbulenceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStitchable(::core::mem::transmute_copy(&stitchable)).into()
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
    fn Resize(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn Trim(&mut self, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionVirtualSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVirtualSurfaceVtbl {
        unsafe extern "system" fn Resize<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Trim<Impl: IDCompositionVirtualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim(::core::mem::transmute_copy(&rectangles), ::core::mem::transmute_copy(&count)).into()
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
    fn SetOffsetX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&mut self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&mut self, offsety: f32) -> ::windows::core::Result<()>;
    fn SetTransform(&mut self, transform: ::core::option::Option<IDCompositionTransform>) -> ::windows::core::Result<()>;
    fn SetTransform2(&mut self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetTransformParent(&mut self, visual: ::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn SetEffect(&mut self, effect: ::core::option::Option<IDCompositionEffect>) -> ::windows::core::Result<()>;
    fn SetBitmapInterpolationMode(&mut self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()>;
    fn SetBorderMode(&mut self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()>;
    fn SetClip(&mut self, clip: ::core::option::Option<IDCompositionClip>) -> ::windows::core::Result<()>;
    fn SetClip2(&mut self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()>;
    fn SetContent(&mut self, content: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddVisual(&mut self, visual: ::core::option::Option<IDCompositionVisual>, insertabove: super::super::Foundation::BOOL, referencevisual: ::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn RemoveVisual(&mut self, visual: ::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn RemoveAllVisuals(&mut self) -> ::windows::core::Result<()>;
    fn SetCompositeMode(&mut self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisualVtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform2(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetTransformParent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformParent(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn SetEffect<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffect(::core::mem::transmute(&effect)).into()
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip(::core::mem::transmute(&clip)).into()
        }
        unsafe extern "system" fn SetClip2<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip2(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn SetContent<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn AddVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVisual(::core::mem::transmute(&visual), ::core::mem::transmute_copy(&insertabove), ::core::mem::transmute(&referencevisual)).into()
        }
        unsafe extern "system" fn RemoveVisual<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisual(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn RemoveAllVisuals<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllVisuals().into()
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IDCompositionVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(::core::mem::transmute_copy(&compositemode)).into()
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
    fn SetOpacityMode(&mut self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()>;
    fn SetBackFaceVisibility(&mut self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual2Vtbl {
        unsafe extern "system" fn SetOpacityMode<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetBackFaceVisibility<Impl: IDCompositionVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackFaceVisibility(::core::mem::transmute_copy(&visibility)).into()
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
    fn SetDepthMode(&mut self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::Result<()>;
    fn SetOffsetZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetZ2(&mut self, offsetz: f32) -> ::windows::core::Result<()>;
    fn SetOpacity(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOpacity2(&mut self, opacity: f32) -> ::windows::core::Result<()>;
    fn SetTransform(&mut self, transform: ::core::option::Option<IDCompositionTransform3D>) -> ::windows::core::Result<()>;
    fn SetTransform2(&mut self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::Result<()>;
    fn SetVisible(&mut self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual3Vtbl {
        unsafe extern "system" fn SetDepthMode<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepthMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform2(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetVisible<Impl: IDCompositionVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&visible)).into()
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
    fn EnableHeatMap(&mut self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
    fn DisableHeatMap(&mut self) -> ::windows::core::Result<()>;
    fn EnableRedrawRegions(&mut self) -> ::windows::core::Result<()>;
    fn DisableRedrawRegions(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisualDebugVtbl {
        unsafe extern "system" fn EnableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableHeatMap(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn DisableHeatMap<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableHeatMap().into()
        }
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableRedrawRegions().into()
        }
        unsafe extern "system" fn DisableRedrawRegions<Impl: IDCompositionVisualDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableRedrawRegions().into()
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
