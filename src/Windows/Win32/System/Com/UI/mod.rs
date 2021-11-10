#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Com_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDummyHICONIncluder(pub ::windows::runtime::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn Dummy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, h1: Param0, h2: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), h1.into_param().abi(), h2.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
impl ::core::convert::From<IDummyHICONIncluder> for ::windows::runtime::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDummyHICONIncluder> for ::windows::runtime::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDummyHICONIncluder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDummyHICONIncluder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Com_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IThumbnailExtractor(pub ::windows::runtime::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ExtractThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstg.into_param().abi(), ::core::mem::transmute(ullength), ::core::mem::transmute(ulheight), ::core::mem::transmute(puloutputlength), ::core::mem::transmute(puloutputheight), ::core::mem::transmute(phoutputbitmap)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_System_Com_UI`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnFileUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
impl ::core::convert::From<IThumbnailExtractor> for ::windows::runtime::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IThumbnailExtractor> for ::windows::runtime::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IThumbnailExtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IThumbnailExtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
