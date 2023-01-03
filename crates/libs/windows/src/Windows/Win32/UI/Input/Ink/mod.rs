#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkCommitRequestHandler(::windows::core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnCommitRequested)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IInkCommitRequestHandler, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkCommitRequestHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCommitRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer(::windows::core::IUnknown);
impl IInkD2DRenderer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<P0, P1, P2>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, fhighcontrast: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Draw)(::windows::core::Vtable::as_raw(self), pd2d1devicecontext.into().abi(), pinkstrokeiterable.into().abi(), fhighcontrast.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IInkD2DRenderer, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkD2DRenderer {
    type Vtable = IInkD2DRenderer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer2(::windows::core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Draw)(::windows::core::Vtable::as_raw(self), pd2d1devicecontext.into().abi(), pinkstrokeiterable.into().abi(), highcontrastadjustment).ok()
    }
}
::windows::core::interface_hierarchy!(IInkD2DRenderer2, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkDesktopHost(::windows::core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<P0>(&self, workitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IInkHostWorkItem>>,
    {
        (::windows::core::Vtable::vtable(self).QueueWorkItem)(::windows::core::Vtable::as_raw(self), workitem.into().abi()).ok()
    }
    pub unsafe fn CreateInkPresenter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInkPresenter)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndInitializeInkPresenter<P0, T>(&self, rootvisual: P0, width: f32, height: f32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAndInitializeInkPresenter)(::windows::core::Vtable::as_raw(self), rootvisual.into().abi(), width, height, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IInkDesktopHost, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkDesktopHost {
    type Vtable = IInkDesktopHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkDesktopHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueueWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkHostWorkItem(::windows::core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IInkHostWorkItem, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkHostWorkItem {
    type Vtable = IInkHostWorkItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkHostWorkItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccda0a9a_1b78_4632_bb96_97800662e26c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkPresenterDesktop(::windows::core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<P0, P1>(&self, rootvisual: P0, device: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetRootVisual)(::windows::core::Vtable::as_raw(self), rootvisual.into().abi(), device.into().abi()).ok()
    }
    pub unsafe fn SetCommitRequestHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IInkCommitRequestHandler>>,
    {
        (::windows::core::Vtable::vtable(self).SetCommitRequestHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnHighContrastChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IInkPresenterDesktop, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktop_Vtbl;
}
unsafe impl ::windows::core::Interface for IInkPresenterDesktop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetRootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const InkD2DRenderer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const InkDesktopHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
