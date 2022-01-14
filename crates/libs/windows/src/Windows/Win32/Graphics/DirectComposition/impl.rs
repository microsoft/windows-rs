#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetInterpolationMode(&mut self, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()>;
    fn SetBorderMode(&mut self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
    fn SetTransformMatrix(&mut self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetSharpness(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSharpness2(&mut self, sharpness: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionAffineTransform2DEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAffineTransform2DEffect_Vtbl {
        unsafe extern "system" fn SetInterpolationMode<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(::core::mem::transmute_copy(&transformmatrix)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetSharpness<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharpness(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSharpness2<Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharpness2(::core::mem::transmute_copy(&sharpness)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetInterpolationMode: SetInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Impl, IMPL_OFFSET>,
            SetTransformMatrixElement2: SetTransformMatrixElement2::<Impl, IMPL_OFFSET>,
            SetSharpness: SetSharpness::<Impl, IMPL_OFFSET>,
            SetSharpness2: SetSharpness2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionAffineTransform2DEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionAnimation_Impl: Sized {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetAbsoluteBeginTime(&mut self, begintime: i64) -> ::windows::core::Result<()>;
    fn AddCubic(&mut self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()>;
    fn AddSinusoidal(&mut self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()>;
    fn AddRepeat(&mut self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()>;
    fn End(&mut self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionAnimation_Vtbl {
        unsafe extern "system" fn Reset<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbsoluteBeginTime(::core::mem::transmute_copy(&begintime)).into()
        }
        unsafe extern "system" fn AddCubic<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCubic(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSinusoidal(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into()
        }
        unsafe extern "system" fn AddRepeat<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRepeat(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&durationtorepeat)).into()
        }
        unsafe extern "system" fn End<Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT {
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
pub trait IDCompositionArithmeticCompositeEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
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
impl IDCompositionArithmeticCompositeEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionArithmeticCompositeEffect_Vtbl {
        unsafe extern "system" fn SetCoefficients<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficients(::core::mem::transmute_copy(&coefficients)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetCoefficient1<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient1(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient12<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient12(::core::mem::transmute_copy(&coeffcient1)).into()
        }
        unsafe extern "system" fn SetCoefficient2<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient2(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient22<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient22(::core::mem::transmute_copy(&coefficient2)).into()
        }
        unsafe extern "system" fn SetCoefficient3<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient3(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient32<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient32(::core::mem::transmute_copy(&coefficient3)).into()
        }
        unsafe extern "system" fn SetCoefficient4<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient4(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient42<Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoefficient42(::core::mem::transmute_copy(&coefficient4)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetCoefficients: SetCoefficients::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
            SetCoefficient1: SetCoefficient1::<Impl, IMPL_OFFSET>,
            SetCoefficient12: SetCoefficient12::<Impl, IMPL_OFFSET>,
            SetCoefficient2: SetCoefficient2::<Impl, IMPL_OFFSET>,
            SetCoefficient22: SetCoefficient22::<Impl, IMPL_OFFSET>,
            SetCoefficient3: SetCoefficient3::<Impl, IMPL_OFFSET>,
            SetCoefficient32: SetCoefficient32::<Impl, IMPL_OFFSET>,
            SetCoefficient4: SetCoefficient4::<Impl, IMPL_OFFSET>,
            SetCoefficient42: SetCoefficient42::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionArithmeticCompositeEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMode(&mut self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBlendEffect_Vtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionBlendEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
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
impl IDCompositionBrightnessEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionBrightnessEffect_Vtbl {
        unsafe extern "system" fn SetWhitePoint<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn SetBlackPoint<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPoint(::core::mem::transmute_copy(&blackpoint)).into()
        }
        unsafe extern "system" fn SetWhitePointX<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointX2<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointX2(::core::mem::transmute_copy(&whitepointx)).into()
        }
        unsafe extern "system" fn SetWhitePointY<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointY2<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointY2(::core::mem::transmute_copy(&whitepointy)).into()
        }
        unsafe extern "system" fn SetBlackPointX<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointX2<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointX2(::core::mem::transmute_copy(&blackpointx)).into()
        }
        unsafe extern "system" fn SetBlackPointY<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointY2<Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlackPointY2(::core::mem::transmute_copy(&blackpointy)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetWhitePoint: SetWhitePoint::<Impl, IMPL_OFFSET>,
            SetBlackPoint: SetBlackPoint::<Impl, IMPL_OFFSET>,
            SetWhitePointX: SetWhitePointX::<Impl, IMPL_OFFSET>,
            SetWhitePointX2: SetWhitePointX2::<Impl, IMPL_OFFSET>,
            SetWhitePointY: SetWhitePointY::<Impl, IMPL_OFFSET>,
            SetWhitePointY2: SetWhitePointY2::<Impl, IMPL_OFFSET>,
            SetBlackPointX: SetBlackPointX::<Impl, IMPL_OFFSET>,
            SetBlackPointX2: SetBlackPointX2::<Impl, IMPL_OFFSET>,
            SetBlackPointY: SetBlackPointY::<Impl, IMPL_OFFSET>,
            SetBlackPointY2: SetBlackPointY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBrightnessEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionClip_Impl: Sized {}
impl IDCompositionClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionClip_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionColorMatrixEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMatrix(&mut self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetAlphaMode(&mut self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()>;
    fn SetClampOutput(&mut self, clamp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionColorMatrixEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionColorMatrixEffect_Vtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clamp)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Impl, IMPL_OFFSET>,
            SetAlphaMode: SetAlphaMode::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionColorMatrixEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMode(&mut self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionCompositeEffect_Vtbl {
        unsafe extern "system" fn SetMode<Impl: IDCompositionCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetMode: SetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrail_Impl: Sized {
    fn AddTrailPoints(&mut self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&mut self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&mut self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&mut self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrail_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDelegatedInkTrail_Vtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPoints(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPointsWithPrediction(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount), ::core::mem::transmute_copy(&predictedinkpoints), ::core::mem::transmute_copy(&predictedinkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrailPoints(::core::mem::transmute_copy(&generationid)).into()
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
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
pub trait IDCompositionDesktopDevice_Impl: Sized + IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(&mut self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows::core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(&mut self, handle: super::super::Foundation::HANDLE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateSurfaceFromHwnd(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDesktopDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDesktopDevice_Vtbl {
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            base: IDCompositionDevice2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDCompositionDevice_Impl: Sized {
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
impl IDCompositionDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice_Vtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetForHwnd<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *animation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait IDCompositionDevice2_Impl: Sized {
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
impl IDCompositionDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice2_Vtbl {
        unsafe extern "system" fn Commit<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceFactory(::core::mem::transmute(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *surfacefactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDCompositionDevice3_Impl: Sized + IDCompositionDevice2_Impl {
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
impl IDCompositionDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDevice3_Vtbl {
        unsafe extern "system" fn CreateGaussianBlurEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGaussianBlurEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *gaussianblureffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBrightnessEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *brightnesseffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorMatrixEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *colormatrixeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShadowEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *shadoweffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHueRotationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *huerotationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSaturationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *saturationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTurbulenceEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *turbulenceeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *lineartransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *tabletransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *compositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *blendeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateArithmeticCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *arithmeticcompositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IDCompositionDevice2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDCompositionDeviceDebug_Impl: Sized {
    fn EnableDebugCounters(&mut self) -> ::windows::core::Result<()>;
    fn DisableDebugCounters(&mut self) -> ::windows::core::Result<()>;
}
impl IDCompositionDeviceDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebug_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionDeviceDebug_Vtbl {
        unsafe extern "system" fn EnableDebugCounters<Impl: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableDebugCounters().into()
        }
        unsafe extern "system" fn DisableDebugCounters<Impl: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IDCompositionEffect_Impl: Sized {}
impl IDCompositionEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionEffect_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionEffectGroup_Impl: Sized + IDCompositionEffect_Impl {
    fn SetOpacity(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOpacity2(&mut self, opacity: f32) -> ::windows::core::Result<()>;
    fn SetTransform3D(&mut self, transform3d: ::core::option::Option<IDCompositionTransform3D>) -> ::windows::core::Result<()>;
}
impl IDCompositionEffectGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionEffectGroup_Vtbl {
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform3D<Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform3D(::core::mem::transmute(&transform3d)).into()
        }
        Self {
            base: IDCompositionEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity2: SetOpacity2::<Impl, IMPL_OFFSET>,
            SetTransform3D: SetTransform3D::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffectGroup as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionFilterEffect_Impl: Sized + IDCompositionEffect_Impl {
    fn SetInput(&mut self, index: u32, input: ::core::option::Option<::windows::core::IUnknown>, flags: u32) -> ::windows::core::Result<()>;
}
impl IDCompositionFilterEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionFilterEffect_Vtbl {
        unsafe extern "system" fn SetInput<Impl: IDCompositionFilterEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInput(::core::mem::transmute_copy(&index), ::core::mem::transmute(&input), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: IDCompositionEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetInput: SetInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetStandardDeviation2(&mut self, amount: f32) -> ::windows::core::Result<()>;
    fn SetBorderMode(&mut self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionGaussianBlurEffect_Vtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionGaussianBlurEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionHueRotationEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetAngle(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&mut self, amountdegrees: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionHueRotationEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionHueRotationEffect_Vtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&amountdegrees)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle2: SetAngle2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionHueRotationEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionInkTrailDevice_Impl: Sized {
    fn CreateDelegatedInkTrail(&mut self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(&mut self, swapchain: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
}
impl IDCompositionInkTrailDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionInkTrailDevice_Vtbl {
        unsafe extern "system" fn CreateDelegatedInkTrail<Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDelegatedInkTrail() {
                ::core::result::Result::Ok(ok__) => {
                    *inktrail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDCompositionLinearTransferEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
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
impl IDCompositionLinearTransferEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionLinearTransferEffect_Vtbl {
        unsafe extern "system" fn SetRedYIntercept<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedYIntercept2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedYIntercept2(::core::mem::transmute_copy(&redyintercept)).into()
        }
        unsafe extern "system" fn SetRedSlope<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedSlope2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedSlope2(::core::mem::transmute_copy(&redslope)).into()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenYIntercept2(::core::mem::transmute_copy(&greenyintercept)).into()
        }
        unsafe extern "system" fn SetGreenSlope<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenSlope2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenSlope2(::core::mem::transmute_copy(&greenslope)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueYIntercept2(::core::mem::transmute_copy(&blueyintercept)).into()
        }
        unsafe extern "system" fn SetBlueSlope<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueSlope2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueSlope2(::core::mem::transmute_copy(&blueslope)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaYIntercept2(::core::mem::transmute_copy(&alphayintercept)).into()
        }
        unsafe extern "system" fn SetAlphaSlope<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaSlope2<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaSlope2(::core::mem::transmute_copy(&alphaslope)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRedYIntercept: SetRedYIntercept::<Impl, IMPL_OFFSET>,
            SetRedYIntercept2: SetRedYIntercept2::<Impl, IMPL_OFFSET>,
            SetRedSlope: SetRedSlope::<Impl, IMPL_OFFSET>,
            SetRedSlope2: SetRedSlope2::<Impl, IMPL_OFFSET>,
            SetRedDisable: SetRedDisable::<Impl, IMPL_OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Impl, IMPL_OFFSET>,
            SetGreenYIntercept2: SetGreenYIntercept2::<Impl, IMPL_OFFSET>,
            SetGreenSlope: SetGreenSlope::<Impl, IMPL_OFFSET>,
            SetGreenSlope2: SetGreenSlope2::<Impl, IMPL_OFFSET>,
            SetGreenDisable: SetGreenDisable::<Impl, IMPL_OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Impl, IMPL_OFFSET>,
            SetBlueYIntercept2: SetBlueYIntercept2::<Impl, IMPL_OFFSET>,
            SetBlueSlope: SetBlueSlope::<Impl, IMPL_OFFSET>,
            SetBlueSlope2: SetBlueSlope2::<Impl, IMPL_OFFSET>,
            SetBlueDisable: SetBlueDisable::<Impl, IMPL_OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Impl, IMPL_OFFSET>,
            SetAlphaYIntercept2: SetAlphaYIntercept2::<Impl, IMPL_OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Impl, IMPL_OFFSET>,
            SetAlphaSlope2: SetAlphaSlope2::<Impl, IMPL_OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Impl, IMPL_OFFSET>,
            SetClampOutput: SetClampOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionLinearTransferEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetMatrix(&mut self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransform_Vtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDCompositionMatrixTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetMatrix(&mut self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&mut self, row: i32, column: i32, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&mut self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDCompositionMatrixTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionMatrixTransform3D_Vtbl {
        unsafe extern "system" fn SetMatrix<Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrixElement: SetMatrixElement::<Impl, IMPL_OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRectangleClip_Impl: Sized + IDCompositionClip_Impl {
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
impl IDCompositionRectangleClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRectangleClip_Vtbl {
        unsafe extern "system" fn SetLeft<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetLeft2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft2(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn SetTop<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTop2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop2(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn SetRight<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRight2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight2(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetBottom<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottom2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom2(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        Self {
            base: IDCompositionClip_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetLeft: SetLeft::<Impl, IMPL_OFFSET>,
            SetLeft2: SetLeft2::<Impl, IMPL_OFFSET>,
            SetTop: SetTop::<Impl, IMPL_OFFSET>,
            SetTop2: SetTop2::<Impl, IMPL_OFFSET>,
            SetRight: SetRight::<Impl, IMPL_OFFSET>,
            SetRight2: SetRight2::<Impl, IMPL_OFFSET>,
            SetBottom: SetBottom::<Impl, IMPL_OFFSET>,
            SetBottom2: SetBottom2::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusX2: SetTopLeftRadiusX2::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetTopLeftRadiusY2: SetTopLeftRadiusY2::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusX2: SetTopRightRadiusX2::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Impl, IMPL_OFFSET>,
            SetTopRightRadiusY2: SetTopRightRadiusY2::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusX2: SetBottomLeftRadiusX2::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadiusY2: SetBottomLeftRadiusY2::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusX2: SetBottomRightRadiusX2::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Impl, IMPL_OFFSET>,
            SetBottomRightRadiusY2: SetBottomRightRadiusY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRectangleClip as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetAngle(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&mut self, angle: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRotateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransform_Vtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle2: SetAngle2::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX2: SetCenterX2::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY2: SetCenterY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
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
impl IDCompositionRotateTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionRotateTransform3D_Vtbl {
        unsafe extern "system" fn SetAngle<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetAxisX<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisX2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisX2(::core::mem::transmute_copy(&axisx)).into()
        }
        unsafe extern "system" fn SetAxisY<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisY2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisY2(::core::mem::transmute_copy(&axisy)).into()
        }
        unsafe extern "system" fn SetAxisZ<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisZ2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAxisZ2(::core::mem::transmute_copy(&axisz)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngle: SetAngle::<Impl, IMPL_OFFSET>,
            SetAngle2: SetAngle2::<Impl, IMPL_OFFSET>,
            SetAxisX: SetAxisX::<Impl, IMPL_OFFSET>,
            SetAxisX2: SetAxisX2::<Impl, IMPL_OFFSET>,
            SetAxisY: SetAxisY::<Impl, IMPL_OFFSET>,
            SetAxisY2: SetAxisY2::<Impl, IMPL_OFFSET>,
            SetAxisZ: SetAxisZ::<Impl, IMPL_OFFSET>,
            SetAxisZ2: SetAxisZ2::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX2: SetCenterX2::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY2: SetCenterY2::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
            SetCenterZ2: SetCenterZ2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSaturationEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetSaturation(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSaturation2(&mut self, ratio: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSaturationEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSaturationEffect_Vtbl {
        unsafe extern "system" fn SetSaturation<Impl: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaturation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSaturation2<Impl: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaturation2(::core::mem::transmute_copy(&ratio)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSaturation: SetSaturation::<Impl, IMPL_OFFSET>,
            SetSaturation2: SetSaturation2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSaturationEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetScaleX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleX2(&mut self, scalex: f32) -> ::windows::core::Result<()>;
    fn SetScaleY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleY2(&mut self, scaley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionScaleTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransform_Vtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX2: SetScaleX2::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY2: SetScaleY2::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX2: SetCenterX2::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY2: SetCenterY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
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
impl IDCompositionScaleTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionScaleTransform3D_Vtbl {
        unsafe extern "system" fn SetScaleX<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetScaleZ<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleZ2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleZ2(::core::mem::transmute_copy(&scalez)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetScaleX: SetScaleX::<Impl, IMPL_OFFSET>,
            SetScaleX2: SetScaleX2::<Impl, IMPL_OFFSET>,
            SetScaleY: SetScaleY::<Impl, IMPL_OFFSET>,
            SetScaleY2: SetScaleY2::<Impl, IMPL_OFFSET>,
            SetScaleZ: SetScaleZ::<Impl, IMPL_OFFSET>,
            SetScaleZ2: SetScaleZ2::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX2: SetCenterX2::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY2: SetCenterY2::<Impl, IMPL_OFFSET>,
            SetCenterZ: SetCenterZ::<Impl, IMPL_OFFSET>,
            SetCenterZ2: SetCenterZ2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionShadowEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
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
impl IDCompositionShadowEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionShadowEffect_Vtbl {
        unsafe extern "system" fn SetStandardDeviation<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetRed<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRed(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRed2<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRed2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetGreen<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreen(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreen2<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreen2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBlue<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlue(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlue2<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlue2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetAlpha<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlpha(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlpha2<Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlpha2(::core::mem::transmute_copy(&amount)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Impl, IMPL_OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            SetRed: SetRed::<Impl, IMPL_OFFSET>,
            SetRed2: SetRed2::<Impl, IMPL_OFFSET>,
            SetGreen: SetGreen::<Impl, IMPL_OFFSET>,
            SetGreen2: SetGreen2::<Impl, IMPL_OFFSET>,
            SetBlue: SetBlue::<Impl, IMPL_OFFSET>,
            SetBlue2: SetBlue2::<Impl, IMPL_OFFSET>,
            SetAlpha: SetAlpha::<Impl, IMPL_OFFSET>,
            SetAlpha2: SetAlpha2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionShadowEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSkewTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetAngleX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleX2(&mut self, anglex: f32) -> ::windows::core::Result<()>;
    fn SetAngleY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleY2(&mut self, angley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&mut self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&mut self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSkewTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSkewTransform_Vtbl {
        unsafe extern "system" fn SetAngleX<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleX2<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleX2(::core::mem::transmute_copy(&anglex)).into()
        }
        unsafe extern "system" fn SetAngleY<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleY2<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAngleY2(::core::mem::transmute_copy(&angley)).into()
        }
        unsafe extern "system" fn SetCenterX<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAngleX: SetAngleX::<Impl, IMPL_OFFSET>,
            SetAngleX2: SetAngleX2::<Impl, IMPL_OFFSET>,
            SetAngleY: SetAngleY::<Impl, IMPL_OFFSET>,
            SetAngleY2: SetAngleY2::<Impl, IMPL_OFFSET>,
            SetCenterX: SetCenterX::<Impl, IMPL_OFFSET>,
            SetCenterX2: SetCenterX2::<Impl, IMPL_OFFSET>,
            SetCenterY: SetCenterY::<Impl, IMPL_OFFSET>,
            SetCenterY2: SetCenterY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSkewTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionSurface_Impl: Sized {
    fn BeginDraw(&mut self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&mut self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&mut self) -> ::windows::core::Result<()>;
    fn ResumeDraw(&mut self) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurface_Vtbl {
        unsafe extern "system" fn BeginDraw<Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeDraw().into()
        }
        unsafe extern "system" fn Scroll<Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
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
pub trait IDCompositionSurfaceFactory_Impl: Sized {
    fn CreateSurface(&mut self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&mut self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionSurfaceFactory_Vtbl {
        unsafe extern "system" fn CreateSurface<Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDCompositionTableTransferEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
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
impl IDCompositionTableTransferEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTableTransferEffect_Vtbl {
        unsafe extern "system" fn SetRedTable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetGreenTable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBlueTable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetAlphaTable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetRedDisable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetRedTableValue<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedTableValue2<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRedTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetGreenTableValue<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenTableValue2<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGreenTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetBlueTableValue<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueTableValue2<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlueTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue2<Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
            SetRedTableValue2: SetRedTableValue2::<Impl, IMPL_OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Impl, IMPL_OFFSET>,
            SetGreenTableValue2: SetGreenTableValue2::<Impl, IMPL_OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Impl, IMPL_OFFSET>,
            SetBlueTableValue2: SetBlueTableValue2::<Impl, IMPL_OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Impl, IMPL_OFFSET>,
            SetAlphaTableValue2: SetAlphaTableValue2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTableTransferEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTarget_Impl: Sized {
    fn SetRoot(&mut self, visual: ::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
}
impl IDCompositionTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTarget_Vtbl {
        unsafe extern "system" fn SetRoot<Impl: IDCompositionTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoot(::core::mem::transmute(&visual)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetRoot: SetRoot::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTarget as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {}
impl IDCompositionTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTransform_Vtbl {
        Self { base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransform3D_Impl: Sized + IDCompositionEffect_Impl {}
impl IDCompositionTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTransform3D_Vtbl {
        Self { base: IDCompositionEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetOffsetX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&mut self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&mut self, offsety: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransform_Vtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX2: SetOffsetX2::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY2: SetOffsetY2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetOffsetX(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&mut self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&mut self, offsety: f32) -> ::windows::core::Result<()>;
    fn SetOffsetZ(&mut self, animation: ::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetZ2(&mut self, offsetz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTranslateTransform3D_Vtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX2: SetOffsetX2::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY2: SetOffsetY2::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionTurbulenceEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetOffset(&mut self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetBaseFrequency(&mut self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetNumOctaves(&mut self, numoctaves: u32) -> ::windows::core::Result<()>;
    fn SetSeed(&mut self, seed: u32) -> ::windows::core::Result<()>;
    fn SetNoise(&mut self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()>;
    fn SetStitchable(&mut self, stitchable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionTurbulenceEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionTurbulenceEffect_Vtbl {
        unsafe extern "system" fn SetOffset<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetBaseFrequency<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseFrequency(::core::mem::transmute_copy(&frequency)).into()
        }
        unsafe extern "system" fn SetSize<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SetNumOctaves<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumOctaves(::core::mem::transmute_copy(&numoctaves)).into()
        }
        unsafe extern "system" fn SetSeed<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeed(::core::mem::transmute_copy(&seed)).into()
        }
        unsafe extern "system" fn SetNoise<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoise(::core::mem::transmute_copy(&noise)).into()
        }
        unsafe extern "system" fn SetStitchable<Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStitchable(::core::mem::transmute_copy(&stitchable)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDCompositionVirtualSurface_Impl: Sized + IDCompositionSurface_Impl {
    fn Resize(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn Trim(&mut self, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionVirtualSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVirtualSurface_Vtbl {
        unsafe extern "system" fn Resize<Impl: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Trim<Impl: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim(::core::mem::transmute_copy(&rectangles), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: IDCompositionSurface_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Trim: Trim::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVirtualSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual_Impl: Sized {
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
impl IDCompositionVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual_Vtbl {
        unsafe extern "system" fn SetOffsetX<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform2(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetTransformParent<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformParent(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn SetEffect<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffect(::core::mem::transmute(&effect)).into()
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetClip<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip(::core::mem::transmute(&clip)).into()
        }
        unsafe extern "system" fn SetClip2<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip2(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn SetContent<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn AddVisual<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVisual(::core::mem::transmute(&visual), ::core::mem::transmute_copy(&insertabove), ::core::mem::transmute(&referencevisual)).into()
        }
        unsafe extern "system" fn RemoveVisual<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisual(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn RemoveAllVisuals<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllVisuals().into()
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(::core::mem::transmute_copy(&compositemode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOffsetX: SetOffsetX::<Impl, IMPL_OFFSET>,
            SetOffsetX2: SetOffsetX2::<Impl, IMPL_OFFSET>,
            SetOffsetY: SetOffsetY::<Impl, IMPL_OFFSET>,
            SetOffsetY2: SetOffsetY2::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetTransform2: SetTransform2::<Impl, IMPL_OFFSET>,
            SetTransformParent: SetTransformParent::<Impl, IMPL_OFFSET>,
            SetEffect: SetEffect::<Impl, IMPL_OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
            SetClip: SetClip::<Impl, IMPL_OFFSET>,
            SetClip2: SetClip2::<Impl, IMPL_OFFSET>,
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
pub trait IDCompositionVisual2_Impl: Sized + IDCompositionVisual_Impl {
    fn SetOpacityMode(&mut self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()>;
    fn SetBackFaceVisibility(&mut self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual2_Vtbl {
        unsafe extern "system" fn SetOpacityMode<Impl: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetBackFaceVisibility<Impl: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackFaceVisibility(::core::mem::transmute_copy(&visibility)).into()
        }
        Self {
            base: IDCompositionVisual_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOpacityMode: SetOpacityMode::<Impl, IMPL_OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual3_Impl: Sized + IDCompositionVisual_Impl + IDCompositionVisual2_Impl + IDCompositionVisualDebug_Impl {
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
impl IDCompositionVisual3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisual3_Vtbl {
        unsafe extern "system" fn SetDepthMode<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepthMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
        }
        unsafe extern "system" fn SetOpacity<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform2(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetVisible<Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&visible)).into()
        }
        Self {
            base: IDCompositionVisualDebug_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDepthMode: SetDepthMode::<Impl, IMPL_OFFSET>,
            SetOffsetZ: SetOffsetZ::<Impl, IMPL_OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity2: SetOpacity2::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            SetTransform2: SetTransform2::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisualDebug_Impl: Sized + IDCompositionVisual_Impl + IDCompositionVisual2_Impl {
    fn EnableHeatMap(&mut self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
    fn DisableHeatMap(&mut self) -> ::windows::core::Result<()>;
    fn EnableRedrawRegions(&mut self) -> ::windows::core::Result<()>;
    fn DisableRedrawRegions(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDCompositionVisualDebug_Vtbl {
        unsafe extern "system" fn EnableHeatMap<Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableHeatMap(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn DisableHeatMap<Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableHeatMap().into()
        }
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableRedrawRegions().into()
        }
        unsafe extern "system" fn DisableRedrawRegions<Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableRedrawRegions().into()
        }
        Self {
            base: IDCompositionVisual2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
