pub trait IWICImageEncoderImpl: Sized {
    fn WriteFrame();
    fn WriteFrameThumbnail();
    fn WriteThumbnail();
}
impl ::windows::core::RuntimeName for IWICImageEncoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.D2D.IWICImageEncoder";
}
impl IWICImageEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoderImpl, const OFFSET: isize>() -> IWICImageEncoderVtbl {
        unsafe extern "system" fn WriteFrame<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteFrame(
                &*(&pimage as *const <super::super::Direct2D::ID2D1Image as ::windows::core::Abi>::Abi as *const <super::super::Direct2D::ID2D1Image as ::windows::core::DefaultType>::DefaultType),
                &*(&pframeencode as *const <super::IWICBitmapFrameEncode as ::windows::core::Abi>::Abi as *const <super::IWICBitmapFrameEncode as ::windows::core::DefaultType>::DefaultType),
                &*(&pimageparameters as *const <super::WICImageParameters as ::windows::core::Abi>::Abi as *const <super::WICImageParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteFrameThumbnail<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteFrameThumbnail(
                &*(&pimage as *const <super::super::Direct2D::ID2D1Image as ::windows::core::Abi>::Abi as *const <super::super::Direct2D::ID2D1Image as ::windows::core::DefaultType>::DefaultType),
                &*(&pframeencode as *const <super::IWICBitmapFrameEncode as ::windows::core::Abi>::Abi as *const <super::IWICBitmapFrameEncode as ::windows::core::DefaultType>::DefaultType),
                &*(&pimageparameters as *const <super::WICImageParameters as ::windows::core::Abi>::Abi as *const <super::WICImageParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteThumbnail<Impl: IWICImageEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pencoder: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteThumbnail(
                &*(&pimage as *const <super::super::Direct2D::ID2D1Image as ::windows::core::Abi>::Abi as *const <super::super::Direct2D::ID2D1Image as ::windows::core::DefaultType>::DefaultType),
                &*(&pencoder as *const <super::IWICBitmapEncoder as ::windows::core::Abi>::Abi as *const <super::IWICBitmapEncoder as ::windows::core::DefaultType>::DefaultType),
                &*(&pimageparameters as *const <super::WICImageParameters as ::windows::core::Abi>::Abi as *const <super::WICImageParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICImageEncoder>, ::windows::core::GetTrustLevel, WriteFrame::<Impl, OFFSET>, WriteFrameThumbnail::<Impl, OFFSET>, WriteThumbnail::<Impl, OFFSET>)
    }
}
pub trait IWICImagingFactory2Impl: Sized + IWICImagingFactoryImpl {
    fn CreateImageEncoder();
}
impl ::windows::core::RuntimeName for IWICImagingFactory2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.D2D.IWICImagingFactory2";
}
impl IWICImagingFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory2Impl, const OFFSET: isize>() -> IWICImagingFactory2Vtbl {
        unsafe extern "system" fn CreateImageEncoder<Impl: IWICImagingFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2ddevice: ::windows::core::RawPtr, ppwicimageencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImageEncoder(&*(&pd2ddevice as *const <super::super::Direct2D::ID2D1Device as ::windows::core::Abi>::Abi as *const <super::super::Direct2D::ID2D1Device as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwicimageencoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICImagingFactory2>, ::windows::core::GetTrustLevel, CreateImageEncoder::<Impl, OFFSET>)
    }
}
