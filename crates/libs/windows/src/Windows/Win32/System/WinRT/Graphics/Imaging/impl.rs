pub trait ISoftwareBitmapNativeImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for ISoftwareBitmapNative {
    const NAME: &'static str = "";
}
impl ISoftwareBitmapNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapNativeVtbl {
        unsafe extern "system" fn GetData<Impl: ISoftwareBitmapNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmapNative, BASE_OFFSET>(), GetData: GetData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmapNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
pub trait ISoftwareBitmapNativeFactoryImpl: Sized {
    fn CreateFromWICBitmap();
    fn CreateFromMF2DBuffer2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for ISoftwareBitmapNativeFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl ISoftwareBitmapNativeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftwareBitmapNativeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftwareBitmapNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromWICBitmap<Impl: ISoftwareBitmapNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFromMF2DBuffer2<Impl: ISoftwareBitmapNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISoftwareBitmapNativeFactory, BASE_OFFSET>(),
            CreateFromWICBitmap: CreateFromWICBitmap::<Impl, IMPL_OFFSET>,
            CreateFromMF2DBuffer2: CreateFromMF2DBuffer2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftwareBitmapNativeFactory as ::windows::core::Interface>::IID
    }
}
