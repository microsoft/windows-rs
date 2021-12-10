#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_Com_UI'*"]
#[repr(transparent)]
pub struct IDummyHICONIncluder(::windows::core::IUnknown);
impl IDummyHICONIncluder {
    #[doc = "*Required features: 'Win32_System_Com_UI', 'Win32_Graphics_Gdi', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Dummy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, h1: Param0, h2: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), h1.into_param().abi(), h2.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDummyHICONIncluder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDummyHICONIncluder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDummyHICONIncluder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDummyHICONIncluder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDummyHICONIncluder {}
unsafe impl ::windows::core::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com_UI'*"]
#[repr(transparent)]
pub struct IThumbnailExtractor(::windows::core::IUnknown);
impl IThumbnailExtractor {
    #[doc = "*Required features: 'Win32_System_Com_UI', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ExtractThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstg.into_param().abi(), ::core::mem::transmute(ullength), ::core::mem::transmute(ulheight), ::core::mem::transmute(puloutputlength), ::core::mem::transmute(puloutputheight), ::core::mem::transmute(phoutputbitmap)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com_UI', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IThumbnailExtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IThumbnailExtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IThumbnailExtractor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IThumbnailExtractor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailExtractor {}
unsafe impl ::windows::core::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
