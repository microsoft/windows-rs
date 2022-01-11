pub trait ID2D1AnalysisTransformImpl: Sized {
    fn ProcessAnalysisResults();
}
impl ID2D1AnalysisTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1AnalysisTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1AnalysisTransformVtbl {
        unsafe extern "system" fn ProcessAnalysisResults<Impl: ID2D1AnalysisTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysisdata: *const u8, analysisdatacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ProcessAnalysisResults::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1AnalysisTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1BitmapImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSize();
    fn GetPixelSize();
    fn GetPixelFormat();
    fn GetDpi();
    fn CopyFromBitmap();
    fn CopyFromRenderTarget();
    fn CopyFromMemory();
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1BitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BitmapVtbl {
        unsafe extern "system" fn GetSize<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelSize<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDpi<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyFromBitmap<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, bitmap: ::windows::core::RawPtr, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyFromRenderTarget<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, rendertarget: ::windows::core::RawPtr, srcrect: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyFromMemory<Impl: ID2D1BitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetDpi::<Impl, IMPL_OFFSET>, CopyFromBitmap::<Impl, IMPL_OFFSET>, CopyFromRenderTarget::<Impl, IMPL_OFFSET>, CopyFromMemory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Bitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1Bitmap1Impl: Sized + ID2D1BitmapImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetColorContext();
    fn GetOptions();
    fn GetSurface();
    fn Map();
    fn Unmap();
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1Bitmap1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Bitmap1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Bitmap1Vtbl {
        unsafe extern "system" fn GetColorContext<Impl: ID2D1Bitmap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorcontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptions<Impl: ID2D1Bitmap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_OPTIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSurface<Impl: ID2D1Bitmap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgisurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Map<Impl: ID2D1Bitmap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID2D1Bitmap1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            CopyFromBitmap::<Impl, IMPL_OFFSET>,
            CopyFromRenderTarget::<Impl, IMPL_OFFSET>,
            CopyFromMemory::<Impl, IMPL_OFFSET>,
            GetColorContext::<Impl, IMPL_OFFSET>,
            GetOptions::<Impl, IMPL_OFFSET>,
            GetSurface::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Bitmap1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
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
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1BitmapBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BitmapBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BitmapBrushVtbl {
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBitmap<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterpolationMode<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitmap<Impl: ID2D1BitmapBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetExtendModeX::<Impl, IMPL_OFFSET>,
            SetExtendModeY::<Impl, IMPL_OFFSET>,
            SetInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBitmap::<Impl, IMPL_OFFSET>,
            GetExtendModeX::<Impl, IMPL_OFFSET>,
            GetExtendModeY::<Impl, IMPL_OFFSET>,
            GetInterpolationMode::<Impl, IMPL_OFFSET>,
            GetBitmap::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BitmapBrush1Impl: Sized + ID2D1BitmapBrushImpl + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetInterpolationMode1();
    fn GetInterpolationMode1();
}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1BitmapBrush1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BitmapBrush1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BitmapBrush1Vtbl {
        unsafe extern "system" fn SetInterpolationMode1<Impl: ID2D1BitmapBrush1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterpolationMode1<Impl: ID2D1BitmapBrush1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetExtendModeX::<Impl, IMPL_OFFSET>,
            SetExtendModeY::<Impl, IMPL_OFFSET>,
            SetInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBitmap::<Impl, IMPL_OFFSET>,
            GetExtendModeX::<Impl, IMPL_OFFSET>,
            GetExtendModeY::<Impl, IMPL_OFFSET>,
            GetInterpolationMode::<Impl, IMPL_OFFSET>,
            GetBitmap::<Impl, IMPL_OFFSET>,
            SetInterpolationMode1::<Impl, IMPL_OFFSET>,
            GetInterpolationMode1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1BitmapRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn GetBitmap();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1BitmapRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BitmapRenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BitmapRenderTargetVtbl {
        unsafe extern "system" fn GetBitmap<Impl: ID2D1BitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            GetBitmap::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BitmapRenderTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BlendTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetDescription();
    fn GetDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1BlendTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BlendTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BlendTransformVtbl {
        unsafe extern "system" fn SetDescription<Impl: ID2D1BlendTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *const D2D1_BLEND_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: ID2D1BlendTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut D2D1_BLEND_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, SetOutputBuffer::<Impl, IMPL_OFFSET>, SetCached::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BlendTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BorderTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn GetExtendModeX();
    fn GetExtendModeY();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1BorderTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BorderTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BorderTransformVtbl {
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1BorderTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1BorderTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1BorderTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1BorderTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, SetOutputBuffer::<Impl, IMPL_OFFSET>, SetCached::<Impl, IMPL_OFFSET>, SetExtendModeX::<Impl, IMPL_OFFSET>, SetExtendModeY::<Impl, IMPL_OFFSET>, GetExtendModeX::<Impl, IMPL_OFFSET>, GetExtendModeY::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BorderTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BoundsAdjustmentTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBounds();
    fn GetOutputBounds();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1BoundsAdjustmentTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BoundsAdjustmentTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BoundsAdjustmentTransformVtbl {
        unsafe extern "system" fn SetOutputBounds<Impl: ID2D1BoundsAdjustmentTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputbounds: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputBounds<Impl: ID2D1BoundsAdjustmentTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputbounds: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, SetOutputBounds::<Impl, IMPL_OFFSET>, GetOutputBounds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1BoundsAdjustmentTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BrushImpl: Sized + ID2D1ResourceImpl {
    fn SetOpacity();
    fn SetTransform();
    fn GetOpacity();
    fn GetTransform();
}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1BrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1BrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1BrushVtbl {
        unsafe extern "system" fn SetOpacity<Impl: ID2D1BrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1BrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpacity<Impl: ID2D1BrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1BrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, SetOpacity::<Impl, IMPL_OFFSET>, SetTransform::<Impl, IMPL_OFFSET>, GetOpacity::<Impl, IMPL_OFFSET>, GetTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Brush as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1ColorContextImpl: Sized + ID2D1ResourceImpl {
    fn GetColorSpace();
    fn GetProfileSize();
    fn GetProfile();
}
impl ID2D1ColorContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ColorContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ColorContextVtbl {
        unsafe extern "system" fn GetColorSpace<Impl: ID2D1ColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfileSize<Impl: ID2D1ColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfile<Impl: ID2D1ColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut u8, profilesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetColorSpace::<Impl, IMPL_OFFSET>, GetProfileSize::<Impl, IMPL_OFFSET>, GetProfile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ColorContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1ColorContext1Impl: Sized + ID2D1ColorContextImpl + ID2D1ResourceImpl {
    fn GetColorContextType();
    fn GetDXGIColorSpace();
    fn GetSimpleColorProfile();
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1ColorContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ColorContext1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ColorContext1Vtbl {
        unsafe extern "system" fn GetColorContextType<Impl: ID2D1ColorContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDXGIColorSpace<Impl: ID2D1ColorContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSimpleColorProfile<Impl: ID2D1ColorContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetColorSpace::<Impl, IMPL_OFFSET>, GetProfileSize::<Impl, IMPL_OFFSET>, GetProfile::<Impl, IMPL_OFFSET>, GetColorContextType::<Impl, IMPL_OFFSET>, GetDXGIColorSpace::<Impl, IMPL_OFFSET>, GetSimpleColorProfile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ColorContext1 as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1CommandListImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn Stream();
    fn Close();
}
impl ID2D1CommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandListVtbl {
        unsafe extern "system" fn Stream<Impl: ID2D1CommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ID2D1CommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, Stream::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSinkVtbl {
        unsafe extern "system" fn BeginDraw<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDraw<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntialiasMode<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTags<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrimitiveBlend<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnitMode<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawLine<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGeometry<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawRectangle<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawImage<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillMesh<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mesh: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillGeometry<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, opacitybrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillRectangle<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushAxisAlignedClip<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopAxisAlignedClip<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopLayer<Impl: ID2D1CommandSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink1Impl: Sized + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend1();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSink1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSink1Vtbl {
        unsafe extern "system" fn SetPrimitiveBlend1<Impl: ID2D1CommandSink1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink2Impl: Sized + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawInk();
    fn DrawGradientMesh();
    fn DrawGdiMetafile();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSink2Vtbl {
        unsafe extern "system" fn DrawInk<Impl: ID2D1CommandSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, inkstyle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGradientMesh<Impl: ID2D1CommandSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1CommandSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend1::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink3Impl: Sized + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawSpriteBatch();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSink3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSink3Vtbl {
        unsafe extern "system" fn DrawSpriteBatch<Impl: ID2D1CommandSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spritebatch: ::windows::core::RawPtr, startindex: u32, spritecount: u32, bitmap: ::windows::core::RawPtr, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend1::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink4Impl: Sized + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend2();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSink4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSink4Vtbl {
        unsafe extern "system" fn SetPrimitiveBlend2<Impl: ID2D1CommandSink4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend1::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend2::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink5Impl: Sized + ID2D1CommandSink4Impl + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn BlendImage();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1CommandSink5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1CommandSink5Vtbl {
        unsafe extern "system" fn BlendImage<Impl: ID2D1CommandSink5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend1::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend2::<Impl, IMPL_OFFSET>,
            BlendImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1CommandSink5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ComputeInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetComputeShaderConstantBuffer();
    fn SetComputeShader();
    fn SetResourceTexture();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1ComputeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ComputeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ComputeInfoVtbl {
        unsafe extern "system" fn SetComputeShaderConstantBuffer<Impl: ID2D1ComputeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComputeShader<Impl: ID2D1ComputeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResourceTexture<Impl: ID2D1ComputeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetInputDescription::<Impl, IMPL_OFFSET>, SetOutputBuffer::<Impl, IMPL_OFFSET>, SetCached::<Impl, IMPL_OFFSET>, SetInstructionCountHint::<Impl, IMPL_OFFSET>, SetComputeShaderConstantBuffer::<Impl, IMPL_OFFSET>, SetComputeShader::<Impl, IMPL_OFFSET>, SetResourceTexture::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ComputeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ComputeTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetComputeInfo();
    fn CalculateThreadgroups();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1ComputeTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ComputeTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ComputeTransformVtbl {
        unsafe extern "system" fn SetComputeInfo<Impl: ID2D1ComputeTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, computeinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalculateThreadgroups<Impl: ID2D1ComputeTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, MapOutputRectToInputRects::<Impl, IMPL_OFFSET>, MapInputRectsToOutputRect::<Impl, IMPL_OFFSET>, MapInvalidRect::<Impl, IMPL_OFFSET>, SetComputeInfo::<Impl, IMPL_OFFSET>, CalculateThreadgroups::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ComputeTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ConcreteTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBuffer();
    fn SetCached();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1ConcreteTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ConcreteTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ConcreteTransformVtbl {
        unsafe extern "system" fn SetOutputBuffer<Impl: ID2D1ConcreteTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCached<Impl: ID2D1ConcreteTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, SetOutputBuffer::<Impl, IMPL_OFFSET>, SetCached::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ConcreteTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DCRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BindDC();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DCRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DCRenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DCRenderTargetVtbl {
        unsafe extern "system" fn BindDC<Impl: ID2D1DCRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            BindDC::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DCRenderTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1DeviceImpl: Sized + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn CreatePrintControl();
    fn SetMaximumTextureMemory();
    fn GetMaximumTextureMemory();
    fn ClearResources();
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceVtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePrintControl<Impl: ID2D1DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wicfactory: ::windows::core::RawPtr, documenttarget: ::windows::core::RawPtr, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumTextureMemory<Impl: ID2D1DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumTextureMemory<Impl: ID2D1DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearResources<Impl: ID2D1DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, millisecondssinceuse: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, CreateDeviceContext::<Impl, IMPL_OFFSET>, CreatePrintControl::<Impl, IMPL_OFFSET>, SetMaximumTextureMemory::<Impl, IMPL_OFFSET>, GetMaximumTextureMemory::<Impl, IMPL_OFFSET>, ClearResources::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device1Impl: Sized + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn GetRenderingPriority();
    fn SetRenderingPriority();
    fn CreateDeviceContext();
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device1Vtbl {
        unsafe extern "system" fn GetRenderingPriority<Impl: ID2D1Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_RENDERING_PRIORITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderingPriority<Impl: ID2D1Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device2Impl: Sized + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn FlushDeviceContexts();
    fn GetDxgiDevice();
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device2Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushDeviceContexts<Impl: ID2D1Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDxgiDevice<Impl: ID2D1Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            FlushDeviceContexts::<Impl, IMPL_OFFSET>,
            GetDxgiDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device3Impl: Sized + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device3Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            FlushDeviceContexts::<Impl, IMPL_OFFSET>,
            GetDxgiDevice::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device4Impl: Sized + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn SetMaximumColorGlyphCacheMemory();
    fn GetMaximumColorGlyphCacheMemory();
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device4Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumColorGlyphCacheMemory<Impl: ID2D1Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumColorGlyphCacheMemory<Impl: ID2D1Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            FlushDeviceContexts::<Impl, IMPL_OFFSET>,
            GetDxgiDevice::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            SetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
            GetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device5Impl: Sized + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device5Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            FlushDeviceContexts::<Impl, IMPL_OFFSET>,
            GetDxgiDevice::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            SetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
            GetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device6Impl: Sized + ID2D1Device5Impl + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Device6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Device6Vtbl {
        unsafe extern "system" fn CreateDeviceContext<Impl: ID2D1Device6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreatePrintControl::<Impl, IMPL_OFFSET>,
            SetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            GetMaximumTextureMemory::<Impl, IMPL_OFFSET>,
            ClearResources::<Impl, IMPL_OFFSET>,
            GetRenderingPriority::<Impl, IMPL_OFFSET>,
            SetRenderingPriority::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            FlushDeviceContexts::<Impl, IMPL_OFFSET>,
            GetDxgiDevice::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            SetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
            GetMaximumColorGlyphCacheMemory::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
            CreateDeviceContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Device6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContextVtbl {
        unsafe extern "system" fn CreateBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContext<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wiccolorcontext: ::windows::core::RawPtr, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromDxgiSurface<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffect<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGradientStopCollection<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateImageBrush<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapBrush<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCommandList<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDxgiFormatSupported<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageLocalBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, localbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageWorldBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, worldbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphRunWorldBounds<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTarget<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTarget<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderingControls<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderingControls<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrimitiveBlend<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrimitiveBlend<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnitMode<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnitMode<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_UNIT_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawImage<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, targetoffset: *const Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateEffectInputRectangle<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInvalidRectangleCount<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, rectanglecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInvalidRectangles<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectRequiredInputRectangles<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendereffect: ::windows::core::RawPtr, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext1Impl: Sized + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateFilledGeometryRealization();
    fn CreateStrokedGeometryRealization();
    fn DrawGeometryRealization();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext1Vtbl {
        unsafe extern "system" fn CreateFilledGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, flatteningtolerance: f32, geometryrealization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStrokedGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, flatteningtolerance: f32, strokewidth: f32, strokestyle: ::windows::core::RawPtr, geometryrealization: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGeometryRealization<Impl: ID2D1DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryrealization: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext2Vtbl {
        unsafe extern "system" fn CreateInk<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInkStyle<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGradientMesh<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateImageSourceFromWic<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLookupTable3D<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateImageSourceFromDxgi<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surfaces: *const ::windows::core::RawPtr, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientMeshWorldBounds<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr, pbounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInk<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, inkstyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGradientMesh<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientmesh: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGdiMetafile<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdimetafile: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransformedImageSource<Impl: ID2D1DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagesource: ::windows::core::RawPtr, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateInk::<Impl, IMPL_OFFSET>,
            CreateInkStyle::<Impl, IMPL_OFFSET>,
            CreateGradientMesh::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromWic::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromDxgi::<Impl, IMPL_OFFSET>,
            GetGradientMeshWorldBounds::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            CreateTransformedImageSource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext3Impl: Sized + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSpriteBatch();
    fn DrawSpriteBatch();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext3Vtbl {
        unsafe extern "system" fn CreateSpriteBatch<Impl: ID2D1DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spritebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawSpriteBatch<Impl: ID2D1DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spritebatch: ::windows::core::RawPtr, startindex: u32, spritecount: u32, bitmap: ::windows::core::RawPtr, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateInk::<Impl, IMPL_OFFSET>,
            CreateInkStyle::<Impl, IMPL_OFFSET>,
            CreateGradientMesh::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromWic::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromDxgi::<Impl, IMPL_OFFSET>,
            GetGradientMeshWorldBounds::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            CreateTransformedImageSource::<Impl, IMPL_OFFSET>,
            CreateSpriteBatch::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext4Impl: Sized + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgGlyphStyle();
    fn DrawText();
    fn DrawTextLayout();
    fn DrawColorBitmapGlyphRun();
    fn DrawSvgGlyphRun();
    fn GetColorBitmapGlyphImage();
    fn GetSvgGlyphImage();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext4Vtbl {
        unsafe extern "system" fn CreateSvgGlyphStyle<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawText<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawTextLayout<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: ::windows::core::RawPtr, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawColorBitmapGlyphRun<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawSvgGlyphRun<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorBitmapGlyphImage<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: ::windows::core::RawPtr, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSvgGlyphImage<Impl: ID2D1DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorigin: Common::D2D_POINT_2F, fontface: ::windows::core::RawPtr, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: ::windows::core::RawPtr, svgglyphstyle: ::windows::core::RawPtr, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateInk::<Impl, IMPL_OFFSET>,
            CreateInkStyle::<Impl, IMPL_OFFSET>,
            CreateGradientMesh::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromWic::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromDxgi::<Impl, IMPL_OFFSET>,
            GetGradientMeshWorldBounds::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            CreateTransformedImageSource::<Impl, IMPL_OFFSET>,
            CreateSpriteBatch::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
            CreateSvgGlyphStyle::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawColorBitmapGlyphRun::<Impl, IMPL_OFFSET>,
            DrawSvgGlyphRun::<Impl, IMPL_OFFSET>,
            GetColorBitmapGlyphImage::<Impl, IMPL_OFFSET>,
            GetSvgGlyphImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext5Impl: Sized + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgDocument();
    fn DrawSvgDocument();
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1DeviceContext5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext5Vtbl {
        unsafe extern "system" fn CreateSvgDocument<Impl: ID2D1DeviceContext5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputxmlstream: ::windows::core::RawPtr, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawSvgDocument<Impl: ID2D1DeviceContext5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svgdocument: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Impl: ID2D1DeviceContext5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Impl: ID2D1DeviceContext5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateInk::<Impl, IMPL_OFFSET>,
            CreateInkStyle::<Impl, IMPL_OFFSET>,
            CreateGradientMesh::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromWic::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromDxgi::<Impl, IMPL_OFFSET>,
            GetGradientMeshWorldBounds::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            CreateTransformedImageSource::<Impl, IMPL_OFFSET>,
            CreateSpriteBatch::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
            CreateSvgGlyphStyle::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawColorBitmapGlyphRun::<Impl, IMPL_OFFSET>,
            DrawSvgGlyphRun::<Impl, IMPL_OFFSET>,
            GetColorBitmapGlyphImage::<Impl, IMPL_OFFSET>,
            GetSvgGlyphImage::<Impl, IMPL_OFFSET>,
            CreateSvgDocument::<Impl, IMPL_OFFSET>,
            DrawSvgDocument::<Impl, IMPL_OFFSET>,
            CreateColorContextFromDxgiColorSpace::<Impl, IMPL_OFFSET>,
            CreateColorContextFromSimpleColorProfile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext6Impl: Sized + ID2D1DeviceContext5Impl + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BlendImage();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1DeviceContext6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DeviceContext6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DeviceContext6Vtbl {
        unsafe extern "system" fn BlendImage<Impl: ID2D1DeviceContext6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CreateBitmapFromDxgiSurface::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateCommandList::<Impl, IMPL_OFFSET>,
            IsDxgiFormatSupported::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            GetImageLocalBounds::<Impl, IMPL_OFFSET>,
            GetImageWorldBounds::<Impl, IMPL_OFFSET>,
            GetGlyphRunWorldBounds::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
            GetTarget::<Impl, IMPL_OFFSET>,
            SetRenderingControls::<Impl, IMPL_OFFSET>,
            GetRenderingControls::<Impl, IMPL_OFFSET>,
            SetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            GetPrimitiveBlend::<Impl, IMPL_OFFSET>,
            SetUnitMode::<Impl, IMPL_OFFSET>,
            GetUnitMode::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawImage::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            InvalidateEffectInputRectangle::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangleCount::<Impl, IMPL_OFFSET>,
            GetEffectInvalidRectangles::<Impl, IMPL_OFFSET>,
            GetEffectRequiredInputRectangles::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            CreateFilledGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateStrokedGeometryRealization::<Impl, IMPL_OFFSET>,
            DrawGeometryRealization::<Impl, IMPL_OFFSET>,
            CreateInk::<Impl, IMPL_OFFSET>,
            CreateInkStyle::<Impl, IMPL_OFFSET>,
            CreateGradientMesh::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromWic::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateImageSourceFromDxgi::<Impl, IMPL_OFFSET>,
            GetGradientMeshWorldBounds::<Impl, IMPL_OFFSET>,
            DrawInk::<Impl, IMPL_OFFSET>,
            DrawGradientMesh::<Impl, IMPL_OFFSET>,
            DrawGdiMetafile::<Impl, IMPL_OFFSET>,
            CreateTransformedImageSource::<Impl, IMPL_OFFSET>,
            CreateSpriteBatch::<Impl, IMPL_OFFSET>,
            DrawSpriteBatch::<Impl, IMPL_OFFSET>,
            CreateSvgGlyphStyle::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawColorBitmapGlyphRun::<Impl, IMPL_OFFSET>,
            DrawSvgGlyphRun::<Impl, IMPL_OFFSET>,
            GetColorBitmapGlyphImage::<Impl, IMPL_OFFSET>,
            GetSvgGlyphImage::<Impl, IMPL_OFFSET>,
            CreateSvgDocument::<Impl, IMPL_OFFSET>,
            DrawSvgDocument::<Impl, IMPL_OFFSET>,
            CreateColorContextFromDxgiColorSpace::<Impl, IMPL_OFFSET>,
            CreateColorContextFromSimpleColorProfile::<Impl, IMPL_OFFSET>,
            BlendImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DeviceContext6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1DrawInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetPixelShaderConstantBuffer();
    fn SetResourceTexture();
    fn SetVertexShaderConstantBuffer();
    fn SetPixelShader();
    fn SetVertexProcessing();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1DrawInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DrawInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DrawInfoVtbl {
        unsafe extern "system" fn SetPixelShaderConstantBuffer<Impl: ID2D1DrawInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResourceTexture<Impl: ID2D1DrawInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexShaderConstantBuffer<Impl: ID2D1DrawInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelShader<Impl: ID2D1DrawInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVertexProcessing<Impl: ID2D1DrawInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexbuffer: ::windows::core::RawPtr, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetInputDescription::<Impl, IMPL_OFFSET>,
            SetOutputBuffer::<Impl, IMPL_OFFSET>,
            SetCached::<Impl, IMPL_OFFSET>,
            SetInstructionCountHint::<Impl, IMPL_OFFSET>,
            SetPixelShaderConstantBuffer::<Impl, IMPL_OFFSET>,
            SetResourceTexture::<Impl, IMPL_OFFSET>,
            SetVertexShaderConstantBuffer::<Impl, IMPL_OFFSET>,
            SetPixelShader::<Impl, IMPL_OFFSET>,
            SetVertexProcessing::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DrawInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1DrawTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetDrawInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1DrawTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DrawTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DrawTransformVtbl {
        unsafe extern "system" fn SetDrawInfo<Impl: ID2D1DrawTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, MapOutputRectToInputRects::<Impl, IMPL_OFFSET>, MapInputRectsToOutputRect::<Impl, IMPL_OFFSET>, MapInvalidRect::<Impl, IMPL_OFFSET>, SetDrawInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DrawTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlockImpl: Sized + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
    fn SetTextRenderingParams();
    fn GetTextRenderingParams();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1DrawingStateBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DrawingStateBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DrawingStateBlockVtbl {
        unsafe extern "system" fn GetDescription<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextRenderingParams<Impl: ID2D1DrawingStateBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, SetTextRenderingParams::<Impl, IMPL_OFFSET>, GetTextRenderingParams::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlock1Impl: Sized + ID2D1DrawingStateBlockImpl + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1DrawingStateBlock1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1DrawingStateBlock1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1DrawingStateBlock1Vtbl {
        unsafe extern "system" fn GetDescription<Impl: ID2D1DrawingStateBlock1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: ID2D1DrawingStateBlock1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, SetTextRenderingParams::<Impl, IMPL_OFFSET>, GetTextRenderingParams::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1EffectImpl: Sized + ID2D1PropertiesImpl {
    fn SetInput();
    fn SetInputCount();
    fn GetInput();
    fn GetInputCount();
    fn GetOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1EffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EffectVtbl {
        unsafe extern "system" fn SetInput<Impl: ID2D1EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: ::windows::core::RawPtr, invalidate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputCount<Impl: ID2D1EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInput<Impl: ID2D1EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCount<Impl: ID2D1EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutput<Impl: ID2D1EffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputimage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyCount::<Impl, IMPL_OFFSET>,
            GetPropertyName::<Impl, IMPL_OFFSET>,
            GetPropertyNameLength::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetPropertyIndex::<Impl, IMPL_OFFSET>,
            SetValueByName::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            GetValueByName::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            GetValueSize::<Impl, IMPL_OFFSET>,
            GetSubProperties::<Impl, IMPL_OFFSET>,
            SetInput::<Impl, IMPL_OFFSET>,
            SetInputCount::<Impl, IMPL_OFFSET>,
            GetInput::<Impl, IMPL_OFFSET>,
            GetInputCount::<Impl, IMPL_OFFSET>,
            GetOutput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Effect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EffectContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EffectContextVtbl {
        unsafe extern "system" fn GetDpi<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffect<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, effect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumSupportedFeatureLevel<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransformNodeFromEffect<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, transformnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendTransform<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBorderTransform<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOffsetTransform<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBoundsAdjustmentTransform<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputrectangle: *const super::super::Foundation::RECT, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadPixelShader<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadVertexShader<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadComputeShader<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsShaderLoaded<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateResourceTexture<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindResourceTexture<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, resourcetexture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexBuffer<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows::core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindVertexBuffer<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows::core::GUID, buffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContext<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wiccolorcontext: ::windows::core::RawPtr, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Impl: ID2D1EffectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            GetMaximumSupportedFeatureLevel::<Impl, IMPL_OFFSET>,
            CreateTransformNodeFromEffect::<Impl, IMPL_OFFSET>,
            CreateBlendTransform::<Impl, IMPL_OFFSET>,
            CreateBorderTransform::<Impl, IMPL_OFFSET>,
            CreateOffsetTransform::<Impl, IMPL_OFFSET>,
            CreateBoundsAdjustmentTransform::<Impl, IMPL_OFFSET>,
            LoadPixelShader::<Impl, IMPL_OFFSET>,
            LoadVertexShader::<Impl, IMPL_OFFSET>,
            LoadComputeShader::<Impl, IMPL_OFFSET>,
            IsShaderLoaded::<Impl, IMPL_OFFSET>,
            CreateResourceTexture::<Impl, IMPL_OFFSET>,
            FindResourceTexture::<Impl, IMPL_OFFSET>,
            CreateVertexBuffer::<Impl, IMPL_OFFSET>,
            FindVertexBuffer::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1EffectContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext1Impl: Sized + ID2D1EffectContextImpl {
    fn CreateLookupTable3D();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EffectContext1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EffectContext1Vtbl {
        unsafe extern "system" fn CreateLookupTable3D<Impl: ID2D1EffectContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            GetMaximumSupportedFeatureLevel::<Impl, IMPL_OFFSET>,
            CreateTransformNodeFromEffect::<Impl, IMPL_OFFSET>,
            CreateBlendTransform::<Impl, IMPL_OFFSET>,
            CreateBorderTransform::<Impl, IMPL_OFFSET>,
            CreateOffsetTransform::<Impl, IMPL_OFFSET>,
            CreateBoundsAdjustmentTransform::<Impl, IMPL_OFFSET>,
            LoadPixelShader::<Impl, IMPL_OFFSET>,
            LoadVertexShader::<Impl, IMPL_OFFSET>,
            LoadComputeShader::<Impl, IMPL_OFFSET>,
            IsShaderLoaded::<Impl, IMPL_OFFSET>,
            CreateResourceTexture::<Impl, IMPL_OFFSET>,
            FindResourceTexture::<Impl, IMPL_OFFSET>,
            CreateVertexBuffer::<Impl, IMPL_OFFSET>,
            FindVertexBuffer::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1EffectContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext2Impl: Sized + ID2D1EffectContext1Impl + ID2D1EffectContextImpl {
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EffectContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EffectContext2Vtbl {
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Impl: ID2D1EffectContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Impl: ID2D1EffectContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            GetMaximumSupportedFeatureLevel::<Impl, IMPL_OFFSET>,
            CreateTransformNodeFromEffect::<Impl, IMPL_OFFSET>,
            CreateBlendTransform::<Impl, IMPL_OFFSET>,
            CreateBorderTransform::<Impl, IMPL_OFFSET>,
            CreateOffsetTransform::<Impl, IMPL_OFFSET>,
            CreateBoundsAdjustmentTransform::<Impl, IMPL_OFFSET>,
            LoadPixelShader::<Impl, IMPL_OFFSET>,
            LoadVertexShader::<Impl, IMPL_OFFSET>,
            LoadComputeShader::<Impl, IMPL_OFFSET>,
            IsShaderLoaded::<Impl, IMPL_OFFSET>,
            CreateResourceTexture::<Impl, IMPL_OFFSET>,
            FindResourceTexture::<Impl, IMPL_OFFSET>,
            CreateVertexBuffer::<Impl, IMPL_OFFSET>,
            FindVertexBuffer::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorContextFromFilename::<Impl, IMPL_OFFSET>,
            CreateColorContextFromWicColorContext::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            IsBufferPrecisionSupported::<Impl, IMPL_OFFSET>,
            CreateLookupTable3D::<Impl, IMPL_OFFSET>,
            CreateColorContextFromDxgiColorSpace::<Impl, IMPL_OFFSET>,
            CreateColorContextFromSimpleColorProfile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1EffectContext2 as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1EffectImplImpl: Sized {
    fn Initialize();
    fn PrepareForRender();
    fn SetGraph();
}
impl ID2D1EffectImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EffectImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EffectImplVtbl {
        unsafe extern "system" fn Initialize<Impl: ID2D1EffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectcontext: ::windows::core::RawPtr, transformgraph: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareForRender<Impl: ID2D1EffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: D2D1_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGraph<Impl: ID2D1EffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformgraph: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, PrepareForRender::<Impl, IMPL_OFFSET>, SetGraph::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1EffectImpl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1EllipseGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetEllipse();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1EllipseGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1EllipseGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1EllipseGeometryVtbl {
        unsafe extern "system" fn GetEllipse<Impl: ID2D1EllipseGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            GetEllipse::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1EllipseGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1FactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1FactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1FactoryVtbl {
        unsafe extern "system" fn ReloadSystemMetrics<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesktopDpi<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRectangleGeometry<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEllipseGeometry<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryGroup<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, geometries: *const ::windows::core::RawPtr, geometriescount: u32, geometrygroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTransformedGeometry<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcegeometry: ::windows::core::RawPtr, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStrokeStyle<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: ::windows::core::RawPtr, drawingstateblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateHwndRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgisurface: ::windows::core::RawPtr, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDCRenderTarget<Impl: ID2D1FactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory1Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStrokeStyle<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: ::windows::core::RawPtr, drawingstateblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGdiMetafile<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metafilestream: ::windows::core::RawPtr, metafile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEffectFromStream<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: ::windows::core::RawPtr, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEffectFromString<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID, propertyxml: super::super::Foundation::PWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterEffect<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisteredEffects<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut ::windows::core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectProperties<Impl: ID2D1Factory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows::core::GUID, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory2Impl: Sized + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory2Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory3Impl: Sized + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory3Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory4Impl: Sized + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory4Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice3: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory5Impl: Sized + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory5Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory6Impl: Sized + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory6Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory7Impl: Sized + ID2D1Factory6Impl + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1Factory7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1Factory7Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: ID2D1Factory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxgidevice: ::windows::core::RawPtr, d2ddevice6: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReloadSystemMetrics::<Impl, IMPL_OFFSET>,
            GetDesktopDpi::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryGroup::<Impl, IMPL_OFFSET>,
            CreateTransformedGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateWicBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateHwndRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDxgiSurfaceRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDCRenderTarget::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateStrokeStyle::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreateDrawingStateBlock::<Impl, IMPL_OFFSET>,
            CreateGdiMetafile::<Impl, IMPL_OFFSET>,
            RegisterEffectFromStream::<Impl, IMPL_OFFSET>,
            RegisterEffectFromString::<Impl, IMPL_OFFSET>,
            UnregisterEffect::<Impl, IMPL_OFFSET>,
            GetRegisteredEffects::<Impl, IMPL_OFFSET>,
            GetEffectProperties::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Factory7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ID2D1GdiInteropRenderTargetImpl: Sized {
    fn GetDC();
    fn ReleaseDC();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ID2D1GdiInteropRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GdiInteropRenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GdiInteropRenderTargetVtbl {
        unsafe extern "system" fn GetDC<Impl: ID2D1GdiInteropRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: ID2D1GdiInteropRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, update: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDC::<Impl, IMPL_OFFSET>, ReleaseDC::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GdiInteropRenderTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafileImpl: Sized + ID2D1ResourceImpl {
    fn Stream();
    fn GetBounds();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GdiMetafileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GdiMetafileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GdiMetafileVtbl {
        unsafe extern "system" fn Stream<Impl: ID2D1GdiMetafileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBounds<Impl: ID2D1GdiMetafileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, Stream::<Impl, IMPL_OFFSET>, GetBounds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafile1Impl: Sized + ID2D1GdiMetafileImpl + ID2D1ResourceImpl {
    fn GetDpi();
    fn GetSourceBounds();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GdiMetafile1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GdiMetafile1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GdiMetafile1Vtbl {
        unsafe extern "system" fn GetDpi<Impl: ID2D1GdiMetafile1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceBounds<Impl: ID2D1GdiMetafile1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, Stream::<Impl, IMPL_OFFSET>, GetBounds::<Impl, IMPL_OFFSET>, GetDpi::<Impl, IMPL_OFFSET>, GetSourceBounds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile1 as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1GdiMetafileSinkImpl: Sized {
    fn ProcessRecord();
}
impl ID2D1GdiMetafileSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GdiMetafileSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GdiMetafileSinkVtbl {
        unsafe extern "system" fn ProcessRecord<Impl: ID2D1GdiMetafileSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ProcessRecord::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1GdiMetafileSink1Impl: Sized + ID2D1GdiMetafileSinkImpl {
    fn ProcessRecord();
}
impl ID2D1GdiMetafileSink1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GdiMetafileSink1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GdiMetafileSink1Vtbl {
        unsafe extern "system" fn ProcessRecord<Impl: ID2D1GdiMetafileSink1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ProcessRecord::<Impl, IMPL_OFFSET>, ProcessRecord::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1GeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GeometryVtbl {
        unsafe extern "system" fn GetBounds<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWidenedBounds<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StrokeContainsPoint<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillContainsPoint<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareWithGeometry<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputgeometry: ::windows::core::RawPtr, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Simplify<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tessellate<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CombineWithGeometry<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputgeometry: ::windows::core::RawPtr, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Outline<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeArea<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeLength<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputePointAtLength<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Widen<Impl: ID2D1GeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Geometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1GeometryGroupImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetFillMode();
    fn GetSourceGeometryCount();
    fn GetSourceGeometries();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1GeometryGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GeometryGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GeometryGroupVtbl {
        unsafe extern "system" fn GetFillMode<Impl: ID2D1GeometryGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> Common::D2D1_FILL_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceGeometryCount<Impl: ID2D1GeometryGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceGeometries<Impl: ID2D1GeometryGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometries: *mut ::windows::core::RawPtr, geometriescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            GetFillMode::<Impl, IMPL_OFFSET>,
            GetSourceGeometryCount::<Impl, IMPL_OFFSET>,
            GetSourceGeometries::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GeometryGroup as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1GeometryRealizationImpl: Sized + ID2D1ResourceImpl {}
impl ID2D1GeometryRealizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GeometryRealizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GeometryRealizationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GeometryRealization as ::windows::core::Interface>::IID
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
impl ID2D1GeometrySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GeometrySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GeometrySinkVtbl {
        unsafe extern "system" fn AddLine<Impl: ID2D1GeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBezier<Impl: ID2D1GeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddQuadraticBezier<Impl: ID2D1GeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddQuadraticBeziers<Impl: ID2D1GeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddArc<Impl: ID2D1GeometrySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetFillMode::<Impl, IMPL_OFFSET>,
            SetSegmentFlags::<Impl, IMPL_OFFSET>,
            BeginFigure::<Impl, IMPL_OFFSET>,
            AddLines::<Impl, IMPL_OFFSET>,
            AddBeziers::<Impl, IMPL_OFFSET>,
            EndFigure::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            AddLine::<Impl, IMPL_OFFSET>,
            AddBezier::<Impl, IMPL_OFFSET>,
            AddQuadraticBezier::<Impl, IMPL_OFFSET>,
            AddQuadraticBeziers::<Impl, IMPL_OFFSET>,
            AddArc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GeometrySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientMeshImpl: Sized + ID2D1ResourceImpl {
    fn GetPatchCount();
    fn GetPatches();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientMeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GradientMeshImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GradientMeshVtbl {
        unsafe extern "system" fn GetPatchCount<Impl: ID2D1GradientMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPatches<Impl: ID2D1GradientMeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetPatchCount::<Impl, IMPL_OFFSET>, GetPatches::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GradientMesh as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollectionImpl: Sized + ID2D1ResourceImpl {
    fn GetGradientStopCount();
    fn GetGradientStops();
    fn GetColorInterpolationGamma();
    fn GetExtendMode();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientStopCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GradientStopCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GradientStopCollectionVtbl {
        unsafe extern "system" fn GetGradientStopCount<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientStops<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorInterpolationGamma<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_GAMMA {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendMode<Impl: ID2D1GradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetGradientStopCount::<Impl, IMPL_OFFSET>, GetGradientStops::<Impl, IMPL_OFFSET>, GetColorInterpolationGamma::<Impl, IMPL_OFFSET>, GetExtendMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollection1Impl: Sized + ID2D1GradientStopCollectionImpl + ID2D1ResourceImpl {
    fn GetGradientStops1();
    fn GetPreInterpolationSpace();
    fn GetPostInterpolationSpace();
    fn GetBufferPrecision();
    fn GetColorInterpolationMode();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientStopCollection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1GradientStopCollection1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1GradientStopCollection1Vtbl {
        unsafe extern "system" fn GetGradientStops1<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreInterpolationSpace<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostInterpolationSpace<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferPrecision<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_BUFFER_PRECISION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: ID2D1GradientStopCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetGradientStopCount::<Impl, IMPL_OFFSET>,
            GetGradientStops::<Impl, IMPL_OFFSET>,
            GetColorInterpolationGamma::<Impl, IMPL_OFFSET>,
            GetExtendMode::<Impl, IMPL_OFFSET>,
            GetGradientStops1::<Impl, IMPL_OFFSET>,
            GetPreInterpolationSpace::<Impl, IMPL_OFFSET>,
            GetPostInterpolationSpace::<Impl, IMPL_OFFSET>,
            GetBufferPrecision::<Impl, IMPL_OFFSET>,
            GetColorInterpolationMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1HwndRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CheckWindowState();
    fn Resize();
    fn GetHwnd();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1HwndRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1HwndRenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1HwndRenderTargetVtbl {
        unsafe extern "system" fn CheckWindowState<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_WINDOW_STATE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resize<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelsize: *const Common::D2D_SIZE_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHwnd<Impl: ID2D1HwndRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HWND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
            CheckWindowState::<Impl, IMPL_OFFSET>,
            Resize::<Impl, IMPL_OFFSET>,
            GetHwnd::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1HwndRenderTarget as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1ImageImpl: Sized + ID2D1ResourceImpl {}
impl ID2D1ImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ImageVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Image as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1ImageBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ImageBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ImageBrushVtbl {
        unsafe extern "system" fn SetImage<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendModeX<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendModeY<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterpolationMode<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceRectangle<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImage<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeX<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtendModeY<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterpolationMode<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceRectangle<Impl: ID2D1ImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerectangle: *mut Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
            SetExtendModeX::<Impl, IMPL_OFFSET>,
            SetExtendModeY::<Impl, IMPL_OFFSET>,
            SetInterpolationMode::<Impl, IMPL_OFFSET>,
            SetSourceRectangle::<Impl, IMPL_OFFSET>,
            GetImage::<Impl, IMPL_OFFSET>,
            GetExtendModeX::<Impl, IMPL_OFFSET>,
            GetExtendModeY::<Impl, IMPL_OFFSET>,
            GetInterpolationMode::<Impl, IMPL_OFFSET>,
            GetSourceRectangle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ImageBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn OfferResources();
    fn TryReclaimResources();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1ImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ImageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ImageSourceVtbl {
        unsafe extern "system" fn OfferResources<Impl: ID2D1ImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TryReclaimResources<Impl: ID2D1ImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, OfferResources::<Impl, IMPL_OFFSET>, TryReclaimResources::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ImageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1ImageSourceFromWicImpl: Sized + ID2D1ImageSourceImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn EnsureCached();
    fn TrimCache();
    fn GetSource();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1ImageSourceFromWicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ImageSourceFromWicImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ImageSourceFromWicVtbl {
        unsafe extern "system" fn EnsureCached<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangletofill: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrimCache<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: ID2D1ImageSourceFromWicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, OfferResources::<Impl, IMPL_OFFSET>, TryReclaimResources::<Impl, IMPL_OFFSET>, EnsureCached::<Impl, IMPL_OFFSET>, TrimCache::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ImageSourceFromWic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1InkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1InkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1InkVtbl {
        unsafe extern "system" fn SetStartPoint<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartPoint<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut D2D1_INK_POINT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSegments<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSegmentsAtEnd<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSegments<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSegmentAtEnd<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentCount<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegments<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StreamAsGeometry<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBounds<Impl: ID2D1InkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstyle: ::windows::core::RawPtr, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetStartPoint::<Impl, IMPL_OFFSET>,
            GetStartPoint::<Impl, IMPL_OFFSET>,
            AddSegments::<Impl, IMPL_OFFSET>,
            RemoveSegmentsAtEnd::<Impl, IMPL_OFFSET>,
            SetSegments::<Impl, IMPL_OFFSET>,
            SetSegmentAtEnd::<Impl, IMPL_OFFSET>,
            GetSegmentCount::<Impl, IMPL_OFFSET>,
            GetSegments::<Impl, IMPL_OFFSET>,
            StreamAsGeometry::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Ink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1InkStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetNibTransform();
    fn GetNibTransform();
    fn SetNibShape();
    fn GetNibShape();
}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1InkStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1InkStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1InkStyleVtbl {
        unsafe extern "system" fn SetNibTransform<Impl: ID2D1InkStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNibTransform<Impl: ID2D1InkStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNibShape<Impl: ID2D1InkStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNibShape<Impl: ID2D1InkStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_INK_NIB_SHAPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, SetNibTransform::<Impl, IMPL_OFFSET>, GetNibTransform::<Impl, IMPL_OFFSET>, SetNibShape::<Impl, IMPL_OFFSET>, GetNibShape::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1InkStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1LayerImpl: Sized + ID2D1ResourceImpl {
    fn GetSize();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1LayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1LayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1LayerVtbl {
        unsafe extern "system" fn GetSize<Impl: ID2D1LayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Layer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1LinearGradientBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetStartPoint();
    fn SetEndPoint();
    fn GetStartPoint();
    fn GetEndPoint();
    fn GetGradientStopCollection();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1LinearGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1LinearGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1LinearGradientBrushVtbl {
        unsafe extern "system" fn SetStartPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEndPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEndPoint<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientStopCollection<Impl: ID2D1LinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetStartPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint::<Impl, IMPL_OFFSET>,
            GetStartPoint::<Impl, IMPL_OFFSET>,
            GetEndPoint::<Impl, IMPL_OFFSET>,
            GetGradientStopCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1LinearGradientBrush as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1LookupTable3DImpl: Sized + ID2D1ResourceImpl {}
impl ID2D1LookupTable3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1LookupTable3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1LookupTable3DVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1LookupTable3D as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1MeshImpl: Sized + ID2D1ResourceImpl {
    fn Open();
}
impl ID2D1MeshVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1MeshImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1MeshVtbl {
        unsafe extern "system" fn Open<Impl: ID2D1MeshImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tessellationsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, Open::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Mesh as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1MultithreadImpl: Sized {
    fn GetMultithreadProtected();
    fn Enter();
    fn Leave();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1MultithreadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1MultithreadVtbl {
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID2D1MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enter<Impl: ID2D1MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Leave<Impl: ID2D1MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMultithreadProtected::<Impl, IMPL_OFFSET>, Enter::<Impl, IMPL_OFFSET>, Leave::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Multithread as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1OffsetTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOffset();
    fn GetOffset();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1OffsetTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1OffsetTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1OffsetTransformVtbl {
        unsafe extern "system" fn SetOffset<Impl: ID2D1OffsetTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOffset<Impl: ID2D1OffsetTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::POINT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, SetOffset::<Impl, IMPL_OFFSET>, GetOffset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1OffsetTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn Open();
    fn Stream();
    fn GetSegmentCount();
    fn GetFigureCount();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1PathGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1PathGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1PathGeometryVtbl {
        unsafe extern "system" fn Open<Impl: ID2D1PathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometrysink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stream<Impl: ID2D1PathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentCount<Impl: ID2D1PathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFigureCount<Impl: ID2D1PathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Stream::<Impl, IMPL_OFFSET>,
            GetSegmentCount::<Impl, IMPL_OFFSET>,
            GetFigureCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1PathGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometry1Impl: Sized + ID2D1PathGeometryImpl + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn ComputePointAndSegmentAtLength();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1PathGeometry1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1PathGeometry1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1PathGeometry1Vtbl {
        unsafe extern "system" fn ComputePointAndSegmentAtLength<Impl: ID2D1PathGeometry1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Stream::<Impl, IMPL_OFFSET>,
            GetSegmentCount::<Impl, IMPL_OFFSET>,
            GetFigureCount::<Impl, IMPL_OFFSET>,
            ComputePointAndSegmentAtLength::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1PathGeometry1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
pub trait ID2D1PrintControlImpl: Sized {
    fn AddPage();
    fn Close();
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ID2D1PrintControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1PrintControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1PrintControlVtbl {
        unsafe extern "system" fn AddPage<Impl: ID2D1PrintControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, pagesize: Common::D2D_SIZE_F, pageprintticketstream: ::windows::core::RawPtr, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ID2D1PrintControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddPage::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1PrintControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ID2D1PropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1PropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1PropertiesVtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyName<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyNameLength<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyIndex<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueByName<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueByName<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueSize<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubProperties<Impl: ID2D1PropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, subproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyCount::<Impl, IMPL_OFFSET>,
            GetPropertyName::<Impl, IMPL_OFFSET>,
            GetPropertyNameLength::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetPropertyIndex::<Impl, IMPL_OFFSET>,
            SetValueByName::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            GetValueByName::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            GetValueSize::<Impl, IMPL_OFFSET>,
            GetSubProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Properties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RadialGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1RadialGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1RadialGradientBrushVtbl {
        unsafe extern "system" fn SetCenter<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGradientOriginOffset<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientoriginoffset: Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRadiusX<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiusx: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRadiusY<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiusy: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCenter<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientOriginOffset<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRadiusX<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRadiusY<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientStopCollection<Impl: ID2D1RadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetCenter::<Impl, IMPL_OFFSET>,
            SetGradientOriginOffset::<Impl, IMPL_OFFSET>,
            SetRadiusX::<Impl, IMPL_OFFSET>,
            SetRadiusY::<Impl, IMPL_OFFSET>,
            GetCenter::<Impl, IMPL_OFFSET>,
            GetGradientOriginOffset::<Impl, IMPL_OFFSET>,
            GetRadiusX::<Impl, IMPL_OFFSET>,
            GetRadiusY::<Impl, IMPL_OFFSET>,
            GetGradientStopCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1RadialGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRect();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RectangleGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1RectangleGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1RectangleGeometryVtbl {
        unsafe extern "system" fn GetRect<Impl: ID2D1RectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            GetRect::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1RectangleGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1RenderInfoImpl: Sized {
    fn SetInputDescription();
    fn SetOutputBuffer();
    fn SetCached();
    fn SetInstructionCountHint();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1RenderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1RenderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1RenderInfoVtbl {
        unsafe extern "system" fn SetInputDescription<Impl: ID2D1RenderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputBuffer<Impl: ID2D1RenderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCached<Impl: ID2D1RenderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInstructionCountHint<Impl: ID2D1RenderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instructioncount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetInputDescription::<Impl, IMPL_OFFSET>, SetOutputBuffer::<Impl, IMPL_OFFSET>, SetCached::<Impl, IMPL_OFFSET>, SetInstructionCountHint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1RenderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1RenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1RenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1RenderTargetVtbl {
        unsafe extern "system" fn CreateBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wicbitmapsource: ::windows::core::RawPtr, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSharedBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGradientStopCollection<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::windows::core::RawPtr, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::windows::core::RawPtr, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *const Common::D2D_SIZE_F, layer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMesh<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mesh: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawLine<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawRoundedRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillRoundedRectangle<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawEllipse<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillEllipse<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGeometry<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, strokewidth: f32, strokestyle: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillGeometry<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, opacitybrush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillMesh<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mesh: ::windows::core::RawPtr, brush: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillOpacityMask<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymask: ::windows::core::RawPtr, brush: ::windows::core::RawPtr, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawBitmap<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawText<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::windows::core::RawPtr, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawTextLayout<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: ::windows::core::RawPtr, defaultfillbrush: ::windows::core::RawPtr, options: D2D1_DRAW_TEXT_OPTIONS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawGlyphRun<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: ::windows::core::RawPtr, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextAntialiasMode<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextRenderingParams<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrenderingparams: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextRenderingParams<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTags<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTags<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopLayer<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveDrawingState<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingstateblock: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDrawingState<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingstateblock: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushAxisAlignedClip<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopAxisAlignedClip<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clearcolor: *const Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDraw<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDraw<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDpi<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDpi<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelSize<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumBitmapSize<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSupported<Impl: ID2D1RenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromWicBitmap::<Impl, IMPL_OFFSET>,
            CreateSharedBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapBrush::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateGradientStopCollection::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCompatibleRenderTarget::<Impl, IMPL_OFFSET>,
            CreateLayer::<Impl, IMPL_OFFSET>,
            CreateMesh::<Impl, IMPL_OFFSET>,
            DrawLine::<Impl, IMPL_OFFSET>,
            DrawRectangle::<Impl, IMPL_OFFSET>,
            FillRectangle::<Impl, IMPL_OFFSET>,
            DrawRoundedRectangle::<Impl, IMPL_OFFSET>,
            FillRoundedRectangle::<Impl, IMPL_OFFSET>,
            DrawEllipse::<Impl, IMPL_OFFSET>,
            FillEllipse::<Impl, IMPL_OFFSET>,
            DrawGeometry::<Impl, IMPL_OFFSET>,
            FillGeometry::<Impl, IMPL_OFFSET>,
            FillMesh::<Impl, IMPL_OFFSET>,
            FillOpacityMask::<Impl, IMPL_OFFSET>,
            DrawBitmap::<Impl, IMPL_OFFSET>,
            DrawText::<Impl, IMPL_OFFSET>,
            DrawTextLayout::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetAntialiasMode::<Impl, IMPL_OFFSET>,
            GetAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextRenderingParams::<Impl, IMPL_OFFSET>,
            GetTextRenderingParams::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            GetTags::<Impl, IMPL_OFFSET>,
            PushLayer::<Impl, IMPL_OFFSET>,
            PopLayer::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            SaveDrawingState::<Impl, IMPL_OFFSET>,
            RestoreDrawingState::<Impl, IMPL_OFFSET>,
            PushAxisAlignedClip::<Impl, IMPL_OFFSET>,
            PopAxisAlignedClip::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            SetDpi::<Impl, IMPL_OFFSET>,
            GetDpi::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelSize::<Impl, IMPL_OFFSET>,
            GetMaximumBitmapSize::<Impl, IMPL_OFFSET>,
            IsSupported::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1RenderTarget as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1ResourceImpl: Sized {
    fn GetFactory();
}
impl ID2D1ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ResourceVtbl {
        unsafe extern "system" fn GetFactory<Impl: ID2D1ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1ResourceTextureImpl: Sized {
    fn Update();
}
impl ID2D1ResourceTextureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1ResourceTextureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1ResourceTextureVtbl {
        unsafe extern "system" fn Update<Impl: ID2D1ResourceTextureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Update::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1ResourceTexture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RoundedRectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRoundedRect();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RoundedRectangleGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1RoundedRectangleGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1RoundedRectangleGeometryVtbl {
        unsafe extern "system" fn GetRoundedRect<Impl: ID2D1RoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            GetRoundedRect::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1RoundedRectangleGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SolidColorBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetColor();
    fn GetColor();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SolidColorBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SolidColorBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SolidColorBrushVtbl {
        unsafe extern "system" fn SetColor<Impl: ID2D1SolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColor<Impl: ID2D1SolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, SetOpacity::<Impl, IMPL_OFFSET>, SetTransform::<Impl, IMPL_OFFSET>, GetOpacity::<Impl, IMPL_OFFSET>, GetTransform::<Impl, IMPL_OFFSET>, SetColor::<Impl, IMPL_OFFSET>, GetColor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SolidColorBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SourceTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetRenderInfo();
    fn Draw();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SourceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SourceTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SourceTransformVtbl {
        unsafe extern "system" fn SetRenderInfo<Impl: ID2D1SourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: ID2D1SourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, MapOutputRectToInputRects::<Impl, IMPL_OFFSET>, MapInputRectsToOutputRect::<Impl, IMPL_OFFSET>, MapInvalidRect::<Impl, IMPL_OFFSET>, SetRenderInfo::<Impl, IMPL_OFFSET>, Draw::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SpriteBatchImpl: Sized + ID2D1ResourceImpl {
    fn AddSprites();
    fn SetSprites();
    fn GetSprites();
    fn GetSpriteCount();
    fn Clear();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SpriteBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SpriteBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SpriteBatchVtbl {
        unsafe extern "system" fn AddSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSprites<Impl: ID2D1SpriteBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpriteCount<Impl: ID2D1SpriteBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ID2D1SpriteBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, AddSprites::<Impl, IMPL_OFFSET>, SetSprites::<Impl, IMPL_OFFSET>, GetSprites::<Impl, IMPL_OFFSET>, GetSpriteCount::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SpriteBatch as ::windows::core::Interface>::IID
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
impl ID2D1StrokeStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1StrokeStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1StrokeStyleVtbl {
        unsafe extern "system" fn GetStartCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEndCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashCap<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMiterLimit<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineJoin<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_LINE_JOIN {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashOffset<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashStyle<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_DASH_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashesCount<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1StrokeStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetStartCap::<Impl, IMPL_OFFSET>,
            GetEndCap::<Impl, IMPL_OFFSET>,
            GetDashCap::<Impl, IMPL_OFFSET>,
            GetMiterLimit::<Impl, IMPL_OFFSET>,
            GetLineJoin::<Impl, IMPL_OFFSET>,
            GetDashOffset::<Impl, IMPL_OFFSET>,
            GetDashStyle::<Impl, IMPL_OFFSET>,
            GetDashesCount::<Impl, IMPL_OFFSET>,
            GetDashes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1StrokeStyle1Impl: Sized + ID2D1StrokeStyleImpl + ID2D1ResourceImpl {
    fn GetStrokeTransformType();
}
impl ID2D1StrokeStyle1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1StrokeStyle1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1StrokeStyle1Vtbl {
        unsafe extern "system" fn GetStrokeTransformType<Impl: ID2D1StrokeStyle1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetStartCap::<Impl, IMPL_OFFSET>,
            GetEndCap::<Impl, IMPL_OFFSET>,
            GetDashCap::<Impl, IMPL_OFFSET>,
            GetMiterLimit::<Impl, IMPL_OFFSET>,
            GetLineJoin::<Impl, IMPL_OFFSET>,
            GetDashOffset::<Impl, IMPL_OFFSET>,
            GetDashStyle::<Impl, IMPL_OFFSET>,
            GetDashesCount::<Impl, IMPL_OFFSET>,
            GetDashes::<Impl, IMPL_OFFSET>,
            GetStrokeTransformType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle1 as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1SvgAttributeImpl: Sized + ID2D1ResourceImpl {
    fn GetElement();
    fn Clone();
}
impl ID2D1SvgAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgAttributeVtbl {
        unsafe extern "system" fn GetElement<Impl: ID2D1SvgAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: ID2D1SvgAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetElement::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ID2D1SvgDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgDocumentVtbl {
        unsafe extern "system" fn SetViewportSize<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewportSize<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRoot<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRoot<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindElementById<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR, svgelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputxmlstream: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deserialize<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputxmlstream: ::windows::core::RawPtr, subtree: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePaint<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: super::super::Foundation::PWSTR, paint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStrokeDashArray<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePointCollection<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePathData<Impl: ID2D1SvgDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            SetViewportSize::<Impl, IMPL_OFFSET>,
            GetViewportSize::<Impl, IMPL_OFFSET>,
            SetRoot::<Impl, IMPL_OFFSET>,
            GetRoot::<Impl, IMPL_OFFSET>,
            FindElementById::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            Deserialize::<Impl, IMPL_OFFSET>,
            CreatePaint::<Impl, IMPL_OFFSET>,
            CreateStrokeDashArray::<Impl, IMPL_OFFSET>,
            CreatePointCollection::<Impl, IMPL_OFFSET>,
            CreatePathData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ID2D1SvgElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgElementVtbl {
        unsafe extern "system" fn GetDocument<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTagName<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTagNameLength<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTextContent<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParent<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasChildren<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referencechild: ::windows::core::RawPtr, previouschild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referencechild: ::windows::core::RawPtr, nextchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertChildBefore<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, referencechild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppendChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplaceChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateChild<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: super::super::Foundation::PWSTR, newchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAttributeSpecified<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpecifiedAttributeCount<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpecifiedAttributeName<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: super::super::Foundation::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpecifiedAttributeNameLength<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAttribute<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, namecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextValueLength<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: super::super::Foundation::PWSTR, valuecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValueLength<Impl: ID2D1SvgElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetDocument::<Impl, IMPL_OFFSET>,
            GetTagName::<Impl, IMPL_OFFSET>,
            GetTagNameLength::<Impl, IMPL_OFFSET>,
            IsTextContent::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            HasChildren::<Impl, IMPL_OFFSET>,
            GetFirstChild::<Impl, IMPL_OFFSET>,
            GetLastChild::<Impl, IMPL_OFFSET>,
            GetPreviousChild::<Impl, IMPL_OFFSET>,
            GetNextChild::<Impl, IMPL_OFFSET>,
            InsertChildBefore::<Impl, IMPL_OFFSET>,
            AppendChild::<Impl, IMPL_OFFSET>,
            ReplaceChild::<Impl, IMPL_OFFSET>,
            RemoveChild::<Impl, IMPL_OFFSET>,
            CreateChild::<Impl, IMPL_OFFSET>,
            IsAttributeSpecified::<Impl, IMPL_OFFSET>,
            GetSpecifiedAttributeCount::<Impl, IMPL_OFFSET>,
            GetSpecifiedAttributeName::<Impl, IMPL_OFFSET>,
            GetSpecifiedAttributeNameLength::<Impl, IMPL_OFFSET>,
            RemoveAttribute::<Impl, IMPL_OFFSET>,
            SetTextValue::<Impl, IMPL_OFFSET>,
            GetTextValue::<Impl, IMPL_OFFSET>,
            GetTextValueLength::<Impl, IMPL_OFFSET>,
            SetAttributeValue::<Impl, IMPL_OFFSET>,
            SetAttributeValue::<Impl, IMPL_OFFSET>,
            SetAttributeValue::<Impl, IMPL_OFFSET>,
            GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetAttributeValue::<Impl, IMPL_OFFSET>,
            GetAttributeValueLength::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgElement as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1SvgGlyphStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetFill();
    fn GetFill();
    fn SetStroke();
    fn GetStrokeDashesCount();
    fn GetStroke();
}
impl ID2D1SvgGlyphStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgGlyphStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgGlyphStyleVtbl {
        unsafe extern "system" fn SetFill<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFill<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStroke<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeDashesCount<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStroke<Impl: ID2D1SvgGlyphStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, SetFill::<Impl, IMPL_OFFSET>, GetFill::<Impl, IMPL_OFFSET>, SetStroke::<Impl, IMPL_OFFSET>, GetStrokeDashesCount::<Impl, IMPL_OFFSET>, GetStroke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgGlyphStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SvgPaintImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn SetPaintType();
    fn GetPaintType();
    fn SetColor();
    fn GetColor();
    fn SetId();
    fn GetId();
    fn GetIdLength();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SvgPaintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgPaintImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgPaintVtbl {
        unsafe extern "system" fn SetPaintType<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPaintType<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColor<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColor<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut Common::D2D1_COLOR_F) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetId<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetId<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: super::super::Foundation::PWSTR, idcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdLength<Impl: ID2D1SvgPaintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetElement::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, SetPaintType::<Impl, IMPL_OFFSET>, GetPaintType::<Impl, IMPL_OFFSET>, SetColor::<Impl, IMPL_OFFSET>, GetColor::<Impl, IMPL_OFFSET>, SetId::<Impl, IMPL_OFFSET>, GetId::<Impl, IMPL_OFFSET>, GetIdLength::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgPaint as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
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
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SvgPathDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgPathDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgPathDataVtbl {
        unsafe extern "system" fn RemoveSegmentDataAtEnd<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datacount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSegmentData<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentData<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveCommandsAtEnd<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateCommands<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCommands<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCommandsCount<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ID2D1SvgPathDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            RemoveSegmentDataAtEnd::<Impl, IMPL_OFFSET>,
            UpdateSegmentData::<Impl, IMPL_OFFSET>,
            GetSegmentData::<Impl, IMPL_OFFSET>,
            GetSegmentDataCount::<Impl, IMPL_OFFSET>,
            RemoveCommandsAtEnd::<Impl, IMPL_OFFSET>,
            UpdateCommands::<Impl, IMPL_OFFSET>,
            GetCommands::<Impl, IMPL_OFFSET>,
            GetCommandsCount::<Impl, IMPL_OFFSET>,
            CreatePathGeometry::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgPathData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPointCollectionImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemovePointsAtEnd();
    fn UpdatePoints();
    fn GetPoints();
    fn GetPointsCount();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SvgPointCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgPointCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgPointCollectionVtbl {
        unsafe extern "system" fn RemovePointsAtEnd<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdatePoints<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPoints<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPointsCount<Impl: ID2D1SvgPointCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetElement::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, RemovePointsAtEnd::<Impl, IMPL_OFFSET>, UpdatePoints::<Impl, IMPL_OFFSET>, GetPoints::<Impl, IMPL_OFFSET>, GetPointsCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgPointCollection as ::windows::core::Interface>::IID
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
impl ID2D1SvgStrokeDashArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1SvgStrokeDashArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1SvgStrokeDashArrayVtbl {
        unsafe extern "system" fn RemoveDashesAtEnd<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashescount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashes<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDashesCount<Impl: ID2D1SvgStrokeDashArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetElement::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, RemoveDashesAtEnd::<Impl, IMPL_OFFSET>, UpdateDashes::<Impl, IMPL_OFFSET>, UpdateDashes::<Impl, IMPL_OFFSET>, GetDashes::<Impl, IMPL_OFFSET>, GetDashes::<Impl, IMPL_OFFSET>, GetDashesCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1SvgStrokeDashArray as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1TessellationSinkImpl: Sized {
    fn AddTriangles();
    fn Close();
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1TessellationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TessellationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TessellationSinkVtbl {
        unsafe extern "system" fn AddTriangles<Impl: ID2D1TessellationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ID2D1TessellationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddTriangles::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1TessellationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1TransformImpl: Sized + ID2D1TransformNodeImpl {
    fn MapOutputRectToInputRects();
    fn MapInputRectsToOutputRect();
    fn MapInvalidRect();
}
#[cfg(feature = "Win32_Foundation")]
impl ID2D1TransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TransformVtbl {
        unsafe extern "system" fn MapOutputRectToInputRects<Impl: ID2D1TransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapInputRectsToOutputRect<Impl: ID2D1TransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapInvalidRect<Impl: ID2D1TransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>, MapOutputRectToInputRects::<Impl, IMPL_OFFSET>, MapInputRectsToOutputRect::<Impl, IMPL_OFFSET>, MapInvalidRect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1Transform as ::windows::core::Interface>::IID
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
impl ID2D1TransformGraphVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TransformGraphImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TransformGraphVtbl {
        unsafe extern "system" fn GetInputCount<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSingleTransformNode<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNode<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveNode<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputNode<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectNode<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromnode: ::windows::core::RawPtr, tonode: ::windows::core::RawPtr, tonodeinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectToEffectInput<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toeffectinputindex: u32, node: ::windows::core::RawPtr, tonodeinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassthroughGraph<Impl: ID2D1TransformGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectinputindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetInputCount::<Impl, IMPL_OFFSET>,
            SetSingleTransformNode::<Impl, IMPL_OFFSET>,
            AddNode::<Impl, IMPL_OFFSET>,
            RemoveNode::<Impl, IMPL_OFFSET>,
            SetOutputNode::<Impl, IMPL_OFFSET>,
            ConnectNode::<Impl, IMPL_OFFSET>,
            ConnectToEffectInput::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            SetPassthroughGraph::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1TransformGraph as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1TransformNodeImpl: Sized {
    fn GetInputCount();
}
impl ID2D1TransformNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TransformNodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TransformNodeVtbl {
        unsafe extern "system" fn GetInputCount<Impl: ID2D1TransformNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1TransformNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1TransformedGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetSourceGeometry();
    fn GetTransform();
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1TransformedGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TransformedGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TransformedGeometryVtbl {
        unsafe extern "system" fn GetSourceGeometry<Impl: ID2D1TransformedGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcegeometry: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: ID2D1TransformedGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
            GetBounds::<Impl, IMPL_OFFSET>,
            GetWidenedBounds::<Impl, IMPL_OFFSET>,
            StrokeContainsPoint::<Impl, IMPL_OFFSET>,
            FillContainsPoint::<Impl, IMPL_OFFSET>,
            CompareWithGeometry::<Impl, IMPL_OFFSET>,
            Simplify::<Impl, IMPL_OFFSET>,
            Tessellate::<Impl, IMPL_OFFSET>,
            CombineWithGeometry::<Impl, IMPL_OFFSET>,
            Outline::<Impl, IMPL_OFFSET>,
            ComputeArea::<Impl, IMPL_OFFSET>,
            ComputeLength::<Impl, IMPL_OFFSET>,
            ComputePointAtLength::<Impl, IMPL_OFFSET>,
            Widen::<Impl, IMPL_OFFSET>,
            GetSourceGeometry::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1TransformedGeometry as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1TransformedImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSource();
    fn GetProperties();
}
impl ID2D1TransformedImageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1TransformedImageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1TransformedImageSourceVtbl {
        unsafe extern "system" fn GetSource<Impl: ID2D1TransformedImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagesource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: ID2D1TransformedImageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1TransformedImageSource as ::windows::core::Interface>::IID
    }
}
pub trait ID2D1VertexBufferImpl: Sized {
    fn Map();
    fn Unmap();
}
impl ID2D1VertexBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID2D1VertexBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID2D1VertexBufferVtbl {
        unsafe extern "system" fn Map<Impl: ID2D1VertexBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID2D1VertexBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Map::<Impl, IMPL_OFFSET>, Unmap::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID2D1VertexBuffer as ::windows::core::Interface>::IID
    }
}
