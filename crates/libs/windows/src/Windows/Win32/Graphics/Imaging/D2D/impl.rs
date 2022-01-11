#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoderImpl: Sized {
    fn WriteFrame();
    fn WriteFrameThumbnail();
    fn WriteThumbnail();
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICImageEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICImageEncoderVtbl {
        unsafe extern "system" fn WriteFrame<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteFrameThumbnail<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteThumbnail<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pencoder: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, WriteFrame::<Impl, IMPL_OFFSET>, WriteFrameThumbnail::<Impl, IMPL_OFFSET>, WriteThumbnail::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImageEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2Impl: Sized + IWICImagingFactoryImpl {
    fn CreateImageEncoder();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICImagingFactory2Vtbl {
        unsafe extern "system" fn CreateImageEncoder<Impl: IWICImagingFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2ddevice: ::windows::core::RawPtr, ppwicimageencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateDecoderFromFilename::<Impl, IMPL_OFFSET>,
            CreateDecoderFromStream::<Impl, IMPL_OFFSET>,
            CreateDecoderFromFileHandle::<Impl, IMPL_OFFSET>,
            CreateComponentInfo::<Impl, IMPL_OFFSET>,
            CreateDecoder::<Impl, IMPL_OFFSET>,
            CreateEncoder::<Impl, IMPL_OFFSET>,
            CreatePalette::<Impl, IMPL_OFFSET>,
            CreateFormatConverter::<Impl, IMPL_OFFSET>,
            CreateBitmapScaler::<Impl, IMPL_OFFSET>,
            CreateBitmapClipper::<Impl, IMPL_OFFSET>,
            CreateBitmapFlipRotator::<Impl, IMPL_OFFSET>,
            CreateStream::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorTransformer::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSource::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSourceRect::<Impl, IMPL_OFFSET>,
            CreateBitmapFromMemory::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHBITMAP::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHICON::<Impl, IMPL_OFFSET>,
            CreateComponentEnumerator::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromDecoder::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode::<Impl, IMPL_OFFSET>,
            CreateQueryWriter::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromReader::<Impl, IMPL_OFFSET>,
            CreateImageEncoder::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as ::windows::core::Interface>::IID
    }
}
