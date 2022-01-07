pub trait ID2D1AnalysisTransformImpl: Sized {
    fn ProcessAnalysisResults();
}
impl ::windows::core::RuntimeName for ID2D1AnalysisTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1AnalysisTransform";
}
impl ID2D1AnalysisTransformVtbl {
    pub const fn new<Impl: ID2D1AnalysisTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1AnalysisTransformVtbl {
        unsafe extern "system" fn ProcessAnalysisResults<Impl: ID2D1AnalysisTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysisdata: *const u8, analysisdatacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessAnalysisResults(analysisdata, analysisdatacount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1AnalysisTransform>, base.5, ProcessAnalysisResults::<Impl, OFFSET>)
    }
}
pub trait ID2D1BitmapImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSize();
    fn GetPixelSize();
    fn GetPixelFormat();
    fn GetDpi();
    fn CopyFromBitmap();
    fn CopyFromRenderTarget();
    fn CopyFromMemory();
}
impl ::windows::core::RuntimeName for ID2D1Bitmap {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Bitmap";
}
impl ID2D1BitmapVtbl {
    pub const fn new<Impl: ID2D1BitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BitmapVtbl {
        unsafe extern "system" fn GetSize<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelSize<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDpi<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDpi(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn CopyFromBitmap<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, bitmap: ::windows::core::RawPtr, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromBitmap(&*(&destpoint as *const <Common::D2D_POINT_2U as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2U as ::windows::core::DefaultType>::DefaultType), &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType), &*(&srcrect as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromRenderTarget<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, rendertarget: ::windows::core::RawPtr, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromRenderTarget(&*(&destpoint as *const <Common::D2D_POINT_2U as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2U as ::windows::core::DefaultType>::DefaultType), &*(&rendertarget as *const <ID2D1RenderTarget as ::windows::core::Abi>::Abi as *const <ID2D1RenderTarget as ::windows::core::DefaultType>::DefaultType), &*(&srcrect as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromMemory<Impl: ID2D1BitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromMemory(&*(&dstrect as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType), &*(&srcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pitch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Bitmap>, base.5, GetSize::<Impl, OFFSET>, GetPixelSize::<Impl, OFFSET>, GetPixelFormat::<Impl, OFFSET>, GetDpi::<Impl, OFFSET>, CopyFromBitmap::<Impl, OFFSET>, CopyFromRenderTarget::<Impl, OFFSET>, CopyFromMemory::<Impl, OFFSET>)
    }
}
pub trait ID2D1Bitmap1Impl: Sized + ID2D1BitmapImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetColorContext();
    fn GetOptions();
    fn GetSurface();
    fn Map();
    fn Unmap();
}
impl ::windows::core::RuntimeName for ID2D1Bitmap1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Bitmap1";
}
impl ID2D1Bitmap1Vtbl {
    pub const fn new<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Bitmap1Vtbl {
        unsafe extern "system" fn GetColorContext<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorcontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetColorContext(::core::mem::transmute_copy(&colorcontext)).into()
        }
        unsafe extern "system" fn GetOptions<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_OPTIONS {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurface<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgisurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSurface(::core::mem::transmute_copy(&dxgisurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Map(options, ::core::mem::transmute_copy(&mappedrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID2D1Bitmap1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Bitmap1>, base.5, GetColorContext::<Impl, OFFSET>, GetOptions::<Impl, OFFSET>, GetSurface::<Impl, OFFSET>, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>)
    }
}
pub trait ID2D1BitmapBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn SetInterpolationMode();
    fn SetBitmap();
    fn GetExtendModeX();
    fn GetExtendModeY();
    fn GetInterpolationMode();
    fn GetBitmap();
}
impl ::windows::core::RuntimeName for ID2D1BitmapBrush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BitmapBrush";
}
impl ID2D1BitmapBrushVtbl {
    pub const fn new<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BitmapBrushVtbl {
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeX(extendmodex).into()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeY(extendmodey).into()
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(interpolationmode).into()
        }
        unsafe extern "system" fn SetBitmap<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitmap(&*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterpolationMode<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmap<Impl: ID2D1BitmapBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetBitmap(::core::mem::transmute_copy(&bitmap)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BitmapBrush>, base.5, SetExtendModeX::<Impl, OFFSET>, SetExtendModeY::<Impl, OFFSET>, SetInterpolationMode::<Impl, OFFSET>, SetBitmap::<Impl, OFFSET>, GetExtendModeX::<Impl, OFFSET>, GetExtendModeY::<Impl, OFFSET>, GetInterpolationMode::<Impl, OFFSET>, GetBitmap::<Impl, OFFSET>)
    }
}
pub trait ID2D1BitmapBrush1Impl: Sized + ID2D1BitmapBrushImpl + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetInterpolationMode1();
    fn GetInterpolationMode1();
}
impl ::windows::core::RuntimeName for ID2D1BitmapBrush1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BitmapBrush1";
}
impl ID2D1BitmapBrush1Vtbl {
    pub const fn new<Impl: ID2D1BitmapBrush1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BitmapBrush1Vtbl {
        unsafe extern "system" fn SetInterpolationMode1<Impl: ID2D1BitmapBrush1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode1(interpolationmode).into()
        }
        unsafe extern "system" fn GetInterpolationMode1<Impl: ID2D1BitmapBrush1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterpolationMode1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BitmapBrush1>, base.5, SetInterpolationMode1::<Impl, OFFSET>, GetInterpolationMode1::<Impl, OFFSET>)
    }
}
pub trait ID2D1BitmapRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn GetBitmap();
}
impl ::windows::core::RuntimeName for ID2D1BitmapRenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BitmapRenderTarget";
}
impl ID2D1BitmapRenderTargetVtbl {
    pub const fn new<Impl: ID2D1BitmapRenderTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BitmapRenderTargetVtbl {
        unsafe extern "system" fn GetBitmap<Impl: ID2D1BitmapRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBitmap(::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BitmapRenderTarget>, base.5, GetBitmap::<Impl, OFFSET>)
    }
}
pub trait ID2D1BlendTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetDescription();
    fn GetDescription();
}
impl ::windows::core::RuntimeName for ID2D1BlendTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BlendTransform";
}
impl ID2D1BlendTransformVtbl {
    pub const fn new<Impl: ID2D1BlendTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BlendTransformVtbl {
        unsafe extern "system" fn SetDescription<Impl: ID2D1BlendTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *const D2D1_BLEND_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&description as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: ID2D1BlendTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut D2D1_BLEND_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDescription(::core::mem::transmute_copy(&description)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BlendTransform>, base.5, SetDescription::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>)
    }
}
pub trait ID2D1BorderTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn GetExtendModeX();
    fn GetExtendModeY();
}
impl ::windows::core::RuntimeName for ID2D1BorderTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BorderTransform";
}
impl ID2D1BorderTransformVtbl {
    pub const fn new<Impl: ID2D1BorderTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BorderTransformVtbl {
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1BorderTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeX(extendmode).into()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1BorderTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeY(extendmode).into()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1BorderTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1BorderTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BorderTransform>, base.5, SetExtendModeX::<Impl, OFFSET>, SetExtendModeY::<Impl, OFFSET>, GetExtendModeX::<Impl, OFFSET>, GetExtendModeY::<Impl, OFFSET>)
    }
}
pub trait ID2D1BoundsAdjustmentTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBounds();
    fn GetOutputBounds();
}
impl ::windows::core::RuntimeName for ID2D1BoundsAdjustmentTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1BoundsAdjustmentTransform";
}
impl ID2D1BoundsAdjustmentTransformVtbl {
    pub const fn new<Impl: ID2D1BoundsAdjustmentTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BoundsAdjustmentTransformVtbl {
        unsafe extern "system" fn SetOutputBounds<Impl: ID2D1BoundsAdjustmentTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputbounds: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutputBounds(&*(&outputbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetOutputBounds<Impl: ID2D1BoundsAdjustmentTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputbounds: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetOutputBounds(::core::mem::transmute_copy(&outputbounds)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1BoundsAdjustmentTransform>, base.5, SetOutputBounds::<Impl, OFFSET>, GetOutputBounds::<Impl, OFFSET>)
    }
}
pub trait ID2D1BrushImpl: Sized + ID2D1ResourceImpl {
    fn SetOpacity();
    fn SetTransform();
    fn GetOpacity();
    fn GetTransform();
}
impl ::windows::core::RuntimeName for ID2D1Brush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Brush";
}
impl ID2D1BrushVtbl {
    pub const fn new<Impl: ID2D1BrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1BrushVtbl {
        unsafe extern "system" fn SetOpacity<Impl: ID2D1BrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpacity(opacity).into()
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1BrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetOpacity<Impl: ID2D1BrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1BrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&transform)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Brush>, base.5, SetOpacity::<Impl, OFFSET>, SetTransform::<Impl, OFFSET>, GetOpacity::<Impl, OFFSET>, GetTransform::<Impl, OFFSET>)
    }
}
pub trait ID2D1ColorContextImpl: Sized + ID2D1ResourceImpl {
    fn GetColorSpace();
    fn GetProfileSize();
    fn GetProfile();
}
impl ::windows::core::RuntimeName for ID2D1ColorContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ColorContext";
}
impl ID2D1ColorContextVtbl {
    pub const fn new<Impl: ID2D1ColorContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ColorContextVtbl {
        unsafe extern "system" fn GetColorSpace<Impl: ID2D1ColorContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileSize<Impl: ID2D1ColorContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProfileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Impl: ID2D1ColorContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: *mut u8, profilesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProfile(::core::mem::transmute_copy(&profile), profilesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ColorContext>, base.5, GetColorSpace::<Impl, OFFSET>, GetProfileSize::<Impl, OFFSET>, GetProfile::<Impl, OFFSET>)
    }
}
pub trait ID2D1ColorContext1Impl: Sized + ID2D1ColorContextImpl + ID2D1ResourceImpl {
    fn GetColorContextType();
    fn GetDXGIColorSpace();
    fn GetSimpleColorProfile();
}
impl ::windows::core::RuntimeName for ID2D1ColorContext1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ColorContext1";
}
impl ID2D1ColorContext1Vtbl {
    pub const fn new<Impl: ID2D1ColorContext1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ColorContext1Vtbl {
        unsafe extern "system" fn GetColorContextType<Impl: ID2D1ColorContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorContextType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDXGIColorSpace<Impl: ID2D1ColorContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDXGIColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimpleColorProfile<Impl: ID2D1ColorContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSimpleColorProfile(::core::mem::transmute_copy(&simpleprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ColorContext1>, base.5, GetColorContextType::<Impl, OFFSET>, GetDXGIColorSpace::<Impl, OFFSET>, GetSimpleColorProfile::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandListImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn Stream();
    fn Close();
}
impl ::windows::core::RuntimeName for ID2D1CommandList {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandList";
}
impl ID2D1CommandListVtbl {
    pub const fn new<Impl: ID2D1CommandListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandListVtbl {
        unsafe extern "system" fn Stream<Impl: ID2D1CommandListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stream(&*(&sink as *const <ID2D1CommandSink as ::windows::core::Abi>::Abi as *const <ID2D1CommandSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ID2D1CommandListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandList>, base.5, Stream::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandSinkImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn SetAntialiasMode();
    fn SetTags();
    fn SetTextAntialiasMode();
    fn SetTextRenderingParams();
    fn SetTransform();
    fn SetPrimitiveBlend();
    fn SetUnitMode();
    fn Clear();
    fn DrawGlyphRun();
    fn DrawLine();
    fn DrawGeometry();
    fn DrawRectangle();
    fn DrawBitmap();
    fn DrawImage();
    fn DrawGdiMetafile();
    fn FillMesh();
    fn FillOpacityMask();
    fn FillGeometry();
    fn FillRectangle();
    fn PushAxisAlignedClip();
    fn PushLayer();
    fn PopAxisAlignedClip();
    fn PopLayer();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink";
}
impl ID2D1CommandSinkVtbl {
    pub const fn new<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSinkVtbl {
        unsafe extern "system" fn BeginDraw<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDraw<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntialiasMode<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAntialiasMode(antialiasmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTags<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTags(tag1, tag2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTextAntialiasMode(textantialiasmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTextRenderingParams(&*(&textrenderingparams as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimitiveBlend<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrimitiveBlend(primitiveblend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnitMode<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUnitMode(unitmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear(&*(&color as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawGlyphRun(
                &*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrundescription as *const <super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                &*(&foregroundbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                measuringmode,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawLine<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawLine(
                &*(&point0 as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&point1 as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                strokewidth,
                &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGeometry<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawGeometry(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawRectangle<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawRectangle(&*(&rect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawBitmap(
                &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                opacity,
                interpolationmode,
                &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                &*(&perspectivetransform as *const <Common::D2D_MATRIX_4X4_F as ::windows::core::Abi>::Abi as *const <Common::D2D_MATRIX_4X4_F as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawImage<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawImage(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&imagerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), interpolationmode, compositemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawGdiMetafile(&*(&gdimetafile as *const <ID2D1GdiMetafile as ::windows::core::Abi>::Abi as *const <ID2D1GdiMetafile as ::windows::core::DefaultType>::DefaultType), &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillMesh<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillMesh(&*(&mesh as *const <ID2D1Mesh as ::windows::core::Abi>::Abi as *const <ID2D1Mesh as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillOpacityMask(
                &*(&opacitymask as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillGeometry<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, opacitybrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillGeometry(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), &*(&opacitybrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillRectangle<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillRectangle(&*(&rect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushAxisAlignedClip<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PushAxisAlignedClip(&*(&cliprect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), antialiasmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PushLayer(&*(&layerparameters1 as *const <D2D1_LAYER_PARAMETERS1 as ::windows::core::Abi>::Abi as *const <D2D1_LAYER_PARAMETERS1 as ::windows::core::DefaultType>::DefaultType), &*(&layer as *const <ID2D1Layer as ::windows::core::Abi>::Abi as *const <ID2D1Layer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopAxisAlignedClip<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PopAxisAlignedClip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopLayer<Impl: ID2D1CommandSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PopLayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1CommandSink>,
            base.5,
            BeginDraw::<Impl, OFFSET>,
            EndDraw::<Impl, OFFSET>,
            SetAntialiasMode::<Impl, OFFSET>,
            SetTags::<Impl, OFFSET>,
            SetTextAntialiasMode::<Impl, OFFSET>,
            SetTextRenderingParams::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            SetPrimitiveBlend::<Impl, OFFSET>,
            SetUnitMode::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            DrawGlyphRun::<Impl, OFFSET>,
            DrawLine::<Impl, OFFSET>,
            DrawGeometry::<Impl, OFFSET>,
            DrawRectangle::<Impl, OFFSET>,
            DrawBitmap::<Impl, OFFSET>,
            DrawImage::<Impl, OFFSET>,
            DrawGdiMetafile::<Impl, OFFSET>,
            FillMesh::<Impl, OFFSET>,
            FillOpacityMask::<Impl, OFFSET>,
            FillGeometry::<Impl, OFFSET>,
            FillRectangle::<Impl, OFFSET>,
            PushAxisAlignedClip::<Impl, OFFSET>,
            PushLayer::<Impl, OFFSET>,
            PopAxisAlignedClip::<Impl, OFFSET>,
            PopLayer::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1CommandSink1Impl: Sized + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend1();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink1";
}
impl ID2D1CommandSink1Vtbl {
    pub const fn new<Impl: ID2D1CommandSink1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSink1Vtbl {
        unsafe extern "system" fn SetPrimitiveBlend1<Impl: ID2D1CommandSink1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrimitiveBlend1(primitiveblend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandSink1>, base.5, SetPrimitiveBlend1::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandSink2Impl: Sized + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawInk();
    fn DrawGradientMesh();
    fn DrawGdiMetafile();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink2";
}
impl ID2D1CommandSink2Vtbl {
    pub const fn new<Impl: ID2D1CommandSink2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSink2Vtbl {
        unsafe extern "system" fn DrawInk<Impl: ID2D1CommandSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, inkstyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawInk(&*(&ink as *const <ID2D1Ink as ::windows::core::Abi>::Abi as *const <ID2D1Ink as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), &*(&inkstyle as *const <ID2D1InkStyle as ::windows::core::Abi>::Abi as *const <ID2D1InkStyle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGradientMesh<Impl: ID2D1CommandSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawGradientMesh(&*(&gradientmesh as *const <ID2D1GradientMesh as ::windows::core::Abi>::Abi as *const <ID2D1GradientMesh as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1CommandSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawGdiMetafile(&*(&gdimetafile as *const <ID2D1GdiMetafile as ::windows::core::Abi>::Abi as *const <ID2D1GdiMetafile as ::windows::core::DefaultType>::DefaultType), &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandSink2>, base.5, DrawInk::<Impl, OFFSET>, DrawGradientMesh::<Impl, OFFSET>, DrawGdiMetafile::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandSink3Impl: Sized + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawSpriteBatch();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink3";
}
impl ID2D1CommandSink3Vtbl {
    pub const fn new<Impl: ID2D1CommandSink3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSink3Vtbl {
        unsafe extern "system" fn DrawSpriteBatch<Impl: ID2D1CommandSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: ::windows::core::RawPtr, startindex: u32, spritecount: u32, bitmap: ::windows::core::RawPtr, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawSpriteBatch(&*(&spritebatch as *const <ID2D1SpriteBatch as ::windows::core::Abi>::Abi as *const <ID2D1SpriteBatch as ::windows::core::DefaultType>::DefaultType), startindex, spritecount, &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType), interpolationmode, spriteoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandSink3>, base.5, DrawSpriteBatch::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandSink4Impl: Sized + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend2();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink4";
}
impl ID2D1CommandSink4Vtbl {
    pub const fn new<Impl: ID2D1CommandSink4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSink4Vtbl {
        unsafe extern "system" fn SetPrimitiveBlend2<Impl: ID2D1CommandSink4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrimitiveBlend2(primitiveblend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandSink4>, base.5, SetPrimitiveBlend2::<Impl, OFFSET>)
    }
}
pub trait ID2D1CommandSink5Impl: Sized + ID2D1CommandSink4Impl + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn BlendImage();
}
impl ::windows::core::RuntimeName for ID2D1CommandSink5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1CommandSink5";
}
impl ID2D1CommandSink5Vtbl {
    pub const fn new<Impl: ID2D1CommandSink5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1CommandSink5Vtbl {
        unsafe extern "system" fn BlendImage<Impl: ID2D1CommandSink5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BlendImage(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), blendmode, &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&imagerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), interpolationmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1CommandSink5>, base.5, BlendImage::<Impl, OFFSET>)
    }
}
pub trait ID2D1ComputeInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetComputeShaderConstantBuffer();
    fn SetComputeShader();
    fn SetResourceTexture();
}
impl ::windows::core::RuntimeName for ID2D1ComputeInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ComputeInfo";
}
impl ID2D1ComputeInfoVtbl {
    pub const fn new<Impl: ID2D1ComputeInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ComputeInfoVtbl {
        unsafe extern "system" fn SetComputeShaderConstantBuffer<Impl: ID2D1ComputeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetComputeShaderConstantBuffer(buffer, buffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComputeShader<Impl: ID2D1ComputeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetComputeShader(&*(&shaderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceTexture<Impl: ID2D1ComputeInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetResourceTexture(textureindex, &*(&resourcetexture as *const <ID2D1ResourceTexture as ::windows::core::Abi>::Abi as *const <ID2D1ResourceTexture as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ComputeInfo>, base.5, SetComputeShaderConstantBuffer::<Impl, OFFSET>, SetComputeShader::<Impl, OFFSET>, SetResourceTexture::<Impl, OFFSET>)
    }
}
pub trait ID2D1ComputeTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetComputeInfo();
    fn CalculateThreadgroups();
}
impl ::windows::core::RuntimeName for ID2D1ComputeTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ComputeTransform";
}
impl ID2D1ComputeTransformVtbl {
    pub const fn new<Impl: ID2D1ComputeTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ComputeTransformVtbl {
        unsafe extern "system" fn SetComputeInfo<Impl: ID2D1ComputeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, computeinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetComputeInfo(&*(&computeinfo as *const <ID2D1ComputeInfo as ::windows::core::Abi>::Abi as *const <ID2D1ComputeInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateThreadgroups<Impl: ID2D1ComputeTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CalculateThreadgroups(&*(&outputrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&dimensionx), ::core::mem::transmute_copy(&dimensiony), ::core::mem::transmute_copy(&dimensionz)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ComputeTransform>, base.5, SetComputeInfo::<Impl, OFFSET>, CalculateThreadgroups::<Impl, OFFSET>)
    }
}
pub trait ID2D1ConcreteTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBuffer();
    fn SetCached();
}
impl ::windows::core::RuntimeName for ID2D1ConcreteTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ConcreteTransform";
}
impl ID2D1ConcreteTransformVtbl {
    pub const fn new<Impl: ID2D1ConcreteTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ConcreteTransformVtbl {
        unsafe extern "system" fn SetOutputBuffer<Impl: ID2D1ConcreteTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputBuffer(bufferprecision, channeldepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCached<Impl: ID2D1ConcreteTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCached(&*(&iscached as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ConcreteTransform>, base.5, SetOutputBuffer::<Impl, OFFSET>, SetCached::<Impl, OFFSET>)
    }
}
pub trait ID2D1DCRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BindDC();
}
impl ::windows::core::RuntimeName for ID2D1DCRenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DCRenderTarget";
}
impl ID2D1DCRenderTargetVtbl {
    pub const fn new<Impl: ID2D1DCRenderTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DCRenderTargetVtbl {
        unsafe extern "system" fn BindDC<Impl: ID2D1DCRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindDC(&*(&hdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), &*(&psubrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DCRenderTarget>, base.5, BindDC::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceImpl: Sized + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn CreatePrintControl();
    fn SetMaximumTextureMemory();
    fn GetMaximumTextureMemory();
    fn ClearResources();
}
impl ::windows::core::RuntimeName for ID2D1Device {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device";
}
impl ID2D1DeviceVtbl {
    pub const fn new<Impl: ID2D1DeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceVtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintControl<Impl: ID2D1DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicfactory: ::windows::core::RawPtr, documenttarget: ::windows::core::RawPtr, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePrintControl(
                &*(&wicfactory as *const <super::Imaging::IWICImagingFactory as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICImagingFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&documenttarget as *const <super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget as ::windows::core::Abi>::Abi as *const <super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget as ::windows::core::DefaultType>::DefaultType),
                &*(&printcontrolproperties as *const <D2D1_PRINT_CONTROL_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_PRINT_CONTROL_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&printcontrol),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumTextureMemory<Impl: ID2D1DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaximumTextureMemory(maximuminbytes).into()
        }
        unsafe extern "system" fn GetMaximumTextureMemory<Impl: ID2D1DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumTextureMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearResources<Impl: ID2D1DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, millisecondssinceuse: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearResources(millisecondssinceuse).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device>, base.5, CreateDeviceContext::<Impl, OFFSET>, CreatePrintControl::<Impl, OFFSET>, SetMaximumTextureMemory::<Impl, OFFSET>, GetMaximumTextureMemory::<Impl, OFFSET>, ClearResources::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device1Impl: Sized + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn GetRenderingPriority();
    fn SetRenderingPriority();
    fn CreateDeviceContext();
}
impl ::windows::core::RuntimeName for ID2D1Device1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device1";
}
impl ID2D1Device1Vtbl {
    pub const fn new<Impl: ID2D1Device1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device1Vtbl {
        unsafe extern "system" fn GetRenderingPriority<Impl: ID2D1Device1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_RENDERING_PRIORITY {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderingPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingPriority<Impl: ID2D1Device1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRenderingPriority(renderingpriority).into()
        }
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device1>, base.5, GetRenderingPriority::<Impl, OFFSET>, SetRenderingPriority::<Impl, OFFSET>, CreateDeviceContext::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device2Impl: Sized + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn FlushDeviceContexts();
    fn GetDxgiDevice();
}
impl ::windows::core::RuntimeName for ID2D1Device2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device2";
}
impl ID2D1Device2Vtbl {
    pub const fn new<Impl: ID2D1Device2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device2Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushDeviceContexts<Impl: ID2D1Device2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FlushDeviceContexts(&*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDxgiDevice<Impl: ID2D1Device2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDxgiDevice(::core::mem::transmute_copy(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device2>, base.5, CreateDeviceContext::<Impl, OFFSET>, FlushDeviceContexts::<Impl, OFFSET>, GetDxgiDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device3Impl: Sized + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
impl ::windows::core::RuntimeName for ID2D1Device3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device3";
}
impl ID2D1Device3Vtbl {
    pub const fn new<Impl: ID2D1Device3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device3Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext3)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device3>, base.5, CreateDeviceContext::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device4Impl: Sized + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn SetMaximumColorGlyphCacheMemory();
    fn GetMaximumColorGlyphCacheMemory();
}
impl ::windows::core::RuntimeName for ID2D1Device4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device4";
}
impl ID2D1Device4Vtbl {
    pub const fn new<Impl: ID2D1Device4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device4Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext4)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumColorGlyphCacheMemory<Impl: ID2D1Device4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaximumColorGlyphCacheMemory(maximuminbytes).into()
        }
        unsafe extern "system" fn GetMaximumColorGlyphCacheMemory<Impl: ID2D1Device4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumColorGlyphCacheMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device4>, base.5, CreateDeviceContext::<Impl, OFFSET>, SetMaximumColorGlyphCacheMemory::<Impl, OFFSET>, GetMaximumColorGlyphCacheMemory::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device5Impl: Sized + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
impl ::windows::core::RuntimeName for ID2D1Device5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device5";
}
impl ID2D1Device5Vtbl {
    pub const fn new<Impl: ID2D1Device5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device5Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext5)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device5>, base.5, CreateDeviceContext::<Impl, OFFSET>)
    }
}
pub trait ID2D1Device6Impl: Sized + ID2D1Device5Impl + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
impl ::windows::core::RuntimeName for ID2D1Device6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Device6";
}
impl ID2D1Device6Vtbl {
    pub const fn new<Impl: ID2D1Device6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Device6Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContext(options, ::core::mem::transmute_copy(&devicecontext6)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Device6>, base.5, CreateDeviceContext::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceContextImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateBitmap();
    fn CreateBitmapFromWicBitmap();
    fn CreateColorContext();
    fn CreateColorContextFromFilename();
    fn CreateColorContextFromWicColorContext();
    fn CreateBitmapFromDxgiSurface();
    fn CreateEffect();
    fn CreateGradientStopCollection();
    fn CreateImageBrush();
    fn CreateBitmapBrush();
    fn CreateCommandList();
    fn IsDxgiFormatSupported();
    fn IsBufferPrecisionSupported();
    fn GetImageLocalBounds();
    fn GetImageWorldBounds();
    fn GetGlyphRunWorldBounds();
    fn GetDevice();
    fn SetTarget();
    fn GetTarget();
    fn SetRenderingControls();
    fn GetRenderingControls();
    fn SetPrimitiveBlend();
    fn GetPrimitiveBlend();
    fn SetUnitMode();
    fn GetUnitMode();
    fn DrawGlyphRun();
    fn DrawImage();
    fn DrawGdiMetafile();
    fn DrawBitmap();
    fn PushLayer();
    fn InvalidateEffectInputRectangle();
    fn GetEffectInvalidRectangleCount();
    fn GetEffectInvalidRectangles();
    fn GetEffectRequiredInputRectangles();
    fn FillOpacityMask();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext";
}
impl ID2D1DeviceContextVtbl {
    pub const fn new<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContextVtbl {
        unsafe extern "system" fn CreateBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmap(
                &*(&size as *const <Common::D2D_SIZE_U as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_U as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcedata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                pitch,
                &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bitmap),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromWicBitmap(&*(&wicbitmapsource as *const <super::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContext(space, profile, profilesize, ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromFilename(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wiccolorcontext: ::windows::core::RawPtr, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromWicColorContext(&*(&wiccolorcontext as *const <super::Imaging::IWICColorContext as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICColorContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromDxgiSurface<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromDxgiSurface(&*(&surface as *const <super::Dxgi::IDXGISurface as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISurface as ::windows::core::DefaultType>::DefaultType), &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffect<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEffect(&*(&effectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&effect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGradientStopCollection(&*(&straightalphagradientstops as *const <D2D1_GRADIENT_STOP as ::windows::core::Abi>::Abi as *const <D2D1_GRADIENT_STOP as ::windows::core::DefaultType>::DefaultType), straightalphagradientstopscount, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, ::core::mem::transmute_copy(&gradientstopcollection1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateImageBrush(
                &*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType),
                &*(&imagebrushproperties as *const <D2D1_IMAGE_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_IMAGE_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&imagebrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmapBrush(
                &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                &*(&bitmapbrushproperties as *const <D2D1_BITMAP_BRUSH_PROPERTIES1 as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_BRUSH_PROPERTIES1 as ::windows::core::DefaultType>::DefaultType),
                &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bitmapbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandList<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCommandList(::core::mem::transmute_copy(&commandlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDxgiFormatSupported<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDxgiFormatSupported(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBufferPrecisionSupported(bufferprecision) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageLocalBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, localbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageLocalBounds(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&localbounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageWorldBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, worldbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageWorldBounds(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&worldbounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphRunWorldBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphRunWorldBounds(&*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), measuringmode, ::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&device)).into()
        }
        unsafe extern "system" fn SetTarget<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTarget<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTarget(::core::mem::transmute_copy(&image)).into()
        }
        unsafe extern "system" fn SetRenderingControls<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRenderingControls(&*(&renderingcontrols as *const <D2D1_RENDERING_CONTROLS as ::windows::core::Abi>::Abi as *const <D2D1_RENDERING_CONTROLS as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetRenderingControls<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetRenderingControls(::core::mem::transmute_copy(&renderingcontrols)).into()
        }
        unsafe extern "system" fn SetPrimitiveBlend<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrimitiveBlend(primitiveblend).into()
        }
        unsafe extern "system" fn GetPrimitiveBlend<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrimitiveBlend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnitMode<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUnitMode(unitmode).into()
        }
        unsafe extern "system" fn GetUnitMode<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_UNIT_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUnitMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawGlyphRun(
                    &*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                    &*(&glyphrundescription as *const <super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                    &*(&foregroundbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    measuringmode,
                )
                .into()
        }
        unsafe extern "system" fn DrawImage<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawImage(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&imagerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), interpolationmode, compositemode)
                .into()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawGdiMetafile(&*(&gdimetafile as *const <ID2D1GdiMetafile as ::windows::core::Abi>::Abi as *const <ID2D1GdiMetafile as ::windows::core::DefaultType>::DefaultType), &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawBitmap(
                    &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                    &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    opacity,
                    interpolationmode,
                    &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    &*(&perspectivetransform as *const <Common::D2D_MATRIX_4X4_F as ::windows::core::Abi>::Abi as *const <Common::D2D_MATRIX_4X4_F as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PushLayer(&*(&layerparameters as *const <D2D1_LAYER_PARAMETERS1 as ::windows::core::Abi>::Abi as *const <D2D1_LAYER_PARAMETERS1 as ::windows::core::DefaultType>::DefaultType), &*(&layer as *const <ID2D1Layer as ::windows::core::Abi>::Abi as *const <ID2D1Layer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InvalidateEffectInputRectangle<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvalidateEffectInputRectangle(&*(&effect as *const <ID2D1Effect as ::windows::core::Abi>::Abi as *const <ID2D1Effect as ::windows::core::DefaultType>::DefaultType), input, &*(&inputrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInvalidRectangleCount<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, rectanglecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEffectInvalidRectangleCount(&*(&effect as *const <ID2D1Effect as ::windows::core::Abi>::Abi as *const <ID2D1Effect as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectanglecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInvalidRectangles<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEffectInvalidRectangles(&*(&effect as *const <ID2D1Effect as ::windows::core::Abi>::Abi as *const <ID2D1Effect as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectangles), rectanglescount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectRequiredInputRectangles<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendereffect: ::windows::core::RawPtr, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEffectRequiredInputRectangles(
                &*(&rendereffect as *const <ID2D1Effect as ::windows::core::Abi>::Abi as *const <ID2D1Effect as ::windows::core::DefaultType>::DefaultType),
                &*(&renderimagerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdescriptions as *const <D2D1_EFFECT_INPUT_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_EFFECT_INPUT_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&requiredinputrects),
                inputcount,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1DeviceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .FillOpacityMask(
                    &*(&opacitymask as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                    &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext>,
            base.5,
            CreateBitmap::<Impl, OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, OFFSET>,
            CreateColorContext::<Impl, OFFSET>,
            CreateColorContextFromFilename::<Impl, OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, OFFSET>,
            CreateEffect::<Impl, OFFSET>,
            CreateGradientStopCollection::<Impl, OFFSET>,
            CreateImageBrush::<Impl, OFFSET>,
            CreateBitmapBrush::<Impl, OFFSET>,
            CreateCommandList::<Impl, OFFSET>,
            IsDxgiFormatSupported::<Impl, OFFSET>,
            IsBufferPrecisionSupported::<Impl, OFFSET>,
            GetImageLocalBounds::<Impl, OFFSET>,
            GetImageWorldBounds::<Impl, OFFSET>,
            GetGlyphRunWorldBounds::<Impl, OFFSET>,
            GetDevice::<Impl, OFFSET>,
            SetTarget::<Impl, OFFSET>,
            GetTarget::<Impl, OFFSET>,
            SetRenderingControls::<Impl, OFFSET>,
            GetRenderingControls::<Impl, OFFSET>,
            SetPrimitiveBlend::<Impl, OFFSET>,
            GetPrimitiveBlend::<Impl, OFFSET>,
            SetUnitMode::<Impl, OFFSET>,
            GetUnitMode::<Impl, OFFSET>,
            DrawGlyphRun::<Impl, OFFSET>,
            DrawImage::<Impl, OFFSET>,
            DrawGdiMetafile::<Impl, OFFSET>,
            DrawBitmap::<Impl, OFFSET>,
            PushLayer::<Impl, OFFSET>,
            InvalidateEffectInputRectangle::<Impl, OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, OFFSET>,
            GetEffectInvalidRectangles::<Impl, OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, OFFSET>,
            FillOpacityMask::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1DeviceContext1Impl: Sized + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateFilledGeometryRealization();
    fn CreateStrokedGeometryRealization();
    fn DrawGeometryRealization();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext1";
}
impl ID2D1DeviceContext1Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext1Vtbl {
        unsafe extern "system" fn CreateFilledGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, flatteningtolerance: f32, geometryrealization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFilledGeometryRealization(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&geometryrealization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokedGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, flatteningtolerance: f32, strokewidth: f32, strokestyle: ::windows::core::RawPtr, geometryrealization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStrokedGeometryRealization(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&geometryrealization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryrealization: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawGeometryRealization(&*(&geometryrealization as *const <ID2D1GeometryRealization as ::windows::core::Abi>::Abi as *const <ID2D1GeometryRealization as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext1>, base.5, CreateFilledGeometryRealization::<Impl, OFFSET>, CreateStrokedGeometryRealization::<Impl, OFFSET>, DrawGeometryRealization::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceContext2Impl: Sized + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateInk();
    fn CreateInkStyle();
    fn CreateGradientMesh();
    fn CreateImageSourceFromWic();
    fn CreateLookupTable3D();
    fn CreateImageSourceFromDxgi();
    fn GetGradientMeshWorldBounds();
    fn DrawInk();
    fn DrawGradientMesh();
    fn DrawGdiMetafile();
    fn CreateTransformedImageSource();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext2";
}
impl ID2D1DeviceContext2Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext2Vtbl {
        unsafe extern "system" fn CreateInk<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInk(&*(&startpoint as *const <D2D1_INK_POINT as ::windows::core::Abi>::Abi as *const <D2D1_INK_POINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInkStyle<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInkStyle(&*(&inkstyleproperties as *const <D2D1_INK_STYLE_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_INK_STYLE_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inkstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientMesh<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGradientMesh(&*(&patches as *const <D2D1_GRADIENT_MESH_PATCH as ::windows::core::Abi>::Abi as *const <D2D1_GRADIENT_MESH_PATCH as ::windows::core::DefaultType>::DefaultType), patchescount, ::core::mem::transmute_copy(&gradientmesh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageSourceFromWic<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateImageSourceFromWic(&*(&wicbitmapsource as *const <super::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), loadingoptions, alphamode, ::core::mem::transmute_copy(&imagesource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLookupTable3D<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLookupTable3D(precision, extents, data, datacount, strides, ::core::mem::transmute_copy(&lookuptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageSourceFromDxgi<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surfaces: *const ::windows::core::RawPtr, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateImageSourceFromDxgi(&*(&surfaces as *const <super::Dxgi::IDXGISurface as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISurface as ::windows::core::DefaultType>::DefaultType), surfacecount, colorspace, options, ::core::mem::transmute_copy(&imagesource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientMeshWorldBounds<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr, pbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGradientMeshWorldBounds(&*(&gradientmesh as *const <ID2D1GradientMesh as ::windows::core::Abi>::Abi as *const <ID2D1GradientMesh as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawInk<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, inkstyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawInk(&*(&ink as *const <ID2D1Ink as ::windows::core::Abi>::Abi as *const <ID2D1Ink as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), &*(&inkstyle as *const <ID2D1InkStyle as ::windows::core::Abi>::Abi as *const <ID2D1InkStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawGradientMesh<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawGradientMesh(&*(&gradientmesh as *const <ID2D1GradientMesh as ::windows::core::Abi>::Abi as *const <ID2D1GradientMesh as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawGdiMetafile(&*(&gdimetafile as *const <ID2D1GdiMetafile as ::windows::core::Abi>::Abi as *const <ID2D1GdiMetafile as ::windows::core::DefaultType>::DefaultType), &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CreateTransformedImageSource<Impl: ID2D1DeviceContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagesource: ::windows::core::RawPtr, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTransformedImageSource(&*(&imagesource as *const <ID2D1ImageSource as ::windows::core::Abi>::Abi as *const <ID2D1ImageSource as ::windows::core::DefaultType>::DefaultType), &*(&properties as *const <D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transformedimagesource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext2>,
            base.5,
            CreateInk::<Impl, OFFSET>,
            CreateInkStyle::<Impl, OFFSET>,
            CreateGradientMesh::<Impl, OFFSET>,
            CreateImageSourceFromWic::<Impl, OFFSET>,
            CreateLookupTable3D::<Impl, OFFSET>,
            CreateImageSourceFromDxgi::<Impl, OFFSET>,
            GetGradientMeshWorldBounds::<Impl, OFFSET>,
            DrawInk::<Impl, OFFSET>,
            DrawGradientMesh::<Impl, OFFSET>,
            DrawGdiMetafile::<Impl, OFFSET>,
            CreateTransformedImageSource::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1DeviceContext3Impl: Sized + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSpriteBatch();
    fn DrawSpriteBatch();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext3";
}
impl ID2D1DeviceContext3Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext3Vtbl {
        unsafe extern "system" fn CreateSpriteBatch<Impl: ID2D1DeviceContext3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSpriteBatch(::core::mem::transmute_copy(&spritebatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawSpriteBatch<Impl: ID2D1DeviceContext3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: ::windows::core::RawPtr, startindex: u32, spritecount: u32, bitmap: ::windows::core::RawPtr, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawSpriteBatch(&*(&spritebatch as *const <ID2D1SpriteBatch as ::windows::core::Abi>::Abi as *const <ID2D1SpriteBatch as ::windows::core::DefaultType>::DefaultType), startindex, spritecount, &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType), interpolationmode, spriteoptions).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext3>, base.5, CreateSpriteBatch::<Impl, OFFSET>, DrawSpriteBatch::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceContext4Impl: Sized + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgGlyphStyle();
    fn DrawText();
    fn DrawTextLayout();
    fn DrawColorBitmapGlyphRun();
    fn DrawSvgGlyphRun();
    fn GetColorBitmapGlyphImage();
    fn GetSvgGlyphImage();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext4";
}
impl ID2D1DeviceContext4Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext4Vtbl {
        unsafe extern "system" fn CreateSvgGlyphStyle<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSvgGlyphStyle(::core::mem::transmute_copy(&svgglyphstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawText<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawText(
                    &*(&string as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                    stringlength,
                    &*(&textformat as *const <super::DirectWrite::IDWriteTextFormat as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteTextFormat as ::windows::core::DefaultType>::DefaultType),
                    &*(&layoutrect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    &*(&svgglyphstyle as *const <ID2D1SvgGlyphStyle as ::windows::core::Abi>::Abi as *const <ID2D1SvgGlyphStyle as ::windows::core::DefaultType>::DefaultType),
                    colorpaletteindex,
                    options,
                    measuringmode,
                )
                .into()
        }
        unsafe extern "system" fn DrawTextLayout<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: ::windows::core::RawPtr, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawTextLayout(
                    &*(&origin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&textlayout as *const <super::DirectWrite::IDWriteTextLayout as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteTextLayout as ::windows::core::DefaultType>::DefaultType),
                    &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    &*(&svgglyphstyle as *const <ID2D1SvgGlyphStyle as ::windows::core::Abi>::Abi as *const <ID2D1SvgGlyphStyle as ::windows::core::DefaultType>::DefaultType),
                    colorpaletteindex,
                    options,
                )
                .into()
        }
        unsafe extern "system" fn DrawColorBitmapGlyphRun<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawColorBitmapGlyphRun(glyphimageformat, &*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), measuringmode, bitmapsnapoption).into()
        }
        unsafe extern "system" fn DrawSvgGlyphRun<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawSvgGlyphRun(
                    &*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                    &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    &*(&svgglyphstyle as *const <ID2D1SvgGlyphStyle as ::windows::core::Abi>::Abi as *const <ID2D1SvgGlyphStyle as ::windows::core::DefaultType>::DefaultType),
                    colorpaletteindex,
                    measuringmode,
                )
                .into()
        }
        unsafe extern "system" fn GetColorBitmapGlyphImage<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: ::windows::core::RawPtr, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorBitmapGlyphImage(
                glyphimageformat,
                &*(&glyphorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&fontface as *const <super::DirectWrite::IDWriteFontFace as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                glyphindex,
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                dpix,
                dpiy,
                ::core::mem::transmute_copy(&glyphtransform),
                ::core::mem::transmute_copy(&glyphimage),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSvgGlyphImage<Impl: ID2D1DeviceContext4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorigin: Common::D2D_POINT_2F, fontface: ::windows::core::RawPtr, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSvgGlyphImage(
                &*(&glyphorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&fontface as *const <super::DirectWrite::IDWriteFontFace as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                glyphindex,
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                &*(&svgglyphstyle as *const <ID2D1SvgGlyphStyle as ::windows::core::Abi>::Abi as *const <ID2D1SvgGlyphStyle as ::windows::core::DefaultType>::DefaultType),
                colorpaletteindex,
                ::core::mem::transmute_copy(&glyphtransform),
                ::core::mem::transmute_copy(&glyphimage),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext4>, base.5, CreateSvgGlyphStyle::<Impl, OFFSET>, DrawText::<Impl, OFFSET>, DrawTextLayout::<Impl, OFFSET>, DrawColorBitmapGlyphRun::<Impl, OFFSET>, DrawSvgGlyphRun::<Impl, OFFSET>, GetColorBitmapGlyphImage::<Impl, OFFSET>, GetSvgGlyphImage::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceContext5Impl: Sized + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgDocument();
    fn DrawSvgDocument();
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext5";
}
impl ID2D1DeviceContext5Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext5Vtbl {
        unsafe extern "system" fn CreateSvgDocument<Impl: ID2D1DeviceContext5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputxmlstream: ::windows::core::RawPtr, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSvgDocument(&*(&inputxmlstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&viewportsize as *const <Common::D2D_SIZE_F as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_F as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&svgdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawSvgDocument<Impl: ID2D1DeviceContext5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, svgdocument: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawSvgDocument(&*(&svgdocument as *const <ID2D1SvgDocument as ::windows::core::Abi>::Abi as *const <ID2D1SvgDocument as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Impl: ID2D1DeviceContext5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromDxgiColorSpace(colorspace, ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Impl: ID2D1DeviceContext5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromSimpleColorProfile(&*(&simpleprofile as *const <D2D1_SIMPLE_COLOR_PROFILE as ::windows::core::Abi>::Abi as *const <D2D1_SIMPLE_COLOR_PROFILE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext5>, base.5, CreateSvgDocument::<Impl, OFFSET>, DrawSvgDocument::<Impl, OFFSET>, CreateColorContextFromDxgiColorSpace::<Impl, OFFSET>, CreateColorContextFromSimpleColorProfile::<Impl, OFFSET>)
    }
}
pub trait ID2D1DeviceContext6Impl: Sized + ID2D1DeviceContext5Impl + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BlendImage();
}
impl ::windows::core::RuntimeName for ID2D1DeviceContext6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DeviceContext6";
}
impl ID2D1DeviceContext6Vtbl {
    pub const fn new<Impl: ID2D1DeviceContext6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DeviceContext6Vtbl {
        unsafe extern "system" fn BlendImage<Impl: ID2D1DeviceContext6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .BlendImage(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), blendmode, &*(&targetoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&imagerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), interpolationmode)
                .into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DeviceContext6>, base.5, BlendImage::<Impl, OFFSET>)
    }
}
pub trait ID2D1DrawInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetPixelShaderConstantBuffer();
    fn SetResourceTexture();
    fn SetVertexShaderConstantBuffer();
    fn SetPixelShader();
    fn SetVertexProcessing();
}
impl ::windows::core::RuntimeName for ID2D1DrawInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DrawInfo";
}
impl ID2D1DrawInfoVtbl {
    pub const fn new<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DrawInfoVtbl {
        unsafe extern "system" fn SetPixelShaderConstantBuffer<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShaderConstantBuffer(buffer, buffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceTexture<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetResourceTexture(textureindex, &*(&resourcetexture as *const <ID2D1ResourceTexture as ::windows::core::Abi>::Abi as *const <ID2D1ResourceTexture as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantBuffer<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexShaderConstantBuffer(buffer, buffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelShader<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPixelShader(&*(&shaderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pixeloptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVertexProcessing<Impl: ID2D1DrawInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexbuffer: ::windows::core::RawPtr, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVertexProcessing(
                &*(&vertexbuffer as *const <ID2D1VertexBuffer as ::windows::core::Abi>::Abi as *const <ID2D1VertexBuffer as ::windows::core::DefaultType>::DefaultType),
                vertexoptions,
                &*(&blenddescription as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                &*(&vertexrange as *const <D2D1_VERTEX_RANGE as ::windows::core::Abi>::Abi as *const <D2D1_VERTEX_RANGE as ::windows::core::DefaultType>::DefaultType),
                &*(&vertexshader as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DrawInfo>, base.5, SetPixelShaderConstantBuffer::<Impl, OFFSET>, SetResourceTexture::<Impl, OFFSET>, SetVertexShaderConstantBuffer::<Impl, OFFSET>, SetPixelShader::<Impl, OFFSET>, SetVertexProcessing::<Impl, OFFSET>)
    }
}
pub trait ID2D1DrawTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetDrawInfo();
}
impl ::windows::core::RuntimeName for ID2D1DrawTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DrawTransform";
}
impl ID2D1DrawTransformVtbl {
    pub const fn new<Impl: ID2D1DrawTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DrawTransformVtbl {
        unsafe extern "system" fn SetDrawInfo<Impl: ID2D1DrawTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDrawInfo(&*(&drawinfo as *const <ID2D1DrawInfo as ::windows::core::Abi>::Abi as *const <ID2D1DrawInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DrawTransform>, base.5, SetDrawInfo::<Impl, OFFSET>)
    }
}
pub trait ID2D1DrawingStateBlockImpl: Sized + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
    fn SetTextRenderingParams();
    fn GetTextRenderingParams();
}
impl ::windows::core::RuntimeName for ID2D1DrawingStateBlock {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DrawingStateBlock";
}
impl ID2D1DrawingStateBlockVtbl {
    pub const fn new<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DrawingStateBlockVtbl {
        unsafe extern "system" fn GetDescription<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDescription(::core::mem::transmute_copy(&statedescription)).into()
        }
        unsafe extern "system" fn SetDescription<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&statedescription as *const <D2D1_DRAWING_STATE_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_DRAWING_STATE_DESCRIPTION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextRenderingParams(&*(&textrenderingparams as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTextRenderingParams<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTextRenderingParams(::core::mem::transmute_copy(&textrenderingparams)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DrawingStateBlock>, base.5, GetDescription::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, SetTextRenderingParams::<Impl, OFFSET>, GetTextRenderingParams::<Impl, OFFSET>)
    }
}
pub trait ID2D1DrawingStateBlock1Impl: Sized + ID2D1DrawingStateBlockImpl + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
}
impl ::windows::core::RuntimeName for ID2D1DrawingStateBlock1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1DrawingStateBlock1";
}
impl ID2D1DrawingStateBlock1Vtbl {
    pub const fn new<Impl: ID2D1DrawingStateBlock1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1DrawingStateBlock1Vtbl {
        unsafe extern "system" fn GetDescription<Impl: ID2D1DrawingStateBlock1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDescription(::core::mem::transmute_copy(&statedescription)).into()
        }
        unsafe extern "system" fn SetDescription<Impl: ID2D1DrawingStateBlock1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&statedescription as *const <D2D1_DRAWING_STATE_DESCRIPTION1 as ::windows::core::Abi>::Abi as *const <D2D1_DRAWING_STATE_DESCRIPTION1 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1DrawingStateBlock1>, base.5, GetDescription::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>)
    }
}
pub trait ID2D1EffectImpl: Sized + ID2D1PropertiesImpl {
    fn SetInput();
    fn SetInputCount();
    fn GetInput();
    fn GetInputCount();
    fn GetOutput();
}
impl ::windows::core::RuntimeName for ID2D1Effect {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Effect";
}
impl ID2D1EffectVtbl {
    pub const fn new<Impl: ID2D1EffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EffectVtbl {
        unsafe extern "system" fn SetInput<Impl: ID2D1EffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, input: ::windows::core::RawPtr, invalidate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInput(index, &*(&input as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType), &*(&invalidate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetInputCount<Impl: ID2D1EffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputCount(inputcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInput<Impl: ID2D1EffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetInput(index, ::core::mem::transmute_copy(&input)).into()
        }
        unsafe extern "system" fn GetInputCount<Impl: ID2D1EffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutput<Impl: ID2D1EffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputimage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&outputimage)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Effect>, base.5, SetInput::<Impl, OFFSET>, SetInputCount::<Impl, OFFSET>, GetInput::<Impl, OFFSET>, GetInputCount::<Impl, OFFSET>, GetOutput::<Impl, OFFSET>)
    }
}
pub trait ID2D1EffectContextImpl: Sized {
    fn GetDpi();
    fn CreateEffect();
    fn GetMaximumSupportedFeatureLevel();
    fn CreateTransformNodeFromEffect();
    fn CreateBlendTransform();
    fn CreateBorderTransform();
    fn CreateOffsetTransform();
    fn CreateBoundsAdjustmentTransform();
    fn LoadPixelShader();
    fn LoadVertexShader();
    fn LoadComputeShader();
    fn IsShaderLoaded();
    fn CreateResourceTexture();
    fn FindResourceTexture();
    fn CreateVertexBuffer();
    fn FindVertexBuffer();
    fn CreateColorContext();
    fn CreateColorContextFromFilename();
    fn CreateColorContextFromWicColorContext();
    fn CheckFeatureSupport();
    fn IsBufferPrecisionSupported();
}
impl ::windows::core::RuntimeName for ID2D1EffectContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1EffectContext";
}
impl ID2D1EffectContextVtbl {
    pub const fn new<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EffectContextVtbl {
        unsafe extern "system" fn GetDpi<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDpi(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn CreateEffect<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEffect(&*(&effectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&effect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumSupportedFeatureLevel<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumSupportedFeatureLevel(featurelevels, featurelevelscount, ::core::mem::transmute_copy(&maximumsupportedfeaturelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformNodeFromEffect<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, transformnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTransformNodeFromEffect(&*(&effect as *const <ID2D1Effect as ::windows::core::Abi>::Abi as *const <ID2D1Effect as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transformnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendTransform<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlendTransform(numinputs, &*(&blenddescription as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_BLEND_DESCRIPTION as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBorderTransform<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBorderTransform(extendmodex, extendmodey, ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOffsetTransform<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOffsetTransform(&*(&offset as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBoundsAdjustmentTransform<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrectangle: *const super::super::Foundation::RECT, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBoundsAdjustmentTransform(&*(&outputrectangle as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadPixelShader<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadPixelShader(&*(&shaderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), shaderbuffer, shaderbuffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadVertexShader<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadVertexShader(&*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), shaderbuffer, shaderbuffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadComputeShader<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadComputeShader(&*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), shaderbuffer, shaderbuffercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShaderLoaded<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsShaderLoaded(&*(&shaderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceTexture<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResourceTexture(&*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&resourcetextureproperties as *const <D2D1_RESOURCE_TEXTURE_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RESOURCE_TEXTURE_PROPERTIES as ::windows::core::DefaultType>::DefaultType), data, strides, datasize, ::core::mem::transmute_copy(&resourcetexture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindResourceTexture<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindResourceTexture(&*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resourcetexture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows::core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVertexBuffer(
                &*(&vertexbufferproperties as *const <D2D1_VERTEX_BUFFER_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_VERTEX_BUFFER_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&customvertexbufferproperties as *const <D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&buffer),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindVertexBuffer<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, buffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindVertexBuffer(&*(&resourceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContext(space, profile, profilesize, ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromFilename(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wiccolorcontext: ::windows::core::RawPtr, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromWicColorContext(&*(&wiccolorcontext as *const <super::Imaging::IWICColorContext as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICColorContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, ::core::mem::transmute_copy(&featuresupportdata), featuresupportdatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Impl: ID2D1EffectContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBufferPrecisionSupported(bufferprecision) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1EffectContext>,
            base.5,
            GetDpi::<Impl, OFFSET>,
            CreateEffect::<Impl, OFFSET>,
            GetMaximumSupportedFeatureLevel::<Impl, OFFSET>,
            CreateTransformNodeFromEffect::<Impl, OFFSET>,
            CreateBlendTransform::<Impl, OFFSET>,
            CreateBorderTransform::<Impl, OFFSET>,
            CreateOffsetTransform::<Impl, OFFSET>,
            CreateBoundsAdjustmentTransform::<Impl, OFFSET>,
            LoadPixelShader::<Impl, OFFSET>,
            LoadVertexShader::<Impl, OFFSET>,
            LoadComputeShader::<Impl, OFFSET>,
            IsShaderLoaded::<Impl, OFFSET>,
            CreateResourceTexture::<Impl, OFFSET>,
            FindResourceTexture::<Impl, OFFSET>,
            CreateVertexBuffer::<Impl, OFFSET>,
            FindVertexBuffer::<Impl, OFFSET>,
            CreateColorContext::<Impl, OFFSET>,
            CreateColorContextFromFilename::<Impl, OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, OFFSET>,
            CheckFeatureSupport::<Impl, OFFSET>,
            IsBufferPrecisionSupported::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1EffectContext1Impl: Sized + ID2D1EffectContextImpl {
    fn CreateLookupTable3D();
}
impl ::windows::core::RuntimeName for ID2D1EffectContext1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1EffectContext1";
}
impl ID2D1EffectContext1Vtbl {
    pub const fn new<Impl: ID2D1EffectContext1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EffectContext1Vtbl {
        unsafe extern "system" fn CreateLookupTable3D<Impl: ID2D1EffectContext1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLookupTable3D(precision, extents, data, datacount, strides, ::core::mem::transmute_copy(&lookuptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1EffectContext1>, base.5, CreateLookupTable3D::<Impl, OFFSET>)
    }
}
pub trait ID2D1EffectContext2Impl: Sized + ID2D1EffectContext1Impl + ID2D1EffectContextImpl {
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
impl ::windows::core::RuntimeName for ID2D1EffectContext2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1EffectContext2";
}
impl ID2D1EffectContext2Vtbl {
    pub const fn new<Impl: ID2D1EffectContext2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EffectContext2Vtbl {
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Impl: ID2D1EffectContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromDxgiColorSpace(colorspace, ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Impl: ID2D1EffectContext2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorContextFromSimpleColorProfile(&*(&simpleprofile as *const <D2D1_SIMPLE_COLOR_PROFILE as ::windows::core::Abi>::Abi as *const <D2D1_SIMPLE_COLOR_PROFILE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1EffectContext2>, base.5, CreateColorContextFromDxgiColorSpace::<Impl, OFFSET>, CreateColorContextFromSimpleColorProfile::<Impl, OFFSET>)
    }
}
pub trait ID2D1EffectImplImpl: Sized {
    fn Initialize();
    fn PrepareForRender();
    fn SetGraph();
}
impl ::windows::core::RuntimeName for ID2D1EffectImpl {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1EffectImpl";
}
impl ID2D1EffectImplVtbl {
    pub const fn new<Impl: ID2D1EffectImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EffectImplVtbl {
        unsafe extern "system" fn Initialize<Impl: ID2D1EffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectcontext: ::windows::core::RawPtr, transformgraph: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&effectcontext as *const <ID2D1EffectContext as ::windows::core::Abi>::Abi as *const <ID2D1EffectContext as ::windows::core::DefaultType>::DefaultType), &*(&transformgraph as *const <ID2D1TransformGraph as ::windows::core::Abi>::Abi as *const <ID2D1TransformGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareForRender<Impl: ID2D1EffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: D2D1_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareForRender(changetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraph<Impl: ID2D1EffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformgraph: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGraph(&*(&transformgraph as *const <ID2D1TransformGraph as ::windows::core::Abi>::Abi as *const <ID2D1TransformGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1EffectImpl>, base.5, Initialize::<Impl, OFFSET>, PrepareForRender::<Impl, OFFSET>, SetGraph::<Impl, OFFSET>)
    }
}
pub trait ID2D1EllipseGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetEllipse();
}
impl ::windows::core::RuntimeName for ID2D1EllipseGeometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1EllipseGeometry";
}
impl ID2D1EllipseGeometryVtbl {
    pub const fn new<Impl: ID2D1EllipseGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1EllipseGeometryVtbl {
        unsafe extern "system" fn GetEllipse<Impl: ID2D1EllipseGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetEllipse(::core::mem::transmute_copy(&ellipse)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1EllipseGeometry>, base.5, GetEllipse::<Impl, OFFSET>)
    }
}
pub trait ID2D1FactoryImpl: Sized {
    fn ReloadSystemMetrics();
    fn GetDesktopDpi();
    fn CreateRectangleGeometry();
    fn CreateRoundedRectangleGeometry();
    fn CreateEllipseGeometry();
    fn CreateGeometryGroup();
    fn CreateTransformedGeometry();
    fn CreatePathGeometry();
    fn CreateStrokeStyle();
    fn CreateDrawingStateBlock();
    fn CreateWicBitmapRenderTarget();
    fn CreateHwndRenderTarget();
    fn CreateDxgiSurfaceRenderTarget();
    fn CreateDCRenderTarget();
}
impl ::windows::core::RuntimeName for ID2D1Factory {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory";
}
impl ID2D1FactoryVtbl {
    pub const fn new<Impl: ID2D1FactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1FactoryVtbl {
        unsafe extern "system" fn ReloadSystemMetrics<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReloadSystemMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesktopDpi<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDesktopDpi(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn CreateRectangleGeometry<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRectangleGeometry(&*(&rectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectanglegeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRoundedRectangleGeometry(&*(&roundedrectangle as *const <D2D1_ROUNDED_RECT as ::windows::core::Abi>::Abi as *const <D2D1_ROUNDED_RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&roundedrectanglegeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEllipseGeometry(&*(&ellipse as *const <D2D1_ELLIPSE as ::windows::core::Abi>::Abi as *const <D2D1_ELLIPSE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ellipsegeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryGroup<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, geometries: *const ::windows::core::RawPtr, geometriescount: u32, geometrygroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGeometryGroup(fillmode, &*(&geometries as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), geometriescount, ::core::mem::transmute_copy(&geometrygroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformedGeometry<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcegeometry: ::windows::core::RawPtr, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTransformedGeometry(&*(&sourcegeometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transformedgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePathGeometry(::core::mem::transmute_copy(&pathgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStrokeStyle(&*(&strokestyleproperties as *const <D2D1_STROKE_STYLE_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_STROKE_STYLE_PROPERTIES as ::windows::core::DefaultType>::DefaultType), dashes, dashescount, ::core::mem::transmute_copy(&strokestyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: ::windows::core::RawPtr, drawingstateblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDrawingStateBlock(&*(&drawingstatedescription as *const <D2D1_DRAWING_STATE_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_DRAWING_STATE_DESCRIPTION as ::windows::core::DefaultType>::DefaultType), &*(&textrenderingparams as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&drawingstateblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWicBitmapRenderTarget(&*(&target as *const <super::Imaging::IWICBitmap as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICBitmap as ::windows::core::DefaultType>::DefaultType), &*(&rendertargetproperties as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHwndRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateHwndRenderTarget(&*(&rendertargetproperties as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType), &*(&hwndrendertargetproperties as *const <D2D1_HWND_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_HWND_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&hwndrendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgisurface: ::windows::core::RawPtr, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDxgiSurfaceRenderTarget(&*(&dxgisurface as *const <super::Dxgi::IDXGISurface as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISurface as ::windows::core::DefaultType>::DefaultType), &*(&rendertargetproperties as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDCRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDCRenderTarget(&*(&rendertargetproperties as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&dcrendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1Factory>,
            base.5,
            ReloadSystemMetrics::<Impl, OFFSET>,
            GetDesktopDpi::<Impl, OFFSET>,
            CreateRectangleGeometry::<Impl, OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, OFFSET>,
            CreateEllipseGeometry::<Impl, OFFSET>,
            CreateGeometryGroup::<Impl, OFFSET>,
            CreateTransformedGeometry::<Impl, OFFSET>,
            CreatePathGeometry::<Impl, OFFSET>,
            CreateStrokeStyle::<Impl, OFFSET>,
            CreateDrawingStateBlock::<Impl, OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, OFFSET>,
            CreateHwndRenderTarget::<Impl, OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, OFFSET>,
            CreateDCRenderTarget::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1Factory1Impl: Sized + ID2D1FactoryImpl {
    fn CreateDevice();
    fn CreateStrokeStyle();
    fn CreatePathGeometry();
    fn CreateDrawingStateBlock();
    fn CreateGdiMetafile();
    fn RegisterEffectFromStream();
    fn RegisterEffectFromString();
    fn UnregisterEffect();
    fn GetRegisteredEffects();
    fn GetEffectProperties();
}
impl ::windows::core::RuntimeName for ID2D1Factory1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory1";
}
impl ID2D1Factory1Vtbl {
    pub const fn new<Impl: ID2D1Factory1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory1Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStrokeStyle(&*(&strokestyleproperties as *const <D2D1_STROKE_STYLE_PROPERTIES1 as ::windows::core::Abi>::Abi as *const <D2D1_STROKE_STYLE_PROPERTIES1 as ::windows::core::DefaultType>::DefaultType), dashes, dashescount, ::core::mem::transmute_copy(&strokestyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePathGeometry(::core::mem::transmute_copy(&pathgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: ::windows::core::RawPtr, drawingstateblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDrawingStateBlock(&*(&drawingstatedescription as *const <D2D1_DRAWING_STATE_DESCRIPTION1 as ::windows::core::Abi>::Abi as *const <D2D1_DRAWING_STATE_DESCRIPTION1 as ::windows::core::DefaultType>::DefaultType), &*(&textrenderingparams as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&drawingstateblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGdiMetafile<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metafilestream: ::windows::core::RawPtr, metafile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGdiMetafile(&*(&metafilestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&metafile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEffectFromStream<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: ::windows::core::RawPtr, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEffectFromStream(
                &*(&classid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyxml as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&bindings as *const <D2D1_PROPERTY_BINDING as ::windows::core::Abi>::Abi as *const <D2D1_PROPERTY_BINDING as ::windows::core::DefaultType>::DefaultType),
                bindingscount,
                &*(&effectfactory as *const <PD2D1_EFFECT_FACTORY as ::windows::core::Abi>::Abi as *const <PD2D1_EFFECT_FACTORY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEffectFromString<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: super::super::Foundation::PWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEffectFromString(
                &*(&classid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyxml as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bindings as *const <D2D1_PROPERTY_BINDING as ::windows::core::Abi>::Abi as *const <D2D1_PROPERTY_BINDING as ::windows::core::DefaultType>::DefaultType),
                bindingscount,
                &*(&effectfactory as *const <PD2D1_EFFECT_FACTORY as ::windows::core::Abi>::Abi as *const <PD2D1_EFFECT_FACTORY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEffect<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterEffect(&*(&classid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredEffects<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effects: *mut ::windows::core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisteredEffects(::core::mem::transmute_copy(&effects), effectscount, ::core::mem::transmute_copy(&effectsreturned), ::core::mem::transmute_copy(&effectsregistered)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectProperties<Impl: ID2D1Factory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEffectProperties(&*(&effectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory1>, base.5, CreateDevice::<Impl, OFFSET>, CreateStrokeStyle::<Impl, OFFSET>, CreatePathGeometry::<Impl, OFFSET>, CreateDrawingStateBlock::<Impl, OFFSET>, CreateGdiMetafile::<Impl, OFFSET>, RegisterEffectFromStream::<Impl, OFFSET>, RegisterEffectFromString::<Impl, OFFSET>, UnregisterEffect::<Impl, OFFSET>, GetRegisteredEffects::<Impl, OFFSET>, GetEffectProperties::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory2Impl: Sized + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory2";
}
impl ID2D1Factory2Vtbl {
    pub const fn new<Impl: ID2D1Factory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory2Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory2>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory3Impl: Sized + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory3";
}
impl ID2D1Factory3Vtbl {
    pub const fn new<Impl: ID2D1Factory3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory3Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory3>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory4Impl: Sized + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory4";
}
impl ID2D1Factory4Vtbl {
    pub const fn new<Impl: ID2D1Factory4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory4Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice3: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice3)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory4>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory5Impl: Sized + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory5";
}
impl ID2D1Factory5Vtbl {
    pub const fn new<Impl: ID2D1Factory5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory5Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice4)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory5>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory6Impl: Sized + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory6";
}
impl ID2D1Factory6Vtbl {
    pub const fn new<Impl: ID2D1Factory6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory6Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice5)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory6>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1Factory7Impl: Sized + ID2D1Factory6Impl + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for ID2D1Factory7 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Factory7";
}
impl ID2D1Factory7Vtbl {
    pub const fn new<Impl: ID2D1Factory7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1Factory7Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice6: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&dxgidevice as *const <super::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&d2ddevice6)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Factory7>, base.5, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait ID2D1GdiInteropRenderTargetImpl: Sized {
    fn GetDC();
    fn ReleaseDC();
}
impl ::windows::core::RuntimeName for ID2D1GdiInteropRenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GdiInteropRenderTarget";
}
impl ID2D1GdiInteropRenderTargetVtbl {
    pub const fn new<Impl: ID2D1GdiInteropRenderTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GdiInteropRenderTargetVtbl {
        unsafe extern "system" fn GetDC<Impl: ID2D1GdiInteropRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDC(mode, ::core::mem::transmute_copy(&hdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: ID2D1GdiInteropRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, update: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&update as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GdiInteropRenderTarget>, base.5, GetDC::<Impl, OFFSET>, ReleaseDC::<Impl, OFFSET>)
    }
}
pub trait ID2D1GdiMetafileImpl: Sized + ID2D1ResourceImpl {
    fn Stream();
    fn GetBounds();
}
impl ::windows::core::RuntimeName for ID2D1GdiMetafile {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GdiMetafile";
}
impl ID2D1GdiMetafileVtbl {
    pub const fn new<Impl: ID2D1GdiMetafileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GdiMetafileVtbl {
        unsafe extern "system" fn Stream<Impl: ID2D1GdiMetafileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stream(&*(&sink as *const <ID2D1GdiMetafileSink as ::windows::core::Abi>::Abi as *const <ID2D1GdiMetafileSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBounds<Impl: ID2D1GdiMetafileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBounds(::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GdiMetafile>, base.5, Stream::<Impl, OFFSET>, GetBounds::<Impl, OFFSET>)
    }
}
pub trait ID2D1GdiMetafile1Impl: Sized + ID2D1GdiMetafileImpl + ID2D1ResourceImpl {
    fn GetDpi();
    fn GetSourceBounds();
}
impl ::windows::core::RuntimeName for ID2D1GdiMetafile1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GdiMetafile1";
}
impl ID2D1GdiMetafile1Vtbl {
    pub const fn new<Impl: ID2D1GdiMetafile1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GdiMetafile1Vtbl {
        unsafe extern "system" fn GetDpi<Impl: ID2D1GdiMetafile1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDpi(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceBounds<Impl: ID2D1GdiMetafile1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceBounds(::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GdiMetafile1>, base.5, GetDpi::<Impl, OFFSET>, GetSourceBounds::<Impl, OFFSET>)
    }
}
pub trait ID2D1GdiMetafileSinkImpl: Sized {
    fn ProcessRecord();
}
impl ::windows::core::RuntimeName for ID2D1GdiMetafileSink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GdiMetafileSink";
}
impl ID2D1GdiMetafileSinkVtbl {
    pub const fn new<Impl: ID2D1GdiMetafileSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GdiMetafileSinkVtbl {
        unsafe extern "system" fn ProcessRecord<Impl: ID2D1GdiMetafileSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessRecord(recordtype, &*(&recorddata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), recorddatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GdiMetafileSink>, base.5, ProcessRecord::<Impl, OFFSET>)
    }
}
pub trait ID2D1GdiMetafileSink1Impl: Sized + ID2D1GdiMetafileSinkImpl {
    fn ProcessRecord();
}
impl ::windows::core::RuntimeName for ID2D1GdiMetafileSink1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GdiMetafileSink1";
}
impl ID2D1GdiMetafileSink1Vtbl {
    pub const fn new<Impl: ID2D1GdiMetafileSink1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GdiMetafileSink1Vtbl {
        unsafe extern "system" fn ProcessRecord<Impl: ID2D1GdiMetafileSink1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessRecord(recordtype, &*(&recorddata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), recorddatasize, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GdiMetafileSink1>, base.5, ProcessRecord::<Impl, OFFSET>)
    }
}
pub trait ID2D1GeometryImpl: Sized + ID2D1ResourceImpl {
    fn GetBounds();
    fn GetWidenedBounds();
    fn StrokeContainsPoint();
    fn FillContainsPoint();
    fn CompareWithGeometry();
    fn Simplify();
    fn Tessellate();
    fn CombineWithGeometry();
    fn Outline();
    fn ComputeArea();
    fn ComputeLength();
    fn ComputePointAtLength();
    fn Widen();
}
impl ::windows::core::RuntimeName for ID2D1Geometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Geometry";
}
impl ID2D1GeometryVtbl {
    pub const fn new<Impl: ID2D1GeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GeometryVtbl {
        unsafe extern "system" fn GetBounds<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBounds(&*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidenedBounds<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWidenedBounds(strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType), &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeContainsPoint<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StrokeContainsPoint(
                &*(&point as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                strokewidth,
                &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType),
                &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                flatteningtolerance,
                ::core::mem::transmute_copy(&contains),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillContainsPoint<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillContainsPoint(&*(&point as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&contains)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareWithGeometry<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputgeometry: ::windows::core::RawPtr, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompareWithGeometry(&*(&inputgeometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&inputgeometrytransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&relation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Simplify<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Simplify(simplificationoption, &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, &*(&geometrysink as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tessellate<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tessellate(&*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, &*(&tessellationsink as *const <ID2D1TessellationSink as ::windows::core::Abi>::Abi as *const <ID2D1TessellationSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombineWithGeometry<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputgeometry: ::windows::core::RawPtr, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CombineWithGeometry(
                &*(&inputgeometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType),
                combinemode,
                &*(&inputgeometrytransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                flatteningtolerance,
                &*(&geometrysink as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Outline<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Outline(&*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, &*(&geometrysink as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeArea<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComputeArea(&*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&area)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeLength<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComputeLength(&*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputePointAtLength<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComputePointAtLength(length, &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&point), ::core::mem::transmute_copy(&unittangentvector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Widen<Impl: ID2D1GeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Widen(
                strokewidth,
                &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType),
                &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                flatteningtolerance,
                &*(&geometrysink as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1Geometry>,
            base.5,
            GetBounds::<Impl, OFFSET>,
            GetWidenedBounds::<Impl, OFFSET>,
            StrokeContainsPoint::<Impl, OFFSET>,
            FillContainsPoint::<Impl, OFFSET>,
            CompareWithGeometry::<Impl, OFFSET>,
            Simplify::<Impl, OFFSET>,
            Tessellate::<Impl, OFFSET>,
            CombineWithGeometry::<Impl, OFFSET>,
            Outline::<Impl, OFFSET>,
            ComputeArea::<Impl, OFFSET>,
            ComputeLength::<Impl, OFFSET>,
            ComputePointAtLength::<Impl, OFFSET>,
            Widen::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1GeometryGroupImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetFillMode();
    fn GetSourceGeometryCount();
    fn GetSourceGeometries();
}
impl ::windows::core::RuntimeName for ID2D1GeometryGroup {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GeometryGroup";
}
impl ID2D1GeometryGroupVtbl {
    pub const fn new<Impl: ID2D1GeometryGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GeometryGroupVtbl {
        unsafe extern "system" fn GetFillMode<Impl: ID2D1GeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> Common::D2D1_FILL_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceGeometryCount<Impl: ID2D1GeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceGeometryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceGeometries<Impl: ID2D1GeometryGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometries: *mut ::windows::core::RawPtr, geometriescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSourceGeometries(::core::mem::transmute_copy(&geometries), geometriescount).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GeometryGroup>, base.5, GetFillMode::<Impl, OFFSET>, GetSourceGeometryCount::<Impl, OFFSET>, GetSourceGeometries::<Impl, OFFSET>)
    }
}
pub trait ID2D1GeometryRealizationImpl: Sized + ID2D1ResourceImpl {}
impl ::windows::core::RuntimeName for ID2D1GeometryRealization {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GeometryRealization";
}
impl ID2D1GeometryRealizationVtbl {
    pub const fn new<Impl: ID2D1GeometryRealizationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GeometryRealizationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GeometryRealization>, base.5)
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GeometrySinkImpl: Sized + ID2D1SimplifiedGeometrySinkImpl {
    fn AddLine();
    fn AddBezier();
    fn AddQuadraticBezier();
    fn AddQuadraticBeziers();
    fn AddArc();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows::core::RuntimeName for ID2D1GeometrySink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GeometrySink";
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySinkVtbl {
    pub const fn new<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GeometrySinkVtbl {
        unsafe extern "system" fn AddLine<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddLine(&*(&point as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddBezier<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddBezier(&*(&bezier as *const <Common::D2D1_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <Common::D2D1_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddQuadraticBezier<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddQuadraticBezier(&*(&bezier as *const <D2D1_QUADRATIC_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_QUADRATIC_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddQuadraticBeziers<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddQuadraticBeziers(&*(&beziers as *const <D2D1_QUADRATIC_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_QUADRATIC_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType), bezierscount).into()
        }
        unsafe extern "system" fn AddArc<Impl: ID2D1GeometrySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddArc(&*(&arc as *const <D2D1_ARC_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_ARC_SEGMENT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GeometrySink>, base.5, AddLine::<Impl, OFFSET>, AddBezier::<Impl, OFFSET>, AddQuadraticBezier::<Impl, OFFSET>, AddQuadraticBeziers::<Impl, OFFSET>, AddArc::<Impl, OFFSET>)
    }
}
pub trait ID2D1GradientMeshImpl: Sized + ID2D1ResourceImpl {
    fn GetPatchCount();
    fn GetPatches();
}
impl ::windows::core::RuntimeName for ID2D1GradientMesh {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GradientMesh";
}
impl ID2D1GradientMeshVtbl {
    pub const fn new<Impl: ID2D1GradientMeshImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GradientMeshVtbl {
        unsafe extern "system" fn GetPatchCount<Impl: ID2D1GradientMeshImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPatchCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatches<Impl: ID2D1GradientMeshImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPatches(startindex, ::core::mem::transmute_copy(&patches), patchescount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GradientMesh>, base.5, GetPatchCount::<Impl, OFFSET>, GetPatches::<Impl, OFFSET>)
    }
}
pub trait ID2D1GradientStopCollectionImpl: Sized + ID2D1ResourceImpl {
    fn GetGradientStopCount();
    fn GetGradientStops();
    fn GetColorInterpolationGamma();
    fn GetExtendMode();
}
impl ::windows::core::RuntimeName for ID2D1GradientStopCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GradientStopCollection";
}
impl ID2D1GradientStopCollectionVtbl {
    pub const fn new<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GradientStopCollectionVtbl {
        unsafe extern "system" fn GetGradientStopCount<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGradientStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientStops<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetGradientStops(::core::mem::transmute_copy(&gradientstops), gradientstopscount).into()
        }
        unsafe extern "system" fn GetColorInterpolationGamma<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_GAMMA {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorInterpolationGamma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendMode<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GradientStopCollection>, base.5, GetGradientStopCount::<Impl, OFFSET>, GetGradientStops::<Impl, OFFSET>, GetColorInterpolationGamma::<Impl, OFFSET>, GetExtendMode::<Impl, OFFSET>)
    }
}
pub trait ID2D1GradientStopCollection1Impl: Sized + ID2D1GradientStopCollectionImpl + ID2D1ResourceImpl {
    fn GetGradientStops1();
    fn GetPreInterpolationSpace();
    fn GetPostInterpolationSpace();
    fn GetBufferPrecision();
    fn GetColorInterpolationMode();
}
impl ::windows::core::RuntimeName for ID2D1GradientStopCollection1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1GradientStopCollection1";
}
impl ID2D1GradientStopCollection1Vtbl {
    pub const fn new<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1GradientStopCollection1Vtbl {
        unsafe extern "system" fn GetGradientStops1<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetGradientStops1(::core::mem::transmute_copy(&gradientstops), gradientstopscount).into()
        }
        unsafe extern "system" fn GetPreInterpolationSpace<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreInterpolationSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostInterpolationSpace<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPostInterpolationSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferPrecision<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BUFFER_PRECISION {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1GradientStopCollection1>, base.5, GetGradientStops1::<Impl, OFFSET>, GetPreInterpolationSpace::<Impl, OFFSET>, GetPostInterpolationSpace::<Impl, OFFSET>, GetBufferPrecision::<Impl, OFFSET>, GetColorInterpolationMode::<Impl, OFFSET>)
    }
}
pub trait ID2D1HwndRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CheckWindowState();
    fn Resize();
    fn GetHwnd();
}
impl ::windows::core::RuntimeName for ID2D1HwndRenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1HwndRenderTarget";
}
impl ID2D1HwndRenderTargetVtbl {
    pub const fn new<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1HwndRenderTargetVtbl {
        unsafe extern "system" fn CheckWindowState<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_WINDOW_STATE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckWindowState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelsize: *const Common::D2D_SIZE_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resize(&*(&pixelsize as *const <Common::D2D_SIZE_U as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwnd<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HWND {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1HwndRenderTarget>, base.5, CheckWindowState::<Impl, OFFSET>, Resize::<Impl, OFFSET>, GetHwnd::<Impl, OFFSET>)
    }
}
pub trait ID2D1ImageImpl: Sized + ID2D1ResourceImpl {}
impl ::windows::core::RuntimeName for ID2D1Image {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Image";
}
impl ID2D1ImageVtbl {
    pub const fn new<Impl: ID2D1ImageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ImageVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Image>, base.5)
    }
}
pub trait ID2D1ImageBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetImage();
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn SetInterpolationMode();
    fn SetSourceRectangle();
    fn GetImage();
    fn GetExtendModeX();
    fn GetExtendModeY();
    fn GetInterpolationMode();
    fn GetSourceRectangle();
}
impl ::windows::core::RuntimeName for ID2D1ImageBrush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ImageBrush";
}
impl ID2D1ImageBrushVtbl {
    pub const fn new<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ImageBrushVtbl {
        unsafe extern "system" fn SetImage<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&image as *const <ID2D1Image as ::windows::core::Abi>::Abi as *const <ID2D1Image as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeX(extendmodex).into()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtendModeY(extendmodey).into()
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterpolationMode(interpolationmode).into()
        }
        unsafe extern "system" fn SetSourceRectangle<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSourceRectangle(&*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetImage<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetImage(::core::mem::transmute_copy(&image)).into()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendModeY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterpolationMode<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceRectangle<Impl: ID2D1ImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerectangle: *mut Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSourceRectangle(::core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ImageBrush>, base.5, SetImage::<Impl, OFFSET>, SetExtendModeX::<Impl, OFFSET>, SetExtendModeY::<Impl, OFFSET>, SetInterpolationMode::<Impl, OFFSET>, SetSourceRectangle::<Impl, OFFSET>, GetImage::<Impl, OFFSET>, GetExtendModeX::<Impl, OFFSET>, GetExtendModeY::<Impl, OFFSET>, GetInterpolationMode::<Impl, OFFSET>, GetSourceRectangle::<Impl, OFFSET>)
    }
}
pub trait ID2D1ImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn OfferResources();
    fn TryReclaimResources();
}
impl ::windows::core::RuntimeName for ID2D1ImageSource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ImageSource";
}
impl ID2D1ImageSourceVtbl {
    pub const fn new<Impl: ID2D1ImageSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ImageSourceVtbl {
        unsafe extern "system" fn OfferResources<Impl: ID2D1ImageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OfferResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryReclaimResources<Impl: ID2D1ImageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryReclaimResources(::core::mem::transmute_copy(&resourcesdiscarded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ImageSource>, base.5, OfferResources::<Impl, OFFSET>, TryReclaimResources::<Impl, OFFSET>)
    }
}
pub trait ID2D1ImageSourceFromWicImpl: Sized + ID2D1ImageSourceImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn EnsureCached();
    fn TrimCache();
    fn GetSource();
}
impl ::windows::core::RuntimeName for ID2D1ImageSourceFromWic {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ImageSourceFromWic";
}
impl ID2D1ImageSourceFromWicVtbl {
    pub const fn new<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ImageSourceFromWicVtbl {
        unsafe extern "system" fn EnsureCached<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangletofill: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnsureCached(&*(&rectangletofill as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrimCache<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrimCache(&*(&rectangletopreserve as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSource(::core::mem::transmute_copy(&wicbitmapsource)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ImageSourceFromWic>, base.5, EnsureCached::<Impl, OFFSET>, TrimCache::<Impl, OFFSET>, GetSource::<Impl, OFFSET>)
    }
}
pub trait ID2D1InkImpl: Sized + ID2D1ResourceImpl {
    fn SetStartPoint();
    fn GetStartPoint();
    fn AddSegments();
    fn RemoveSegmentsAtEnd();
    fn SetSegments();
    fn SetSegmentAtEnd();
    fn GetSegmentCount();
    fn GetSegments();
    fn StreamAsGeometry();
    fn GetBounds();
}
impl ::windows::core::RuntimeName for ID2D1Ink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Ink";
}
impl ID2D1InkVtbl {
    pub const fn new<Impl: ID2D1InkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1InkVtbl {
        unsafe extern "system" fn SetStartPoint<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&startpoint as *const <D2D1_INK_POINT as ::windows::core::Abi>::Abi as *const <D2D1_INK_POINT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStartPoint<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D2D1_INK_POINT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSegments<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSegments(&*(&segments as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType), segmentscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSegmentsAtEnd<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSegmentsAtEnd(segmentscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegments<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSegments(startsegment, &*(&segments as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType), segmentscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentAtEnd<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSegmentAtEnd(&*(&segment as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::Abi>::Abi as *const <D2D1_INK_BEZIER_SEGMENT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentCount<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegments<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegments(startsegment, ::core::mem::transmute_copy(&segments), segmentscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamAsGeometry<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamAsGeometry(
                &*(&inkstyle as *const <ID2D1InkStyle as ::windows::core::Abi>::Abi as *const <ID2D1InkStyle as ::windows::core::DefaultType>::DefaultType),
                &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                flatteningtolerance,
                &*(&geometrysink as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBounds<Impl: ID2D1InkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBounds(&*(&inkstyle as *const <ID2D1InkStyle as ::windows::core::Abi>::Abi as *const <ID2D1InkStyle as ::windows::core::DefaultType>::DefaultType), &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Ink>, base.5, SetStartPoint::<Impl, OFFSET>, GetStartPoint::<Impl, OFFSET>, AddSegments::<Impl, OFFSET>, RemoveSegmentsAtEnd::<Impl, OFFSET>, SetSegments::<Impl, OFFSET>, SetSegmentAtEnd::<Impl, OFFSET>, GetSegmentCount::<Impl, OFFSET>, GetSegments::<Impl, OFFSET>, StreamAsGeometry::<Impl, OFFSET>, GetBounds::<Impl, OFFSET>)
    }
}
pub trait ID2D1InkStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetNibTransform();
    fn GetNibTransform();
    fn SetNibShape();
    fn GetNibShape();
}
impl ::windows::core::RuntimeName for ID2D1InkStyle {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1InkStyle";
}
impl ID2D1InkStyleVtbl {
    pub const fn new<Impl: ID2D1InkStyleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1InkStyleVtbl {
        unsafe extern "system" fn SetNibTransform<Impl: ID2D1InkStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNibTransform(&*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetNibTransform<Impl: ID2D1InkStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetNibTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn SetNibShape<Impl: ID2D1InkStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNibShape(nibshape).into()
        }
        unsafe extern "system" fn GetNibShape<Impl: ID2D1InkStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INK_NIB_SHAPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNibShape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1InkStyle>, base.5, SetNibTransform::<Impl, OFFSET>, GetNibTransform::<Impl, OFFSET>, SetNibShape::<Impl, OFFSET>, GetNibShape::<Impl, OFFSET>)
    }
}
pub trait ID2D1LayerImpl: Sized + ID2D1ResourceImpl {
    fn GetSize();
}
impl ::windows::core::RuntimeName for ID2D1Layer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Layer";
}
impl ID2D1LayerVtbl {
    pub const fn new<Impl: ID2D1LayerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1LayerVtbl {
        unsafe extern "system" fn GetSize<Impl: ID2D1LayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Layer>, base.5, GetSize::<Impl, OFFSET>)
    }
}
pub trait ID2D1LinearGradientBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetStartPoint();
    fn SetEndPoint();
    fn GetStartPoint();
    fn GetEndPoint();
    fn GetGradientStopCollection();
}
impl ::windows::core::RuntimeName for ID2D1LinearGradientBrush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1LinearGradientBrush";
}
impl ID2D1LinearGradientBrushVtbl {
    pub const fn new<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1LinearGradientBrushVtbl {
        unsafe extern "system" fn SetStartPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&startpoint as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEndPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&endpoint as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStartPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientStopCollection<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetGradientStopCollection(::core::mem::transmute_copy(&gradientstopcollection)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1LinearGradientBrush>, base.5, SetStartPoint::<Impl, OFFSET>, SetEndPoint::<Impl, OFFSET>, GetStartPoint::<Impl, OFFSET>, GetEndPoint::<Impl, OFFSET>, GetGradientStopCollection::<Impl, OFFSET>)
    }
}
pub trait ID2D1LookupTable3DImpl: Sized + ID2D1ResourceImpl {}
impl ::windows::core::RuntimeName for ID2D1LookupTable3D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1LookupTable3D";
}
impl ID2D1LookupTable3DVtbl {
    pub const fn new<Impl: ID2D1LookupTable3DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1LookupTable3DVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1LookupTable3D>, base.5)
    }
}
pub trait ID2D1MeshImpl: Sized + ID2D1ResourceImpl {
    fn Open();
}
impl ::windows::core::RuntimeName for ID2D1Mesh {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Mesh";
}
impl ID2D1MeshVtbl {
    pub const fn new<Impl: ID2D1MeshImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1MeshVtbl {
        unsafe extern "system" fn Open<Impl: ID2D1MeshImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tessellationsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&tessellationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Mesh>, base.5, Open::<Impl, OFFSET>)
    }
}
pub trait ID2D1MultithreadImpl: Sized {
    fn GetMultithreadProtected();
    fn Enter();
    fn Leave();
}
impl ::windows::core::RuntimeName for ID2D1Multithread {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Multithread";
}
impl ID2D1MultithreadVtbl {
    pub const fn new<Impl: ID2D1MultithreadImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1MultithreadVtbl {
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID2D1MultithreadImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMultithreadProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Impl: ID2D1MultithreadImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Enter().into()
        }
        unsafe extern "system" fn Leave<Impl: ID2D1MultithreadImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Leave().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Multithread>, base.5, GetMultithreadProtected::<Impl, OFFSET>, Enter::<Impl, OFFSET>, Leave::<Impl, OFFSET>)
    }
}
pub trait ID2D1OffsetTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOffset();
    fn GetOffset();
}
impl ::windows::core::RuntimeName for ID2D1OffsetTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1OffsetTransform";
}
impl ID2D1OffsetTransformVtbl {
    pub const fn new<Impl: ID2D1OffsetTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1OffsetTransformVtbl {
        unsafe extern "system" fn SetOffset<Impl: ID2D1OffsetTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&offset as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetOffset<Impl: ID2D1OffsetTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::POINT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1OffsetTransform>, base.5, SetOffset::<Impl, OFFSET>, GetOffset::<Impl, OFFSET>)
    }
}
pub trait ID2D1PathGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn Open();
    fn Stream();
    fn GetSegmentCount();
    fn GetFigureCount();
}
impl ::windows::core::RuntimeName for ID2D1PathGeometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1PathGeometry";
}
impl ID2D1PathGeometryVtbl {
    pub const fn new<Impl: ID2D1PathGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1PathGeometryVtbl {
        unsafe extern "system" fn Open<Impl: ID2D1PathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometrysink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&geometrysink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ID2D1PathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stream(&*(&geometrysink as *const <ID2D1GeometrySink as ::windows::core::Abi>::Abi as *const <ID2D1GeometrySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentCount<Impl: ID2D1PathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFigureCount<Impl: ID2D1PathGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFigureCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1PathGeometry>, base.5, Open::<Impl, OFFSET>, Stream::<Impl, OFFSET>, GetSegmentCount::<Impl, OFFSET>, GetFigureCount::<Impl, OFFSET>)
    }
}
pub trait ID2D1PathGeometry1Impl: Sized + ID2D1PathGeometryImpl + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn ComputePointAndSegmentAtLength();
}
impl ::windows::core::RuntimeName for ID2D1PathGeometry1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1PathGeometry1";
}
impl ID2D1PathGeometry1Vtbl {
    pub const fn new<Impl: ID2D1PathGeometry1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1PathGeometry1Vtbl {
        unsafe extern "system" fn ComputePointAndSegmentAtLength<Impl: ID2D1PathGeometry1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComputePointAndSegmentAtLength(length, startsegment, &*(&worldtransform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType), flatteningtolerance, ::core::mem::transmute_copy(&pointdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1PathGeometry1>, base.5, ComputePointAndSegmentAtLength::<Impl, OFFSET>)
    }
}
pub trait ID2D1PrintControlImpl: Sized {
    fn AddPage();
    fn Close();
}
impl ::windows::core::RuntimeName for ID2D1PrintControl {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1PrintControl";
}
impl ID2D1PrintControlVtbl {
    pub const fn new<Impl: ID2D1PrintControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1PrintControlVtbl {
        unsafe extern "system" fn AddPage<Impl: ID2D1PrintControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, pagesize: Common::D2D_SIZE_F, pageprintticketstream: ::windows::core::RawPtr, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPage(
                &*(&commandlist as *const <ID2D1CommandList as ::windows::core::Abi>::Abi as *const <ID2D1CommandList as ::windows::core::DefaultType>::DefaultType),
                &*(&pagesize as *const <Common::D2D_SIZE_F as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_F as ::windows::core::DefaultType>::DefaultType),
                &*(&pageprintticketstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&tag1),
                ::core::mem::transmute_copy(&tag2),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ID2D1PrintControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1PrintControl>, base.5, AddPage::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ID2D1PropertiesImpl: Sized {
    fn GetPropertyCount();
    fn GetPropertyName();
    fn GetPropertyNameLength();
    fn GetType();
    fn GetPropertyIndex();
    fn SetValueByName();
    fn SetValue();
    fn GetValueByName();
    fn GetValue();
    fn GetValueSize();
    fn GetSubProperties();
}
impl ::windows::core::RuntimeName for ID2D1Properties {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Properties";
}
impl ID2D1PropertiesVtbl {
    pub const fn new<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1PropertiesVtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyName<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyName(index, ::core::mem::transmute_copy(&name), namecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNameLength<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyNameLength(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyIndex<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyIndex(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueByName<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueByName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, data, datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(index, r#type, data, datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByName<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueByName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&data), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(index, r#type, ::core::mem::transmute_copy(&data), datasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueSize<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueSize(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubProperties<Impl: ID2D1PropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, subproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubProperties(index, ::core::mem::transmute_copy(&subproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Properties>, base.5, GetPropertyCount::<Impl, OFFSET>, GetPropertyName::<Impl, OFFSET>, GetPropertyNameLength::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetPropertyIndex::<Impl, OFFSET>, SetValueByName::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, GetValueByName::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, GetValueSize::<Impl, OFFSET>, GetSubProperties::<Impl, OFFSET>)
    }
}
pub trait ID2D1RadialGradientBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetCenter();
    fn SetGradientOriginOffset();
    fn SetRadiusX();
    fn SetRadiusY();
    fn GetCenter();
    fn GetGradientOriginOffset();
    fn GetRadiusX();
    fn GetRadiusY();
    fn GetGradientStopCollection();
}
impl ::windows::core::RuntimeName for ID2D1RadialGradientBrush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1RadialGradientBrush";
}
impl ID2D1RadialGradientBrushVtbl {
    pub const fn new<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1RadialGradientBrushVtbl {
        unsafe extern "system" fn SetCenter<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&center as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGradientOriginOffset<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientoriginoffset: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGradientOriginOffset(&*(&gradientoriginoffset as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetRadiusX<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiusx: f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRadiusX(radiusx).into()
        }
        unsafe extern "system" fn SetRadiusY<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiusy: f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRadiusY(radiusy).into()
        }
        unsafe extern "system" fn GetCenter<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientOriginOffset<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGradientOriginOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadiusX<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRadiusX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadiusY<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRadiusY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientStopCollection<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetGradientStopCollection(::core::mem::transmute_copy(&gradientstopcollection)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1RadialGradientBrush>, base.5, SetCenter::<Impl, OFFSET>, SetGradientOriginOffset::<Impl, OFFSET>, SetRadiusX::<Impl, OFFSET>, SetRadiusY::<Impl, OFFSET>, GetCenter::<Impl, OFFSET>, GetGradientOriginOffset::<Impl, OFFSET>, GetRadiusX::<Impl, OFFSET>, GetRadiusY::<Impl, OFFSET>, GetGradientStopCollection::<Impl, OFFSET>)
    }
}
pub trait ID2D1RectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRect();
}
impl ::windows::core::RuntimeName for ID2D1RectangleGeometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1RectangleGeometry";
}
impl ID2D1RectangleGeometryVtbl {
    pub const fn new<Impl: ID2D1RectangleGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1RectangleGeometryVtbl {
        unsafe extern "system" fn GetRect<Impl: ID2D1RectangleGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *mut Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetRect(::core::mem::transmute_copy(&rect)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1RectangleGeometry>, base.5, GetRect::<Impl, OFFSET>)
    }
}
pub trait ID2D1RenderInfoImpl: Sized {
    fn SetInputDescription();
    fn SetOutputBuffer();
    fn SetCached();
    fn SetInstructionCountHint();
}
impl ::windows::core::RuntimeName for ID2D1RenderInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1RenderInfo";
}
impl ID2D1RenderInfoVtbl {
    pub const fn new<Impl: ID2D1RenderInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1RenderInfoVtbl {
        unsafe extern "system" fn SetInputDescription<Impl: ID2D1RenderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputDescription(inputindex, &*(&inputdescription as *const <D2D1_INPUT_DESCRIPTION as ::windows::core::Abi>::Abi as *const <D2D1_INPUT_DESCRIPTION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputBuffer<Impl: ID2D1RenderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputBuffer(bufferprecision, channeldepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCached<Impl: ID2D1RenderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCached(&*(&iscached as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetInstructionCountHint<Impl: ID2D1RenderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, instructioncount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInstructionCountHint(instructioncount).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1RenderInfo>, base.5, SetInputDescription::<Impl, OFFSET>, SetOutputBuffer::<Impl, OFFSET>, SetCached::<Impl, OFFSET>, SetInstructionCountHint::<Impl, OFFSET>)
    }
}
pub trait ID2D1RenderTargetImpl: Sized + ID2D1ResourceImpl {
    fn CreateBitmap();
    fn CreateBitmapFromWicBitmap();
    fn CreateSharedBitmap();
    fn CreateBitmapBrush();
    fn CreateSolidColorBrush();
    fn CreateGradientStopCollection();
    fn CreateLinearGradientBrush();
    fn CreateRadialGradientBrush();
    fn CreateCompatibleRenderTarget();
    fn CreateLayer();
    fn CreateMesh();
    fn DrawLine();
    fn DrawRectangle();
    fn FillRectangle();
    fn DrawRoundedRectangle();
    fn FillRoundedRectangle();
    fn DrawEllipse();
    fn FillEllipse();
    fn DrawGeometry();
    fn FillGeometry();
    fn FillMesh();
    fn FillOpacityMask();
    fn DrawBitmap();
    fn DrawText();
    fn DrawTextLayout();
    fn DrawGlyphRun();
    fn SetTransform();
    fn GetTransform();
    fn SetAntialiasMode();
    fn GetAntialiasMode();
    fn SetTextAntialiasMode();
    fn GetTextAntialiasMode();
    fn SetTextRenderingParams();
    fn GetTextRenderingParams();
    fn SetTags();
    fn GetTags();
    fn PushLayer();
    fn PopLayer();
    fn Flush();
    fn SaveDrawingState();
    fn RestoreDrawingState();
    fn PushAxisAlignedClip();
    fn PopAxisAlignedClip();
    fn Clear();
    fn BeginDraw();
    fn EndDraw();
    fn GetPixelFormat();
    fn SetDpi();
    fn GetDpi();
    fn GetSize();
    fn GetPixelSize();
    fn GetMaximumBitmapSize();
    fn IsSupported();
}
impl ::windows::core::RuntimeName for ID2D1RenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1RenderTarget";
}
impl ID2D1RenderTargetVtbl {
    pub const fn new<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1RenderTargetVtbl {
        unsafe extern "system" fn CreateBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmap(
                &*(&size as *const <Common::D2D_SIZE_U as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_U as ::windows::core::DefaultType>::DefaultType),
                &*(&srcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                pitch,
                &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bitmap),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromWicBitmap(&*(&wicbitmapsource as *const <super::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi as *const <super::Imaging::IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSharedBitmap(
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&bitmapproperties as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bitmap),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBitmapBrush(
                &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                &*(&bitmapbrushproperties as *const <D2D1_BITMAP_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BITMAP_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&bitmapbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSolidColorBrush(&*(&color as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType), &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&solidcolorbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGradientStopCollection(&*(&gradientstops as *const <D2D1_GRADIENT_STOP as ::windows::core::Abi>::Abi as *const <D2D1_GRADIENT_STOP as ::windows::core::DefaultType>::DefaultType), gradientstopscount, colorinterpolationgamma, extendmode, ::core::mem::transmute_copy(&gradientstopcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::windows::core::RawPtr, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearGradientBrush(
                &*(&lineargradientbrushproperties as *const <D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&gradientstopcollection as *const <ID2D1GradientStopCollection as ::windows::core::Abi>::Abi as *const <ID2D1GradientStopCollection as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&lineargradientbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::windows::core::RawPtr, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRadialGradientBrush(
                &*(&radialgradientbrushproperties as *const <D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&brushproperties as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_BRUSH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                &*(&gradientstopcollection as *const <ID2D1GradientStopCollection as ::windows::core::Abi>::Abi as *const <ID2D1GradientStopCollection as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&radialgradientbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompatibleRenderTarget(
                &*(&desiredsize as *const <Common::D2D_SIZE_F as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_F as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredpixelsize as *const <Common::D2D_SIZE_U as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_U as ::windows::core::DefaultType>::DefaultType),
                &*(&desiredformat as *const <Common::D2D1_PIXEL_FORMAT as ::windows::core::Abi>::Abi as *const <Common::D2D1_PIXEL_FORMAT as ::windows::core::DefaultType>::DefaultType),
                options,
                ::core::mem::transmute_copy(&bitmaprendertarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *const Common::D2D_SIZE_F, layer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLayer(&*(&size as *const <Common::D2D_SIZE_F as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_F as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&layer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMesh<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMesh(::core::mem::transmute_copy(&mesh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawLine<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawLine(
                    &*(&point0 as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&point1 as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    strokewidth,
                    &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn DrawRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawRectangle(&*(&rect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FillRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FillRectangle(&*(&rect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawRoundedRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawRoundedRectangle(&*(&roundedrect as *const <D2D1_ROUNDED_RECT as ::windows::core::Abi>::Abi as *const <D2D1_ROUNDED_RECT as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn FillRoundedRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FillRoundedRectangle(&*(&roundedrect as *const <D2D1_ROUNDED_RECT as ::windows::core::Abi>::Abi as *const <D2D1_ROUNDED_RECT as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawEllipse<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawEllipse(&*(&ellipse as *const <D2D1_ELLIPSE as ::windows::core::Abi>::Abi as *const <D2D1_ELLIPSE as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FillEllipse<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FillEllipse(&*(&ellipse as *const <D2D1_ELLIPSE as ::windows::core::Abi>::Abi as *const <D2D1_ELLIPSE as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawGeometry<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawGeometry(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, &*(&strokestyle as *const <ID2D1StrokeStyle as ::windows::core::Abi>::Abi as *const <ID2D1StrokeStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FillGeometry<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, opacitybrush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FillGeometry(&*(&geometry as *const <ID2D1Geometry as ::windows::core::Abi>::Abi as *const <ID2D1Geometry as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), &*(&opacitybrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FillMesh<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FillMesh(&*(&mesh as *const <ID2D1Mesh as ::windows::core::Abi>::Abi as *const <ID2D1Mesh as ::windows::core::DefaultType>::DefaultType), &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .FillOpacityMask(
                    &*(&opacitymask as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                    &*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    content,
                    &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawBitmap(
                    &*(&bitmap as *const <ID2D1Bitmap as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap as ::windows::core::DefaultType>::DefaultType),
                    &*(&destinationrectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    opacity,
                    interpolationmode,
                    &*(&sourcerectangle as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn DrawText<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::windows::core::RawPtr, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawText(
                    &*(&string as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                    stringlength,
                    &*(&textformat as *const <super::DirectWrite::IDWriteTextFormat as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteTextFormat as ::windows::core::DefaultType>::DefaultType),
                    &*(&layoutrect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                    &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    options,
                    measuringmode,
                )
                .into()
        }
        unsafe extern "system" fn DrawTextLayout<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: ::windows::core::RawPtr, defaultfillbrush: ::windows::core::RawPtr, options: D2D1_DRAW_TEXT_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawTextLayout(
                    &*(&origin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&textlayout as *const <super::DirectWrite::IDWriteTextLayout as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteTextLayout as ::windows::core::DefaultType>::DefaultType),
                    &*(&defaultfillbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    options,
                )
                .into()
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .DrawGlyphRun(
                    &*(&baselineorigin as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                    &*(&glyphrun as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <super::DirectWrite::DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                    &*(&foregroundbrush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType),
                    measuringmode,
                )
                .into()
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransform(&*(&transform as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn SetAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAntialiasMode(antialiasmode).into()
        }
        unsafe extern "system" fn GetAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAntialiasMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextAntialiasMode(textantialiasmode).into()
        }
        unsafe extern "system" fn GetTextAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTextAntialiasMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextRenderingParams(&*(&textrenderingparams as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <super::DirectWrite::IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTextRenderingParams<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTextRenderingParams(::core::mem::transmute_copy(&textrenderingparams)).into()
        }
        unsafe extern "system" fn SetTags<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTags(tag1, tag2).into()
        }
        unsafe extern "system" fn GetTags<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTags(::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)).into()
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PushLayer(&*(&layerparameters as *const <D2D1_LAYER_PARAMETERS as ::windows::core::Abi>::Abi as *const <D2D1_LAYER_PARAMETERS as ::windows::core::DefaultType>::DefaultType), &*(&layer as *const <ID2D1Layer as ::windows::core::Abi>::Abi as *const <ID2D1Layer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PopLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PopLayer().into()
        }
        unsafe extern "system" fn Flush<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flush(::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveDrawingState<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstateblock: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SaveDrawingState(&*(&drawingstateblock as *const <ID2D1DrawingStateBlock as ::windows::core::Abi>::Abi as *const <ID2D1DrawingStateBlock as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestoreDrawingState<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstateblock: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RestoreDrawingState(&*(&drawingstateblock as *const <ID2D1DrawingStateBlock as ::windows::core::Abi>::Abi as *const <ID2D1DrawingStateBlock as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PushAxisAlignedClip<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PushAxisAlignedClip(&*(&cliprect as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType), antialiasmode).into()
        }
        unsafe extern "system" fn PopAxisAlignedClip<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PopAxisAlignedClip().into()
        }
        unsafe extern "system" fn Clear<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clearcolor: *const Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear(&*(&clearcolor as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginDraw<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BeginDraw().into()
        }
        unsafe extern "system" fn EndDraw<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDraw(::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpi<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDpi(dpix, dpiy).into()
        }
        unsafe extern "system" fn GetDpi<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDpi(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn GetSize<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelSize<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPixelSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumBitmapSize<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumBitmapSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: ID2D1RenderTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported(&*(&rendertargetproperties as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::Abi>::Abi as *const <D2D1_RENDER_TARGET_PROPERTIES as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1RenderTarget>,
            base.5,
            CreateBitmap::<Impl, OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, OFFSET>,
            CreateSharedBitmap::<Impl, OFFSET>,
            CreateBitmapBrush::<Impl, OFFSET>,
            CreateSolidColorBrush::<Impl, OFFSET>,
            CreateGradientStopCollection::<Impl, OFFSET>,
            CreateLinearGradientBrush::<Impl, OFFSET>,
            CreateRadialGradientBrush::<Impl, OFFSET>,
            CreateCompatibleRenderTarget::<Impl, OFFSET>,
            CreateLayer::<Impl, OFFSET>,
            CreateMesh::<Impl, OFFSET>,
            DrawLine::<Impl, OFFSET>,
            DrawRectangle::<Impl, OFFSET>,
            FillRectangle::<Impl, OFFSET>,
            DrawRoundedRectangle::<Impl, OFFSET>,
            FillRoundedRectangle::<Impl, OFFSET>,
            DrawEllipse::<Impl, OFFSET>,
            FillEllipse::<Impl, OFFSET>,
            DrawGeometry::<Impl, OFFSET>,
            FillGeometry::<Impl, OFFSET>,
            FillMesh::<Impl, OFFSET>,
            FillOpacityMask::<Impl, OFFSET>,
            DrawBitmap::<Impl, OFFSET>,
            DrawText::<Impl, OFFSET>,
            DrawTextLayout::<Impl, OFFSET>,
            DrawGlyphRun::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            GetTransform::<Impl, OFFSET>,
            SetAntialiasMode::<Impl, OFFSET>,
            GetAntialiasMode::<Impl, OFFSET>,
            SetTextAntialiasMode::<Impl, OFFSET>,
            GetTextAntialiasMode::<Impl, OFFSET>,
            SetTextRenderingParams::<Impl, OFFSET>,
            GetTextRenderingParams::<Impl, OFFSET>,
            SetTags::<Impl, OFFSET>,
            GetTags::<Impl, OFFSET>,
            PushLayer::<Impl, OFFSET>,
            PopLayer::<Impl, OFFSET>,
            Flush::<Impl, OFFSET>,
            SaveDrawingState::<Impl, OFFSET>,
            RestoreDrawingState::<Impl, OFFSET>,
            PushAxisAlignedClip::<Impl, OFFSET>,
            PopAxisAlignedClip::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            BeginDraw::<Impl, OFFSET>,
            EndDraw::<Impl, OFFSET>,
            GetPixelFormat::<Impl, OFFSET>,
            SetDpi::<Impl, OFFSET>,
            GetDpi::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            GetPixelSize::<Impl, OFFSET>,
            GetMaximumBitmapSize::<Impl, OFFSET>,
            IsSupported::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1ResourceImpl: Sized {
    fn GetFactory();
}
impl ::windows::core::RuntimeName for ID2D1Resource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Resource";
}
impl ID2D1ResourceVtbl {
    pub const fn new<Impl: ID2D1ResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ResourceVtbl {
        unsafe extern "system" fn GetFactory<Impl: ID2D1ResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFactory(::core::mem::transmute_copy(&factory)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Resource>, base.5, GetFactory::<Impl, OFFSET>)
    }
}
pub trait ID2D1ResourceTextureImpl: Sized {
    fn Update();
}
impl ::windows::core::RuntimeName for ID2D1ResourceTexture {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1ResourceTexture";
}
impl ID2D1ResourceTextureVtbl {
    pub const fn new<Impl: ID2D1ResourceTextureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1ResourceTextureVtbl {
        unsafe extern "system" fn Update<Impl: ID2D1ResourceTextureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update(minimumextents, maximimumextents, strides, dimensions, data, datacount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1ResourceTexture>, base.5, Update::<Impl, OFFSET>)
    }
}
pub trait ID2D1RoundedRectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRoundedRect();
}
impl ::windows::core::RuntimeName for ID2D1RoundedRectangleGeometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1RoundedRectangleGeometry";
}
impl ID2D1RoundedRectangleGeometryVtbl {
    pub const fn new<Impl: ID2D1RoundedRectangleGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1RoundedRectangleGeometryVtbl {
        unsafe extern "system" fn GetRoundedRect<Impl: ID2D1RoundedRectangleGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetRoundedRect(::core::mem::transmute_copy(&roundedrect)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1RoundedRectangleGeometry>, base.5, GetRoundedRect::<Impl, OFFSET>)
    }
}
pub trait ID2D1SolidColorBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetColor();
    fn GetColor();
}
impl ::windows::core::RuntimeName for ID2D1SolidColorBrush {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SolidColorBrush";
}
impl ID2D1SolidColorBrushVtbl {
    pub const fn new<Impl: ID2D1SolidColorBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SolidColorBrushVtbl {
        unsafe extern "system" fn SetColor<Impl: ID2D1SolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&color as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetColor<Impl: ID2D1SolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SolidColorBrush>, base.5, SetColor::<Impl, OFFSET>, GetColor::<Impl, OFFSET>)
    }
}
pub trait ID2D1SourceTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetRenderInfo();
    fn Draw();
}
impl ::windows::core::RuntimeName for ID2D1SourceTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SourceTransform";
}
impl ID2D1SourceTransformVtbl {
    pub const fn new<Impl: ID2D1SourceTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SourceTransformVtbl {
        unsafe extern "system" fn SetRenderInfo<Impl: ID2D1SourceTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRenderInfo(&*(&renderinfo as *const <ID2D1RenderInfo as ::windows::core::Abi>::Abi as *const <ID2D1RenderInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: ID2D1SourceTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&target as *const <ID2D1Bitmap1 as ::windows::core::Abi>::Abi as *const <ID2D1Bitmap1 as ::windows::core::DefaultType>::DefaultType), &*(&drawrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&targetorigin as *const <Common::D2D_POINT_2U as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2U as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SourceTransform>, base.5, SetRenderInfo::<Impl, OFFSET>, Draw::<Impl, OFFSET>)
    }
}
pub trait ID2D1SpriteBatchImpl: Sized + ID2D1ResourceImpl {
    fn AddSprites();
    fn SetSprites();
    fn GetSprites();
    fn GetSpriteCount();
    fn Clear();
}
impl ::windows::core::RuntimeName for ID2D1SpriteBatch {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SpriteBatch";
}
impl ID2D1SpriteBatchVtbl {
    pub const fn new<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SpriteBatchVtbl {
        unsafe extern "system" fn AddSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSprites(
                spritecount,
                &*(&destinationrectangles as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcerectangles as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType),
                &*(&colors as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType),
                &*(&transforms as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                destinationrectanglesstride,
                sourcerectanglesstride,
                colorsstride,
                transformsstride,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSprites(
                startindex,
                spritecount,
                &*(&destinationrectangles as *const <Common::D2D_RECT_F as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_F as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcerectangles as *const <Common::D2D_RECT_U as ::windows::core::Abi>::Abi as *const <Common::D2D_RECT_U as ::windows::core::DefaultType>::DefaultType),
                &*(&colors as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType),
                &*(&transforms as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType),
                destinationrectanglesstride,
                sourcerectanglesstride,
                colorsstride,
                transformsstride,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSprites(startindex, spritecount, ::core::mem::transmute_copy(&destinationrectangles), ::core::mem::transmute_copy(&sourcerectangles), ::core::mem::transmute_copy(&colors), ::core::mem::transmute_copy(&transforms)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpriteCount<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpriteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ID2D1SpriteBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SpriteBatch>, base.5, AddSprites::<Impl, OFFSET>, SetSprites::<Impl, OFFSET>, GetSprites::<Impl, OFFSET>, GetSpriteCount::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait ID2D1StrokeStyleImpl: Sized + ID2D1ResourceImpl {
    fn GetStartCap();
    fn GetEndCap();
    fn GetDashCap();
    fn GetMiterLimit();
    fn GetLineJoin();
    fn GetDashOffset();
    fn GetDashStyle();
    fn GetDashesCount();
    fn GetDashes();
}
impl ::windows::core::RuntimeName for ID2D1StrokeStyle {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1StrokeStyle";
}
impl ID2D1StrokeStyleVtbl {
    pub const fn new<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1StrokeStyleVtbl {
        unsafe extern "system" fn GetStartCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStartCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEndCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiterLimit<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineJoin<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_LINE_JOIN {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashOffset<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashStyle<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_DASH_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashesCount<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashesCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1StrokeStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDashes(::core::mem::transmute_copy(&dashes), dashescount).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1StrokeStyle>, base.5, GetStartCap::<Impl, OFFSET>, GetEndCap::<Impl, OFFSET>, GetDashCap::<Impl, OFFSET>, GetMiterLimit::<Impl, OFFSET>, GetLineJoin::<Impl, OFFSET>, GetDashOffset::<Impl, OFFSET>, GetDashStyle::<Impl, OFFSET>, GetDashesCount::<Impl, OFFSET>, GetDashes::<Impl, OFFSET>)
    }
}
pub trait ID2D1StrokeStyle1Impl: Sized + ID2D1StrokeStyleImpl + ID2D1ResourceImpl {
    fn GetStrokeTransformType();
}
impl ::windows::core::RuntimeName for ID2D1StrokeStyle1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1StrokeStyle1";
}
impl ID2D1StrokeStyle1Vtbl {
    pub const fn new<Impl: ID2D1StrokeStyle1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1StrokeStyle1Vtbl {
        unsafe extern "system" fn GetStrokeTransformType<Impl: ID2D1StrokeStyle1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeTransformType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1StrokeStyle1>, base.5, GetStrokeTransformType::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgAttributeImpl: Sized + ID2D1ResourceImpl {
    fn GetElement();
    fn Clone();
}
impl ::windows::core::RuntimeName for ID2D1SvgAttribute {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgAttribute";
}
impl ID2D1SvgAttributeVtbl {
    pub const fn new<Impl: ID2D1SvgAttributeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgAttributeVtbl {
        unsafe extern "system" fn GetElement<Impl: ID2D1SvgAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetElement(::core::mem::transmute_copy(&element)).into()
        }
        unsafe extern "system" fn Clone<Impl: ID2D1SvgAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&attribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgAttribute>, base.5, GetElement::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgDocumentImpl: Sized + ID2D1ResourceImpl {
    fn SetViewportSize();
    fn GetViewportSize();
    fn SetRoot();
    fn GetRoot();
    fn FindElementById();
    fn Serialize();
    fn Deserialize();
    fn CreatePaint();
    fn CreateStrokeDashArray();
    fn CreatePointCollection();
    fn CreatePathData();
}
impl ::windows::core::RuntimeName for ID2D1SvgDocument {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgDocument";
}
impl ID2D1SvgDocumentVtbl {
    pub const fn new<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgDocumentVtbl {
        unsafe extern "system" fn SetViewportSize<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewportSize(&*(&viewportsize as *const <Common::D2D_SIZE_F as ::windows::core::Abi>::Abi as *const <Common::D2D_SIZE_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewportSize<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoot<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRoot(&*(&root as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoot<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetRoot(::core::mem::transmute_copy(&root)).into()
        }
        unsafe extern "system" fn FindElementById<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR, svgelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindElementById(&*(&id as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&svgelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputxmlstream: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&outputxmlstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&subtree as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputxmlstream: ::windows::core::RawPtr, subtree: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deserialize(&*(&inputxmlstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&subtree)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePaint<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: super::super::Foundation::PWSTR, paint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePaint(painttype, &*(&color as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType), &*(&id as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&paint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeDashArray<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStrokeDashArray(&*(&dashes as *const <D2D1_SVG_LENGTH as ::windows::core::Abi>::Abi as *const <D2D1_SVG_LENGTH as ::windows::core::DefaultType>::DefaultType), dashescount, ::core::mem::transmute_copy(&strokedasharray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePointCollection<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePointCollection(&*(&points as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), pointscount, ::core::mem::transmute_copy(&pointcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathData<Impl: ID2D1SvgDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePathData(segmentdata, segmentdatacount, commands, commandscount, ::core::mem::transmute_copy(&pathdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgDocument>, base.5, SetViewportSize::<Impl, OFFSET>, GetViewportSize::<Impl, OFFSET>, SetRoot::<Impl, OFFSET>, GetRoot::<Impl, OFFSET>, FindElementById::<Impl, OFFSET>, Serialize::<Impl, OFFSET>, Deserialize::<Impl, OFFSET>, CreatePaint::<Impl, OFFSET>, CreateStrokeDashArray::<Impl, OFFSET>, CreatePointCollection::<Impl, OFFSET>, CreatePathData::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgElementImpl: Sized + ID2D1ResourceImpl {
    fn GetDocument();
    fn GetTagName();
    fn GetTagNameLength();
    fn IsTextContent();
    fn GetParent();
    fn HasChildren();
    fn GetFirstChild();
    fn GetLastChild();
    fn GetPreviousChild();
    fn GetNextChild();
    fn InsertChildBefore();
    fn AppendChild();
    fn ReplaceChild();
    fn RemoveChild();
    fn CreateChild();
    fn IsAttributeSpecified();
    fn GetSpecifiedAttributeCount();
    fn GetSpecifiedAttributeName();
    fn GetSpecifiedAttributeNameLength();
    fn RemoveAttribute();
    fn SetTextValue();
    fn GetTextValue();
    fn GetTextValueLength();
    fn SetAttributeValue();
    fn SetAttributeValue();
    fn SetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValueLength();
}
impl ::windows::core::RuntimeName for ID2D1SvgElement {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgElement";
}
impl ID2D1SvgElementVtbl {
    pub const fn new<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgElementVtbl {
        unsafe extern "system" fn GetDocument<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDocument(::core::mem::transmute_copy(&document)).into()
        }
        unsafe extern "system" fn GetTagName<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTagName(::core::mem::transmute_copy(&name), namecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTagNameLength<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTagNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTextContent<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTextContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetParent(::core::mem::transmute_copy(&parent)).into()
        }
        unsafe extern "system" fn HasChildren<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, child: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFirstChild(::core::mem::transmute_copy(&child)).into()
        }
        unsafe extern "system" fn GetLastChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, child: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetLastChild(::core::mem::transmute_copy(&child)).into()
        }
        unsafe extern "system" fn GetPreviousChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencechild: ::windows::core::RawPtr, previouschild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousChild(&*(&referencechild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&previouschild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencechild: ::windows::core::RawPtr, nextchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextChild(&*(&referencechild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nextchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChildBefore<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertChildBefore(&*(&newchild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType), &*(&referencechild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendChild(&*(&newchild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReplaceChild(&*(&newchild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType), &*(&oldchild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveChild(&*(&oldchild as *const <ID2D1SvgElement as ::windows::core::Abi>::Abi as *const <ID2D1SvgElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChild<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: super::super::Foundation::PWSTR, newchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateChild(&*(&tagname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAttributeSpecified<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAttributeSpecified(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inherited)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeCount<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeName<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: super::super::Foundation::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedAttributeName(index, ::core::mem::transmute_copy(&name), namecount, ::core::mem::transmute_copy(&inherited)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeNameLength<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedAttributeNameLength(index, ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&inherited)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAttribute<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAttribute(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTextValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), namecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTextValue(::core::mem::transmute_copy(&name), namecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextValueLength<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTextValueLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <ID2D1SvgAttribute as ::windows::core::Abi>::Abi as *const <ID2D1SvgAttribute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, &*(&value as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), valuesizeinbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, &*(&value as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&value), valuesizeinbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: super::super::Foundation::PWSTR, valuecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&value), valuecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValueLength<Impl: ID2D1SvgElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeValueLength(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&valuelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ID2D1SvgElement>,
            base.5,
            GetDocument::<Impl, OFFSET>,
            GetTagName::<Impl, OFFSET>,
            GetTagNameLength::<Impl, OFFSET>,
            IsTextContent::<Impl, OFFSET>,
            GetParent::<Impl, OFFSET>,
            HasChildren::<Impl, OFFSET>,
            GetFirstChild::<Impl, OFFSET>,
            GetLastChild::<Impl, OFFSET>,
            GetPreviousChild::<Impl, OFFSET>,
            GetNextChild::<Impl, OFFSET>,
            InsertChildBefore::<Impl, OFFSET>,
            AppendChild::<Impl, OFFSET>,
            ReplaceChild::<Impl, OFFSET>,
            RemoveChild::<Impl, OFFSET>,
            CreateChild::<Impl, OFFSET>,
            IsAttributeSpecified::<Impl, OFFSET>,
            GetSpecifiedAttributeCount::<Impl, OFFSET>,
            GetSpecifiedAttributeName::<Impl, OFFSET>,
            GetSpecifiedAttributeNameLength::<Impl, OFFSET>,
            RemoveAttribute::<Impl, OFFSET>,
            SetTextValue::<Impl, OFFSET>,
            GetTextValue::<Impl, OFFSET>,
            GetTextValueLength::<Impl, OFFSET>,
            SetAttributeValue::<Impl, OFFSET>,
            SetAttributeValue::<Impl, OFFSET>,
            SetAttributeValue::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetAttributeValueLength::<Impl, OFFSET>,
        )
    }
}
pub trait ID2D1SvgGlyphStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetFill();
    fn GetFill();
    fn SetStroke();
    fn GetStrokeDashesCount();
    fn GetStroke();
}
impl ::windows::core::RuntimeName for ID2D1SvgGlyphStyle {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgGlyphStyle";
}
impl ID2D1SvgGlyphStyleVtbl {
    pub const fn new<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgGlyphStyleVtbl {
        unsafe extern "system" fn SetFill<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFill(&*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFill<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFill(::core::mem::transmute_copy(&brush)).into()
        }
        unsafe extern "system" fn SetStroke<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStroke(&*(&brush as *const <ID2D1Brush as ::windows::core::Abi>::Abi as *const <ID2D1Brush as ::windows::core::DefaultType>::DefaultType), strokewidth, dashes, dashescount, dashoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashesCount<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashesCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStroke<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetStroke(::core::mem::transmute_copy(&brush), ::core::mem::transmute_copy(&strokewidth), ::core::mem::transmute_copy(&dashes), dashescount, ::core::mem::transmute_copy(&dashoffset)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgGlyphStyle>, base.5, SetFill::<Impl, OFFSET>, GetFill::<Impl, OFFSET>, SetStroke::<Impl, OFFSET>, GetStrokeDashesCount::<Impl, OFFSET>, GetStroke::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgPaintImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn SetPaintType();
    fn GetPaintType();
    fn SetColor();
    fn GetColor();
    fn SetId();
    fn GetId();
    fn GetIdLength();
}
impl ::windows::core::RuntimeName for ID2D1SvgPaint {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgPaint";
}
impl ID2D1SvgPaintVtbl {
    pub const fn new<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgPaintVtbl {
        unsafe extern "system" fn SetPaintType<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPaintType(painttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPaintType<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPaintType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColor(&*(&color as *const <Common::D2D1_COLOR_F as ::windows::core::Abi>::Abi as *const <Common::D2D1_COLOR_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColor<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetId<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetId(&*(&id as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR, idcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&id), idcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdLength<Impl: ID2D1SvgPaintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgPaint>, base.5, SetPaintType::<Impl, OFFSET>, GetPaintType::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, GetColor::<Impl, OFFSET>, SetId::<Impl, OFFSET>, GetId::<Impl, OFFSET>, GetIdLength::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgPathDataImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemoveSegmentDataAtEnd();
    fn UpdateSegmentData();
    fn GetSegmentData();
    fn GetSegmentDataCount();
    fn RemoveCommandsAtEnd();
    fn UpdateCommands();
    fn GetCommands();
    fn GetCommandsCount();
    fn CreatePathGeometry();
}
impl ::windows::core::RuntimeName for ID2D1SvgPathData {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgPathData";
}
impl ID2D1SvgPathDataVtbl {
    pub const fn new<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgPathDataVtbl {
        unsafe extern "system" fn RemoveSegmentDataAtEnd<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSegmentDataAtEnd(datacount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSegmentData<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateSegmentData(data, datacount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentData(::core::mem::transmute_copy(&data), datacount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentDataCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandsAtEnd<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveCommandsAtEnd(commandscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCommands<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateCommands(commands, commandscount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommands(::core::mem::transmute_copy(&commands), commandscount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandsCount<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommandsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1SvgPathDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePathGeometry(fillmode, ::core::mem::transmute_copy(&pathgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgPathData>, base.5, RemoveSegmentDataAtEnd::<Impl, OFFSET>, UpdateSegmentData::<Impl, OFFSET>, GetSegmentData::<Impl, OFFSET>, GetSegmentDataCount::<Impl, OFFSET>, RemoveCommandsAtEnd::<Impl, OFFSET>, UpdateCommands::<Impl, OFFSET>, GetCommands::<Impl, OFFSET>, GetCommandsCount::<Impl, OFFSET>, CreatePathGeometry::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgPointCollectionImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemovePointsAtEnd();
    fn UpdatePoints();
    fn GetPoints();
    fn GetPointsCount();
}
impl ::windows::core::RuntimeName for ID2D1SvgPointCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgPointCollection";
}
impl ID2D1SvgPointCollectionVtbl {
    pub const fn new<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgPointCollectionVtbl {
        unsafe extern "system" fn RemovePointsAtEnd<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemovePointsAtEnd(pointscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePoints<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdatePoints(&*(&points as *const <Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), pointscount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoints<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPoints(::core::mem::transmute_copy(&points), pointscount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPointsCount<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPointsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgPointCollection>, base.5, RemovePointsAtEnd::<Impl, OFFSET>, UpdatePoints::<Impl, OFFSET>, GetPoints::<Impl, OFFSET>, GetPointsCount::<Impl, OFFSET>)
    }
}
pub trait ID2D1SvgStrokeDashArrayImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemoveDashesAtEnd();
    fn UpdateDashes();
    fn UpdateDashes();
    fn GetDashes();
    fn GetDashes();
    fn GetDashesCount();
}
impl ::windows::core::RuntimeName for ID2D1SvgStrokeDashArray {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1SvgStrokeDashArray";
}
impl ID2D1SvgStrokeDashArrayVtbl {
    pub const fn new<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1SvgStrokeDashArrayVtbl {
        unsafe extern "system" fn RemoveDashesAtEnd<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveDashesAtEnd(dashescount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateDashes(&*(&dashes as *const <D2D1_SVG_LENGTH as ::windows::core::Abi>::Abi as *const <D2D1_SVG_LENGTH as ::windows::core::DefaultType>::DefaultType), dashescount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateDashes(dashes, dashescount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashes(::core::mem::transmute_copy(&dashes), dashescount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashes(::core::mem::transmute_copy(&dashes), dashescount, startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDashesCount<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDashesCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1SvgStrokeDashArray>, base.5, RemoveDashesAtEnd::<Impl, OFFSET>, UpdateDashes::<Impl, OFFSET>, UpdateDashes::<Impl, OFFSET>, GetDashes::<Impl, OFFSET>, GetDashes::<Impl, OFFSET>, GetDashesCount::<Impl, OFFSET>)
    }
}
pub trait ID2D1TessellationSinkImpl: Sized {
    fn AddTriangles();
    fn Close();
}
impl ::windows::core::RuntimeName for ID2D1TessellationSink {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1TessellationSink";
}
impl ID2D1TessellationSinkVtbl {
    pub const fn new<Impl: ID2D1TessellationSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TessellationSinkVtbl {
        unsafe extern "system" fn AddTriangles<Impl: ID2D1TessellationSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddTriangles(&*(&triangles as *const <D2D1_TRIANGLE as ::windows::core::Abi>::Abi as *const <D2D1_TRIANGLE as ::windows::core::DefaultType>::DefaultType), trianglescount).into()
        }
        unsafe extern "system" fn Close<Impl: ID2D1TessellationSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1TessellationSink>, base.5, AddTriangles::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ID2D1TransformImpl: Sized + ID2D1TransformNodeImpl {
    fn MapOutputRectToInputRects();
    fn MapInputRectsToOutputRect();
    fn MapInvalidRect();
}
impl ::windows::core::RuntimeName for ID2D1Transform {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1Transform";
}
impl ID2D1TransformVtbl {
    pub const fn new<Impl: ID2D1TransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TransformVtbl {
        unsafe extern "system" fn MapOutputRectToInputRects<Impl: ID2D1TransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapOutputRectToInputRects(&*(&outputrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inputrects), inputrectscount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapInputRectsToOutputRect<Impl: ID2D1TransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapInputRectsToOutputRect(&*(&inputrects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&inputopaquesubrects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), inputrectcount, ::core::mem::transmute_copy(&outputrect), ::core::mem::transmute_copy(&outputopaquesubrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapInvalidRect<Impl: ID2D1TransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapInvalidRect(inputindex, &*(&invalidinputrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&invalidoutputrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1Transform>, base.5, MapOutputRectToInputRects::<Impl, OFFSET>, MapInputRectsToOutputRect::<Impl, OFFSET>, MapInvalidRect::<Impl, OFFSET>)
    }
}
pub trait ID2D1TransformGraphImpl: Sized {
    fn GetInputCount();
    fn SetSingleTransformNode();
    fn AddNode();
    fn RemoveNode();
    fn SetOutputNode();
    fn ConnectNode();
    fn ConnectToEffectInput();
    fn Clear();
    fn SetPassthroughGraph();
}
impl ::windows::core::RuntimeName for ID2D1TransformGraph {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1TransformGraph";
}
impl ID2D1TransformGraphVtbl {
    pub const fn new<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TransformGraphVtbl {
        unsafe extern "system" fn GetInputCount<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSingleTransformNode<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSingleTransformNode(&*(&node as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNode<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddNode(&*(&node as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNode<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveNode(&*(&node as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputNode<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputNode(&*(&node as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectNode<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fromnode: ::windows::core::RawPtr, tonode: ::windows::core::RawPtr, tonodeinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectNode(&*(&fromnode as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType), &*(&tonode as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType), tonodeinputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectToEffectInput<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, toeffectinputindex: u32, node: ::windows::core::RawPtr, tonodeinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectToEffectInput(toeffectinputindex, &*(&node as *const <ID2D1TransformNode as ::windows::core::Abi>::Abi as *const <ID2D1TransformNode as ::windows::core::DefaultType>::DefaultType), tonodeinputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetPassthroughGraph<Impl: ID2D1TransformGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPassthroughGraph(effectinputindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1TransformGraph>, base.5, GetInputCount::<Impl, OFFSET>, SetSingleTransformNode::<Impl, OFFSET>, AddNode::<Impl, OFFSET>, RemoveNode::<Impl, OFFSET>, SetOutputNode::<Impl, OFFSET>, ConnectNode::<Impl, OFFSET>, ConnectToEffectInput::<Impl, OFFSET>, Clear::<Impl, OFFSET>, SetPassthroughGraph::<Impl, OFFSET>)
    }
}
pub trait ID2D1TransformNodeImpl: Sized {
    fn GetInputCount();
}
impl ::windows::core::RuntimeName for ID2D1TransformNode {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1TransformNode";
}
impl ID2D1TransformNodeVtbl {
    pub const fn new<Impl: ID2D1TransformNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TransformNodeVtbl {
        unsafe extern "system" fn GetInputCount<Impl: ID2D1TransformNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1TransformNode>, base.5, GetInputCount::<Impl, OFFSET>)
    }
}
pub trait ID2D1TransformedGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetSourceGeometry();
    fn GetTransform();
}
impl ::windows::core::RuntimeName for ID2D1TransformedGeometry {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1TransformedGeometry";
}
impl ID2D1TransformedGeometryVtbl {
    pub const fn new<Impl: ID2D1TransformedGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TransformedGeometryVtbl {
        unsafe extern "system" fn GetSourceGeometry<Impl: ID2D1TransformedGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcegeometry: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSourceGeometry(::core::mem::transmute_copy(&sourcegeometry)).into()
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1TransformedGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&transform)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1TransformedGeometry>, base.5, GetSourceGeometry::<Impl, OFFSET>, GetTransform::<Impl, OFFSET>)
    }
}
pub trait ID2D1TransformedImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSource();
    fn GetProperties();
}
impl ::windows::core::RuntimeName for ID2D1TransformedImageSource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1TransformedImageSource";
}
impl ID2D1TransformedImageSourceVtbl {
    pub const fn new<Impl: ID2D1TransformedImageSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1TransformedImageSourceVtbl {
        unsafe extern "system" fn GetSource<Impl: ID2D1TransformedImageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagesource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSource(::core::mem::transmute_copy(&imagesource)).into()
        }
        unsafe extern "system" fn GetProperties<Impl: ID2D1TransformedImageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetProperties(::core::mem::transmute_copy(&properties)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1TransformedImageSource>, base.5, GetSource::<Impl, OFFSET>, GetProperties::<Impl, OFFSET>)
    }
}
pub trait ID2D1VertexBufferImpl: Sized {
    fn Map();
    fn Unmap();
}
impl ::windows::core::RuntimeName for ID2D1VertexBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct2D.ID2D1VertexBuffer";
}
impl ID2D1VertexBufferVtbl {
    pub const fn new<Impl: ID2D1VertexBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID2D1VertexBufferVtbl {
        unsafe extern "system" fn Map<Impl: ID2D1VertexBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Map(::core::mem::transmute_copy(&data), buffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID2D1VertexBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID2D1VertexBuffer>, base.5, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>)
    }
}
