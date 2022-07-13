#[doc = "*Required features: `\"Win32_System_Com_UI\"`*"]
#[repr(transparent)]
pub struct IDummyHICONIncluder(::windows::core::IUnknown);
impl IDummyHICONIncluder {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Dummy<'a, P0, P1>(&self, h1: P0, h2: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::UI::WindowsAndMessaging::HICON>,
        P1: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).Dummy)(::windows::core::Interface::as_raw(self), h1.into(), h2.into()).ok()
    }
}
impl ::core::convert::From<IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDummyHICONIncluder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDummyHICONIncluder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDummyHICONIncluder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDummyHICONIncluder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_UI\"`*"]
#[repr(transparent)]
pub struct IThumbnailExtractor(::windows::core::IUnknown);
impl IThumbnailExtractor {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ExtractThumbnail<'a, P0>(&self, pstg: P0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::StructuredStorage::IStorage>>,
    {
        (::windows::core::Interface::vtable(self).ExtractThumbnail)(::windows::core::Interface::as_raw(self), pstg.into().abi(), ullength, ulheight, ::core::mem::transmute(puloutputlength), ::core::mem::transmute(puloutputheight), ::core::mem::transmute(phoutputbitmap)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<'a, P0>(&self, pstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::StructuredStorage::IStorage>>,
    {
        (::windows::core::Interface::vtable(self).OnFileUpdated)(::windows::core::Interface::as_raw(self), pstg.into().abi()).ok()
    }
}
impl ::core::convert::From<IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IThumbnailExtractor> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IThumbnailExtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IThumbnailExtractor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailExtractor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub ExtractThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    ExtractThumbnail: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub OnFileUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    OnFileUpdated: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
