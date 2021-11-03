#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkCommitRequestHandler(::windows::runtime::IUnknown);
impl IInkCommitRequestHandler {
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn OnCommitRequested(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4206797820, 45320, 17846, [169, 252, 141, 8, 250, 159, 133, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkD2DRenderer(::windows::runtime::IUnknown);
impl IInkD2DRenderer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Ink`, `Win32_Foundation`*"]
    pub unsafe fn Draw<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, fhighcontrast: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), fhighcontrast.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkD2DRenderer {
    type Vtable = IInkD2DRenderer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1082110430, 63578, 16720, [151, 207, 183, 251, 39, 79, 180, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pd2d1devicecontext: ::windows::runtime::RawPtr, pinkstrokeiterable: ::windows::runtime::RawPtr, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkD2DRenderer2(::windows::runtime::IUnknown);
impl IInkD2DRenderer2 {
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn Draw<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), ::std::mem::transmute(highcontrastadjustment)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(177593561, 17784, 19313, [178, 11, 191, 102, 77, 75, 254, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pd2d1devicecontext: ::windows::runtime::RawPtr, pinkstrokeiterable: ::windows::runtime::RawPtr, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkDesktopHost(::windows::runtime::IUnknown);
impl IInkDesktopHost {
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn QueueWorkItem<'a, Param0: ::windows::runtime::IntoParam<'a, IInkHostWorkItem>>(&self, workitem: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), workitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn CreateInkPresenter<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn CreateAndInitializeInkPresenter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, rootvisual: Param0, width: f32, height: f32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), rootvisual.into_param().abi(), ::std::mem::transmute(width), ::std::mem::transmute(height), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDesktopHost {
    type Vtable = IInkDesktopHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290262645, 43393, 16704, [161, 255, 173, 147, 37, 142, 141, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, workitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rootvisual: ::windows::runtime::RawPtr, width: f32, height: f32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkHostWorkItem(::windows::runtime::IUnknown);
impl IInkHostWorkItem {
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn Invoke(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkHostWorkItem {
    type Vtable = IInkHostWorkItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3436841626, 7032, 17970, [187, 150, 151, 128, 6, 98, 226, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInkPresenterDesktop(::windows::runtime::IUnknown);
impl IInkPresenterDesktop {
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn SetRootVisual<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, rootvisual: Param0, device: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), rootvisual.into_param().abi(), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn SetCommitRequestHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IInkCommitRequestHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Input_Ink`*"]
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1945354457, 11915, 18675, [137, 94, 32, 203, 210, 123, 114, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rootvisual: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: *mut f32, height: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: f32, height: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Input_Ink`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
impl ::std::convert::From<i32> for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INK_HIGH_CONTRAST_ADJUSTMENT {
    type Abi = Self;
}
pub const InkD2DRenderer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1078257164, 31489, 18033, [169, 124, 4, 224, 33, 10, 7, 165]);
pub const InkDesktopHost: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(103122086, 63536, 19420, [164, 210, 10, 16, 171, 6, 43, 29]);
