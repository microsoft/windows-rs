#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct IInkCommitRequestHandler(::windows::core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkCommitRequestHandler {
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
unsafe impl ::windows::core::Interface for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IInkD2DRenderer(::windows::core::IUnknown);
impl IInkD2DRenderer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, fhighcontrast: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), fhighcontrast.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkD2DRenderer {
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
unsafe impl ::windows::core::Interface for IInkD2DRenderer {
    type Vtable = IInkD2DRendererVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRendererVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IInkD2DRenderer2(::windows::core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pd2d1devicecontext: Param0, pinkstrokeiterable: Param1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), ::core::mem::transmute(highcontrastadjustment)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkD2DRenderer2 {
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
unsafe impl ::windows::core::Interface for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IInkDesktopHost(::windows::core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<'a, Param0: ::windows::core::IntoParam<'a, IInkHostWorkItem>>(&self, workitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), workitem.into_param().abi()).ok()
    }
    pub unsafe fn CreateInkPresenter<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateAndInitializeInkPresenter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, rootvisual: Param0, width: f32, height: f32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), rootvisual.into_param().abi(), ::core::mem::transmute(width), ::core::mem::transmute(height), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkDesktopHost {
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
unsafe impl ::windows::core::Interface for IInkDesktopHost {
    type Vtable = IInkDesktopHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IInkHostWorkItem(::windows::core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkHostWorkItem {
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
unsafe impl ::windows::core::Interface for IInkHostWorkItem {
    type Vtable = IInkHostWorkItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccda0a9a_1b78_4632_bb96_97800662e26c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItemVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IInkPresenterDesktop(::windows::core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, rootvisual: Param0, device: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), rootvisual.into_param().abi(), device.into_param().abi()).ok()
    }
    pub unsafe fn SetCommitRequestHandler<'a, Param0: ::windows::core::IntoParam<'a, IInkCommitRequestHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkPresenterDesktop {
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
unsafe impl ::windows::core::Interface for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktopVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktopVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
pub type INK_HIGH_CONTRAST_ADJUSTMENT = i32;
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = 0i32;
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 1i32;
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 2i32;
pub const InkD2DRenderer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
