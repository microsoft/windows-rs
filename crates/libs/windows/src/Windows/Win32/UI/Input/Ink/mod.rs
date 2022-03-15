#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkCommitRequestHandler(::windows::core::IUnknown);
impl IInkCommitRequestHandler {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn OnCommitRequested(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCommitRequested)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IInkCommitRequestHandler> for ::windows::core::IUnknown {
    fn from(value: IInkCommitRequestHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkCommitRequestHandler> for ::windows::core::IUnknown {
    fn from(value: &IInkCommitRequestHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkCommitRequestHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkCommitRequestHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkCommitRequestHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkCommitRequestHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkCommitRequestHandler {}
impl ::core::fmt::Debug for IInkCommitRequestHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCommitRequestHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnCommitRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer(::windows::core::IUnknown);
impl IInkD2DRenderer {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, fhighcontrast: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Draw)(::core::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), fhighcontrast.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IInkD2DRenderer> for ::windows::core::IUnknown {
    fn from(value: IInkD2DRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkD2DRenderer> for ::windows::core::IUnknown {
    fn from(value: &IInkD2DRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkD2DRenderer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkD2DRenderer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkD2DRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkD2DRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkD2DRenderer {}
impl ::core::fmt::Debug for IInkD2DRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkD2DRenderer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer {
    type Vtable = IInkD2DRenderer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer2(::windows::core::IUnknown);
impl IInkD2DRenderer2 {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn Draw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Draw)(::core::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), ::core::mem::transmute(highcontrastadjustment)).ok()
    }
}
impl ::core::convert::From<IInkD2DRenderer2> for ::windows::core::IUnknown {
    fn from(value: IInkD2DRenderer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkD2DRenderer2> for ::windows::core::IUnknown {
    fn from(value: &IInkD2DRenderer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkD2DRenderer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkD2DRenderer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkD2DRenderer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkD2DRenderer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkD2DRenderer2 {}
impl ::core::fmt::Debug for IInkD2DRenderer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkD2DRenderer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkDesktopHost(::windows::core::IUnknown);
impl IInkDesktopHost {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn QueueWorkItem<'a, Param0: ::windows::core::IntoParam<'a, IInkHostWorkItem>>(&self, workitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueueWorkItem)(::core::mem::transmute_copy(self), workitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn CreateInkPresenter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInkPresenter)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn CreateAndInitializeInkPresenter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, rootvisual: Param0, width: f32, height: f32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateAndInitializeInkPresenter)(::core::mem::transmute_copy(self), rootvisual.into_param().abi(), ::core::mem::transmute(width), ::core::mem::transmute(height), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IInkDesktopHost> for ::windows::core::IUnknown {
    fn from(value: IInkDesktopHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkDesktopHost> for ::windows::core::IUnknown {
    fn from(value: &IInkDesktopHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkDesktopHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkDesktopHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkDesktopHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkDesktopHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkDesktopHost {}
impl ::core::fmt::Debug for IInkDesktopHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDesktopHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkDesktopHost {
    type Vtable = IInkDesktopHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueueWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkHostWorkItem(::windows::core::IUnknown);
impl IInkHostWorkItem {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IInkHostWorkItem> for ::windows::core::IUnknown {
    fn from(value: IInkHostWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkHostWorkItem> for ::windows::core::IUnknown {
    fn from(value: &IInkHostWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkHostWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkHostWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkHostWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkHostWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkHostWorkItem {}
impl ::core::fmt::Debug for IInkHostWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkHostWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkHostWorkItem {
    type Vtable = IInkHostWorkItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccda0a9a_1b78_4632_bb96_97800662e26c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkPresenterDesktop(::windows::core::IUnknown);
impl IInkPresenterDesktop {
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn SetRootVisual<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, rootvisual: Param0, device: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRootVisual)(::core::mem::transmute_copy(self), rootvisual.into_param().abi(), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn SetCommitRequestHandler<'a, Param0: ::windows::core::IntoParam<'a, IInkCommitRequestHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCommitRequestHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnHighContrastChanged)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IInkPresenterDesktop> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterDesktop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterDesktop> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterDesktop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPresenterDesktop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkPresenterDesktop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkPresenterDesktop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterDesktop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterDesktop {}
impl ::core::fmt::Debug for IInkPresenterDesktop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterDesktop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetRootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
impl ::core::marker::Copy for INK_HIGH_CONTRAST_ADJUSTMENT {}
impl ::core::clone::Clone for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INK_HIGH_CONTRAST_ADJUSTMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INK_HIGH_CONTRAST_ADJUSTMENT").field(&self.0).finish()
    }
}
pub const InkD2DRenderer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
