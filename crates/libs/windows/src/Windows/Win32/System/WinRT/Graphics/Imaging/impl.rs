pub trait ISoftwareBitmapNativeImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for ISoftwareBitmapNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Imaging.ISoftwareBitmapNative";
}
impl ISoftwareBitmapNativeVtbl {
    pub const fn new<Impl: ISoftwareBitmapNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISoftwareBitmapNativeVtbl {
        unsafe extern "system" fn GetData<Impl: ISoftwareBitmapNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISoftwareBitmapNative>, base.5, GetData::<Impl, OFFSET>)
    }
}
pub trait ISoftwareBitmapNativeFactoryImpl: Sized {
    fn CreateFromWICBitmap();
    fn CreateFromMF2DBuffer2();
}
impl ::windows::core::RuntimeName for ISoftwareBitmapNativeFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Imaging.ISoftwareBitmapNativeFactory";
}
impl ISoftwareBitmapNativeFactoryVtbl {
    pub const fn new<Impl: ISoftwareBitmapNativeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISoftwareBitmapNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromWICBitmap<Impl: ISoftwareBitmapNativeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromWICBitmap(
                &*(&data as *const <super::super::super::super::Graphics::Imaging::IWICBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::IWICBitmap as ::windows::core::DefaultType>::DefaultType),
                &*(&forcereadonly as *const <super::super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMF2DBuffer2<Impl: ISoftwareBitmapNativeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMF2DBuffer2(
                &*(&data as *const <super::super::super::super::Media::MediaFoundation::IMF2DBuffer2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Media::MediaFoundation::IMF2DBuffer2 as ::windows::core::DefaultType>::DefaultType),
                &*(&subtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                width,
                height,
                &*(&forcereadonly as *const <super::super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&mindisplayaperture as *const <super::super::super::super::Media::MediaFoundation::MFVideoArea as ::windows::core::Abi>::Abi as *const <super::super::super::super::Media::MediaFoundation::MFVideoArea as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISoftwareBitmapNativeFactory>, base.5, CreateFromWICBitmap::<Impl, OFFSET>, CreateFromMF2DBuffer2::<Impl, OFFSET>)
    }
}
