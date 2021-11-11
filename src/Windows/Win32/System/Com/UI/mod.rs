#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Com_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDummyHICONIncluder(pub ::windows::core::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn Dummy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, h1: Param0, h2: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), h1.into_param().abi(), h2.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
impl ::core::convert::From<IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDummyHICONIncluder> for ::windows::core::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDummyHICONIncluder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDummyHICONIncluder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Com_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IThumbnailExtractor(pub ::windows::core::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ExtractThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstg.into_param().abi(), ::core::mem::transmute(ullength), ::core::mem::transmute(ulheight), ::core::mem::transmute(puloutputlength), ::core::mem::transmute(puloutputheight), ::core::mem::transmute(phoutputbitmap)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnFileUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
impl ::core::convert::From<IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IThumbnailExtractor> for ::windows::core::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IThumbnailExtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IThumbnailExtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstg: ::windows::core::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
