#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDummyHICONIncluder_Impl: Sized {
    fn Dummy(&self, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IDummyHICONIncluder {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDummyHICONIncluder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDummyHICONIncluder_Impl, const OFFSET: isize>() -> IDummyHICONIncluder_Vtbl {
        unsafe extern "system" fn Dummy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDummyHICONIncluder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Dummy(::core::mem::transmute_copy(&h1), ::core::mem::transmute_copy(&h2)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Dummy: Dummy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDummyHICONIncluder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IThumbnailExtractor_Impl: Sized {
    fn ExtractThumbnail(&self, pstg: &::core::option::Option<super::StructuredStorage::IStorage>, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn OnFileUpdated(&self, pstg: &::core::option::Option<super::StructuredStorage::IStorage>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IThumbnailExtractor {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IThumbnailExtractor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: isize>() -> IThumbnailExtractor_Vtbl {
        unsafe extern "system" fn ExtractThumbnail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExtractThumbnail(::core::mem::transmute(&pstg), ::core::mem::transmute_copy(&ullength), ::core::mem::transmute_copy(&ulheight), ::core::mem::transmute_copy(&puloutputlength), ::core::mem::transmute_copy(&puloutputheight), ::core::mem::transmute_copy(&phoutputbitmap)).into()
        }
        unsafe extern "system" fn OnFileUpdated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFileUpdated(::core::mem::transmute(&pstg)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ExtractThumbnail: ExtractThumbnail::<Identity, Impl, OFFSET>,
            OnFileUpdated: OnFileUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbnailExtractor as ::windows::core::Interface>::IID
    }
}
