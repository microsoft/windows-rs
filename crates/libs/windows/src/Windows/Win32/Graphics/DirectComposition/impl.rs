#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()>;
    fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
    fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement(&self, row: i32, column: i32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetSharpness(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSharpness2(&self, sharpness: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionAffineTransform2DEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>() -> IDCompositionAffineTransform2DEffect_Vtbl {
        unsafe extern "system" fn SetInterpolationMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetTransformMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransformMatrix(::core::mem::transmute_copy(&transformmatrix)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransformMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransformMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetSharpness<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSharpness(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSharpness2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSharpness2(::core::mem::transmute_copy(&sharpness)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetInterpolationMode: SetInterpolationMode::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Identity, Impl, OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Identity, Impl, OFFSET>,
            SetTransformMatrixElement2: SetTransformMatrixElement2::<Identity, Impl, OFFSET>,
            SetSharpness: SetSharpness::<Identity, Impl, OFFSET>,
            SetSharpness2: SetSharpness2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionAffineTransform2DEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionAnimation_Impl: Sized {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows::core::Result<()>;
    fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()>;
    fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()>;
    fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()>;
    fn End(&self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>() -> IDCompositionAnimation_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAbsoluteBeginTime(::core::mem::transmute_copy(&begintime)).into()
        }
        unsafe extern "system" fn AddCubic<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddCubic(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSinusoidal(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into()
        }
        unsafe extern "system" fn AddRepeat<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRepeat(::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&durationtorepeat)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End(::core::mem::transmute_copy(&endoffset), ::core::mem::transmute_copy(&endvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Identity, Impl, OFFSET>,
            AddCubic: AddCubic::<Identity, Impl, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, Impl, OFFSET>,
            AddRepeat: AddRepeat::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionArithmeticCompositeEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCoefficient1(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient2(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient22(&self, coefficient2: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient3(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient32(&self, coefficient3: f32) -> ::windows::core::Result<()>;
    fn SetCoefficient4(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCoefficient42(&self, coefficient4: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionArithmeticCompositeEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>() -> IDCompositionArithmeticCompositeEffect_Vtbl {
        unsafe extern "system" fn SetCoefficients<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficients(::core::mem::transmute_copy(&coefficients)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetCoefficient1<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient1(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient12<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient12(::core::mem::transmute_copy(&coeffcient1)).into()
        }
        unsafe extern "system" fn SetCoefficient2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient2(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient22<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient22(::core::mem::transmute_copy(&coefficient2)).into()
        }
        unsafe extern "system" fn SetCoefficient3<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient3(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient32<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient32(::core::mem::transmute_copy(&coefficient3)).into()
        }
        unsafe extern "system" fn SetCoefficient4<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient4(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient42<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoefficient42(::core::mem::transmute_copy(&coefficient4)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetCoefficients: SetCoefficients::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
            SetCoefficient1: SetCoefficient1::<Identity, Impl, OFFSET>,
            SetCoefficient12: SetCoefficient12::<Identity, Impl, OFFSET>,
            SetCoefficient2: SetCoefficient2::<Identity, Impl, OFFSET>,
            SetCoefficient22: SetCoefficient22::<Identity, Impl, OFFSET>,
            SetCoefficient3: SetCoefficient3::<Identity, Impl, OFFSET>,
            SetCoefficient32: SetCoefficient32::<Identity, Impl, OFFSET>,
            SetCoefficient4: SetCoefficient4::<Identity, Impl, OFFSET>,
            SetCoefficient42: SetCoefficient42::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionArithmeticCompositeEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffect_Impl, const OFFSET: isize>() -> IDCompositionBlendEffect_Vtbl {
        unsafe extern "system" fn SetMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBlendEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(), SetMode: SetMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetWhitePointX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows::core::Result<()>;
    fn SetWhitePointY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows::core::Result<()>;
    fn SetBlackPointX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows::core::Result<()>;
    fn SetBlackPointY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBrightnessEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>() -> IDCompositionBrightnessEffect_Vtbl {
        unsafe extern "system" fn SetWhitePoint<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn SetBlackPoint<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlackPoint(::core::mem::transmute_copy(&blackpoint)).into()
        }
        unsafe extern "system" fn SetWhitePointX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointX2(::core::mem::transmute_copy(&whitepointx)).into()
        }
        unsafe extern "system" fn SetWhitePointY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointY2(::core::mem::transmute_copy(&whitepointy)).into()
        }
        unsafe extern "system" fn SetBlackPointX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlackPointX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlackPointX2(::core::mem::transmute_copy(&blackpointx)).into()
        }
        unsafe extern "system" fn SetBlackPointY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlackPointY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlackPointY2(::core::mem::transmute_copy(&blackpointy)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetWhitePoint: SetWhitePoint::<Identity, Impl, OFFSET>,
            SetBlackPoint: SetBlackPoint::<Identity, Impl, OFFSET>,
            SetWhitePointX: SetWhitePointX::<Identity, Impl, OFFSET>,
            SetWhitePointX2: SetWhitePointX2::<Identity, Impl, OFFSET>,
            SetWhitePointY: SetWhitePointY::<Identity, Impl, OFFSET>,
            SetWhitePointY2: SetWhitePointY2::<Identity, Impl, OFFSET>,
            SetBlackPointX: SetBlackPointX::<Identity, Impl, OFFSET>,
            SetBlackPointX2: SetBlackPointX2::<Identity, Impl, OFFSET>,
            SetBlackPointY: SetBlackPointY::<Identity, Impl, OFFSET>,
            SetBlackPointY2: SetBlackPointY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionBrightnessEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionClip_Impl: Sized {}
impl IDCompositionClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionClip_Impl, const OFFSET: isize>() -> IDCompositionClip_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionColorMatrixEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
    fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()>;
    fn SetClampOutput(&self, clamp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionColorMatrixEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>() -> IDCompositionColorMatrixEffect_Vtbl {
        unsafe extern "system" fn SetMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clamp)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionColorMatrixEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffect_Impl, const OFFSET: isize>() -> IDCompositionCompositeEffect_Vtbl {
        unsafe extern "system" fn SetMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionCompositeEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(), SetMode: SetMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrail_Impl: Sized {
    fn AddTrailPoints(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrail_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>() -> IDCompositionDelegatedInkTrail_Vtbl {
        unsafe extern "system" fn AddTrailPoints<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddTrailPoints(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddTrailPointsWithPrediction(::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount), ::core::mem::transmute_copy(&predictedinkpoints), ::core::mem::transmute_copy(&predictedinkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveTrailPoints(::core::mem::transmute_copy(&generationid)).into()
        }
        unsafe extern "system" fn StartNewTrail<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartNewTrail(::core::mem::transmute_copy(&color)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddTrailPoints: AddTrailPoints::<Identity, Impl, OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Identity, Impl, OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Identity, Impl, OFFSET>,
            StartNewTrail: StartNewTrail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDelegatedInkTrail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionDesktopDevice_Impl: Sized + IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows::core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDesktopDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>() -> IDCompositionDesktopDevice_Vtbl {
        unsafe extern "system" fn CreateTargetForHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDCompositionDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDesktopDevice as ::windows::core::Interface>::IID || iid == &<IDCompositionDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionDevice_Impl: Sized {
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows::core::Result<IDCompositionTarget>;
    fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation>;
    fn CheckDeviceState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>() -> IDCompositionDevice_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetForHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTargetForHwnd(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurfaceFromHandle(::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurfaceFromHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *animation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckDeviceState() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, Impl, OFFSET>,
            CreateVisual: CreateVisual::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, Impl, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, Impl, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, Impl, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, Impl, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, Impl, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, Impl, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, Impl, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, Impl, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, Impl, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, Impl, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, Impl, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, Impl, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, Impl, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice2_Impl: Sized {
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS>;
    fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(&self, renderingdevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>() -> IDCompositionDevice2_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForCommitCompletion().into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *statistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurfaceFactory(::core::mem::transmute(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *surfacefactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTranslateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScaleTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRotateTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSkewTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *skewtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransformGroup(::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transformgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTranslateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *translatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScaleTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *scaletransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRotateTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *rotatetransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMatrixTransform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTransform3DGroup(::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform3dgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEffectGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *effectgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *clip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *animation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            CreateVisual: CreateVisual::<Identity, Impl, OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, Impl, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, Impl, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, Impl, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, Impl, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, Impl, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, Impl, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, Impl, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, Impl, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, Impl, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, Impl, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, Impl, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice3_Impl: Sized + IDCompositionDevice2_Impl {
    fn CreateGaussianBlurEffect(&self) -> ::windows::core::Result<IDCompositionGaussianBlurEffect>;
    fn CreateBrightnessEffect(&self) -> ::windows::core::Result<IDCompositionBrightnessEffect>;
    fn CreateColorMatrixEffect(&self) -> ::windows::core::Result<IDCompositionColorMatrixEffect>;
    fn CreateShadowEffect(&self) -> ::windows::core::Result<IDCompositionShadowEffect>;
    fn CreateHueRotationEffect(&self) -> ::windows::core::Result<IDCompositionHueRotationEffect>;
    fn CreateSaturationEffect(&self) -> ::windows::core::Result<IDCompositionSaturationEffect>;
    fn CreateTurbulenceEffect(&self) -> ::windows::core::Result<IDCompositionTurbulenceEffect>;
    fn CreateLinearTransferEffect(&self) -> ::windows::core::Result<IDCompositionLinearTransferEffect>;
    fn CreateTableTransferEffect(&self) -> ::windows::core::Result<IDCompositionTableTransferEffect>;
    fn CreateCompositeEffect(&self) -> ::windows::core::Result<IDCompositionCompositeEffect>;
    fn CreateBlendEffect(&self) -> ::windows::core::Result<IDCompositionBlendEffect>;
    fn CreateArithmeticCompositeEffect(&self) -> ::windows::core::Result<IDCompositionArithmeticCompositeEffect>;
    fn CreateAffineTransform2DEffect(&self) -> ::windows::core::Result<IDCompositionAffineTransform2DEffect>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>() -> IDCompositionDevice3_Vtbl {
        unsafe extern "system" fn CreateGaussianBlurEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGaussianBlurEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *gaussianblureffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBrightnessEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *brightnesseffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateColorMatrixEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *colormatrixeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateShadowEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *shadoweffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateHueRotationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *huerotationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSaturationEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *saturationeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTurbulenceEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *turbulenceeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLinearTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *lineartransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTableTransferEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *tabletransfereffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *compositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlendEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *blendeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateArithmeticCompositeEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *arithmeticcompositeeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAffineTransform2DEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *affinetransform2deffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDCompositionDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateGaussianBlurEffect: CreateGaussianBlurEffect::<Identity, Impl, OFFSET>,
            CreateBrightnessEffect: CreateBrightnessEffect::<Identity, Impl, OFFSET>,
            CreateColorMatrixEffect: CreateColorMatrixEffect::<Identity, Impl, OFFSET>,
            CreateShadowEffect: CreateShadowEffect::<Identity, Impl, OFFSET>,
            CreateHueRotationEffect: CreateHueRotationEffect::<Identity, Impl, OFFSET>,
            CreateSaturationEffect: CreateSaturationEffect::<Identity, Impl, OFFSET>,
            CreateTurbulenceEffect: CreateTurbulenceEffect::<Identity, Impl, OFFSET>,
            CreateLinearTransferEffect: CreateLinearTransferEffect::<Identity, Impl, OFFSET>,
            CreateTableTransferEffect: CreateTableTransferEffect::<Identity, Impl, OFFSET>,
            CreateCompositeEffect: CreateCompositeEffect::<Identity, Impl, OFFSET>,
            CreateBlendEffect: CreateBlendEffect::<Identity, Impl, OFFSET>,
            CreateArithmeticCompositeEffect: CreateArithmeticCompositeEffect::<Identity, Impl, OFFSET>,
            CreateAffineTransform2DEffect: CreateAffineTransform2DEffect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDevice3 as ::windows::core::Interface>::IID || iid == &<IDCompositionDevice2 as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionDeviceDebug_Impl: Sized {
    fn EnableDebugCounters(&self) -> ::windows::core::Result<()>;
    fn DisableDebugCounters(&self) -> ::windows::core::Result<()>;
}
impl IDCompositionDeviceDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: isize>() -> IDCompositionDeviceDebug_Vtbl {
        unsafe extern "system" fn EnableDebugCounters<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableDebugCounters().into()
        }
        unsafe extern "system" fn DisableDebugCounters<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableDebugCounters().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnableDebugCounters: EnableDebugCounters::<Identity, Impl, OFFSET>,
            DisableDebugCounters: DisableDebugCounters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionDeviceDebug as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionEffect_Impl: Sized {}
impl IDCompositionEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffect_Impl, const OFFSET: isize>() -> IDCompositionEffect_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionEffectGroup_Impl: Sized + IDCompositionEffect_Impl {
    fn SetOpacity(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()>;
    fn SetTransform3D(&self, transform3d: &::core::option::Option<IDCompositionTransform3D>) -> ::windows::core::Result<()>;
}
impl IDCompositionEffectGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>() -> IDCompositionEffectGroup_Vtbl {
        unsafe extern "system" fn SetOpacity<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform3D<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform3D(::core::mem::transmute(&transform3d)).into()
        }
        Self {
            base: IDCompositionEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, Impl, OFFSET>,
            SetTransform3D: SetTransform3D::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionEffectGroup as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionFilterEffect_Impl: Sized + IDCompositionEffect_Impl {
    fn SetInput(&self, index: u32, input: &::core::option::Option<::windows::core::IUnknown>, flags: u32) -> ::windows::core::Result<()>;
}
impl IDCompositionFilterEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffect_Impl, const OFFSET: isize>() -> IDCompositionFilterEffect_Vtbl {
        unsafe extern "system" fn SetInput<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionFilterEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInput(::core::mem::transmute_copy(&index), ::core::mem::transmute(&input), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: IDCompositionEffect_Vtbl::new::<Identity, Impl, OFFSET>(), SetInput: SetInput::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()>;
    fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>() -> IDCompositionGaussianBlurEffect_Vtbl {
        unsafe extern "system" fn SetStandardDeviation<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, Impl, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionGaussianBlurEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionHueRotationEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetAngle(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&self, amountdegrees: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionHueRotationEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>() -> IDCompositionHueRotationEffect_Vtbl {
        unsafe extern "system" fn SetAngle<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&amountdegrees)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionHueRotationEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionInkTrailDevice_Impl: Sized {
    fn CreateDelegatedInkTrail(&self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(&self, swapchain: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>;
}
impl IDCompositionInkTrailDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>() -> IDCompositionInkTrailDevice_Vtbl {
        unsafe extern "system" fn CreateDelegatedInkTrail<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDelegatedInkTrail() {
                ::core::result::Result::Ok(ok__) => {
                    *inktrail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDelegatedInkTrailForSwapChain(::core::mem::transmute(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *inktrail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDelegatedInkTrail: CreateDelegatedInkTrail::<Identity, Impl, OFFSET>,
            CreateDelegatedInkTrailForSwapChain: CreateDelegatedInkTrailForSwapChain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionInkTrailDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionLinearTransferEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetRedYIntercept(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows::core::Result<()>;
    fn SetRedSlope(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedSlope2(&self, redslope: f32) -> ::windows::core::Result<()>;
    fn SetRedDisable(&self, reddisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGreenYIntercept(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows::core::Result<()>;
    fn SetGreenSlope(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenSlope2(&self, greenslope: f32) -> ::windows::core::Result<()>;
    fn SetGreenDisable(&self, greendisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBlueYIntercept(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows::core::Result<()>;
    fn SetBlueSlope(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueSlope2(&self, blueslope: f32) -> ::windows::core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetAlphaYIntercept(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows::core::Result<()>;
    fn SetAlphaSlope(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows::core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionLinearTransferEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>() -> IDCompositionLinearTransferEffect_Vtbl {
        unsafe extern "system" fn SetRedYIntercept<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedYIntercept2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedYIntercept2(::core::mem::transmute_copy(&redyintercept)).into()
        }
        unsafe extern "system" fn SetRedSlope<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedSlope2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedSlope2(::core::mem::transmute_copy(&redslope)).into()
        }
        unsafe extern "system" fn SetRedDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenYIntercept2(::core::mem::transmute_copy(&greenyintercept)).into()
        }
        unsafe extern "system" fn SetGreenSlope<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenSlope2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenSlope2(::core::mem::transmute_copy(&greenslope)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueYIntercept2(::core::mem::transmute_copy(&blueyintercept)).into()
        }
        unsafe extern "system" fn SetBlueSlope<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueSlope2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueSlope2(::core::mem::transmute_copy(&blueslope)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaYIntercept(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaYIntercept2(::core::mem::transmute_copy(&alphayintercept)).into()
        }
        unsafe extern "system" fn SetAlphaSlope<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaSlope(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaSlope2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaSlope2(::core::mem::transmute_copy(&alphaslope)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRedYIntercept: SetRedYIntercept::<Identity, Impl, OFFSET>,
            SetRedYIntercept2: SetRedYIntercept2::<Identity, Impl, OFFSET>,
            SetRedSlope: SetRedSlope::<Identity, Impl, OFFSET>,
            SetRedSlope2: SetRedSlope2::<Identity, Impl, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, Impl, OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Identity, Impl, OFFSET>,
            SetGreenYIntercept2: SetGreenYIntercept2::<Identity, Impl, OFFSET>,
            SetGreenSlope: SetGreenSlope::<Identity, Impl, OFFSET>,
            SetGreenSlope2: SetGreenSlope2::<Identity, Impl, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, Impl, OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Identity, Impl, OFFSET>,
            SetBlueYIntercept2: SetBlueYIntercept2::<Identity, Impl, OFFSET>,
            SetBlueSlope: SetBlueSlope::<Identity, Impl, OFFSET>,
            SetBlueSlope2: SetBlueSlope2::<Identity, Impl, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, Impl, OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Identity, Impl, OFFSET>,
            SetAlphaYIntercept2: SetAlphaYIntercept2::<Identity, Impl, OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Identity, Impl, OFFSET>,
            SetAlphaSlope2: SetAlphaSlope2::<Identity, Impl, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionLinearTransferEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>() -> IDCompositionMatrixTransform_Vtbl {
        unsafe extern "system" fn SetMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDCompositionMatrixTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetMatrix(&self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDCompositionMatrixTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>() -> IDCompositionMatrixTransform3D_Vtbl {
        unsafe extern "system" fn SetMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixElement2(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRectangleClip_Impl: Sized + IDCompositionClip_Impl {
    fn SetLeft(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetLeft2(&self, left: f32) -> ::windows::core::Result<()>;
    fn SetTop(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTop2(&self, top: f32) -> ::windows::core::Result<()>;
    fn SetRight(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRight2(&self, right: f32) -> ::windows::core::Result<()>;
    fn SetBottom(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottom2(&self, bottom: f32) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRectangleClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>() -> IDCompositionRectangleClip_Vtbl {
        unsafe extern "system" fn SetLeft<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLeft(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetLeft2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLeft2(::core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn SetTop<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTop(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTop2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTop2(::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn SetRight<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRight(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRight2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRight2(::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottom(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottom2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottom2(::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTopRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomLeftRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomLeftRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomLeftRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomLeftRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomRightRadiusX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomRightRadiusX2(::core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomRightRadiusY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottomRightRadiusY2(::core::mem::transmute_copy(&radius)).into()
        }
        Self {
            base: IDCompositionClip_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetLeft: SetLeft::<Identity, Impl, OFFSET>,
            SetLeft2: SetLeft2::<Identity, Impl, OFFSET>,
            SetTop: SetTop::<Identity, Impl, OFFSET>,
            SetTop2: SetTop2::<Identity, Impl, OFFSET>,
            SetRight: SetRight::<Identity, Impl, OFFSET>,
            SetRight2: SetRight2::<Identity, Impl, OFFSET>,
            SetBottom: SetBottom::<Identity, Impl, OFFSET>,
            SetBottom2: SetBottom2::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusX2: SetTopLeftRadiusX2::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusY2: SetTopLeftRadiusY2::<Identity, Impl, OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Identity, Impl, OFFSET>,
            SetTopRightRadiusX2: SetTopRightRadiusX2::<Identity, Impl, OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Identity, Impl, OFFSET>,
            SetTopRightRadiusY2: SetTopRightRadiusY2::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusX2: SetBottomLeftRadiusX2::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusY2: SetBottomLeftRadiusY2::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusX2: SetBottomRightRadiusX2::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusY2: SetBottomRightRadiusY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRectangleClip as ::windows::core::Interface>::IID || iid == &<IDCompositionClip as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetAngle(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRotateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>() -> IDCompositionRotateTransform_Vtbl {
        unsafe extern "system" fn SetAngle<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetAngle(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()>;
    fn SetAxisX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisX2(&self, axisx: f32) -> ::windows::core::Result<()>;
    fn SetAxisY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisY2(&self, axisy: f32) -> ::windows::core::Result<()>;
    fn SetAxisZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAxisZ2(&self, axisz: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()>;
    fn SetCenterZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionRotateTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>() -> IDCompositionRotateTransform3D_Vtbl {
        unsafe extern "system" fn SetAngle<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngle2(::core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetAxisX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisX2(::core::mem::transmute_copy(&axisx)).into()
        }
        unsafe extern "system" fn SetAxisY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisY2(::core::mem::transmute_copy(&axisy)).into()
        }
        unsafe extern "system" fn SetAxisZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAxisZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAxisZ2(::core::mem::transmute_copy(&axisz)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
            SetAxisX: SetAxisX::<Identity, Impl, OFFSET>,
            SetAxisX2: SetAxisX2::<Identity, Impl, OFFSET>,
            SetAxisY: SetAxisY::<Identity, Impl, OFFSET>,
            SetAxisY2: SetAxisY2::<Identity, Impl, OFFSET>,
            SetAxisZ: SetAxisZ::<Identity, Impl, OFFSET>,
            SetAxisZ2: SetAxisZ2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, Impl, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSaturationEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetSaturation(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetSaturation2(&self, ratio: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSaturationEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: isize>() -> IDCompositionSaturationEffect_Vtbl {
        unsafe extern "system" fn SetSaturation<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSaturation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetSaturation2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSaturation2(::core::mem::transmute_copy(&ratio)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSaturation: SetSaturation::<Identity, Impl, OFFSET>,
            SetSaturation2: SetSaturation2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSaturationEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetScaleX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()>;
    fn SetScaleY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionScaleTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>() -> IDCompositionScaleTransform_Vtbl {
        unsafe extern "system" fn SetScaleX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, Impl, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, Impl, OFFSET>,
            SetScaleY: SetScaleY::<Identity, Impl, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetScaleX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()>;
    fn SetScaleY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()>;
    fn SetScaleZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetScaleZ2(&self, scalez: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()>;
    fn SetCenterZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionScaleTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>() -> IDCompositionScaleTransform3D_Vtbl {
        unsafe extern "system" fn SetScaleX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleX2(::core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleY2(::core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetScaleZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetScaleZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleZ2(::core::mem::transmute_copy(&scalez)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterZ2(::core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, Impl, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, Impl, OFFSET>,
            SetScaleY: SetScaleY::<Identity, Impl, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, Impl, OFFSET>,
            SetScaleZ: SetScaleZ::<Identity, Impl, OFFSET>,
            SetScaleZ2: SetScaleZ2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, Impl, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionShadowEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()>;
    fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()>;
    fn SetRed(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRed2(&self, amount: f32) -> ::windows::core::Result<()>;
    fn SetGreen(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreen2(&self, amount: f32) -> ::windows::core::Result<()>;
    fn SetBlue(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlue2(&self, amount: f32) -> ::windows::core::Result<()>;
    fn SetAlpha(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlpha2(&self, amount: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionShadowEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>() -> IDCompositionShadowEffect_Vtbl {
        unsafe extern "system" fn SetStandardDeviation<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStandardDeviation(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStandardDeviation2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetColor<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetRed<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRed(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRed2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRed2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetGreen<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreen(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreen2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreen2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBlue<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlue(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlue2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlue2(::core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetAlpha<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlpha(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlpha2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionShadowEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlpha2(::core::mem::transmute_copy(&amount)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, Impl, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            SetRed: SetRed::<Identity, Impl, OFFSET>,
            SetRed2: SetRed2::<Identity, Impl, OFFSET>,
            SetGreen: SetGreen::<Identity, Impl, OFFSET>,
            SetGreen2: SetGreen2::<Identity, Impl, OFFSET>,
            SetBlue: SetBlue::<Identity, Impl, OFFSET>,
            SetBlue2: SetBlue2::<Identity, Impl, OFFSET>,
            SetAlpha: SetAlpha::<Identity, Impl, OFFSET>,
            SetAlpha2: SetAlpha2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionShadowEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionSkewTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetAngleX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleX2(&self, anglex: f32) -> ::windows::core::Result<()>;
    fn SetAngleY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAngleY2(&self, angley: f32) -> ::windows::core::Result<()>;
    fn SetCenterX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()>;
    fn SetCenterY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionSkewTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>() -> IDCompositionSkewTransform_Vtbl {
        unsafe extern "system" fn SetAngleX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngleX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngleX2(::core::mem::transmute_copy(&anglex)).into()
        }
        unsafe extern "system" fn SetAngleY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngleY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAngleY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAngleY2(::core::mem::transmute_copy(&angley)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterX2(::core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSkewTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCenterY2(::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAngleX: SetAngleX::<Identity, Impl, OFFSET>,
            SetAngleX2: SetAngleX2::<Identity, Impl, OFFSET>,
            SetAngleY: SetAngleY::<Identity, Impl, OFFSET>,
            SetAngleY2: SetAngleY2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSkewTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionSurface_Impl: Sized {
    fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&self) -> ::windows::core::Result<()>;
    fn ResumeDraw(&self) -> ::windows::core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>() -> IDCompositionSurface_Vtbl {
        unsafe extern "system" fn BeginDraw<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResumeDraw().into()
        }
        unsafe extern "system" fn Scroll<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionSurfaceFactory_Impl: Sized {
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>() -> IDCompositionSurfaceFactory_Vtbl {
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSurface(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *surface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVirtualSurface(::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    *virtualsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionTableTransferEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetRedTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetGreenTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetBlueTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetAlphaTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()>;
    fn SetRedDisable(&self, reddisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGreenDisable(&self, greendisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetRedTableValue(&self, index: u32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetGreenTableValue(&self, index: u32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetBlueTableValue(&self, index: u32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()>;
    fn SetAlphaTableValue(&self, index: u32, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionTableTransferEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>() -> IDCompositionTableTransferEffect_Vtbl {
        unsafe extern "system" fn SetRedTable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetGreenTable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBlueTable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetAlphaTable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaTable(::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetRedDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedDisable(::core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenDisable(::core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueDisable(::core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaDisable(::core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClampOutput(::core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetRedTableValue<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetRedTableValue2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRedTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetGreenTableValue<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetGreenTableValue2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGreenTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetBlueTableValue<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetBlueTableValue2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlueTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaTableValue(::core::mem::transmute_copy(&index), ::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlphaTableValue2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRedTable: SetRedTable::<Identity, Impl, OFFSET>,
            SetGreenTable: SetGreenTable::<Identity, Impl, OFFSET>,
            SetBlueTable: SetBlueTable::<Identity, Impl, OFFSET>,
            SetAlphaTable: SetAlphaTable::<Identity, Impl, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, Impl, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, Impl, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, Impl, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
            SetRedTableValue: SetRedTableValue::<Identity, Impl, OFFSET>,
            SetRedTableValue2: SetRedTableValue2::<Identity, Impl, OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Identity, Impl, OFFSET>,
            SetGreenTableValue2: SetGreenTableValue2::<Identity, Impl, OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Identity, Impl, OFFSET>,
            SetBlueTableValue2: SetBlueTableValue2::<Identity, Impl, OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Identity, Impl, OFFSET>,
            SetAlphaTableValue2: SetAlphaTableValue2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTableTransferEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTarget_Impl: Sized {
    fn SetRoot(&self, visual: &::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
}
impl IDCompositionTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTarget_Impl, const OFFSET: isize>() -> IDCompositionTarget_Vtbl {
        unsafe extern "system" fn SetRoot<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRoot(::core::mem::transmute(&visual)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetRoot: SetRoot::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTarget as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {}
impl IDCompositionTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform_Impl, const OFFSET: isize>() -> IDCompositionTransform_Vtbl {
        Self { base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTransform3D_Impl: Sized + IDCompositionEffect_Impl {}
impl IDCompositionTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTransform3D_Impl, const OFFSET: isize>() -> IDCompositionTransform3D_Vtbl {
        Self { base: IDCompositionEffect_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl + IDCompositionTransform_Impl {
    fn SetOffsetX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>() -> IDCompositionTranslateTransform_Vtbl {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        Self {
            base: IDCompositionTransform_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform3D_Impl: Sized + IDCompositionEffect_Impl + IDCompositionTransform3D_Impl {
    fn SetOffsetX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()>;
    fn SetOffsetZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()>;
}
impl IDCompositionTranslateTransform3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>() -> IDCompositionTranslateTransform3D_Vtbl {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
        }
        Self {
            base: IDCompositionTransform3D_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, Impl, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform3D as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionTransform3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionTurbulenceEffect_Impl: Sized + IDCompositionEffect_Impl + IDCompositionFilterEffect_Impl {
    fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()>;
    fn SetNumOctaves(&self, numoctaves: u32) -> ::windows::core::Result<()>;
    fn SetSeed(&self, seed: u32) -> ::windows::core::Result<()>;
    fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()>;
    fn SetStitchable(&self, stitchable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionTurbulenceEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>() -> IDCompositionTurbulenceEffect_Vtbl {
        unsafe extern "system" fn SetOffset<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetBaseFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBaseFrequency(::core::mem::transmute_copy(&frequency)).into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SetNumOctaves<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumOctaves(::core::mem::transmute_copy(&numoctaves)).into()
        }
        unsafe extern "system" fn SetSeed<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeed(::core::mem::transmute_copy(&seed)).into()
        }
        unsafe extern "system" fn SetNoise<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNoise(::core::mem::transmute_copy(&noise)).into()
        }
        unsafe extern "system" fn SetStitchable<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStitchable(::core::mem::transmute_copy(&stitchable)).into()
        }
        Self {
            base: IDCompositionFilterEffect_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            SetBaseFrequency: SetBaseFrequency::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SetNumOctaves: SetNumOctaves::<Identity, Impl, OFFSET>,
            SetSeed: SetSeed::<Identity, Impl, OFFSET>,
            SetNoise: SetNoise::<Identity, Impl, OFFSET>,
            SetStitchable: SetStitchable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionTurbulenceEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionEffect as ::windows::core::Interface>::IID || iid == &<IDCompositionFilterEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionVirtualSurface_Impl: Sized + IDCompositionSurface_Impl {
    fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn Trim(&self, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDCompositionVirtualSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: isize>() -> IDCompositionVirtualSurface_Vtbl {
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Trim<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Trim(::core::mem::transmute_copy(&rectangles), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base: IDCompositionSurface_Vtbl::new::<Identity, Impl, OFFSET>(), Resize: Resize::<Identity, Impl, OFFSET>, Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVirtualSurface as ::windows::core::Interface>::IID || iid == &<IDCompositionSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual_Impl: Sized {
    fn SetOffsetX(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()>;
    fn SetOffsetY(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()>;
    fn SetTransform(&self, transform: &::core::option::Option<IDCompositionTransform>) -> ::windows::core::Result<()>;
    fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetTransformParent(&self, visual: &::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn SetEffect(&self, effect: &::core::option::Option<IDCompositionEffect>) -> ::windows::core::Result<()>;
    fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()>;
    fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()>;
    fn SetClip(&self, clip: &::core::option::Option<IDCompositionClip>) -> ::windows::core::Result<()>;
    fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()>;
    fn SetContent(&self, content: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddVisual(&self, visual: &::core::option::Option<IDCompositionVisual>, insertabove: super::super::Foundation::BOOL, referencevisual: &::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn RemoveVisual(&self, visual: &::core::option::Option<IDCompositionVisual>) -> ::windows::core::Result<()>;
    fn RemoveAllVisuals(&self) -> ::windows::core::Result<()>;
    fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>() -> IDCompositionVisual_Vtbl {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetX2(::core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetY2(::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform2(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetTransformParent<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransformParent(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn SetEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffect(::core::mem::transmute(&effect)).into()
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBitmapInterpolationMode(::core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderMode(::core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetClip<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClip(::core::mem::transmute(&clip)).into()
        }
        unsafe extern "system" fn SetClip2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClip2(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn SetContent<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn AddVisual<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddVisual(::core::mem::transmute(&visual), ::core::mem::transmute_copy(&insertabove), ::core::mem::transmute(&referencevisual)).into()
        }
        unsafe extern "system" fn RemoveVisual<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveVisual(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn RemoveAllVisuals<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllVisuals().into()
        }
        unsafe extern "system" fn SetCompositeMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositeMode(::core::mem::transmute_copy(&compositemode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            SetTransform2: SetTransform2::<Identity, Impl, OFFSET>,
            SetTransformParent: SetTransformParent::<Identity, Impl, OFFSET>,
            SetEffect: SetEffect::<Identity, Impl, OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
            SetClip: SetClip::<Identity, Impl, OFFSET>,
            SetClip2: SetClip2::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            AddVisual: AddVisual::<Identity, Impl, OFFSET>,
            RemoveVisual: RemoveVisual::<Identity, Impl, OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Identity, Impl, OFFSET>,
            SetCompositeMode: SetCompositeMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual2_Impl: Sized + IDCompositionVisual_Impl {
    fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()>;
    fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2_Impl, const OFFSET: isize>() -> IDCompositionVisual2_Vtbl {
        unsafe extern "system" fn SetOpacityMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpacityMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetBackFaceVisibility<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackFaceVisibility(::core::mem::transmute_copy(&visibility)).into()
        }
        Self {
            base: IDCompositionVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOpacityMode: SetOpacityMode::<Identity, Impl, OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual2 as ::windows::core::Interface>::IID || iid == &<IDCompositionVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual3_Impl: Sized + IDCompositionVisual_Impl + IDCompositionVisual2_Impl + IDCompositionVisualDebug_Impl {
    fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::Result<()>;
    fn SetOffsetZ(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()>;
    fn SetOpacity(&self, animation: &::core::option::Option<IDCompositionAnimation>) -> ::windows::core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()>;
    fn SetTransform3(&self, transform: &::core::option::Option<IDCompositionTransform3D>) -> ::windows::core::Result<()>;
    fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::Result<()>;
    fn SetVisible(&self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>() -> IDCompositionVisual3_Vtbl {
        unsafe extern "system" fn SetDepthMode<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDepthMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetZ(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOffsetZ2(::core::mem::transmute_copy(&offsetz)).into()
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpacity2(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform3<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform3(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn SetTransform4<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform4(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&visible)).into()
        }
        Self {
            base: IDCompositionVisualDebug_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDepthMode: SetDepthMode::<Identity, Impl, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, Impl, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, Impl, OFFSET>,
            SetTransform3: SetTransform3::<Identity, Impl, OFFSET>,
            SetTransform4: SetTransform4::<Identity, Impl, OFFSET>,
            SetVisible: SetVisible::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisual3 as ::windows::core::Interface>::IID || iid == &<IDCompositionVisual as ::windows::core::Interface>::IID || iid == &<IDCompositionVisual2 as ::windows::core::Interface>::IID || iid == &<IDCompositionVisualDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisualDebug_Impl: Sized + IDCompositionVisual_Impl + IDCompositionVisual2_Impl {
    fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()>;
    fn DisableHeatMap(&self) -> ::windows::core::Result<()>;
    fn EnableRedrawRegions(&self) -> ::windows::core::Result<()>;
    fn DisableRedrawRegions(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>() -> IDCompositionVisualDebug_Vtbl {
        unsafe extern "system" fn EnableHeatMap<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableHeatMap(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn DisableHeatMap<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableHeatMap().into()
        }
        unsafe extern "system" fn EnableRedrawRegions<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableRedrawRegions().into()
        }
        unsafe extern "system" fn DisableRedrawRegions<Identity: ::windows::core::IUnknownImpl, Impl: IDCompositionVisualDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableRedrawRegions().into()
        }
        Self {
            base: IDCompositionVisual2_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableHeatMap: EnableHeatMap::<Identity, Impl, OFFSET>,
            DisableHeatMap: DisableHeatMap::<Identity, Impl, OFFSET>,
            EnableRedrawRegions: EnableRedrawRegions::<Identity, Impl, OFFSET>,
            DisableRedrawRegions: DisableRedrawRegions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDCompositionVisualDebug as ::windows::core::Interface>::IID || iid == &<IDCompositionVisual as ::windows::core::Interface>::IID || iid == &<IDCompositionVisual2 as ::windows::core::Interface>::IID
    }
}
