#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDummyHICONIncluder(::windows_core::IUnknown);
impl IDummyHICONIncluder {
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Dummy<P0, P1>(&self, h1: P0, h2: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::UI::WindowsAndMessaging::HICON>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).Dummy)(::windows_core::Interface::as_raw(self), h1.into_param().abi(), h2.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDummyHICONIncluder, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDummyHICONIncluder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThumbnailExtractor(::windows_core::IUnknown);
impl IThumbnailExtractor {
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ExtractThumbnail<P0>(&self, pstg: P0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::StructuredStorage::IStorage>,
    {
        (::windows_core::Interface::vtable(self).ExtractThumbnail)(::windows_core::Interface::as_raw(self), pstg.into_param().abi(), ullength, ulheight, puloutputlength, puloutputheight, phoutputbitmap).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<P0>(&self, pstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::StructuredStorage::IStorage>,
    {
        (::windows_core::Interface::vtable(self).OnFileUpdated)(::windows_core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IThumbnailExtractor, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThumbnailExtractor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub ExtractThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    ExtractThumbnail: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub OnFileUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    OnFileUpdated: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
