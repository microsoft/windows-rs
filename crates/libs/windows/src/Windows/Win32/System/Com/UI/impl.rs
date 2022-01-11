#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDummyHICONIncluderImpl: Sized {
    fn Dummy();
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDummyHICONIncluderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDummyHICONIncluderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDummyHICONIncluderVtbl {
        unsafe extern "system" fn Dummy<Impl: IDummyHICONIncluderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Dummy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDummyHICONIncluder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IThumbnailExtractorImpl: Sized {
    fn ExtractThumbnail();
    fn OnFileUpdated();
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IThumbnailExtractorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbnailExtractorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbnailExtractorVtbl {
        unsafe extern "system" fn ExtractThumbnail<Impl: IThumbnailExtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFileUpdated<Impl: IThumbnailExtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ExtractThumbnail::<Impl, IMPL_OFFSET>, OnFileUpdated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbnailExtractor as ::windows::core::Interface>::IID
    }
}
