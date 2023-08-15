#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkCommitRequestHandler(::windows_core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCommitRequested)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInkCommitRequestHandler, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandler_Vtbl;
}
impl ::core::clone::Clone for IInkCommitRequestHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkCommitRequestHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnCommitRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer(::windows_core::IUnknown);
impl IInkD2DRenderer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<P0, P1, P2>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, fhighcontrast: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), fhighcontrast.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInkD2DRenderer, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkD2DRenderer {
    type Vtable = IInkD2DRenderer_Vtbl;
}
impl ::core::clone::Clone for IInkD2DRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkD2DRenderer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer2(::windows_core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), pd2d1devicecontext.into_param().abi(), pinkstrokeiterable.into_param().abi(), highcontrastadjustment).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInkD2DRenderer2, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2_Vtbl;
}
impl ::core::clone::Clone for IInkD2DRenderer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkD2DRenderer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkDesktopHost(::windows_core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<P0>(&self, workitem: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IInkHostWorkItem>,
    {
        (::windows_core::Interface::vtable(self).QueueWorkItem)(::windows_core::Interface::as_raw(self), workitem.into_param().abi()).ok()
    }
    pub unsafe fn CreateInkPresenter<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateInkPresenter)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAndInitializeInkPresenter<P0, T>(&self, rootvisual: P0, width: f32, height: f32) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateAndInitializeInkPresenter)(::windows_core::Interface::as_raw(self), rootvisual.into_param().abi(), width, height, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IInkDesktopHost, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkDesktopHost {
    type Vtable = IInkDesktopHost_Vtbl;
}
impl ::core::clone::Clone for IInkDesktopHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkDesktopHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub QueueWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkHostWorkItem(::windows_core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInkHostWorkItem, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkHostWorkItem {
    type Vtable = IInkHostWorkItem_Vtbl;
}
impl ::core::clone::Clone for IInkHostWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkHostWorkItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccda0a9a_1b78_4632_bb96_97800662e26c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkPresenterDesktop(::windows_core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<P0, P1>(&self, rootvisual: P0, device: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetRootVisual)(::windows_core::Interface::as_raw(self), rootvisual.into_param().abi(), device.into_param().abi()).ok()
    }
    pub unsafe fn SetCommitRequestHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IInkCommitRequestHandler>,
    {
        (::windows_core::Interface::vtable(self).SetCommitRequestHandler)(::windows_core::Interface::as_raw(self), handler.into_param().abi()).ok()
    }
    pub unsafe fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSize)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnHighContrastChanged)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInkPresenterDesktop, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktop_Vtbl;
}
impl ::core::clone::Clone for IInkPresenterDesktop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInkPresenterDesktop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetRootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows_core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const InkD2DRenderer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const InkDesktopHost: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
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
impl ::windows_core::TypeKind for INK_HIGH_CONTRAST_ADJUSTMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INK_HIGH_CONTRAST_ADJUSTMENT").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
